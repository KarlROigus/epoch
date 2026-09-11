use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct StopRequest {
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateEntryRequest {
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryResponse {
    pub id: Uuid,
    pub description: Option<String>,
    pub started_at: NaiveDateTime,
    pub stopped_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusResponse {
    pub running: bool,
    pub entry: Option<EntryResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}
