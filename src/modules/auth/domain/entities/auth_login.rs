use super::AuthUser;

#[derive(Debug, Clone)]
pub struct AuthLoginResult {
    pub requires_mfa: bool,
    pub user: Option<AuthUser>,
    pub access_token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LoginOutcome {
    pub user: AuthUser,
    pub access_token: String,
}
