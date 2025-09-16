use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

use crate::config::Config;

#[derive(Clone)]
pub struct ProxyRepository {
    client: Client,
    target_url: String,
}

impl ProxyRepository {
    pub fn new(config: &Config) -> Self {
        let client = Client::new();

        Self {
            client,
            target_url: config.reverse_proxy_target_url.clone(),
        }
    }

    pub async fn get(
        &self,
        rest: &str,
        query_params: &HashMap<String, String>,
    ) -> Result<Value, reqwest::Error> {
        let url = format!("{}/{}", self.target_url, rest);
        let response = self.client.get(&url).query(&query_params).send().await?;
        let body = response.json().await?;
        Ok(body)
    }

    pub async fn post(
        &self,
        rest: &str,
        query_params: &HashMap<String, String>,
        body: &Value,
    ) -> Result<Value, reqwest::Error> {
        let url = format!("{}/{}", self.target_url, rest);
        let response = self
            .client
            .post(&url)
            .query(&query_params)
            .json(body)
            .send()
            .await?;
        let body = response.json().await?;
        Ok(body)
    }
}
