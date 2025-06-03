use crate::domain::DomainError;
use crate::domain::{FirebaseApp, FirebaseToken};
use crate::infrastructure::FirebaseApi;

pub async fn get_token(app: &FirebaseApp) -> Result<FirebaseToken, DomainError> {
    let api = FirebaseApi::new();
    api.fetch_token(app).await
}
