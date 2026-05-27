use std::sync::Arc;

use crate::modules::auth::domain::traits::AuthGatewayPort;

pub struct LogoutUseCase {
    gateway: Arc<dyn AuthGatewayPort>,
}

impl LogoutUseCase {
    pub fn new(gateway: Arc<dyn AuthGatewayPort>) -> Self {
        Self { gateway }
    }

    pub async fn execute(&self, access_token: &str) {
        let _ = self.gateway.logout(access_token).await;
    }
}
