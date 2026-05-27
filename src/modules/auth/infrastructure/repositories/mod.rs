use sqlx::postgres::PgPool;
use uuid::Uuid;

pub async fn has_admin_role(pool: &PgPool, user_id: Uuid) -> Result<bool, sqlx::Error> {
    let row = sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM user_roles ur
            INNER JOIN roles r ON r.id = ur.role_id
            WHERE ur.user_id = $1
              AND UPPER(r.name) = 'ADMIN'
        )
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(row)
}
