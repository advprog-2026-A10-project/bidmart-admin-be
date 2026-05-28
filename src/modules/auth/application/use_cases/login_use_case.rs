use std::sync::Arc;

use crate::modules::auth::domain::entities::LoginOutcome;
use crate::modules::auth::domain::errors::AuthError;
use crate::modules::auth::domain::traits::AuthGatewayPort;

pub struct LoginUseCase {
    gateway: Arc<dyn AuthGatewayPort>,
}

impl LoginUseCase {
    pub fn new(gateway: Arc<dyn AuthGatewayPort>) -> Self {
        Self { gateway }
    }

    pub async fn execute(&self, email: &str, password: &str) -> Result<LoginOutcome, AuthError> {
        let auth_response = self.gateway.login(email, password).await?;

        if auth_response.requires_mfa {
            return Err(AuthError::Forbidden(
                "Admin login with MFA is not yet supported in admin portal.".to_string(),
            ));
        }

        let access_token = auth_response
            .access_token
            .ok_or_else(|| AuthError::Unauthorized("Unauthorized.".to_string()))?;
        let user = auth_response
            .user
            .ok_or_else(|| AuthError::Unauthorized("Unauthorized.".to_string()))?;

        let validated = self.gateway.validate(&access_token).await?;
        let has_admin_role = validated
            .roles
            .iter()
            .any(|role| role.eq_ignore_ascii_case("ADMIN"));

        if !has_admin_role {
            let _ = self.gateway.logout(&access_token).await;
            return Err(AuthError::Forbidden(
                "Forbidden. Admin role is required.".to_string(),
            ));
        }

        Ok(LoginOutcome { user, access_token })
    }
}
