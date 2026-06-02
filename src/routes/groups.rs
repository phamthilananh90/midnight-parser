use axum::Router;
use serde::Deserialize;
use steamid::SteamID;

use crate::{
    auth::{to_steam_ids, ApiError, ApiResponse, SteamUserAuth, OK},
    post_routes,
};

pub fn routes() -> Router {
    post_routes!(Router::new(),
        join_group,
        leave_group,
        get_group_members,
        post_group_announcement,
        kick_group_member,
        invite_user_to_group,
        invite_users_to_group,
        accept_group_invite,
        ignore_group_invite,
        get_group_overview,
        get_group_steam_id_from_vanity_url,
        get_group_info_xml,
        get_group_info_xml_full,
        get_invitable_groups,
        invite_all_friends_to_group,
    )
}

#[derive(Deserialize)]
struct GroupIdReq {
    group_id: u64,
}

#[derive(Deserialize)]
struct AnnouncementReq {
    group_id: u64,
    headline: String,
    content: String,
}

#[derive(Deserialize)]
struct KickMemberReq {
    group_id: u64,
    member_id: u64,
}

#[derive(Deserialize)]
struct InviteUserReq {
    user_id: u64,
    group_id: u64,
}

#[derive(Deserialize)]
struct InviteUsersReq {
    user_ids: Vec<u64>,
    group_id: u64,
}

#[derive(Deserialize)]
struct GroupOverviewReq {
    #[serde(default)]
    gid: Option<u64>,
    #[serde(default)]
    group_url: Option<String>,
    #[serde(default = "default_page")]
    page: i32,
    #[serde(default)]
    search_key: Option<String>,
}

fn default_page() -> i32 {
    1
}

#[derive(Deserialize)]
struct GroupVanityReq {
    vanity_url: String,
}

#[derive(Deserialize)]
struct GroupInfoXmlReq {
    #[serde(default)]
    gid: Option<u64>,
    #[serde(default)]
    group_url: Option<String>,
    #[serde(default)]
    page: Option<u32>,
}

#[derive(Deserialize)]
struct InvitableGroupsReq {
    user_steam_id: u64,
}

async fn join_group(SteamUserAuth { user, data: req, .. }: SteamUserAuth<GroupIdReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.join_group(SteamID::from(req.group_id)).await?;
    OK
}

async fn leave_group(SteamUserAuth { user, data: req, .. }: SteamUserAuth<GroupIdReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.leave_group(SteamID::from(req.group_id)).await?;
    OK
}

async fn get_group_members(SteamUserAuth { user, data: req, .. }: SteamUserAuth<GroupIdReq>) -> Result<ApiResponse<Vec<String>>, ApiError> {
    let members = user.get_group_members(SteamID::from(req.group_id)).await?;
    let ids: Vec<String> = members.iter().map(|m| m.steam_id64().to_string()).collect();
    Ok(ApiResponse::ok(ids))
}

async fn post_group_announcement(SteamUserAuth { user, data: req, .. }: SteamUserAuth<AnnouncementReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.post_group_announcement(SteamID::from(req.group_id), &req.headline, &req.content).await?;
    OK
}

async fn kick_group_member(SteamUserAuth { user, data: req, .. }: SteamUserAuth<KickMemberReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.kick_group_member(SteamID::from(req.group_id), SteamID::from(req.member_id)).await?;
    OK
}

async fn invite_user_to_group(SteamUserAuth { user, data: req, .. }: SteamUserAuth<InviteUserReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.invite_user_to_group(SteamID::from(req.user_id), SteamID::from(req.group_id)).await?;
    OK
}

async fn invite_users_to_group(SteamUserAuth { user, data: req, .. }: SteamUserAuth<InviteUsersReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.invite_users_to_group(&to_steam_ids(&req.user_ids), SteamID::from(req.group_id)).await?;
    OK
}

async fn accept_group_invite(SteamUserAuth { user, data: req, .. }: SteamUserAuth<GroupIdReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.accept_group_invite(SteamID::from(req.group_id)).await?;
    OK
}

async fn ignore_group_invite(SteamUserAuth { user, data: req, .. }: SteamUserAuth<GroupIdReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.ignore_group_invite(SteamID::from(req.group_id)).await?;
    OK
}

async fn get_group_overview(SteamUserAuth { user, data: req, .. }: SteamUserAuth<GroupOverviewReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    let options = steam_user::types::GroupOverviewOptions { gid: req.gid.map(SteamID::from), group_url: req.group_url, page: req.page, search_key: req.search_key };
    ApiResponse::json(user.get_group_overview(options).await?)
}

async fn get_group_steam_id_from_vanity_url(SteamUserAuth { user, data: req, .. }: SteamUserAuth<GroupVanityReq>) -> Result<ApiResponse<String>, ApiError> {
    Ok(ApiResponse::ok(user.get_group_steam_id_from_vanity_url(&req.vanity_url).await?))
}

async fn get_group_info_xml(SteamUserAuth { user, data: req, .. }: SteamUserAuth<GroupInfoXmlReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    let gid = req.gid.map(SteamID::from);
    ApiResponse::json(user.get_group_info_xml(gid, req.group_url.as_deref(), req.page).await?)
}

async fn get_group_info_xml_full(SteamUserAuth { user, data: req, .. }: SteamUserAuth<GroupInfoXmlReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    let gid = req.gid.map(SteamID::from);
    ApiResponse::json(user.get_group_info_xml_full(gid, req.group_url.as_deref()).await?)
}

async fn get_invitable_groups(SteamUserAuth { user, data: req, .. }: SteamUserAuth<InvitableGroupsReq>) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    ApiResponse::json(user.get_invitable_groups(SteamID::from(req.user_steam_id)).await?)
}

async fn invite_all_friends_to_group(SteamUserAuth { user, data: req, .. }: SteamUserAuth<GroupIdReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    user.invite_all_friends_to_group(SteamID::from(req.group_id)).await?;
    OK
}
