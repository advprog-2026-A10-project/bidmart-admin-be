use crate::modules::auth::application::dto::{AdminMeResponseDto, ValidateResponseDto};
use crate::modules::auth::domain::entities::AuthValidation;

pub fn to_validate_response(auth: &AuthValidation) -> ValidateResponseDto {
    ValidateResponseDto {
        user_id: auth.user_id,
        name: auth.name.clone(),
        email: auth.email.clone(),
        email_verified: auth.email_verified,
        mfa_satisfied: auth.mfa_satisfied,
        session_expiry: auth.session_expiry.clone(),
        is_admin: true,
    }
}

pub fn to_admin_me_response(auth: &AuthValidation) -> AdminMeResponseDto {
    AdminMeResponseDto {
        user_id: auth.user_id,
        name: auth.name.clone(),
        email: auth.email.clone(),
        email_verified: auth.email_verified,
        mfa_satisfied: auth.mfa_satisfied,
        session_expiry: auth.session_expiry.clone(),
        roles: auth.roles.clone(),
        permissions: auth.permissions.clone(),
    }
}
