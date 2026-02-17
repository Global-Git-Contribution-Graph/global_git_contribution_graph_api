use axum::{routing::get, Router, Json};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct HealthCheck {
    status: String,
    version: String
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api", get(health_check));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Serveur lancé sur http://127.0.0.1:3000");

    match axum::serve(listener, app).await {
        Ok(_) => { println!("Server stopped"); },
        Err(e) => { eprint!("Error : {}", e); },
    };
}

async fn health_check() -> Json<HealthCheck> {
    let response = HealthCheck {
        status: "UP".to_string(),
        version: "0.1.0".to_string()
    };

    return Json(response);
}