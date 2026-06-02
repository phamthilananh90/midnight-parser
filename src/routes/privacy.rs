use axum::Router;
use serde::Deserialize;

use crate::{
    auth::{ApiError, ApiResponse, SteamUserAuth},
    post_routes,
};

pub fn routes() -> Router {
    post_routes!(Router::new(),
        get_privacy,
        set_privacy,
        set_all_privacy,
    )
}

#[derive(Deserialize)]
struct SetPrivacyReq {
    settings: steam_user::types::PrivacySettings,
}

#[derive(Deserialize)]
struct SetAllPrivacyReq {
    level: String,
}

async fn get_privacy(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_privacy_settings().await?)
}

async fn set_privacy(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SetPrivacyReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.set_privacy_settings(req.settings).await?)
}

async fn set_all_privacy(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SetAllPrivacyReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    let level = match req.level.as_str() {
        "public" => steam_user::types::PrivacyState::Public,
        "friends_only" => steam_user::types::PrivacyState::FriendsOnly,
        "private" => steam_user::types::PrivacyState::Private,
        _ => return Err(ApiError::BadRequest(format!("Unknown privacy level: {}", req.level))),
    };
    ApiResponse::json(user.set_all_privacy(level).await?)
}
