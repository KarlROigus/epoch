mod db;
mod routes;

use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::routing::{delete, get, post, put};
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use std::env;

async fn auth_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    let api_key = env::var("API_KEY").unwrap_or_default();
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let token = auth_header.strip_prefix("Bearer ").unwrap_or("");

    if token != api_key {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}

#[tokio::main]
async fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    if api_key.is_empty() {
        panic!("API_KEY must not be empty");
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("../migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let app = Router::new()
        .route("/api/timer/start", post(routes::start_timer))
        .route("/api/timer/stop", post(routes::stop_timer))
        .route("/api/timer/status", get(routes::timer_status))
        .route("/api/entries", get(routes::list_entries))
        .route("/api/entries/{id}", delete(routes::delete_entry))
        .route("/api/entries/{id}", put(routes::update_entry))
        .layer(middleware::from_fn(auth_middleware))
        .with_state(pool);

    let addr = format!("0.0.0.0:{}", port);
    println!("epoch-server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
