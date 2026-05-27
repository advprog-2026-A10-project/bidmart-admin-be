use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RbacRole {
    pub id: i32,
    pub name: String,
    pub permissions: Vec<String>,
    pub member_count: i64,
}

#[derive(Debug, Clone)]
pub struct RbacRoleMember {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct RbacRoleDetail {
    pub id: i32,
    pub name: String,
    pub permissions: Vec<String>,
    pub members: Vec<RbacRoleMember>,
}

#[derive(Debug, Clone)]
pub struct RbacUserAssignment {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub status: String,
    pub roles: Vec<String>,
}
