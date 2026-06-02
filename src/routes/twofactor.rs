use axum::Router;
use serde::Deserialize;

use crate::{
    auth::{ApiError, ApiResponse, SteamUserAuth, OK},
    post_routes,
};

pub fn routes() -> Router {
    post_routes!(Router::new(),
        get_steam_guard_status,
        enable_two_factor,
        finalize_two_factor,
        disable_two_factor,
        deauthorize_devices,
        add_authenticator,
        finalize_authenticator,
        remove_authenticator,
        enable_steam_guard_email,
        disable_steam_guard_email,
    )
}

#[derive(Deserialize)]
struct FinalizeReq {
    shared_secret: String,
    activation_code: String,
}

#[derive(Deserialize)]
struct DisableReq {
    revocation_code: String,
}

async fn get_steam_guard_status(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_steam_guard_status().await?)
}

async fn enable_two_factor(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.enable_two_factor().await?)
}

async fn finalize_two_factor(SteamUserAuth { user, data: req, .. }: SteamUserAuth<FinalizeReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.finalize_two_factor(&req.shared_secret, &req.activation_code).await?;
    OK
}

async fn disable_two_factor(SteamUserAuth { user, data: req, .. }: SteamUserAuth<DisableReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.disable_two_factor(&req.revocation_code).await?;
    OK
}

async fn deauthorize_devices(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<&'static str>, ApiError> {
    user.deauthorize_devices().await?;
    OK
}

async fn add_authenticator(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.add_authenticator().await?)
}

async fn finalize_authenticator(SteamUserAuth { user, data: req, .. }: SteamUserAuth<FinalizeReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.finalize_authenticator(&req.activation_code).await?;
    OK
}

async fn remove_authenticator(SteamUserAuth { user, data: req, .. }: SteamUserAuth<DisableReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.remove_authenticator(&req.revocation_code).await?;
    OK
}

async fn enable_steam_guard_email(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<bool>, ApiError> {
    Ok(ApiResponse::ok(user.enable_steam_guard_email().await?))
}

async fn disable_steam_guard_email(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<bool>, ApiError> {
    Ok(ApiResponse::ok(user.disable_steam_guard_email().await?))
}
