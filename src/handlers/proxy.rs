use crate::state::ProxyRouterState;

use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use std::collections::HashMap;

pub async fn generic_get_proxy_handler(
    State(state): State<ProxyRouterState>,
    Path(path): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    match state.proxy_repo.get(&path, &params).await {
        Ok(response) => Json(response).into_response(),
        Err(e) => {
            eprintln!("Proxy error: {}", e);
            Json(serde_json::json!({"error": e.to_string()})).into_response()
        }
    }
}

pub async fn generic_post_proxy_handler(
    State(state): State<ProxyRouterState>,
    Path(path): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    match state.proxy_repo.post(&path, &params, &body).await {
        Ok(response) => Json(response).into_response(),
        Err(e) => {
            eprintln!("Proxy error: {}", e);
            Json(serde_json::json!({"error": e.to_string()})).into_response()
        }
    }
}
