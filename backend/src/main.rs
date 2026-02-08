use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Result, guard, web};
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, http::GraphiQLSource};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

mod db;

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

type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

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

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

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
