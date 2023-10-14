use crate::{
    handlers::{bounds, health},
    lookup::RegionMap,
    settings::Settings,
};
use anyhow::{Error, Result};
use axum::{routing::get, Router};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[derive(Debug, Clone)]
pub struct AppState {
    pub region_map: Arc<RegionMap>,
}

impl AppState {
    pub async fn from_settings(settings: &Settings) -> Result<Self> {
        let region_map = Arc::new(RegionMap::load(&settings.region_dir)?);
        Ok(Self { region_map })
    }
}

pub async fn run(settings: &Settings) -> Result<()> {
    let state = Arc::new(AppState::from_settings(settings).await?);

    let json_routes = Router::new().route("/bounds", get(bounds::bounds));

    let api_v1 = Router::new().nest("/json", json_routes);

    let app = Router::new()
        .route("/health", get(health::health))
        .nest("/api/v1", api_v1)
        .with_state(state)
        .layer(CorsLayer::permissive());

    let addr = settings.socket_addr()?;
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(Error::from)
}
