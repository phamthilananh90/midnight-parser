use axum::Router;
use serde::Deserialize;

use crate::{
    auth::{ApiError, ApiResponse, SteamUserAuth},
    post_routes,
};

pub fn routes() -> Router {
    post_routes!(Router::new(),
        get_player_reports,
        add_free_license,
        add_sub_free_license,
        redeem_points,
        get_help_requests,
        get_help_request_detail,
        get_match_history,
    )
}

#[derive(Deserialize)]
struct PackageIdReq {
    package_id: u32,
}

#[derive(Deserialize)]
struct SubIdReq {
    sub_id: u32,
}

#[derive(Deserialize)]
struct RedeemPointsReq {
    definition_id: u32,
}

#[derive(Deserialize)]
struct HelpRequestDetailReq {
    id: String,
}

#[derive(Deserialize)]
struct MatchHistoryReq {
    match_type: String,
    #[serde(default)]
    token: Option<String>,
}

async fn get_player_reports(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_player_reports().await?)
}

async fn add_free_license(SteamUserAuth { user, data: req, .. }: SteamUserAuth<PackageIdReq>) -> Result<ApiResponse<bool>, ApiError> {
    Ok(ApiResponse::ok(user.add_free_license(req.package_id).await?))
}

async fn add_sub_free_license(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SubIdReq>) -> Result<ApiResponse<bool>, ApiError> {
    Ok(ApiResponse::ok(user.add_sub_free_license(req.sub_id).await?))
}

async fn redeem_points(SteamUserAuth { user, data: req, .. }: SteamUserAuth<RedeemPointsReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    let result = user.redeem_points(req.definition_id).await?;
    ApiResponse::json(format!("{:?}", result))
}

async fn get_help_requests(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_help_requests().await?)
}

async fn get_help_request_detail(SteamUserAuth { user, data: req, .. }: SteamUserAuth<HelpRequestDetailReq>) -> Result<ApiResponse<String>, ApiError> {
    Ok(ApiResponse::ok(user.get_help_request_detail(&req.id).await?))
}

async fn get_match_history(SteamUserAuth { user, data: req, .. }: SteamUserAuth<MatchHistoryReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    let match_type: steam_user::MatchHistoryType = req
        .match_type
        .parse()
        .map_err(|_| ApiError::BadRequest(format!("Unknown match_type: {}", req.match_type)))?;
    ApiResponse::json(user.get_match_history(match_type, req.token.as_deref()).await?)
}
