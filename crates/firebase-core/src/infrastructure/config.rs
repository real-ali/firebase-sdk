use crate::domain::DomainError;
use crate::domain::FirebaseApp;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct ConfigFile {
    api_key: String,
    project_id: String,
    app_id: String,
}

pub fn load_config(path: &str) -> Result<FirebaseApp, DomainError> {
    let content = fs::read_to_string(path).map_err(|e| {
        DomainError::InvalidConfig(format!("Failed to read config file {}: {}", path, e))
    })?;

    let config: ConfigFile = toml::from_str(&content).map_err(|e| {
        DomainError::InvalidConfig(format!("Failed to parse config file {}: {}", path, e))
    })?;

    if config.api_key.is_empty() || config.project_id.is_empty() || config.app_id.is_empty() {
        return Err(DomainError::InvalidConfig(
            "Missing required fields in config".to_string(),
        ));
    }

    Ok(FirebaseApp {
        api_key: config.api_key,
        project_id: config.project_id,
        app_id: config.app_id,
    })
}
