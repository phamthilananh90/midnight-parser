mod auth;
mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into())).init();

    let port: u16 = std::env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(3000);

    // Evict parked (awaiting-email-code) login sessions past their TTL.
    routes::auth::spawn_session_sweeper();

    let app = routes::all_routes().layer(tower_http::cors::CorsLayer::permissive());

    tracing::info!("🚀 Listening on 0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(("0.0.0.0", port)).await.map_err(|e| {
        tracing::error!(port, error = %e, "failed to bind TCP listener");
        e
    })?;
    axum::serve(listener, app).await.map_err(|e| {
        tracing::error!(error = %e, "axum::serve terminated with error");
        e
    })?;
    Ok(())
}
