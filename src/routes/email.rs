use axum::Router;

use crate::{
    auth::{ApiError, ApiResponse, SteamUserAuth},
    post_routes,
};

pub fn routes() -> Router {
    post_routes!(Router::new(),
        get_account_email,
        get_current_steam_login,
    )
}

async fn get_account_email(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<String>, ApiError> {
    Ok(ApiResponse::ok(user.get_account_email().await?))
}

async fn get_current_steam_login(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<String>, ApiError> {
    Ok(ApiResponse::ok(user.get_current_steam_login().await?))
}
