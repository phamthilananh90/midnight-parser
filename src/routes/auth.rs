//! Credential login routes.
//!
//! Runs the `steam-auth` `LoginSession` handshake server-side so callers (the
//! `RemoteSteamUser` client) get Steam IP diversity on the login itself, not
//! just on Community fetches. Steam throttles logins per source IP
//! (`AccountLoginDeniedThrottle`); spreading them across the proxy fleet avoids
//! the throttle, and a throttle reply is mapped to a retryable status so the
//! client retries on another node (a different IP).
//!
//! Two-step, stateful flow:
//!
//! * [`begin_login`] runs `start_with_credentials` and does everything that
//!   needs no externally-delivered code: a codeless login, a login trusted via
//!   a stored Steam Guard machine token, or a TOTP login (the caller passes the
//!   precomputed code). Those finish inline and return the tokens. A login that
//!   requires an **emailed** code instead parks the live `LoginSession` in an
//!   in-memory store and returns a `session_id`.
//! * [`submit_guard`] takes the `session_id` plus one or more Steam Guard codes
//!   the caller fetched, submits them against the parked session, and finishes.
//!
//! IMPORTANT: this server never fetches email codes itself (no inbox / macro
//! access lives here). The caller (the bot) fetches the emailed code and hands
//! it to [`submit_guard`]. Because the parked session lives on one node, the
//! caller must send the `submit_guard` for a given `session_id` to the **same
//! node** that served its `begin_login`.

use std::{
    sync::OnceLock,
    time::{Duration, Instant},
};

use axum::{Json, Router};
use dashmap::DashMap;
use serde::Deserialize;
use steam_auth::{CredentialsDetails, EAuthSessionGuardType, EAuthTokenPlatformType, LoginSession, PollResult};

use crate::{
    auth::{ApiError, ApiResponse},
    post_routes,
};

/// Max time to wait for a login to complete once the handshake can proceed.
const LOGIN_POLL_TIMEOUT: Duration = Duration::from_secs(60);

/// How long a parked (awaiting-email-code) session is kept before eviction.
/// Generous enough to cover the caller's email-arrival wait + retries.
const SESSION_TTL: Duration = Duration::from_secs(300);

/// A `LoginSession` parked between `begin_login` and `submit_guard`.
///
/// The session is wrapped in a `tokio::sync::Mutex` so the value is `Send +
/// Sync` regardless of whether `LoginSession` is `Sync` — required for the
/// global store to be shareable. It is only ever locked after being removed
/// from the map (no contention), never across a map shard guard.
struct PendingLogin {
    session: tokio::sync::Mutex<LoginSession>,
    created: Instant,
}

type LoginStore = DashMap<String, PendingLogin>;

/// Process-global store of parked email-guard sessions.
fn store() -> &'static LoginStore {
    static STORE: OnceLock<LoginStore> = OnceLock::new();
    STORE.get_or_init(LoginStore::new)
}

/// Spawn the periodic sweeper that evicts parked sessions older than
/// [`SESSION_TTL`]. Call once at startup.
pub fn spawn_session_sweeper() {
    tokio::spawn(async {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            let now = Instant::now();
            let before = store().len();
            store().retain(|_, p| now.duration_since(p.created) < SESSION_TTL);
            let evicted = before.saturating_sub(store().len());
            if evicted > 0 {
                tracing::info!(evicted, remaining = store().len(), "swept expired login sessions");
            }
        }
    });
}

pub fn routes() -> Router {
    post_routes!(Router::new(), begin_login, submit_guard)
}

#[derive(Deserialize)]
struct BeginReq {
    account_name: String,
    password: String,
    /// Precomputed TOTP code, for accounts with a mobile authenticator.
    #[serde(default)]
    steam_guard_code: Option<String>,
    /// Stored Steam Guard machine token; when Steam trusts it the login is
    /// codeless (no email/TOTP challenge).
    #[serde(default)]
    steam_guard_machine_token: Option<String>,
}

#[derive(Deserialize)]
struct SubmitReq {
    session_id: String,
    /// One or more Steam Guard codes the caller fetched (e.g. all unread codes
    /// from the account's inbox). Tried in order; the first accepted one wins.
    codes: Vec<String>,
}

