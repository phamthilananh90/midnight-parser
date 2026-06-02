use axum::Router;
use serde::Deserialize;

use crate::{
    auth::{ApiError, ApiResponse, SteamUserAuth},
    post_routes,
};

pub fn routes() -> Router {
    post_routes!(Router::new(),
        get_phone_number_status,
        add_phone_number,
        confirm_phone_code_for_add,
        resend_phone_verification_code,
        get_remove_phone_number_type,
        send_account_recovery_code,
        confirm_remove_phone_number_code,
        send_confirmation_2_steam_mobile_app,
        send_confirmation_2_steam_mobile_app_final,
    )
}

#[derive(Deserialize)]
struct AddPhoneReq {
    phone: String,
}

#[derive(Deserialize)]
struct ConfirmCodeReq {
    code: String,
}

#[derive(Deserialize)]
struct SendRecoveryCodeReq {
    wizard_param: serde_json::Value,
    method: i32,
}

#[derive(Deserialize)]
struct ConfirmRemoveReq {
    wizard_param: serde_json::Value,
    code: String,
}

#[derive(Deserialize)]
struct WizardParamReq {
    wizard_param: serde_json::Value,
}

async fn get_phone_number_status(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    let status = user.get_phone_number_status().await?;
    Ok(ApiResponse::ok(serde_json::json!({ "status": status })))
}

async fn add_phone_number(SteamUserAuth { user, data: req, .. }: SteamUserAuth<AddPhoneReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.add_phone_number(&req.phone).await?)
}

async fn confirm_phone_code_for_add(SteamUserAuth { user, data: req, .. }: SteamUserAuth<ConfirmCodeReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.confirm_phone_code_for_add(&req.code).await?)
}

async fn resend_phone_verification_code(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    Ok(ApiResponse::ok(user.resend_phone_verification_code().await?))
}

async fn get_remove_phone_number_type(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_remove_phone_number_type().await?)
}

async fn send_account_recovery_code(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SendRecoveryCodeReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    Ok(ApiResponse::ok(user.send_account_recovery_code(req.wizard_param, req.method).await?))
}

async fn confirm_remove_phone_number_code(SteamUserAuth { user, data: req, .. }: SteamUserAuth<ConfirmRemoveReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    Ok(ApiResponse::ok(user.confirm_remove_phone_number_code(req.wizard_param, &req.code).await?))
}

async fn send_confirmation_2_steam_mobile_app(SteamUserAuth { user, data: req, .. }: SteamUserAuth<WizardParamReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    Ok(ApiResponse::ok(user.send_confirmation_2_steam_mobile_app(req.wizard_param).await?))
}

async fn send_confirmation_2_steam_mobile_app_final(SteamUserAuth { user, data: req, .. }: SteamUserAuth<WizardParamReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    Ok(ApiResponse::ok(user.send_confirmation_2_steam_mobile_app_final(req.wizard_param).await?))
}
