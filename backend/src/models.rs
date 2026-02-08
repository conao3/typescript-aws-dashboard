use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, SimpleObject)]
pub struct User {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub name: String,
    #[graphql(skip)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, SimpleObject)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(InputObject)]
pub struct RegisterTenantInput {
    pub name: String,
    pub slug: String,
    pub admin_email: String,
    pub admin_name: String,
    pub admin_password: String,
}

#[derive(InputObject)]
pub struct CreateUserInput {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(SimpleObject)]
pub struct AuthPayload {
    pub token: String,
    pub user: User,
}

#[derive(Debug, FromRow)]
pub struct AwsCredentialRow {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub access_key_id_encrypted: String,
    pub secret_access_key_encrypted: String,
    pub region: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, SimpleObject)]
pub struct AwsCredential {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub region: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(InputObject)]
pub struct CreateAwsCredentialInput {
    pub name: String,
    pub access_key_id: String,
    pub secret_access_key: String,
    pub region: String,
}

#[derive(InputObject)]
pub struct UpdateAwsCredentialInput {
    pub name: Option<String>,
    pub region: Option<String>,
}
