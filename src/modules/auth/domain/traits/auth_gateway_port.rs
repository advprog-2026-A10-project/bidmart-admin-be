use async_trait::async_trait;

use crate::modules::auth::domain::entities::{AuthLoginResult, AuthValidation};
use crate::modules::auth::domain::errors::AuthError;

#[async_trait]
pub trait AuthGatewayPort: Send + Sync {
    async fn login(&self, email: &str, password: &str) -> Result<AuthLoginResult, AuthError>;
    async fn validate(&self, access_token: &str) -> Result<AuthValidation, AuthError>;
    async fn logout(&self, access_token: &str) -> Result<(), AuthError>;
}
