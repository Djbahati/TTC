use sqlx::PgPool;
use uuid::Uuid;
use crate::models::*;
use anyhow::Result;

pub async fn get_user_messages(pool: &PgPool, user_id: Uuid) -> Result<Vec<Message>> {
    let messages = sqlx::query_as!(
        Message,
        r#"
        SELECT id, sender_id, recipient_id, message_type, content, is_read, is_urgent, created_at
        FROM messages 
        WHERE recipient_id = $1 OR sender_id = $1
        ORDER BY created_at DESC
        LIMIT 50
        "#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(messages)
}

pub async fn send_message(
    pool: &PgPool,
    sender_id: Uuid,
    request: SendMessageRequest,
) -> Result<Message> {
    let message = sqlx::query_as!(
        Message,
        r#"
        INSERT INTO messages (sender_id, recipient_id, content, is_urgent)
        VALUES ($1, $2, $3, $4)
        RETURNING id, sender_id, recipient_id, message_type, content, is_read, is_urgent, created_at
        "#,
        sender_id,
        request.recipient_id,
        request.content,
        request.is_urgent.unwrap_or(false)
    )
    .fetch_one(pool)
    .await?;

    Ok(message)
}

pub async fn mark_message_as_read(pool: &PgPool, message_id: Uuid) -> Result<()> {
    sqlx::query!(
        "UPDATE messages SET is_read = true WHERE id = $1",
        message_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
