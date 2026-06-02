use axum::Router;
use serde::Deserialize;
use steam_user::types::{Amount, AppId, AssetId, ContextId, ItemNameId, PriceCents};

use crate::{
    auth::{ApiError, ApiResponse, SteamUserAuth},
    post_routes,
};

pub fn routes() -> Router {
    post_routes!(Router::new(),
        get_my_listings,
        get_market_history,
        sell_item,
        remove_listing,
        get_gem_value,
        turn_into_gems,
        get_booster_catalog,
        create_booster,
        open_booster,
        get_restrictions,
        get_market_apps,
        get_item_nameid,
        get_orders_histogram,
    )
}

#[derive(Deserialize)]
struct PaginatedReq {
    #[serde(default)]
    start: u32,
    #[serde(default = "default_count")]
    count: u32,
}

fn default_count() -> u32 {
    100
}

#[derive(Deserialize)]
struct SellReq {
    appid: u32,
    contextid: u32,
    assetid: u64,
    amount: u32,
    price: u32,
}

#[derive(Deserialize)]
struct RemoveListingReq {
    listing_id: String,
}

#[derive(Deserialize)]
struct GemValueReq {
    appid: u32,
    assetid: u64,
}

#[derive(Deserialize)]
struct TurnIntoGemsReq {
    appid: u32,
    assetid: u64,
    expected_value: u32,
}

#[derive(Deserialize)]
struct CreateBoosterReq {
    appid: u32,
    #[serde(default)]
    use_untradable_gems: bool,
}

#[derive(Deserialize)]
struct OpenBoosterReq {
    appid: u32,
    assetid: u64,
}

#[derive(Deserialize)]
struct ItemNameIdReq {
    app_id: u32,
    market_hash_name: String,
}

#[derive(Deserialize)]
struct OrdersHistogramReq {
    item_nameid: u64,
    #[serde(default = "default_country")]
    country: String,
    #[serde(default = "default_currency")]
    currency: u32,
}

fn default_country() -> String {
    "US".to_string()
}
fn default_currency() -> u32 {
    1
}

async fn get_my_listings(SteamUserAuth { user, data: req, .. }: SteamUserAuth<PaginatedReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    let (listings, assets, total_count) = user.get_my_listings(req.start, req.count).await?;
    Ok(ApiResponse::ok(serde_json::json!({ "listings": listings, "assets": assets, "total_count": total_count })))
}

async fn get_market_history(SteamUserAuth { user, data: req, .. }: SteamUserAuth<PaginatedReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_market_history(req.start, req.count).await?)
}

async fn sell_item(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SellReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.sell_item(AppId(req.appid), ContextId(u64::from(req.contextid)), AssetId(req.assetid), Amount(req.amount), PriceCents(req.price)).await?)
}

async fn remove_listing(SteamUserAuth { user, data: req, .. }: SteamUserAuth<RemoveListingReq>) -> Result<ApiResponse<bool>, ApiError> {
    Ok(ApiResponse::ok(user.remove_listing(&req.listing_id).await?))
}

async fn get_gem_value(SteamUserAuth { user, data: req, .. }: SteamUserAuth<GemValueReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_gem_value(AppId(req.appid), AssetId(req.assetid)).await?)
}

async fn turn_into_gems(SteamUserAuth { user, data: req, .. }: SteamUserAuth<TurnIntoGemsReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.turn_item_into_gems(AppId(req.appid), AssetId(req.assetid), req.expected_value).await?)
}

async fn get_booster_catalog(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_booster_pack_catalog().await?)
}

async fn create_booster(SteamUserAuth { user, data: req, .. }: SteamUserAuth<CreateBoosterReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.create_booster_pack(AppId(req.appid), req.use_untradable_gems).await?)
}

async fn open_booster(SteamUserAuth { user, data: req, .. }: SteamUserAuth<OpenBoosterReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.open_booster_pack(AppId(req.appid), AssetId(req.assetid)).await?)
}

async fn get_restrictions(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_market_restrictions().await?)
}

async fn get_market_apps(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_market_apps().await?)
}

async fn get_item_nameid(SteamUserAuth { user, data: req, .. }: SteamUserAuth<ItemNameIdReq>) -> Result<ApiResponse<u64>, ApiError> {
    Ok(ApiResponse::ok(user.get_item_nameid(AppId(req.app_id), &req.market_hash_name).await?.get()))
}

async fn get_orders_histogram(SteamUserAuth { user, data: req, .. }: SteamUserAuth<OrdersHistogramReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_item_orders_histogram(ItemNameId(req.item_nameid), &req.country, req.currency).await?)
}
