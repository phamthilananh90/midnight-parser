use axum::Router;
use serde::Deserialize;
use steamid::SteamID;

use crate::{
    auth::{ApiError, ApiResponse, SteamUserAuth},
    post_routes,
};

pub fn routes() -> Router {
    post_routes!(Router::new(),
        get_friend_activity,
        get_friend_activity_full,
        comment_user_received_new_game,
        rate_up_user_received_new_game,
        delete_comment_user_received_new_game,
    )
}

#[derive(Deserialize)]
struct ActivityFeedReq {
    #[serde(default)]
    start: Option<u64>,
}

#[derive(Deserialize)]
struct ActivityCommentReq {
    steam_id: u64,
    thread_id: u64,
    comment: String,
}

#[derive(Deserialize)]
struct ActivityRateUpReq {
    steam_id: u64,
    thread_id: u64,
}

#[derive(Deserialize)]
struct ActivityDeleteCommentReq {
    steam_id: u64,
    thread_id: u64,
    comment_id: String,
}

async fn get_friend_activity(SteamUserAuth { user, data: req, .. }: SteamUserAuth<ActivityFeedReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_friend_activity(req.start).await?)
}

async fn get_friend_activity_full(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_friend_activity_full().await?)
}

async fn comment_user_received_new_game(SteamUserAuth { user, data: req, .. }: SteamUserAuth<ActivityCommentReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.comment_user_received_new_game(SteamID::from(req.steam_id), req.thread_id, &req.comment).await?)
}

async fn rate_up_user_received_new_game(SteamUserAuth { user, data: req, .. }: SteamUserAuth<ActivityRateUpReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.rate_up_user_received_new_game(SteamID::from(req.steam_id), req.thread_id).await?)
}

async fn delete_comment_user_received_new_game(SteamUserAuth { user, data: req, .. }: SteamUserAuth<ActivityDeleteCommentReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.delete_comment_user_received_new_game(SteamID::from(req.steam_id), req.thread_id, &req.comment_id).await?)
}
