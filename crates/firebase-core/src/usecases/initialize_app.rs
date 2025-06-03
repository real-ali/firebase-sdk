use crate::domain::DomainError;
use crate::domain::FirebaseApp;

/// Initializes the FirebaseApp from config parameters.
pub fn initialize_app(
    api_key: &str,
    project_id: &str,
    app_id: &str,
) -> Result<FirebaseApp, DomainError> {
    if api_key.is_empty() || project_id.is_empty() || app_id.is_empty() {
        return Err(DomainError::InvalidConfig(
            "api_key, project_id, and app_id must not be empty".to_string(),
        ));
    }

    Ok(FirebaseApp {
        api_key: api_key.to_string(),
        project_id: project_id.to_string(),
        app_id: app_id.to_string(),
    })
}
