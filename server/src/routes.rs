use axum::extract::connect_info::ConnectInfo;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use epoch_common::{EntryResponse, ErrorResponse, StatusResponse, StopRequest, UpdateEntryRequest};
use sqlx::PgPool;
use std::net::SocketAddr;
use uuid::Uuid;

use crate::db;

fn entry_to_response(e: db::Entry) -> EntryResponse {
    EntryResponse {
        id: e.id,
        description: e.description,
        started_at: e.started_at,
        stopped_at: e.stopped_at,
    }
}

pub async fn start_timer(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    println!("[{}] Timer started (from {})", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"), addr);
    if let Ok(Some(_)) = db::get_running(&pool).await {
        return (
            StatusCode::CONFLICT,
            Json(serde_json::to_value(ErrorResponse {
                error: "A timer is already running".to_string(),
            }).unwrap()),
        );
    }

    match db::create_entry(&pool).await {
        Ok(entry) => (
            StatusCode::CREATED,
            Json(serde_json::to_value(entry_to_response(entry)).unwrap()),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::to_value(ErrorResponse {
                error: e.to_string(),
            }).unwrap()),
        ),
    }
}

pub async fn stop_timer(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(pool): State<PgPool>,
    Json(body): Json<StopRequest>,
) -> impl IntoResponse {
    println!("[{}] Timer stopped: \"{}\" (from {})", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"), body.description, addr);
    match db::stop_entry(&pool, &body.description).await {
        Ok(Some(entry)) => (
            StatusCode::OK,
            Json(serde_json::to_value(entry_to_response(entry)).unwrap()),
        ),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::to_value(ErrorResponse {
                error: "No running timer".to_string(),
            }).unwrap()),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::to_value(ErrorResponse {
                error: e.to_string(),
            }).unwrap()),
        ),
    }
}

pub async fn timer_status(State(pool): State<PgPool>) -> impl IntoResponse {
    match db::get_running(&pool).await {
        Ok(Some(entry)) => Json(serde_json::to_value(StatusResponse {
            running: true,
            entry: Some(entry_to_response(entry)),
        }).unwrap()),
        Ok(None) => Json(serde_json::to_value(StatusResponse {
            running: false,
            entry: None,
        }).unwrap()),
        Err(e) => Json(serde_json::to_value(ErrorResponse {
            error: e.to_string(),
        }).unwrap()),
    }
}

pub async fn list_entries(State(pool): State<PgPool>) -> impl IntoResponse {
    match db::list_entries_today(&pool).await {
        Ok(entries) => {
            let responses: Vec<EntryResponse> =
                entries.into_iter().map(entry_to_response).collect();
            (StatusCode::OK, Json(serde_json::to_value(responses).unwrap()))
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::to_value(ErrorResponse {
                error: e.to_string(),
            }).unwrap()),
        ),
    }
}

pub async fn delete_entry(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match db::delete_entry(&pool, id).await {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "Entry not found".to_string(),
            }),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: e.to_string(),
            }),
        )
            .into_response(),
    }
}

pub async fn update_entry(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateEntryRequest>,
) -> impl IntoResponse {
    let desc = match body.description {
        Some(d) => d,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::to_value(ErrorResponse {
                    error: "description is required".to_string(),
                }).unwrap()),
            )
        }
    };

    match db::update_entry(&pool, id, &desc).await {
        Ok(Some(entry)) => (
            StatusCode::OK,
            Json(serde_json::to_value(entry_to_response(entry)).unwrap()),
        ),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::to_value(ErrorResponse {
                error: "Entry not found".to_string(),
            }).unwrap()),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::to_value(ErrorResponse {
                error: e.to_string(),
            }).unwrap()),
        ),
    }
}
