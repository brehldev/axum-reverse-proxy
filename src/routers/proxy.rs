use crate::{
    config::Config,
    handlers::proxy::{generic_get_proxy_handler, generic_post_proxy_handler},
    state::ProxyRouterState,
};
use axum::{Router, routing::get};

pub fn create_router(config: &Config) -> Router {
    Router::new()
        .route(
            "/{*rest}",
            get(generic_get_proxy_handler).post(generic_post_proxy_handler),
        )
        .with_state(ProxyRouterState::new(config))
}
