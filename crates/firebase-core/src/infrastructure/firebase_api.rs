use crate::domain::DomainError;
use crate::domain::{FirebaseApp, FirebaseToken};
use crate::infrastructure::http_client::HttpClient;
use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct TokenRequest<'a> {
    grant_type: &'a str,
    refresh_token: Option<&'a str>,
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    expires_in: String,
    token_type: String,
    refresh_token: Option<String>,
}

pub struct FirebaseApi {
    http_client: HttpClient,
}

impl FirebaseApi {
    pub fn new() -> Self {
        Self {
            http_client: HttpClient::new(),
        }
    }

    /// Fetches an OAuth2 token using a refresh token
    pub async fn fetch_token(&self, app: &FirebaseApp) -> Result<FirebaseToken, DomainError> {
        // In real usage, you would store and use a refresh token here.
        // For demonstration, let's just error out:
        Err(DomainError::AuthError(
            "fetch_token requires a refresh token to be implemented".into(),
        ))
    }

    /// Refresh OAuth2 token with a refresh token
    pub async fn refresh_token(
        &self,
        app: &FirebaseApp,
        refresh_token: &str,
    ) -> Result<FirebaseToken, DomainError> {
        let url = format!(
            "https://securetoken.googleapis.com/v1/token?key={}",
            app.api_key
        );
        let form = vec![
            ("grant_type".to_string(), "refresh_token".to_string()),
            ("refresh_token".to_string(), refresh_token.to_string()),
        ];

        info!("Requesting refreshed token from Firebase Secure Token API");

        let resp = self.http_client.post_form(&url, &form).await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            error!("Token refresh failed with status {}: {}", status, text);
            return Err(DomainError::AuthError(format!(
                "Failed to refresh token: status {}, message {}",
                status, text
            )));
        }

        let token_resp: TokenResponse = resp.json().await.map_err(|e| {
            DomainError::Unexpected(format!("Failed to parse token response: {}", e))
        })?;

        let expires_in = token_resp.expires_in.parse::<u64>().unwrap_or(3600);

        Ok(FirebaseToken {
            access_token: token_resp.access_token,
            expires_in,
            token_type: token_resp.token_type,
            refresh_token: token_resp.refresh_token,
        })
    }
}

impl Default for FirebaseApi {
    fn default() -> Self {
        Self::new()
    }
}
