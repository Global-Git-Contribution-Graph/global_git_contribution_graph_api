mod providers;
mod state;
mod graphql;
mod services;

use std::sync::Arc;
use axum::{routing::get, routing::post, routing::options, Router, http::StatusCode};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use tower_http::cors::{CorsLayer, Any};
use http::Method;

use crate::providers::{GitHub, GitLab, ForgeJo};
use crate::state::AppState;
use crate::graphql::{QueryRoot, graphql_handler, graphiql};

async fn preflight() -> StatusCode {
    StatusCode::NO_CONTENT
}

#[tokio::main]
async fn main() {
    let git_hub_instance = GitHub;
    let git_lab_instance = GitLab;
    let forgejo_instance = ForgeJo;
    let shared_state = Arc::new(AppState {
        providers: vec![
            Arc::new(git_hub_instance),
            Arc::new(git_lab_instance),
            Arc::new(forgejo_instance)
        ]
    });

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(Arc::clone(&shared_state)) 
        .finish();

    let cors = CorsLayer::new()
        .allow_origin([
            "http://127.0.0.1:3000".parse().unwrap(),
            "http://localhost:3000".parse().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);

    
    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/graphql", options(preflight))
        .route("/graphql", get(graphql_handler))
        .route("/", get(graphiql))
        .with_state(schema)
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    println!("Serveur lancé sur http://127.0.0.1:8080");

    match axum::serve(listener, app).await {
        Ok(_) => { println!("Server stopped"); },
        Err(e) => { eprint!("Error : {}", e); },
    };
}
