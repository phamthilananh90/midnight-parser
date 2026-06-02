use axum::Router;
use serde::Deserialize;

use crate::{
    auth::{with_blocking_user, ApiError, ApiResponse, SteamUserAuth, OK},
    post_routes,
};

pub fn routes() -> Router {
    post_routes!(Router::new(),
        logged_in,
        get_notifications,
        get_web_api_key,
        resolve_vanity_url,
        get_client_js_token,
        revoke_web_api_key,
    )
}

#[derive(Deserialize)]
struct WebApiKeyReq {
    #[serde(default = "default_domain")]
    domain: String,
}

fn default_domain() -> String {
    "localhost".to_string()
}

#[derive(Deserialize)]
struct ResolveVanityReq {
    api_key: String,
    vanity_name: String,
}

async fn logged_in(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    let (is_logged_in, family_view) = user.logged_in().await?;
    Ok(ApiResponse::ok(serde_json::json!({
        "logged_in": is_logged_in,
        "family_view_restricted": family_view,
    })))
}

async fn get_notifications(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    let notifs = user.get_notifications().await?;
    Ok(ApiResponse::ok(serde_json::json!({
        "trades": notifs.trades,
        "game_turns": notifs.game_turns,
        "moderator_messages": notifs.moderator_messages,
        "comments": notifs.comments,
        "items": notifs.items,
        "invites": notifs.invites,
        "gifts": notifs.gifts,
        "chat": notifs.chat,
        "help_request_replies": notifs.help_request_replies,
        "account_alerts": notifs.account_alerts,
    })))
}

async fn get_web_api_key(SteamUserAuth { auth, data: req, .. }: SteamUserAuth<WebApiKeyReq>) -> Result<ApiResponse<String>, ApiError> {
    let domain = req.domain;
    let key = with_blocking_user(auth, move |user| async move { Ok(user.get_web_api_key(&domain).await?) }).await?;
    Ok(ApiResponse::ok(key))
}

async fn resolve_vanity_url(SteamUserAuth { user, data: req, .. }: SteamUserAuth<ResolveVanityReq>) -> Result<ApiResponse<String>, ApiError> {
    let steam_id = user.resolve_vanity_url(&req.api_key, &req.vanity_name).await?;
    Ok(ApiResponse::ok(steam_id.steam_id64().to_string()))
}

async fn get_client_js_token(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    let token = user.get_client_js_token().await?;
    Ok(ApiResponse::ok(serde_json::json!({
        "logged_in": token.logged_in,
        "steamid": token.steamid,
        "account_name": token.account_name,
        "token": token.token,
    })))
}

async fn revoke_web_api_key(SteamUserAuth { auth, .. }: SteamUserAuth) -> Result<ApiResponse<&'static str>, ApiError> {
    with_blocking_user(auth, move |user| async move {
        user.revoke_web_api_key().await?;
        Ok(())
    })
    .await?;
    OK
}
