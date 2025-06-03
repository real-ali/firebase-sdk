use serde::{Deserialize, Serialize};

/// Core Firebase app config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirebaseApp {
    pub api_key: String,
    pub project_id: String,
    pub app_id: String,
}

/// OAuth2 token info from Firebase Auth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirebaseToken {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub refresh_token: Option<String>,
}
