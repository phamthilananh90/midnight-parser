use axum::Router;
use serde::Deserialize;
use steamid::SteamID;

use crate::{
    auth::{to_steam_ids, ApiError, ApiResponse, SteamUserAuth, OK},
    post_routes,
    routes::common::{SteamIdReq, SteamIdsReq},
};

pub fn routes() -> Router {
    post_routes!(Router::new(),
        add_friend,
        remove_friend,
        accept_friend_request,
        ignore_friend_request,
        block_user,
        unblock_user,
        get_friends_list,
        get_friends_details,
        get_friends_details_of_user,
        search_users,
        create_instant_invite,
        follow_user,
        unfollow_user,
        get_following_list,
        get_following_list_of_user,
        get_my_friends_id_list,
        get_pending_friend_list,
        remove_friends,
        unfollow_users,
        cancel_friend_request,
        get_friends_in_common,
        get_friends_gameplay_info,
        get_friend_since,
        get_quick_invite_data,
        get_current_quick_invite_tokens,
    )
}

#[derive(Deserialize)]
struct SearchUsersReq {
    query: String,
    #[serde(default = "default_page")]
    page: u32,
}

fn default_page() -> u32 {
    1
}

#[derive(Deserialize)]
struct AppIdReq {
    app_id: u32,
}

async fn get_friends_list(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<std::collections::HashMap<String, i32>>, ApiError> {
    let friends = user.get_friends_list().await?;
    let map: std::collections::HashMap<String, i32> = friends.into_iter().map(|(id, rel)| (id.steam_id64().to_string(), rel)).collect();
    Ok(ApiResponse::ok(map))
}

async fn get_friends_details(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_friends_details().await?)
}

async fn add_friend(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SteamIdReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.add_friend(SteamID::from(req.steam_id)).await?;
    OK
}

async fn remove_friend(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SteamIdReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.remove_friend(SteamID::from(req.steam_id)).await?;
    OK
}

async fn accept_friend_request(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SteamIdReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.accept_friend_request(SteamID::from(req.steam_id)).await?;
    OK
}

async fn ignore_friend_request(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SteamIdReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.ignore_friend_request(SteamID::from(req.steam_id)).await?;
    OK
}

async fn block_user(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SteamIdReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.block_user(SteamID::from(req.steam_id)).await?;
    OK
}

async fn unblock_user(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SteamIdReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.unblock_user(SteamID::from(req.steam_id)).await?;
    OK
}

async fn search_users(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SearchUsersReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.search_users(&req.query, req.page).await?)
}

async fn create_instant_invite(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<String>, ApiError> {
    Ok(ApiResponse::ok(user.create_instant_invite().await?))
}

async fn get_friends_details_of_user(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SteamIdReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_friends_details_of_user(SteamID::from(req.steam_id)).await?)
}

async fn follow_user(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SteamIdReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.follow_user(SteamID::from(req.steam_id)).await?;
    OK
}

async fn unfollow_user(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SteamIdReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.unfollow_user(SteamID::from(req.steam_id)).await?;
    OK
}

async fn get_following_list(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_following_list().await?)
}

async fn get_following_list_of_user(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SteamIdReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_following_list_of_user(SteamID::from(req.steam_id)).await?)
}

async fn get_my_friends_id_list(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_my_friends_id_list().await?)
}

async fn get_pending_friend_list(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_pending_friend_list().await?)
}

async fn remove_friends(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SteamIdsReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.remove_friends(&to_steam_ids(&req.steam_ids)).await?;
    OK
}

async fn unfollow_users(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SteamIdsReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.unfollow_users(&to_steam_ids(&req.steam_ids)).await?;
    OK
}

async fn cancel_friend_request(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SteamIdReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.cancel_friend_request(SteamID::from(req.steam_id)).await?;
    OK
}

async fn get_friends_in_common(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SteamIdReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_friends_in_common(SteamID::from(req.steam_id)).await?)
}

async fn get_friends_gameplay_info(SteamUserAuth { user, data: req, .. }: SteamUserAuth<AppIdReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_friends_gameplay_info(req.app_id).await?)
}

async fn get_friend_since(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SteamIdReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_friend_since(SteamID::from(req.steam_id)).await?)
}

async fn get_quick_invite_data(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_quick_invite_data().await?)
}

async fn get_current_quick_invite_tokens(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_current_quick_invite_tokens().await?)
}
