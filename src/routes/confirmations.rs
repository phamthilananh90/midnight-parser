use axum::Router;
use serde::Deserialize;

use crate::{
    auth::{ApiError, ApiResponse, SteamUserAuth, OK},
    post_routes,
};

pub fn routes() -> Router {
    post_routes!(Router::new(),
        get_confirmations,
        accept_confirmation,
        deny_confirmation,
    )
}

#[derive(Deserialize)]
struct ListConfirmationsReq {
    identity_secret: String,
    tag: Option<String>,
}

#[derive(Deserialize)]
struct ConfirmationActionReq {
    identity_secret: String,
    object_id: u64,
}

async fn get_confirmations(SteamUserAuth { user, data: req, .. }: SteamUserAuth<ListConfirmationsReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_confirmations(&req.identity_secret, req.tag.as_deref()).await?)
}

async fn accept_confirmation(SteamUserAuth { user, data: req, .. }: SteamUserAuth<ConfirmationActionReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.accept_confirmation_for_object(&req.identity_secret, req.object_id).await?;
    OK
}

async fn deny_confirmation(SteamUserAuth { user, data: req, .. }: SteamUserAuth<ConfirmationActionReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.deny_confirmation_for_object(&req.identity_secret, req.object_id).await?;
    OK
}
