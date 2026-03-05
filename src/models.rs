use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Paste {
    pub id: Uuid,
    pub content: String,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePaste {
    pub content: String,
    pub title: Option<String>,
    pub expires_in_seconds: Option<i64>,
}
