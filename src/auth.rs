use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use steamid::SteamID;
use thiserror::Error;

/// Middleware to validate API key.
///
/// When `API_KEY` is not set, all requests are allowed through (open mode).
pub async fn api_key_auth(req: Request, next: Next) -> Result<Response, ApiError> {
    let api_key = match std::env::var("API_KEY") {
        Ok(key) if !key.is_empty() => key,
        _ => return Ok(next.run(req).await),
    };

    let auth_header = req.headers().get(header::AUTHORIZATION).and_then(|h| h.to_str().ok()).ok_or_else(|| ApiError::Unauthorized("Missing Authorization header".to_string()))?;

    if auth_header != format!("Bearer {}", api_key) && auth_header != api_key {
        return Err(ApiError::Unauthorized("Invalid API key".to_string()));
    }

    Ok(next.run(req).await)
}

/// Common auth input included in every request body.
#[derive(Debug, Deserialize, Clone)]
pub struct AuthInput {
    /// Cookie strings, e.g. ["steamLoginSecure=...", "sessionid=..."]
    pub cookies: Vec<String>,
    /// OAuth access token (optional)
    #[serde(default)]
    pub access_token: Option<String>,
    /// OAuth refresh token (optional)
    #[serde(default)]
    pub refresh_token: Option<String>,
    /// Mobile access token for 2FA operations (optional)
    #[serde(default)]
    pub mobile_access_token: Option<String>,
    /// Identity secret for confirmations (optional)
    #[serde(default)]
    #[allow(dead_code)]
    pub identity_secret: Option<String>,
    /// Shared secret for 2FA finalization (optional)
    #[serde(default)]
    #[allow(dead_code)]
    pub shared_secret: Option<String>,
}

/// Create a SteamUser client from auth input.
pub fn create_steam_user(auth: &AuthInput) -> Result<steam_user::SteamUser, ApiError> {
    let cookie_refs: Vec<&str> = auth.cookies.iter().map(|s| s.as_str()).collect();
    let mut user = steam_user::SteamUser::new(&cookie_refs)?;

    if let Some(ref token) = auth.access_token {
        user.set_access_token(token.clone());
    }
    if let Some(ref token) = auth.refresh_token {
        user.set_refresh_token(token.clone());
    }
    if let Some(ref token) = auth.mobile_access_token {
        user.set_mobile_access_token(token.clone());
    }

    Ok(user)
}

/// Convert a slice of raw SteamID64 values into `SteamID`s.
pub fn to_steam_ids(ids: &[u64]) -> Vec<SteamID> {
    ids.iter().map(|&id| SteamID::from(id)).collect()
}

/// Run a `SteamUser` operation that produces a `!Send` future.
///
/// A handful of `steam-user` methods return futures that are not `Send`, so
/// they cannot be awaited directly inside an Axum handler. This runs the work
/// on a blocking thread with its own current-thread runtime. The `SteamUser`
/// is rebuilt from `auth` inside that runtime because it is not `Send` either.
pub async fn with_blocking_user<T, F, Fut>(auth: AuthInput, f: F) -> Result<T, ApiError>
where
    F: FnOnce(steam_user::SteamUser) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = Result<T, ApiError>>,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(move || {
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
        rt.block_on(async move { f(create_steam_user(&auth)?).await })
    })
    .await?
}

/// Generic request wrapper: a nested `auth` object plus flattened endpoint data.
///
/// Wire format (matches `steam_user::remote::RemoteSteamUser` and the README):
/// `{ "auth": { "cookies": [...], ... }, "<data fields>": ... }`.
#[derive(Deserialize)]
pub struct Req<T> {
    pub auth: AuthInput,
    #[serde(flatten)]
    pub data: T,
}

/// Axum extractor that initializes a SteamUser from AuthInput in JSON body.
pub struct SteamUserAuth<T = ()> {
    pub user: steam_user::SteamUser,
    pub auth: AuthInput,
    pub data: T,
}

impl<S, T> axum::extract::FromRequest<S> for SteamUserAuth<T>
where
    T: serde::de::DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(wrapped): Json<Req<T>> = Json::from_request(req, state).await.map_err(|e| ApiError::BadRequest(format!("Invalid request body: {}", e)))?;

        let user = create_steam_user(&wrapped.auth)?;
        Ok(SteamUserAuth { user, auth: wrapped.auth, data: wrapped.data })
    }
}

/// Macro to register multiple POST routes where the path matches the function
/// name.
#[macro_export]
macro_rules! post_routes {
    ($router:expr, $($func:ident),* $(,)?) => {
        $router
            $(.route(concat!("/", stringify!($func)), axum::routing::post($func)))*
    };
}

/// Macro to register POST routes with explicit paths: `"/path" => handler`.
///
/// Paths follow the canonical contract consumed by
/// `steam_user::remote::RemoteSteamUser`, decoupled from the Rust fn names.
#[macro_export]
macro_rules! post_routes_at {
    ($router:expr, $($path:literal => $func:ident),* $(,)?) => {
        $router
            $(.route($path, axum::routing::post($func)))*
    };
}

/// Macro to register a POST route where the path matches the function name.
#[macro_export]
macro_rules! post_route {
    ($router:expr, $func:ident) => {
        $router.route(concat!("/", stringify!($func)), axum::routing::post($func))
    };
}

/// Macro to nest multiple routes with the same pattern: .nest("/name",
/// name::routes())
#[macro_export]
macro_rules! nest_routes {
    ($router:expr, $($name:ident),* $(,)?) => {
        $router
            $(.nest(concat!("/", stringify!($name)), $name::routes()))*
    };
}

/// Unified API response wrapper.
#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> ApiResponse<T> {
        ApiResponse { success: true, data: Some(data), error: None }
    }

    pub fn error(message: String) -> ApiResponse<T> {
        ApiResponse { success: false, data: None, error: Some(message) }
    }
}

impl ApiResponse<serde_json::Value> {
    /// Serialize any value into a JSON success envelope.
    pub fn json<T: Serialize>(value: T) -> Result<ApiResponse<serde_json::Value>, ApiError> {
        Ok(ApiResponse::ok(serde_json::to_value(value)?))
    }
}

/// Canonical `{ "success": true, "data": "ok" }` response for fire-and-forget
/// actions that return no data.
pub const OK: Result<ApiResponse<&'static str>, ApiError> = Ok(ApiResponse { success: true, data: Some("ok"), error: None });

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

/// API error type.
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Steam error: {0:#}")]
    Steam(#[from] steam_user::SteamUserError),

    #[allow(dead_code)]
    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Task join error: {0}")]
    Join(#[from] tokio::task::JoinError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match &self {
            ApiError::Steam(e) => match e {
                steam_user::SteamUserError::RateLimited => StatusCode::TOO_MANY_REQUESTS,
                steam_user::SteamUserError::NotLoggedIn | steam_user::SteamUserError::SessionExpired => StatusCode::UNAUTHORIZED,
                _ => StatusCode::BAD_GATEWAY,
            },
            ApiError::Internal(_) | ApiError::Join(_) | ApiError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::BadRequest(_) | ApiError::Serialization(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        };

        (status, ApiResponse::<()>::error(self.to_string())).into_response()
    }
}
