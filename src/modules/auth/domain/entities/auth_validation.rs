#[derive(Debug, Clone)]
pub struct AuthValidation {
    pub user_id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub email_verified: bool,
    pub mfa_satisfied: bool,
    pub session_expiry: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}
