use crate::{config::Config, repositories::proxy::ProxyRepository};

/// Shared state for Proxy routes.
#[derive(Clone)]
pub struct ProxyRouterState {
    pub(crate) proxy_repo: ProxyRepository,
}

impl ProxyRouterState {
    pub fn new(config: &Config) -> Self {
        Self {
            proxy_repo: ProxyRepository::new(config),
        }
    }
}
