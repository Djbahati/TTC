use sqlx::PgPool;
use crate::models::*;
use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,       // user id
    pub email: String,
    pub role: String,
    pub exp: usize,        // expiry timestamp
    pub iat: usize,        // issued at
}

fn get_jwt_secret() -> String {
    std::env::var("JWT_SECRET")
        .expect("JWT_SECRET environment variable must be set")
}

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
    let email = request["email"].as_str()
        .ok_or_else(|| anyhow::anyhow!("Email is required"))?;
    let password = request["password"].as_str()
        .ok_or_else(|| anyhow::anyhow!("Password is required"))?;
    let first_name = request["first_name"].as_str()
        .ok_or_else(|| anyhow::anyhow!("First name is required"))?;
    let last_name = request["last_name"].as_str()
        .ok_or_else(|| anyhow::anyhow!("Last name is required"))?;
    let role = request["role"].as_str().unwrap_or("patient");

    // Validate password strength
    if password.len() < 8 {
        return Err(anyhow::anyhow!("Password must be at least 8 characters"));
    }

    let password_hash = hash_password(password)?;

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, password_hash, role, first_name, last_name)
        VALUES ($1, $2, $3::user_role, $4, $5)
        RETURNING 
            id, email, password_hash, role as "role: UserRole",
            first_name, last_name, phone, is_active, last_login, created_at, updated_at
        "#,
        email,
        password_hash,
        role,
        first_name,
        last_name
    )
    .fetch_one(pool)
    .await?;

    Ok(UserInfo {
        id: user.id,
        email: user.email,
        role: user.role,
        first_name: user.first_name,
        last_name: user.last_name,
    })
}

fn verify_password(password: &str, password_hash: &str) -> bool {
    verify(password, password_hash).unwrap_or(false)
}

pub fn hash_password(password: &str) -> Result<String> {
    Ok(hash(password, DEFAULT_COST)?)
}

fn generate_jwt_token(user: &User) -> Result<String> {
    let secret = get_jwt_secret();
    let now = Utc::now();
    let expiration = now + Duration::hours(24);

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        role: format!("{:?}", user.role).to_lowercase(),
        exp: expiration.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

pub fn validate_jwt_token(token: &str) -> Result<Claims> {
    let secret = get_jwt_secret();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_data.claims)
}
