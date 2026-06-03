---
title: Rust Axum Server
emoji: 🦀
colorFrom: blue
colorTo: red
sdk: docker
app_port: 3000
pinned: false
---


# steam-user-api

REST API proxy for the [`steam-user`](https://crates.io/crates/steam-user) crate, built with [Axum](https://github.com/tokio-rs/axum). Exposes Steam account operations as JSON endpoints that can be consumed by any HTTP client.

## Features

- **70+ API endpoints** across 12 route modules
- **Unified JSON responses** with consistent `{ success, data, error }` envelope
- **Cookie + token-based auth** — pass Steam cookies and optional OAuth/mobile tokens per request
- **CORS enabled** — ready for cross-origin browser clients
- **Configurable port** via the `PORT` environment variable (default: `3000`)

## Quick Start

```bash
# Build
cargo build -p steam-user-api

# Run (default port 3000)
cargo run -p steam-user-api

# Run on a custom port
PORT=8080 cargo run -p steam-user-api
```

The server starts at `http://0.0.0.0:<PORT>`.

## Authentication

Every endpoint is a **POST** request that accepts a JSON body containing an `auth` object:

```json
{
  "auth": {
    "cookies": ["steamLoginSecure=...", "sessionid=..."],
    "access_token": "optional",
    "refresh_token": "optional",
    "mobile_access_token": "optional",
    "identity_secret": "optional",
    "shared_secret": "optional"
  }
}
```

| Field                | Required | Description                              |
|----------------------|----------|------------------------------------------|
| `cookies`            | ✅       | Array of Steam cookie strings            |
| `access_token`       | ❌       | OAuth access token                       |
| `refresh_token`      | ❌       | OAuth refresh token                      |
| `mobile_access_token`| ❌       | Mobile access token for 2FA operations   |
| `identity_secret`    | ❌       | Identity secret for confirmations        |
| `shared_secret`      | ❌       | Shared secret for 2FA finalization       |

## Response Format

All responses follow a uniform structure:

```json
// Success
{ "success": true, "data": { ... } }

// Error
{ "success": false, "error": "Error description" }
```

HTTP status codes: `200` on success, `400` for bad requests, `502` for Steam errors, `500` for internal errors.

## API Endpoints

Each route's path is the handler **function name**, i.e. `POST /api/<module>/<function_name>`. The tables below list the path segment per module.

### Account — `/api/account`

| Endpoint                      | Extra Fields | Description                     |
|-------------------------------|--------------|---------------------------------|
| `POST /get_account_details`   | —            | Get account details             |
| `POST /get_wallet_balance`    | —            | Get Steam wallet balance        |
| `POST /get_amount_spent`      | —            | Get total amount spent on Steam |
| `POST /get_purchase_history`  | —            | Get purchase history            |
| `POST /redeem_wallet_code`    | `code`       | Redeem a wallet code            |
| `POST /parental_unlock`       | `pin`        | Unlock parental controls        |

---

### Apps — `/api/apps`

| Endpoint                    | Extra Fields     | Description                   |
|-----------------------------|------------------|-------------------------------|
| `POST /get_owned_apps`      | —                | Get owned apps                |
| `POST /get_owned_apps_id`   | —                | Get owned app IDs only        |
| `POST /get_owned_apps_detail`| —               | Get owned apps with details   |
| `POST /get_app_detail`      | `app_ids: [u32]` | Get details for specific apps |
| `POST /fetch_csgo_stats`    | —                | Fetch CS:GO account stats     |
| `POST /get_app_list`        | —                | Get full Steam app list       |
| `POST /suggest_app_list`    | `term`           | Suggest apps by search term   |
| `POST /query_app_list`      | `term`           | Search apps by term           |
| `POST /get_version_info`    | `app_id: u32`    | Get app version info          |
| `POST /get_dynamic_store`   | —                | Get dynamic store user data   |
| `POST /fetch_loyalty_rewards`| `app_ids: [u32]`| Batched loyalty reward items  |

---

### Confirmations — `/api/confirmations`

| Endpoint                    | Extra Fields                  | Description                |
|-----------------------------|-------------------------------|----------------------------|
| `POST /get_confirmations`   | `identity_secret`, `tag?`     | List pending confirmations |
| `POST /accept_confirmation` | `identity_secret`, `object_id`| Accept a confirmation      |
| `POST /deny_confirmation`   | `identity_secret`, `object_id`| Deny a confirmation        |

---

### Friends — `/api/friends`

| Endpoint                      | Extra Fields              | Description             |
|-------------------------------|---------------------------|-------------------------|
| `POST /get_friends_list`      | —                         | Get friends list        |
| `POST /get_friends_details`   | —                         | Get friends with details|
| `POST /add_friend`            | `steam_id: u64`           | Send a friend request   |
| `POST /remove_friend`         | `steam_id: u64`           | Remove a friend         |
| `POST /accept_friend_request` | `steam_id: u64`           | Accept a friend request |
| `POST /ignore_friend_request` | `steam_id: u64`           | Ignore a friend request |
| `POST /block_user`            | `steam_id: u64`           | Block a user            |
| `POST /unblock_user`          | `steam_id: u64`           | Unblock a user          |
| `POST /search_users`          | `query`, `page?` (def: 1) | Search for users        |
| `POST /create_instant_invite` | —                         | Create an instant invite|

> Also available: `/get_friends_details_of_user`, `/follow_user`, `/unfollow_user`, `/get_following_list`, `/get_following_list_of_user`, `/get_my_friends_id_list`, `/get_pending_friend_list`, `/remove_friends` (`steam_ids: [u64]`), `/unfollow_users` (`steam_ids: [u64]`), `/cancel_friend_request`, `/get_friends_in_common`.

---

### Inventory — `/api/inventory`

| Endpoint                  | Extra Fields                                 | Description                  |
|---------------------------|----------------------------------------------|------------------------------|
| `POST /get_inventory`     | `appid: u32`, `context_id?` (def: 2)         | Get own inventory            |
| `POST /get_user_inventory`| `steam_id: u64`, `appid: u32`, `context_id?` | Get another user's inventory |
| `POST /get_history`       | —                                            | Get inventory history        |
| `POST /get_full_history`  | —                                            | Get full inventory history   |
| `POST /get_price_overview`| `appid: u32`, `market_hash_name`             | Get item price overview      |
| `POST /get_active_inventories`| —                                        | Get active inventories       |

> Also available: `/get_inventory_trading`, `/get_inventory_trading_partner`.

---

### Market — `/api/market`

| Endpoint                   | Extra Fields                                                     | Description             |
|----------------------------|------------------------------------------------------------------|-------------------------|
| `POST /get_my_listings`    | `start?` (def: 0), `count?` (def: 100)                          | Get own market listings |
| `POST /get_market_history` | `start?` (def: 0), `count?` (def: 100)                          | Get market history      |
| `POST /sell_item`          | `appid`, `contextid`, `assetid: u64`, `amount: u32`, `price: u32`| List an item for sale  |
| `POST /remove_listing`     | `listing_id`                                                    | Remove a listing        |
| `POST /get_gem_value`      | `appid: u32`, `assetid: u64`                                    | Get item's gem value    |
| `POST /turn_into_gems`     | `appid`, `assetid: u64`, `expected_value: u32`                  | Turn item into gems     |
| `POST /get_booster_catalog`| —                                                               | Get booster pack catalog|
| `POST /create_booster`     | `appid: u32`, `use_untradable_gems?`                            | Create a booster pack   |
| `POST /open_booster`       | `appid: u32`, `assetid: u64`                                    | Open a booster pack     |
| `POST /get_restrictions`   | —                                                                | Get market restrictions |
| `POST /get_market_apps`    | —                                                                | Get apps on the market  |

> Also available: `/get_item_nameid` (`app_id: u32`, `market_hash_name`), `/get_orders_histogram` (`item_nameid: u64`, `country?`, `currency?`).

---

### Misc — `/api/misc`

| Endpoint                   | Extra Fields                  | Description                      |
|----------------------------|-------------------------------|----------------------------------|
| `POST /logged_in`          | —                             | Check login & family view status |
| `POST /get_notifications`  | —                             | Get notification counts          |
| `POST /get_web_api_key`    | `domain?` (def: "localhost")  | Get/register a Web API key       |
| `POST /resolve_vanity_url` | `api_key`, `vanity_name`      | Resolve a vanity URL to SteamID  |
| `POST /get_client_js_token`| —                             | Get client JS token              |
| `POST /revoke_web_api_key` | —                             | Revoke the Web API key           |

---

### Privacy — `/api/privacy`

| Endpoint              | Extra Fields | Description                       |
|-----------------------|--------------|-----------------------------------|
| `POST /get_privacy`   | —            | Get privacy settings              |
| `POST /set_privacy`   | `settings`   | Update privacy settings           |
| `POST /set_all_privacy`| `level`     | Set all to `public`/`friends_only`/`private` |

---

### Profile — `/api/profile`

| Endpoint                       | Extra Fields                | Description                 |
|--------------------------------|-----------------------------|-----------------------------|
| `POST /get_profile`            | `steam_id?: u64`            | Get profile (self or other) |
| `POST /edit_profile`           | `settings: ProfileSettings` | Edit profile settings       |
| `POST /set_persona_name`       | `name`                      | Set persona name            |
| `POST /get_alias_history`      | `steam_id: u64`             | Get alias history           |
| `POST /clear_aliases`          | —                           | Clear own previous aliases  |
| `POST /set_nickname`           | `steam_id: u64`, `nickname` | Set nickname for a friend   |
| `POST /post_status`            | `text`, `app_id?: u32`      | Post a profile status       |
| `POST /select_previous_avatar` | `avatar_hash`               | Select a previous avatar    |

> Also available: `/remove_nickname`, `/setup_profile`, `/get_user_summary_from_xml`, `/get_user_summary_from_profile`, `/fetch_full_profile`, `/resolve_user`, `/get_avatar_history`, `/upload_avatar_from_url` (`url`).

---

### Tokens — `/api/tokens`

| Endpoint                  | Extra Fields                             | Description                    |
|---------------------------|------------------------------------------|--------------------------------|
| `POST /enumerate_tokens`  | —                                        | List all refresh tokens        |
| `POST /check_token`       | `token_id`                               | Check if a token exists        |
| `POST /revoke_token`      | `token_ids: [String]`, `shared_secret?`  | Revoke tokens (`shared_secret` optional; required by Steam for some tokens) |
| `POST /renew_access_token`| —                                        | Renew the current access token |

---

### Trade — `/api/trade`

| Endpoint                | Extra Fields                               | Description           |
|-------------------------|--------------------------------------------|-----------------------|
| `POST /get_trade_url`   | —                                          | Get trade URL         |
| `POST /get_trade_offers`| —                                          | Get trade offers      |
| `POST /accept_trade_offer`| `trade_offer_id: u64`, `partner_steam_id?`| Accept a trade offer |
| `POST /decline_trade_offer`| `trade_offer_id: u64`                   | Decline a trade offer |
| `POST /send_trade_offer`| `trade_url`, `my_assets`, `their_assets`, `message?` | Send a trade offer |

---

### Two-Factor — `/api/twofactor`

| Endpoint                       | Extra Fields                       | Description                   |
|--------------------------------|------------------------------------|-------------------------------|
| `POST /get_steam_guard_status` | —                                  | Get Steam Guard status        |
| `POST /enable_two_factor`      | —                                  | Enable two-factor auth        |
| `POST /finalize_two_factor`    | `shared_secret`, `activation_code` | Finalize 2FA setup            |
| `POST /disable_two_factor`     | `revocation_code`                  | Disable two-factor auth       |
| `POST /deauthorize_devices`    | —                                  | Deauthorize all other devices |

> Also available: `/add_authenticator`, `/finalize_authenticator` (`activation_code`), `/remove_authenticator` (`revocation_code`), `/enable_steam_guard_email`, `/disable_steam_guard_email`.

---

> **Other modules** (not tabled above) follow the same `function_name` path convention: `/api/activity/*`, `/api/comments/*`, `/api/email/*`, `/api/extra/*`, `/api/groups/*`, `/api/phone/*`. See the handler names in `src/routes/<module>.rs`, or the typed client in `steam_user::remote::RemoteSteamUser`.

## Example

```bash
curl -X POST http://localhost:3000/api/account/get_wallet_balance \
  -H "Content-Type: application/json" \
  -d '{
    "auth": {
      "cookies": ["steamLoginSecure=76561198...%7C%7C..."]
    }
  }'
```

Response:

```json
{
  "success": true,
  "data": {
    "balance": "10.50",
    "currency": "USD"
  }
}
```

## Tech Stack

| Crate              | Role                            |
|--------------------|---------------------------------|
| `axum` 0.7         | HTTP framework                  |
| `tokio`            | Async runtime                   |
| `tower-http`       | CORS middleware                  |
| `serde` / `serde_json` | JSON serialization         |
| `steam-user`       | Core Steam operations           |
| `steam-protos`     | Steam protobuf message types    |
| `steam-totp`       | TOTP code generation            |
| `steamid`          | SteamID parsing & conversion    |
| `tracing`          | Structured logging              |

## License

MIT
