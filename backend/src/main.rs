use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Result, guard, web};
use async_graphql::{Context, EmptySubscription, Object, Schema, http::GraphiQLSource};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

mod auth;
mod aws;
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

    async fn ec2_ami_import_tasks(
        &self,
        ctx: &Context<'_>,
        filter: Option<models::Ec2AmiImportTaskFilter>,
    ) -> async_graphql::Result<Vec<models::Ec2AmiImportTask>> {
        let tenant_ctx = ctx
            .data::<middleware::TenantContext>()
            .map_err(|_| async_graphql::Error::new("unauthorized"))?;
        let pool = ctx.data::<sqlx::PgPool>()?;

        let filter = filter.unwrap_or(models::Ec2AmiImportTaskFilter {
            aws_credential_id: None,
            status: None,
            import_task_id: None,
            limit: Some(100),
            offset: Some(0),
        });

        let limit = filter.limit.unwrap_or(100).min(1000);
        let offset = filter.offset.unwrap_or(0);

        let mut query = String::from(
            "SELECT * FROM dashboard.ec2_ami_import_tasks WHERE tenant_id = $1",
        );
        let mut bind_count = 1;

        if filter.aws_credential_id.is_some() {
            bind_count += 1;
            query.push_str(&format!(" AND aws_credential_id = ${}", bind_count));
        }

        if filter.status.is_some() {
            bind_count += 1;
            query.push_str(&format!(" AND status = ${}", bind_count));
        }

        if filter.import_task_id.is_some() {
            bind_count += 1;
            query.push_str(&format!(" AND import_task_id = ${}", bind_count));
        }

        query.push_str(" ORDER BY created_at DESC");
        query.push_str(&format!(" LIMIT ${} OFFSET ${}", bind_count + 1, bind_count + 2));

        let mut sqlx_query = sqlx::query_as::<_, models::Ec2AmiImportTask>(&query)
            .bind(tenant_ctx.tenant_id);

        if let Some(credential_id) = filter.aws_credential_id {
            sqlx_query = sqlx_query.bind(credential_id);
        }

        if let Some(status) = filter.status {
            sqlx_query = sqlx_query.bind(status);
        }

        if let Some(import_task_id) = filter.import_task_id {
            sqlx_query = sqlx_query.bind(import_task_id);
        }

        sqlx_query = sqlx_query.bind(limit).bind(offset);

        let tasks = sqlx_query.fetch_all(pool).await?;

        Ok(tasks)
    }

    async fn ec2_ami_import_task(
        &self,
        ctx: &Context<'_>,
        id: uuid::Uuid,
    ) -> async_graphql::Result<models::Ec2AmiImportTask> {
        let tenant_ctx = ctx
            .data::<middleware::TenantContext>()
            .map_err(|_| async_graphql::Error::new("unauthorized"))?;
        let pool = ctx.data::<sqlx::PgPool>()?;

        let task = sqlx::query_as::<_, models::Ec2AmiImportTask>(
            "SELECT * FROM dashboard.ec2_ami_import_tasks WHERE id = $1 AND tenant_id = $2",
        )
        .bind(id)
        .bind(tenant_ctx.tenant_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| async_graphql::Error::new("task not found"))?;

        Ok(task)
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

    async fn sync_import_image_tasks(
        &self,
        ctx: &Context<'_>,
        aws_credential_id: uuid::Uuid,
    ) -> async_graphql::Result<Vec<models::Ec2AmiImportTask>> {
        let tenant_ctx = ctx
            .data::<middleware::TenantContext>()
            .map_err(|_| async_graphql::Error::new("unauthorized"))?;
        let pool = ctx.data::<sqlx::PgPool>()?;
        let aws_client_factory = ctx.data::<aws::AwsClientFactory>()?;

        let credential = sqlx::query_as::<_, models::AwsCredentialRow>(
            "SELECT * FROM dashboard.aws_credentials WHERE id = $1 AND tenant_id = $2",
        )
        .bind(aws_credential_id)
        .bind(tenant_ctx.tenant_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| async_graphql::Error::new("credential not found"))?;

        let import_tasks = aws_client_factory
            .fetch_import_image_tasks(
                &credential.access_key_id_encrypted,
                &credential.secret_access_key_encrypted,
                &credential.region,
            )
            .await
            .map_err(|e| async_graphql::Error::new(format!("AWS API error: {}", e)))?;

        let mut saved_tasks = Vec::new();

        for task in import_tasks {
            let import_task_id = task.import_task_id().unwrap_or("");
            let status = task.status().unwrap_or("");
            let status_message = task.status_message().map(|s| s.to_string());
            let image_id = task.image_id().map(|s| s.to_string());
            let architecture = task.architecture().map(|s| s.to_string());
            let description = task.description().map(|s| s.to_string());
            let hypervisor = task.hypervisor().map(|s| s.to_string());
            let license_type = task.license_type().map(|s| s.to_string());
            let platform = task.platform().map(|s| s.to_string());
            let progress = task.progress().map(|s| s.to_string());

            let snapshot_details = if !task.snapshot_details().is_empty() {
                let details: Vec<serde_json::Value> = task
                    .snapshot_details()
                    .iter()
                    .map(|detail| {
                        serde_json::json!({
                            "description": detail.description(),
                            "device_name": detail.device_name(),
                            "disk_image_size": detail.disk_image_size(),
                            "format": detail.format(),
                            "progress": detail.progress(),
                            "snapshot_id": detail.snapshot_id(),
                            "status": detail.status(),
                            "status_message": detail.status_message(),
                            "url": detail.url(),
                            "user_bucket": detail.user_bucket().map(|b| {
                                serde_json::json!({
                                    "s3_bucket": b.s3_bucket(),
                                    "s3_key": b.s3_key(),
                                })
                            }),
                        })
                    })
                    .collect();
                Some(serde_json::Value::Array(details))
            } else {
                None
            };

            let tags = if !task.tags().is_empty() {
                let tag_list: Vec<serde_json::Value> = task
                    .tags()
                    .iter()
                    .map(|tag| {
                        serde_json::json!({
                            "key": tag.key(),
                            "value": tag.value(),
                        })
                    })
                    .collect();
                Some(serde_json::Value::Array(tag_list))
            } else {
                None
            };

            let saved_task = sqlx::query_as::<_, models::Ec2AmiImportTask>(
                r#"
                INSERT INTO dashboard.ec2_ami_import_tasks
                (tenant_id, aws_credential_id, import_task_id, status, status_message,
                 image_id, architecture, description, hypervisor, license_type,
                 platform, progress, snapshot_details, tags)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
                ON CONFLICT (tenant_id, import_task_id)
                DO UPDATE SET
                    status = EXCLUDED.status,
                    status_message = EXCLUDED.status_message,
                    image_id = EXCLUDED.image_id,
                    architecture = EXCLUDED.architecture,
                    description = EXCLUDED.description,
                    hypervisor = EXCLUDED.hypervisor,
                    license_type = EXCLUDED.license_type,
                    platform = EXCLUDED.platform,
                    progress = EXCLUDED.progress,
                    snapshot_details = EXCLUDED.snapshot_details,
                    tags = EXCLUDED.tags,
                    updated_at = now()
                RETURNING *
                "#,
            )
            .bind(tenant_ctx.tenant_id)
            .bind(aws_credential_id)
            .bind(import_task_id)
            .bind(status)
            .bind(status_message)
            .bind(image_id)
            .bind(architecture)
            .bind(description)
            .bind(hypervisor)
            .bind(license_type)
            .bind(platform)
            .bind(progress)
            .bind(snapshot_details)
            .bind(tags)
            .fetch_one(pool)
            .await?;

            saved_tasks.push(saved_task);
        }

        Ok(saved_tasks)
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
    let aws_client_factory = aws::AwsClientFactory::new(crypto_config.clone());

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool.clone())
        .data(jwt_config)
        .data(crypto_config)
        .data(aws_client_factory)
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
