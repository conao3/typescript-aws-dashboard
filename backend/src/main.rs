use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Result, guard, web};
use async_graphql::{Context, EmptySubscription, Object, Schema, http::GraphiQLSource};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

mod auth;
mod db;
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
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn login(&self, ctx: &Context<'_>, email: String, password: String) -> async_graphql::Result<String> {
        let pool = ctx.data::<sqlx::PgPool>()?;
        let jwt_config = ctx.data::<auth::JwtConfig>()?;

        let user = sqlx::query_as::<_, models::User>(
            "SELECT * FROM dashboard.users WHERE email = $1"
        )
        .bind(&email)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| async_graphql::Error::new("invalid credentials"))?;

        if !auth::verify_password(&password, &user.password_hash)? {
            return Err(async_graphql::Error::new("invalid credentials"));
        }

        let tenant = sqlx::query_as::<_, models::Tenant>(
            "SELECT * FROM dashboard.tenants WHERE id = $1 AND is_active = true"
        )
        .bind(user.tenant_id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| async_graphql::Error::new("tenant not active"))?;

        let token = jwt_config.generate_token(user.id, tenant.id, user.email)?;

        Ok(token)
    }
}

type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

async fn graphql(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
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

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(pool.clone())
        .data(jwt_config)
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
