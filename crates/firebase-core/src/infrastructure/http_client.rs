use crate::domain::DomainError;
use log::debug;
use reqwest::{Client, Response};
use std::time::Duration;

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to build HTTP client");
        Self { client }
    }

    pub async fn post_json<T: serde::Serialize>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<Response, DomainError> {
        debug!("POST JSON to URL: {}", url);
        self.client
            .post(url)
            .json(body)
            .send()
            .await
            .map_err(|e| DomainError::NetworkError(e.to_string()))
    }

    pub async fn post_form(
        &self,
        url: &str,
        form: &[(String, String)],
    ) -> Result<Response, DomainError> {
        debug!("POST form to URL: {}", url);
        self.client
            .post(url)
            .form(form)
            .send()
            .await
            .map_err(|e| DomainError::NetworkError(e.to_string()))
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}
