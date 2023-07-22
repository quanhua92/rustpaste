use async_graphql::futures_util::lock::Mutex;
use async_graphql::http::GraphiQLSource;
use async_graphql::{EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::response::{self, IntoResponse};
use axum::{routing::get, Extension, Router, Server};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;

use rustpaste::{MutationRoot, PasteStorage, QueryRoot, ServiceSchema};

pub async fn graphql_handler(
    schema: Extension<ServiceSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let pool = PgPool::connect(&database_url).await.unwrap();

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Arc::new(Mutex::new(PasteStorage::new(pool))))
        .finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    println!("Server is running at http://localhost:8080");
    Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
