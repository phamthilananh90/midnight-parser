use axum::Router;
use serde::Deserialize;

use crate::{
    auth::{ApiError, ApiResponse, SteamUserAuth, OK},
    post_routes,
};

pub fn routes() -> Router {
    post_routes!(Router::new(),
        get_account_details,
        get_wallet_balance,
        get_amount_spent,
        get_purchase_history,
        redeem_wallet_code,
        parental_unlock,
    )
}

#[derive(Deserialize)]
struct RedeemWalletCodeReq {
    code: String,
}

#[derive(Deserialize)]
struct ParentalUnlockReq {
    pin: String,
}

async fn get_account_details(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_account_details().await?)
}

async fn get_wallet_balance(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_steam_wallet_balance().await?)
}

async fn get_amount_spent(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<String>, ApiError> {
    Ok(ApiResponse::ok(user.get_amount_spent_on_steam().await?))
}

async fn get_purchase_history(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_purchase_history().await?)
}

async fn redeem_wallet_code(SteamUserAuth { user, data: req, .. }: SteamUserAuth<RedeemWalletCodeReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.redeem_wallet_code(&req.code).await?)
}

async fn parental_unlock(SteamUserAuth { user, data: req, .. }: SteamUserAuth<ParentalUnlockReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.parental_unlock(&req.pin).await?;
    OK
}
