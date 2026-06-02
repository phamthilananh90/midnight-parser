use axum::Router;
use serde::Deserialize;
use steam_protos::messages::base::cmsg_ip_address;

use crate::{
    auth::{ApiError, ApiResponse, SteamUserAuth, OK},
    post_routes,
};

pub fn routes() -> Router {
    post_routes!(Router::new(),
        enumerate_tokens,
        check_token,
        revoke_token,
        renew_access_token,
    )
}

#[derive(Deserialize)]
struct CheckTokenReq {
    token_id: String,
}

#[derive(Deserialize)]
struct RevokeTokenReq {
    token_ids: Vec<String>,
    shared_secret: Option<String>,
}

fn ip_to_json(ip: &Option<steam_protos::messages::base::CMsgIPAddress>) -> serde_json::Value {
    match ip {
        Some(addr) => match &addr.ip {
            Some(cmsg_ip_address::Ip::V4(v4)) => {
                serde_json::json!({ "v4": v4 })
            }
            Some(cmsg_ip_address::Ip::V6(v6)) => {
                serde_json::json!({ "v6": hex::encode(v6) })
            }
            None => serde_json::Value::Null,
        },
        None => serde_json::Value::Null,
    }
}

async fn enumerate_tokens(SteamUserAuth { user, .. }: SteamUserAuth) -> Result<ApiResponse<serde_json::Value>, ApiError> {
    let tokens = user.enumerate_tokens().await?;
    let value = serde_json::json!({
        "refresh_tokens": tokens.refresh_tokens.iter().map(|t| {
            serde_json::json!({
                "token_id": t.token_id,
                "token_description": t.token_description,
                "time_updated": t.time_updated,
                "platform_type": t.platform_type,
                "logged_in": t.logged_in,
                "os_platform": t.os_platform,
                "auth_type": t.auth_type,
                "gaming_device_type": t.gaming_device_type,
                "first_seen": t.first_seen.as_ref().map(|f| serde_json::json!({
                    "time": f.time,
                    "ip": ip_to_json(&f.ip),
                    "country": f.country,
                    "state": f.state,
                    "city": f.city,
                })),
                "last_seen": t.last_seen.as_ref().map(|l| serde_json::json!({
                    "time": l.time,
                    "ip": ip_to_json(&l.ip),
                    "country": l.country,
                    "state": l.state,
                    "city": l.city,
                })),
            })
        }).collect::<Vec<_>>(),
    });
    Ok(ApiResponse::ok(value))
}

async fn check_token(SteamUserAuth { user, data: req, .. }: SteamUserAuth<CheckTokenReq>) -> Result<ApiResponse<bool>, ApiError> {
    Ok(ApiResponse::ok(user.check_token_exists(&req.token_id).await?))
}

async fn revoke_token(SteamUserAuth { user, data: req, .. }: SteamUserAuth<RevokeTokenReq>) -> Result<ApiResponse<&'static str>, ApiError> {
    let ids: Vec<&str> = req.token_ids.iter().map(String::as_str).collect();
    user.revoke_tokens(&ids, req.shared_secret.as_deref()).await?;
    OK
}

async fn renew_access_token(SteamUserAuth { mut user, .. }: SteamUserAuth) -> Result<ApiResponse<&'static str>, ApiError> {
    user.renew_access_token().await?;
    OK
}
