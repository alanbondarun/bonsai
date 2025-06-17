use axum::extract::State;
use sqlx::PgPool;

use crate::error::ApiError;

pub async fn test_db(State(db_conn): State<PgPool>) -> Result<String, ApiError> {
    let result = sqlx::query!(r"SELECT 'Hello, world!' AS message")
        .fetch_one(&db_conn)
        .await
        .map_err(|_| ApiError::DatabaseError)?;

    let message = result.message.ok_or(ApiError::DatabaseError)?;

    Ok(message)
}
