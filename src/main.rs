use actix_web::{web::{self, Data}, HttpResponse, Result, App, HttpServer, guard};
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, http::{GraphQLPlaygroundConfig, playground_source}};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

struct Query;

#[Object]
impl Query {
    async fn total_photos(&self) -> usize {
        42
    }
}

type ApiSchema = Schema<Query, EmptyMutation, EmptySubscription>;

async fn index(schema: web::Data<ApiSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_playground))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