/// Begin a credential login. Completes inline unless an emailed code is needed.
async fn begin_login(Json(req): Json<BeginReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    let mut session = LoginSession::new(EAuthTokenPlatformType::KEAuthTokenPlatformTypeMobileApp, None);

    let details = CredentialsDetails {
        account_name: req.account_name,
        password: req.password,
        persistence: None,
        steam_guard_machine_token: req.steam_guard_machine_token,
        steam_guard_code: req.steam_guard_code.clone(),
    };

    let start = session.start_with_credentials(details).await?;

    if start.action_required {
        let actions = start.valid_actions.as_deref().unwrap_or(&[]);
        let needs_device = actions.iter().any(|a| a.guard_type == EAuthSessionGuardType::KEAuthSessionGuardTypeDeviceCode);
        let needs_email = actions.iter().any(|a| a.guard_type == EAuthSessionGuardType::KEAuthSessionGuardTypeEmailCode);

        if needs_device {
            // TOTP: the caller already supplied the code, so submit it inline
            // and finish — no need to round-trip.
            let code = req
                .steam_guard_code
                .as_deref()
                .ok_or_else(|| ApiError::BadRequest("device (TOTP) code required but none supplied".to_string()))?;
            session.submit_steam_guard_code(code).await?;
        } else if needs_email {
            // The emailed code is not knowable here. Park the live session and
            // hand the caller a handle; it will fetch the code and call
            // submit_guard on THIS node.
            let session_id = uuid::Uuid::now_v7().to_string();
            let steam_id = session.steam_id().map(ToString::to_string);
            store().insert(session_id.clone(), PendingLogin { session: tokio::sync::Mutex::new(session), created: Instant::now() });
            return Ok(ApiResponse::ok(serde_json::json!({
                "status": "needs_email_code",
                "session_id": session_id,
                "steam_id": steam_id,
            })));
        } else {
            return Err(ApiError::BadRequest("login requires an unsupported Steam Guard action".to_string()));
        }
    }

    // Codeless, trusted-machine, or TOTP-just-submitted → poll to completion.
    Ok(ApiResponse::ok(finish(&mut session).await?))
}

/// Submit emailed Steam Guard code(s) against a parked session and finish.
async fn submit_guard(Json(req): Json<SubmitReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    // Remove (take ownership) so we never hold a DashMap shard guard across an
    // await. A node restart between begin/submit yields "not found" here, which
    // the caller treats as: restart from begin_login.
    let pending = store()
        .remove(&req.session_id)
        .ok_or_else(|| ApiError::BadRequest("login session not found or expired".to_string()))?
        .1;
    let mut session = pending.session.into_inner();

    let mut accepted = false;
    for code in &req.codes {
        if session.submit_steam_guard_code(code).await.is_ok() {
            accepted = true;
            break;
        }
    }
    if !accepted {
        return Err(ApiError::BadRequest("all Steam Guard codes rejected".to_string()));
    }

    Ok(ApiResponse::ok(finish(&mut session).await?))
}

/// Poll the session to completion, then collect tokens + cookies into the
/// canonical login-result JSON. `new_guard_data` carries any Steam Guard
/// machine token Steam minted (so the caller can persist it for codeless
/// re-logins next time).
async fn finish(session: &mut LoginSession) -> Result<serde_json::Value, ApiError> {
    let poll_result = poll_to_completion(session).await?;
    let cookies = session.get_web_cookies().await?;
    let access_token = poll_result
        .access_token
        .clone()
        .or_else(|| session.access_token().map(str::to_owned))
        .ok_or_else(|| ApiError::BadRequest("no access token after login".to_string()))?;

    Ok(serde_json::json!({
        "status": "completed",
        "steam_id": session.steam_id().map(ToString::to_string),
        "account_name": poll_result.account_name,
        "access_token": access_token,
        "refresh_token": poll_result.refresh_token,
        "cookies": cookies,
        "new_guard_data": poll_result.new_guard_data,
    }))
}

/// Poll until the login completes or [`LOGIN_POLL_TIMEOUT`] elapses.
async fn poll_to_completion(session: &mut LoginSession) -> Result<PollResult, ApiError> {
    let deadline = Instant::now() + LOGIN_POLL_TIMEOUT;
    loop {
        if Instant::now() >= deadline {
            return Err(ApiError::BadRequest("login poll timed out".to_string()));
        }
        match session.poll().await? {
            Some(result) => return Ok(result),
            None => tokio::time::sleep(Duration::from_secs_f32(session.poll_interval())).await,
        }
    }
}
