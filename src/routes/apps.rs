use axum::Router;
use serde::Deserialize;

use crate::{
    auth::{ApiError, ApiResponse, SteamUserAuth},
    post_routes,
};

pub fn routes() -> Router {
    post_routes!(Router::new(),
        get_owned_apps,
        get_owned_apps_id,
        get_owned_apps_detail,
        get_app_detail,
        fetch_csgo_stats,
        get_app_list,
        suggest_app_list,
        query_app_list,
        get_version_info,
        get_dynamic_store,
        fetch_loyalty_rewards,
        fetch_matchmaking_stats,
    )
}

#[derive(Deserialize)]
struct AppDetailReq {
    app_ids: Vec<u32>,
}

#[derive(Deserialize)]
struct SearchReq {
    term: String,
}

#[derive(Deserialize)]
struct VersionInfoReq {
    app_id: u32,
}

async fn get_owned_apps(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_owned_apps().await?)
}

async fn get_owned_apps_id(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<Vec<u32>>, ApiError> {
    Ok(ApiResponse::ok(user.get_owned_apps_id().await?))
}

async fn get_owned_apps_detail(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_owned_apps_detail().await?)
}

async fn get_app_detail(SteamUserAuth { user, data: req, .. }: SteamUserAuth<AppDetailReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_app_detail(&req.app_ids).await?)
}

async fn fetch_csgo_stats(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.fetch_csgo_account_stats().await?)
}

async fn get_app_list(SteamUserAuth { .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(steam_user::SteamUser::get_app_list().await?)
}

async fn suggest_app_list(SteamUserAuth { data: req, .. }: SteamUserAuth<SearchReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(steam_user::SteamUser::suggest_app_list(&req.term).await?)
}

async fn query_app_list(SteamUserAuth { data: req, .. }: SteamUserAuth<SearchReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(steam_user::SteamUser::query_app_list(&req.term).await?)
}

async fn get_version_info(SteamUserAuth { data: req, .. }: SteamUserAuth<VersionInfoReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(steam_user::SteamUser::get_steam_app_version_info(req.app_id).await?)
}

async fn get_dynamic_store(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_dynamic_store_user_data().await?)
}

async fn fetch_loyalty_rewards(SteamUserAuth { user, data: req, .. }: SteamUserAuth<AppDetailReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.fetch_batched_loyalty_reward_items(&req.app_ids).await?)
}

async fn fetch_matchmaking_stats(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.fetch_matchmaking_stats().await?)
}
