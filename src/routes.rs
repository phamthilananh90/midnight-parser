pub mod account;
pub mod activity;
pub mod common;
pub mod apps;
pub mod comments;
pub mod confirmations;
pub mod email;
pub mod extra;
pub mod friends;
pub mod groups;
pub mod inventory;
pub mod market;
pub mod misc;
pub mod phone;
pub mod privacy;
pub mod profile;
pub mod status;
pub mod tokens;
pub mod trade;
pub mod twofactor;

use axum::{middleware, Router};

/// Build the full API router with all service routes nested under `/api`.
pub fn all_routes() -> Router {
    let api_routes = crate::nest_routes!(Router::new(), account, activity, apps, comments, confirmations, email, extra, friends, groups, inventory, market, misc, phone, privacy, profile, tokens, trade, twofactor,).layer(middleware::from_fn(crate::auth::api_key_auth));

    Router::new().nest("/api/status", status::routes()).nest("/api", api_routes)
}
