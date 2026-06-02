use axum::Router;
use serde::Deserialize;
use steamid::SteamID;

use crate::{
    auth::{with_blocking_user, ApiError, ApiResponse, SteamUserAuth, OK},
    post_routes,
    routes::common::SteamIdReq,
};

pub fn routes() -> Router {
    post_routes!(Router::new(),
        get_profile,
        edit_profile,
        set_persona_name,
        get_alias_history,
        clear_aliases,
        set_nickname,
        remove_nickname,
        post_status,
        select_previous_avatar,
        setup_profile,
        get_user_summary_from_xml,
        get_user_summary_from_profile,
        fetch_full_profile,
        resolve_user,
        get_avatar_history,
        upload_avatar_from_url,
    )
}

#[derive(Deserialize)]
struct GetProfileReq {
    #[serde(default)]
    steam_id: Option<u64>,
}

#[derive(Deserialize)]
struct SetNameReq {
    name: String,
}

#[derive(Deserialize)]
struct AliasReq {
    steam_id: u64,
}

#[derive(Deserialize)]
struct NicknameReq {
    steam_id: u64,
    nickname: String,
}

#[derive(Deserialize)]
struct PostStatusReq {
    text: String,
    #[serde(default)]
    app_id: Option<u32>,
}

#[derive(Deserialize)]
struct EditProfileReq {
    settings: steam_user::types::ProfileSettings,
}

#[derive(Deserialize)]
struct SelectAvatarReq {
    avatar_hash: String,
}

#[derive(Deserialize)]
struct UploadAvatarUrlReq {
    url: String,
}

async fn get_profile(SteamUserAuth { user, data: req, .. }: SteamUserAuth<GetProfileReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    let sid = req.steam_id.map(SteamID::from);
    ApiResponse::json(user.get_profile(sid).await?)
}

async fn edit_profile(SteamUserAuth { auth, data: req, .. }: SteamUserAuth<EditProfileReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    let settings = req.settings;
    with_blocking_user(auth, move |user| async move {
        user.edit_profile(settings).await?;
        Ok(())
    })
    .await?;
    OK
}

async fn set_persona_name(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SetNameReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.set_persona_name(&req.name).await?;
    OK
}

async fn get_alias_history(SteamUserAuth { user, data: req, .. }: SteamUserAuth<AliasReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_alias_history(SteamID::from(req.steam_id)).await?)
}

async fn clear_aliases(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<&'static str>, ApiError> {
    user.clear_previous_aliases().await?;
    OK
}

async fn set_nickname(SteamUserAuth { user, data: req, .. }: SteamUserAuth<NicknameReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.set_nickname(SteamID::from(req.steam_id), &req.nickname).await?;
    OK
}

async fn post_status(SteamUserAuth { user, data: req, .. }: SteamUserAuth<PostStatusReq>) -> Result<ApiResponse<u64>, ApiError> {
    Ok(ApiResponse::ok(user.post_profile_status(&req.text, req.app_id).await?))
}

async fn select_previous_avatar(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SelectAvatarReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.select_previous_avatar(&req.avatar_hash).await?;
    OK
}

async fn remove_nickname(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SteamIdReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.remove_nickname(SteamID::from(req.steam_id)).await?;
    OK
}

async fn setup_profile(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<bool>, ApiError> {
    Ok(ApiResponse::ok(user.setup_profile().await?))
}

async fn get_user_summary_from_xml(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SteamIdReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_user_summary_from_xml(SteamID::from(req.steam_id)).await?)
}

async fn get_user_summary_from_profile(SteamUserAuth { auth, data: req, .. }: SteamUserAuth<SteamIdReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    let sid = SteamID::from(req.steam_id);
    let result = with_blocking_user(auth, move |user| async move {
        let summary = user.get_user_summary_from_profile(Some(sid)).await?;
        Ok(serde_json::to_value(summary)?)
    })
    .await?;
    Ok(ApiResponse::ok(result))
}

async fn fetch_full_profile(SteamUserAuth { auth, data: req, .. }: SteamUserAuth<SteamIdReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    let sid_str = req.steam_id.to_string();
    let result = with_blocking_user(auth, move |user| async move {
        let profile = user.fetch_full_profile(&sid_str).await?;
        Ok(serde_json::to_value(profile)?)
    })
    .await?;
    Ok(ApiResponse::ok(result))
}

async fn resolve_user(SteamUserAuth { user, data: req, .. }: SteamUserAuth<SteamIdReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.resolve_user(SteamID::from(req.steam_id)).await?)
}

async fn get_avatar_history(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_avatar_history().await?)
}

async fn upload_avatar_from_url(SteamUserAuth { user, data: req, .. }: SteamUserAuth<UploadAvatarUrlReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.upload_avatar_from_url(&req.url).await?;
    OK
}
