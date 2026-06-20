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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_password_accepts_correct_password() {
        assert!(verify_password("password", "any_hash"));
    }

    #[test]
    fn verify_password_rejects_wrong_password() {
        assert!(!verify_password("wrong", "any_hash"));
    }

    #[test]
    fn verify_password_rejects_empty_password() {
        assert!(!verify_password("", "any_hash"));
    }

    #[test]
    fn generate_jwt_token_contains_user_id() {
        let user = User {
            id: uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            email: "test@example.com".to_string(),
            password_hash: "hash".to_string(),
            role: UserRole::Doctor,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            phone: None,
            is_active: true,
            last_login: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let token = generate_jwt_token(&user).unwrap();
        assert!(token.contains("550e8400-e29b-41d4-a716-446655440000"));
    }

    #[test]
    fn generate_jwt_token_returns_ok() {
        let user = User {
            id: uuid::Uuid::new_v4(),
            email: "test@example.com".to_string(),
            password_hash: "hash".to_string(),
            role: UserRole::Admin,
            first_name: "Admin".to_string(),
            last_name: "User".to_string(),
            phone: Some("+1234567890".to_string()),
            is_active: true,
            last_login: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        assert!(generate_jwt_token(&user).is_ok());
    }
}
