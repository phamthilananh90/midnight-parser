use axum::Router;
use serde::Deserialize;
use steam_user::types::{AppId, ContextId};
use steamid::SteamID;

use crate::{
    auth::{ApiError, ApiResponse, SteamUserAuth},
    post_routes,
};

pub fn routes() -> Router {
    post_routes!(Router::new(),
        get_inventory,
        get_user_inventory,
        get_history,
        get_full_history,
        get_price_overview,
        get_active_inventories,
        get_inventory_trading,
        get_inventory_trading_partner,
    )
}

#[derive(Deserialize)]
struct InventoryReq {
    appid: u32,
    #[serde(default = "default_context")]
    context_id: u64,
}

fn default_context() -> u64 {
    2
}

#[derive(Deserialize)]
struct UserInventoryReq {
    steam_id: u64,
    appid: u32,
    #[serde(default = "default_context")]
    context_id: u64,
}

#[derive(Deserialize)]
struct PriceOverviewReq {
    appid: u32,
    market_hash_name: String,
}

#[derive(Deserialize)]
struct TradingPartnerReq {
    appid: u32,
    partner: u64,
    #[serde(default = "default_context")]
    context_id: u64,
}

async fn get_inventory(SteamUserAuth { user, data: req, .. }: SteamUserAuth<InventoryReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_inventory(AppId(req.appid), ContextId(req.context_id)).await?)
}

async fn get_user_inventory(SteamUserAuth { user, data: req, .. }: SteamUserAuth<UserInventoryReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_user_inventory_contents(SteamID::from(req.steam_id), AppId(req.appid), ContextId(req.context_id)).await?)
}

async fn get_history(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_inventory_history(None).await?)
}

async fn get_price_overview(SteamUserAuth { user, data: req, .. }: SteamUserAuth<PriceOverviewReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_price_overview(AppId(req.appid), &req.market_hash_name).await?)
}

async fn get_active_inventories(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_active_inventories().await?)
}

async fn get_full_history(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_full_inventory_history().await?)
}

async fn get_inventory_trading(SteamUserAuth { user, data: req, .. }: SteamUserAuth<InventoryReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    Ok(ApiResponse::ok(user.get_inventory_trading(AppId(req.appid), ContextId(req.context_id)).await?))
}

async fn get_inventory_trading_partner(SteamUserAuth { user, data: req, .. }: SteamUserAuth<TradingPartnerReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    Ok(ApiResponse::ok(user.get_inventory_trading_partner(AppId(req.appid), SteamID::from(req.partner), ContextId(req.context_id)).await?))
}
