use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::admin::domain::entities::{
    RbacRole, RbacRoleDetail, RbacRoleMember, RbacUserAssignment,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RbacRoleDto {
    pub id: i32,
    pub name: String,
    pub permissions: Vec<String>,
    pub member_count: i64,
}

impl From<RbacRole> for RbacRoleDto {
    fn from(value: RbacRole) -> Self {
        Self {
            id: value.id,
            name: value.name,
            permissions: value.permissions,
            member_count: value.member_count,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RbacRoleMemberDto {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub status: String,
}

impl From<RbacRoleMember> for RbacRoleMemberDto {
    fn from(value: RbacRoleMember) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
            status: value.status,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RbacRoleDetailDto {
    pub id: i32,
    pub name: String,
    pub permissions: Vec<String>,
    pub members: Vec<RbacRoleMemberDto>,
}

impl From<RbacRoleDetail> for RbacRoleDetailDto {
    fn from(value: RbacRoleDetail) -> Self {
        Self {
            id: value.id,
            name: value.name,
            permissions: value.permissions,
            members: value
                .members
                .into_iter()
                .map(RbacRoleMemberDto::from)
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RbacUserAssignmentDto {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub status: String,
    pub roles: Vec<String>,
}

impl From<RbacUserAssignment> for RbacUserAssignmentDto {
    fn from(value: RbacUserAssignment) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
            status: value.status,
            roles: value.roles,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RbacPermissionsPanelDto {
    pub roles: Vec<RbacRoleDto>,
    pub users: Vec<RbacUserAssignmentDto>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RbacMutationResultDto {
    pub message: String,
    pub changed: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRoleCommand {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRoleMutationCommand {
    pub role: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RolePermissionMutationCommand {
    pub permission: String,
}
