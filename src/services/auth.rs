use sqlx::PgPool;
use crate::models::*;
use anyhow::Result;

pub async fn authenticate_user(
    pool: &PgPool,
    request: LoginRequest,
) -> Result<LoginResponse> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT 
            id, email, password_hash, role as "role: UserRole", 
            first_name, last_name, phone, is_active, last_login, created_at, updated_at
        FROM users 
        WHERE email = $1 AND is_active = true
        "#,
        request.email
    )
    .fetch_optional(pool)
    .await?;

    if let Some(user) = user {
        // Verify password (in real implementation, use bcrypt)
        if verify_password(&request.password, &user.password_hash) {
            // Generate JWT token (simplified)
            let token = generate_jwt_token(&user)?;
            
            // Update last login
            sqlx::query!(
                "UPDATE users SET last_login = NOW() WHERE id = $1",
                user.id
            )
            .execute(pool)
            .await?;

            return Ok(LoginResponse {
                token,
                user: UserInfo {
                    id: user.id,
                    email: user.email,
                    role: user.role,
                    first_name: user.first_name,
                    last_name: user.last_name,
                },
            });
        }
    }

    Err(anyhow::anyhow!("Invalid credentials"))
}

pub async fn register_user(
    pool: &PgPool,
    request: serde_json::Value,
) -> Result<UserInfo> {
    // Implementation for user registration
    // This would include password hashing, validation, etc.
    Err(anyhow::anyhow!("Not implemented"))
}

fn verify_password(password: &str, hash: &str) -> bool {
    // In real implementation, use bcrypt::verify
    password == "password" // Simplified for demo
}

fn generate_jwt_token(user: &User) -> Result<String> {
    // In real implementation, use jsonwebtoken crate
    Ok(format!("jwt_token_for_{}", user.id)) // Simplified for demo
}
