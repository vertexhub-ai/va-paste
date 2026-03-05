use axum::{extract::{Path, State}, Json, http::StatusCode};
use sqlx::PgPool;
use uuid::Uuid;
use crate::models::{Paste, CreatePaste};
use crate::error::AppError;
use chrono::{Utc, Duration};

pub async fn create_paste(
    State(pool): State<PgPool>,
    Json(payload): Json<CreatePaste>,
) -> Result<(StatusCode, Json<Paste>), AppError> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let expires_at = payload.expires_in_seconds.map(|s| now + Duration::seconds(s));

    let paste = sqlx::query_as::<_, Paste>(
        "INSERT INTO pastes (id, content, title, created_at, expires_at) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(id)
    .bind(payload.content)
    .bind(payload.title)
    .bind(now)
    .bind(expires_at)
    .fetch_one(&pool)
    .await?;

    Ok((StatusCode::CREATED, Json(paste)))
}

pub async fn get_paste(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Paste>, AppError> {
    let paste = sqlx::query_as::<_, Paste>(
        "SELECT * FROM pastes WHERE id = $1 AND (expires_at IS NULL OR expires_at > NOW())"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await?;

    match paste {
        Some(p) => Ok(Json(p)),
        None => Err(AppError::NotFound),
    }
}
