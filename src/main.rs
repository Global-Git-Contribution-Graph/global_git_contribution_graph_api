use std::sync::Arc;
use axum::{routing::get, Router, Json, extract::State};
use serde::{Serialize, Deserialize};

use crate::providers::{GitProvider, GitHub};

mod providers;

#[derive(Serialize, Deserialize, Debug)]
struct HealthCheck {
    status: String,
    version: String
}

struct AppState {
    providers: Vec<Box<dyn GitProvider + Send + Sync>>
}

#[tokio::main]
async fn main() {
    let git_hub_instance = GitHub;
    let shared_state = Arc::new(AppState {
        providers: vec![Box::new(git_hub_instance)],
    });
    
    let app = Router::new()
        .route("/api", get(health_check))
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Serveur lancé sur http://127.0.0.1:3000");

    match axum::serve(listener, app).await {
        Ok(_) => { println!("Server stopped"); },
        Err(e) => { eprint!("Error : {}", e); },
    };
}

async fn health_check(State(state): State<Arc<AppState>>) -> Json<HealthCheck> {
    let name = match state.providers.get(0) {
        Some(provider) => provider.get_name(),
        None => "No provider".to_string()
    };

    Json(HealthCheck {
        status: name,
        version: "0.1.0".to_string(),
    })
}