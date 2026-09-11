use chrono::NaiveDateTime;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(FromRow)]
pub struct Entry {
    pub id: Uuid,
    pub description: Option<String>,
    pub started_at: NaiveDateTime,
    pub stopped_at: Option<NaiveDateTime>,
}

pub async fn get_running(pool: &PgPool) -> Result<Option<Entry>, sqlx::Error> {
    sqlx::query_as::<_, Entry>(
        "SELECT id, description, started_at, stopped_at FROM entries WHERE stopped_at IS NULL",
    )
    .fetch_optional(pool)
    .await
}

pub async fn create_entry(pool: &PgPool) -> Result<Entry, sqlx::Error> {
    sqlx::query_as::<_, Entry>(
        "INSERT INTO entries DEFAULT VALUES RETURNING id, description, started_at, stopped_at",
    )
    .fetch_one(pool)
    .await
}

pub async fn stop_entry(pool: &PgPool, description: &str) -> Result<Option<Entry>, sqlx::Error> {
    sqlx::query_as::<_, Entry>(
        "UPDATE entries SET stopped_at = now(), description = $1, updated_at = now() WHERE stopped_at IS NULL RETURNING id, description, started_at, stopped_at",
    )
    .bind(description)
    .fetch_optional(pool)
    .await
}

pub async fn list_entries_today(pool: &PgPool) -> Result<Vec<Entry>, sqlx::Error> {
    sqlx::query_as::<_, Entry>(
        "SELECT id, description, started_at, stopped_at FROM entries WHERE started_at::date = CURRENT_DATE ORDER BY started_at DESC",
    )
    .fetch_all(pool)
    .await
}

pub async fn delete_entry(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM entries WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn update_entry(
    pool: &PgPool,
    id: Uuid,
    description: &str,
) -> Result<Option<Entry>, sqlx::Error> {
    sqlx::query_as::<_, Entry>(
        "UPDATE entries SET description = $1, updated_at = now() WHERE id = $2 RETURNING id, description, started_at, stopped_at",
    )
    .bind(description)
    .bind(id)
    .fetch_optional(pool)
    .await
}
