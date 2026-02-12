use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;

pub async fn connect_db() -> Result<sqlx::PgPool, anyhow::Error> {
    dotenvy::dotenv().ok();
    
    // Reads DATABASE_URL from .env file
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");
    
    let pool = sqlx::PgPool::connect(&database_url).await?;
    Ok(pool)
}