use crate::{error::AppError, server::AppState};
use anyhow::Result;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use h3o::{LatLng, Resolution};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

type Resp = Json<Value>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestQuery {
    lat1: f64,
    lon1: f64,
    lat2: f64,
    lon2: f64,
    res: u8,
}

pub async fn bounds(
    Query(query): Query<RequestQuery>,
    State(_state): State<Arc<AppState>>,
) -> Result<(StatusCode, Resp), AppError> {
    tracing::info!("query: {:?}", query);

    let latlng1 = LatLng::new(query.lat1, query.lon1)?;
    let latlng2 = LatLng::new(query.lat2, query.lon2)?;
    let cell1: u64 = latlng1.to_cell(Resolution::try_from(query.res)?).into();
    let cell2: u64 = latlng2.to_cell(Resolution::try_from(query.res)?).into();
    tracing::info!("cell1: {:?}", cell1);
    tracing::info!("cell2: {:?}", cell2);

    // TODO: Figure out how to return the cells using hextree which correspond to the bounds

    Ok((StatusCode::OK, Json(json!("ok"))))
}
