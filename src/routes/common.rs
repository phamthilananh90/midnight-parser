//! Request DTOs shared across multiple route modules.

use serde::Deserialize;

/// A request carrying a single SteamID64.
#[derive(Deserialize)]
pub struct SteamIdReq {
    pub steam_id: u64,
}

/// A request carrying a list of SteamID64 values.
#[derive(Deserialize)]
pub struct SteamIdsReq {
    pub steam_ids: Vec<u64>,
}
