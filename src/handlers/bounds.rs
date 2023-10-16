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
use std::{collections::HashMap, sync::Arc};

type Resp = Json<Value>;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestQuery {
    /// top left latitude (in degrees)
    top_left_lat: f64,
    /// top left longitude (in degrees)
    top_left_lon: f64,
    /// bottom right latitude (in degrees)
    bottom_right_lat: f64,
    /// bottom right longitude (in degrees)
    bottom_right_lon: f64,
    /// resolution (0-15)
    res: u8,
}

pub async fn bounds(
    Query(query): Query<RequestQuery>,
    State(_state): State<Arc<AppState>>,
) -> Result<(StatusCode, Resp), AppError> {
    // top left coordinates
    let top_left = LatLng::new(query.top_left_lat, query.top_left_lon)?;

    // top right coordinates
    let top_right = LatLng::new(query.top_left_lat, query.bottom_right_lon)?;

    // bottom right coordinates
    let bottom_right = LatLng::new(query.bottom_right_lat, query.bottom_right_lon)?;

    // bottom left coordinates
    let bottom_left = LatLng::new(query.bottom_right_lat, query.top_left_lon)?;

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
    let cells: Vec<HashMap<String, String>> = polygon
        .to_cells(PolyfillConfig::new(Resolution::try_from(query.res)?))
        .map(|cell| {
            let mut map = HashMap::new();
            map.insert("hex_id".to_string(), cell.to_string());
            map
        })
        .collect();

    Ok((StatusCode::OK, Json(json!(cells))))
}
