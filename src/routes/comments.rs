use axum::Router;
use serde::Deserialize;
use steamid::SteamID;

use crate::{
    auth::{ApiError, ApiResponse, SteamUserAuth, OK},
    post_routes,
};

pub fn routes() -> Router {
    post_routes!(Router::new(),
        get_my_comments,
        get_user_comments,
        post_comment,
        delete_comment,
    )
}

#[derive(Deserialize)]
struct UserCommentsReq {
    steam_id: u64,
}

#[derive(Deserialize)]
struct PostCommentReq {
    steam_id: u64,
    message: String,
}

#[derive(Deserialize)]
struct DeleteCommentReq {
    steam_id: u64,
    gidcomment: String,
}

async fn get_my_comments(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_my_comments().await?)
}

async fn get_user_comments(SteamUserAuth { user, data: req, .. }: SteamUserAuth<UserCommentsReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_user_comments(SteamID::from(req.steam_id)).await?)
}

async fn post_comment(SteamUserAuth { user, data: req, .. }: SteamUserAuth<PostCommentReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.post_comment(SteamID::from(req.steam_id), &req.message).await?)
}

async fn delete_comment(SteamUserAuth { user, data: req, .. }: SteamUserAuth<DeleteCommentReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.delete_comment(SteamID::from(req.steam_id), &req.gidcomment).await?;
    OK
}
