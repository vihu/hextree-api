use crate::{error::AppError, server::AppState};
use anyhow::Result;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use geo::polygon;
use h3o::{
    geom::{PolyfillConfig, Polygon, ToCells},
    LatLng, Resolution,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

type Resp = Json<Value>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestQuery {
    /// top left latitude (in degrees)
    top: f64,
    /// top left longitude (in degrees)
    left: f64,
    /// bottom right latitude (in degrees)
    bottom: f64,
    /// bottom right longitude (in degrees)
    right: f64,
    /// resolution (0-15)
    res: u8,
}

pub async fn bounds(
    Query(query): Query<RequestQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<(StatusCode, Resp), AppError> {
    // top left coordinates
    let top_left = LatLng::new(query.top, query.left)?;

    // top right coordinates
    let top_right = LatLng::new(query.top, query.right)?;

    // bottom right coordinates
    let bottom_right = LatLng::new(query.bottom, query.right)?;

    // bottom left coordinates
    let bottom_left = LatLng::new(query.bottom, query.left)?;

    // bounding box (closing the loop)
    let bounding_box = polygon![
        top_left.into(),
        top_right.into(),
        bottom_right.into(),
        bottom_left.into(),
        top_left.into()
    ];

    // convert to polygon
    let polygon = Polygon::from_degrees(bounding_box)?;

    // get cells from polygon
    let cells: Vec<Value> = polygon
        .to_cells(PolyfillConfig::new(Resolution::try_from(query.res)?))
        .filter_map(|cell| state.region_map.get(cell).map(|pop| (cell, pop)))
        .map(|(cell, pop)| json!({"hex_id": cell.to_string(), "population": pop}))
        .collect();

    Ok((StatusCode::OK, Json(Value::Array(cells))))
}
