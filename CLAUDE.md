# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

`steam-user-api` is an Axum HTTP proxy that exposes the [`steam-user`](https://crates.io/crates/steam-user) crate's operations as JSON endpoints. Each endpoint is a thin handler that builds a `SteamUser` client from per-request credentials, calls one `steam-user` method, and wraps the result in a uniform JSON envelope.

## Commands

```bash
cargo build                 # debug build
cargo run                   # run on port 3000 (override with PORT env var)
cargo test                  # run tests
cargo test <name>           # run a single test by name substring
cargo clippy                # lint (lint levels are configured in Cargo.toml)
cargo build --release       # release build (fat LTO, stripped, panic=abort)
```

Runtime env vars: `PORT` (default 3000), `API_KEY` (required for `/api/*` auth — see below), `RUST_LOG` (tracing filter, default `info`).

## Architecture

Three source areas: `main.rs` (bootstrap), `auth.rs` (the shared request/response machinery), and `routes/` (one module per Steam feature area).

### Request flow

1. `routes::all_routes()` (`src/routes.rs`) nests every feature module under `/api` and applies the `api_key_auth` middleware. `/api/status` is nested *outside* that middleware so health checks need no API key.
2. `api_key_auth` (`src/auth.rs`) checks the `Authorization` header against the `API_KEY` env var (accepts both `Bearer <key>` and the bare key). If `API_KEY` is unset, **all** `/api/*` requests are rejected as unauthorized.
3. Handlers extract `SteamUserAuth<T>`, a custom `FromRequest` extractor that deserializes the JSON body, builds a `steam_user::SteamUser` from the auth fields, and exposes `{ user, auth, data }`.

### Two layers of auth (do not confuse them)

- **API gate:** the `Authorization` header + `API_KEY` env var. Controls who may call this server.
- **Steam credentials:** Steam `cookies` / tokens carried *in the JSON body*. Passed through to `steam-user` per request; the server is stateless and holds no Steam session.

### Body shape — nested `auth`, flattened data

`Req<T>` in `auth.rs` has a **named `auth` object** plus `#[serde(flatten)]` on the handler's `data: T`. So the Steam credentials sit under an `auth` key and the endpoint params sit at the top level, e.g.:

```json
{ "auth": { "cookies": ["..."], "access_token": "..." }, "code": "WALLETCODE" }
```

This wire format is the canonical contract: it matches the README **and** `steam_user::remote::RemoteSteamUser`, the typed client for this server that ships in the `steam-user` crate (`remote/services/*.rs`). Keep them in sync — if you change a path, payload field, or the auth nesting here, you break that client.

### Adding an endpoint

Follow the pattern in any module under `src/routes/` (e.g. `account.rs`):

1. Write `async fn my_handler(SteamUserAuth { user, data: req, .. }: SteamUserAuth<MyReq>) -> Result<ApiResponse<T>, ApiError>`.
2. Define a `#[derive(Deserialize)]` request struct for any params (omit the type param, use `SteamUserAuth`, when the endpoint takes only auth). Field names must match what `RemoteSteamUser` sends.
3. Add the fn name to the module's `post_routes!(Router::new(), my_handler, …)` list. **The route path is the function name** — `post_routes!` registers `/<fn_name>` via `stringify!`, so `get_wallet_balance` is served at `/api/account/get_wallet_balance`. The matching `RemoteSteamUser` client method must POST to that exact path.
4. Register a new module by adding it to both the `pub mod` list and the `nest_routes!(...)` call in `src/routes.rs`. `nest_routes!` mounts it at `/api/<module_name>`.

Macros in `auth.rs`: `post_routes!` (batch-register handlers; path = fn name — the one in use), `post_route!` (single), `nest_routes!` (mount modules by name). `post_routes_at!` (explicit `"/path" => fn`) remains defined but is no longer used.

### Responses and errors

- Wrap successes in `ApiResponse::ok(data)`; the envelope is `{ success, data?, error? }` with `None` fields skipped. Most handlers return `ApiResponse<serde_json::Value>` via `serde_json::to_value(...)`; action endpoints return `ApiResponse<&'static str>` with `"ok"`.
- Return `ApiError` for failures — it implements `IntoResponse` and maps variants to status codes in one place: Steam `RateLimited`→429, `NotLoggedIn`/`SessionExpired`→401, other Steam errors→502, bad request/serialization→400, internal/IO/join→500. `steam_user::SteamUserError` converts into `ApiError::Steam` via `#[from]`, so `?` propagates Steam errors with the right status automatically.

## Conventions

- Clippy is strict (see `[lints]` in `Cargo.toml`): `correctness` is denied; `unwrap_used`, `expect_used`, `panic`, `todo`, `print_stdout`/`print_stderr` are warnings. Use the `?` operator and `tracing` macros instead of unwraps and prints.
- Logging is via `tracing` only.

## Deployment

`steam-user-api.service` is a hardened systemd unit (DynamicUser, ProtectSystem=strict, locked-down syscall filter). It expects the binary at `/usr/local/bin/steam-user-api` and sets `PORT=3000`. Note it does **not** set `API_KEY`, so that must be added before `/api/*` routes will accept requests.
