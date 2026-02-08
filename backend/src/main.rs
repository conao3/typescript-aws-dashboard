use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Result, guard, web};
use async_graphql::{Context, EmptySubscription, Object, Schema, http::GraphiQLSource};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

mod auth;
mod crypto;
mod db;
mod middleware;
mod models;

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> &str {
        "Hello, GraphQL!"
    }

    async fn version(&self) -> &str {
        "1.0.0"
    }

    async fn current_user(&self, ctx: &Context<'_>) -> async_graphql::Result<models::User> {
        let tenant_ctx = ctx.data::<middleware::TenantContext>()
            .map_err(|_| async_graphql::Error::new("unauthorized"))?;
        let pool = ctx.data::<sqlx::PgPool>()?;

        let user = sqlx::query_as::<_, models::User>(
            "SELECT * FROM dashboard.users WHERE id = $1 AND tenant_id = $2"
        )
        .bind(tenant_ctx.user_id)
        .bind(tenant_ctx.tenant_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| async_graphql::Error::new("user not found"))?;

        Ok(user)
    }

    async fn aws_credentials(
        &self,
        ctx: &Context<'_>,
    ) -> async_graphql::Result<Vec<models::AwsCredential>> {
        let tenant_ctx = ctx
            .data::<middleware::TenantContext>()
            .map_err(|_| async_graphql::Error::new("unauthorized"))?;
        let pool = ctx.data::<sqlx::PgPool>()?;

        let rows = sqlx::query_as::<_, models::AwsCredentialRow>(
            "SELECT * FROM dashboard.aws_credentials WHERE tenant_id = $1 ORDER BY created_at DESC",
        )
        .bind(tenant_ctx.tenant_id)
        .fetch_all(pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| models::AwsCredential {
                id: row.id,
                tenant_id: row.tenant_id,
                name: row.name,
                region: row.region,
                created_at: row.created_at,
                updated_at: row.updated_at,
            })
            .collect())
    }

    async fn aws_credential(
        &self,
        ctx: &Context<'_>,
        id: uuid::Uuid,
    ) -> async_graphql::Result<models::AwsCredential> {
        let tenant_ctx = ctx
            .data::<middleware::TenantContext>()
            .map_err(|_| async_graphql::Error::new("unauthorized"))?;
        let pool = ctx.data::<sqlx::PgPool>()?;

        let row = sqlx::query_as::<_, models::AwsCredentialRow>(
            "SELECT * FROM dashboard.aws_credentials WHERE id = $1 AND tenant_id = $2",
        )
        .bind(id)
        .bind(tenant_ctx.tenant_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| async_graphql::Error::new("credential not found"))?;

        Ok(models::AwsCredential {
            id: row.id,
            tenant_id: row.tenant_id,
            name: row.name,
            region: row.region,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn login(
        &self,
        ctx: &Context<'_>,
        email: String,
        password: String,
    ) -> async_graphql::Result<String> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let jwt_config = ctx.data::<auth::JwtConfig>()?;

        let user =
            sqlx::query_as::<_, models::User>("SELECT * FROM dashboard.users WHERE email = $1")
                .bind(&email)
                .fetch_optional(pool)
                .await?
                .ok_or_else(|| async_graphql::Error::new("invalid credentials"))?;

        if !auth::verify_password(&password, &user.password_hash)? {
            return Err(async_graphql::Error::new("invalid credentials"));
        }

        let tenant = sqlx::query_as::<_, models::Tenant>(
            "SELECT * FROM dashboard.tenants WHERE id = $1 AND is_active = true",
        )
        .bind(user.tenant_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| async_graphql::Error::new("tenant not active"))?;

        let token = jwt_config.generate_token(user.id, tenant.id, user.email)?;

        Ok(token)
    }

    async fn logout(&self) -> bool {
        true
    }

    async fn register_tenant(
        &self,
        ctx: &Context<'_>,
        input: models::RegisterTenantInput,
    ) -> async_graphql::Result<models::Tenant> {
        let pool = ctx.data::<sqlx::PgPool>()?;

        let existing_tenant = sqlx::query_as::<_, models::Tenant>(
            "SELECT * FROM dashboard.tenants WHERE slug = $1",
        )
        .bind(&input.slug)
        .fetch_optional(pool)
        .await?;

        if existing_tenant.is_some() {
            return Err(async_graphql::Error::new("slug already exists"));
        }

        let mut tx = pool.begin().await?;

        let tenant = sqlx::query_as::<_, models::Tenant>(
            "INSERT INTO dashboard.tenants (name, slug) VALUES ($1, $2) RETURNING *",
        )
        .bind(&input.name)
        .bind(&input.slug)
        .fetch_one(&mut *tx)
        .await?;

        let password_hash = auth::hash_password(&input.admin_password)?;

        sqlx::query(
            "INSERT INTO dashboard.users (tenant_id, email, name, password_hash) VALUES ($1, $2, $3, $4)",
        )
        .bind(tenant.id)
        .bind(&input.admin_email)
        .bind(&input.admin_name)
        .bind(&password_hash)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(tenant)
    }

    async fn create_user(
        &self,
        ctx: &Context<'_>,
        input: models::CreateUserInput,
    ) -> async_graphql::Result<models::User> {
        let tenant_ctx = ctx
            .data::<middleware::TenantContext>()
            .map_err(|_| async_graphql::Error::new("unauthorized"))?;
        let pool = ctx.data::<sqlx::PgPool>()?;

        let existing_user = sqlx::query_as::<_, models::User>(
            "SELECT * FROM dashboard.users WHERE tenant_id = $1 AND email = $2",
        )
        .bind(tenant_ctx.tenant_id)
        .bind(&input.email)
        .fetch_optional(pool)
        .await?;

        if existing_user.is_some() {
            return Err(async_graphql::Error::new("email already exists"));
        }

        let password_hash = auth::hash_password(&input.password)?;

        let user = sqlx::query_as::<_, models::User>(
            "INSERT INTO dashboard.users (tenant_id, email, name, password_hash) VALUES ($1, $2, $3, $4) RETURNING *",
        )
        .bind(tenant_ctx.tenant_id)
        .bind(&input.email)
        .bind(&input.name)
        .bind(&password_hash)
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    async fn create_aws_credential(
        &self,
        ctx: &Context<'_>,
        input: models::CreateAwsCredentialInput,
    ) -> async_graphql::Result<models::AwsCredential> {
        let tenant_ctx = ctx
            .data::<middleware::TenantContext>()
            .map_err(|_| async_graphql::Error::new("unauthorized"))?;
        let pool = ctx.data::<sqlx::PgPool>()?;
        let crypto_config = ctx.data::<crypto::CryptoConfig>()?;

        let access_key_encrypted = crypto_config
            .encrypt(&input.access_key_id)
            .map_err(|e| async_graphql::Error::new(format!("encryption failed: {}", e)))?;

        let secret_key_encrypted = crypto_config
            .encrypt(&input.secret_access_key)
            .map_err(|e| async_graphql::Error::new(format!("encryption failed: {}", e)))?;

        let row = sqlx::query_as::<_, models::AwsCredentialRow>(
            "INSERT INTO dashboard.aws_credentials (tenant_id, name, access_key_id_encrypted, secret_access_key_encrypted, region) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        )
        .bind(tenant_ctx.tenant_id)
        .bind(&input.name)
        .bind(&access_key_encrypted)
        .bind(&secret_key_encrypted)
        .bind(&input.region)
        .fetch_one(pool)
        .await?;

        Ok(models::AwsCredential {
            id: row.id,
            tenant_id: row.tenant_id,
            name: row.name,
            region: row.region,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }

    async fn update_aws_credential(
        &self,
        ctx: &Context<'_>,
        id: uuid::Uuid,
        input: models::UpdateAwsCredentialInput,
    ) -> async_graphql::Result<models::AwsCredential> {
        let tenant_ctx = ctx
            .data::<middleware::TenantContext>()
            .map_err(|_| async_graphql::Error::new("unauthorized"))?;
        let pool = ctx.data::<sqlx::PgPool>()?;

        let existing = sqlx::query_as::<_, models::AwsCredentialRow>(
            "SELECT * FROM dashboard.aws_credentials WHERE id = $1 AND tenant_id = $2",
        )
        .bind(id)
        .bind(tenant_ctx.tenant_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| async_graphql::Error::new("credential not found"))?;

        let name = input.name.unwrap_or(existing.name);
        let region = input.region.unwrap_or(existing.region);

        let row = sqlx::query_as::<_, models::AwsCredentialRow>(
            "UPDATE dashboard.aws_credentials SET name = $1, region = $2, updated_at = now() WHERE id = $3 AND tenant_id = $4 RETURNING *",
        )
        .bind(&name)
        .bind(&region)
        .bind(id)
        .bind(tenant_ctx.tenant_id)
        .fetch_one(pool)
        .await?;

        Ok(models::AwsCredential {
            id: row.id,
            tenant_id: row.tenant_id,
            name: row.name,
            region: row.region,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }

    async fn delete_aws_credential(
        &self,
        ctx: &Context<'_>,
        id: uuid::Uuid,
    ) -> async_graphql::Result<bool> {
        let tenant_ctx = ctx
            .data::<middleware::TenantContext>()
            .map_err(|_| async_graphql::Error::new("unauthorized"))?;
        let pool = ctx.data::<sqlx::PgPool>()?;

        let result = sqlx::query(
            "DELETE FROM dashboard.aws_credentials WHERE id = $1 AND tenant_id = $2",
        )
        .bind(id)
        .bind(tenant_ctx.tenant_id)
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}

type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

async fn graphql(
    schema: web::Data<AppSchema>,
    http_req: actix_web::HttpRequest,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let jwt_config = schema.data::<auth::JwtConfig>().unwrap();
    let mut request = req.into_inner();

    if let Some(tenant_ctx) = middleware::extract_tenant_context(&http_req, jwt_config) {
        request = request.data(tenant_ctx);
    }

    schema.execute(request).await.into()
}

async fn graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/graphql").finish()))
}

async fn health(pool: web::Data<sqlx::PgPool>) -> Result<HttpResponse> {
    match sqlx::query("SELECT 1").fetch_one(pool.get_ref()).await {
        Ok(_) => Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "ok",
            "database": "connected"
        }))),
        Err(e) => {
            log::error!("database health check failed: {}", e);
            Ok(HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "status": "error",
                "database": "disconnected"
            })))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = db::create_pool()
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    log::info!("database connection pool created");

    let jwt_config = auth::JwtConfig::new();
    let crypto_config = crypto::CryptoConfig::new();

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool.clone())
        .data(jwt_config)
        .data(crypto_config)
        .finish();

    log::info!("GraphiQL IDE: http://localhost:17231/admin/graphiql");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/health").guard(guard::Get()).to(health))
            .service(web::resource("/graphql").guard(guard::Post()).to(graphql))
            .service(
                web::resource("/admin/graphiql")
                    .guard(guard::Get())
                    .to(graphiql),
            )
    })
    .bind(("127.0.0.1", 17231))?
    .run()
    .await
}
