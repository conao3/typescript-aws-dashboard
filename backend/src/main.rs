use actix_cors::Cors;
use actix_web::{guard, web, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

    log::info!("GraphiQL IDE: http://localhost:17231/admin/graphiql");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(schema.clone()))
            .service(web::resource("/graphql").guard(guard::Post()).to(graphql))
            .service(web::resource("/admin/graphiql").guard(guard::Get()).to(graphiql))
    })
    .bind(("127.0.0.1", 17231))?
    .run()
    .await
}
