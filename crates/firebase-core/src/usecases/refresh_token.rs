use crate::domain::DomainError;
use crate::domain::{FirebaseApp, FirebaseToken};
use crate::infrastructure::FirebaseApi;

pub async fn refresh_token(
    app: &FirebaseApp,
    refresh_token: &str,
) -> Result<FirebaseToken, DomainError> {
    let api = FirebaseApi::new();
    api.refresh_token(app, refresh_token).await
}
