use axum::Router;
use serde::Deserialize;

use crate::{
    auth::{ApiError, ApiResponse, SteamUserAuth, OK},
    post_routes,
};

pub fn routes() -> Router {
    post_routes!(Router::new(),
        get_trade_url,
        get_trade_offers,
        accept_trade_offer,
        decline_trade_offer,
        send_trade_offer,
    )
}

#[derive(Deserialize)]
struct AcceptTradeReq {
    trade_offer_id: u64,
    #[serde(default)]
    partner_steam_id: Option<String>,
}

#[derive(Deserialize)]
struct DeclineTradeReq {
    trade_offer_id: u64,
}

#[derive(Deserialize)]
struct SendTradeOfferReq {
    trade_url: String,
    my_assets: Vec<steam_user::types::TradeOfferAsset>,
    their_assets: Vec<steam_user::types::TradeOfferAsset>,
    #[serde(default)]
    message: String,
}

async fn get_trade_url(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<Option<String>>, ApiError> {
    Ok(ApiResponse::ok(user.get_trade_url().await?))
}

async fn get_trade_offers(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_trade_offer().await?)
}

async fn accept_trade_offer(SteamUserAuth { user, data: req, .. }: SteamUserAuth<AcceptTradeReq>) -> Result<ApiResponse<String>, ApiError> {
    Ok(ApiResponse::ok(user.accept_trade_offer(req.trade_offer_id, req.partner_steam_id).await?))
}

async fn decline_trade_offer(SteamUserAuth { user, data: req, .. }: SteamUserAuth<DeclineTradeReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.decline_trade_offer(req.trade_offer_id).await?;
    OK
}

async fn send_trade_offer(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SendTradeOfferReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.send_trade_offer(&req.trade_url, req.my_assets, req.their_assets, &req.message).await?)
}
