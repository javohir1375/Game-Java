use sqlx::sqlite::SqlitePool;

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    // TODO: Initialize SQLite database
    // Create connection pool
    // Run migrations
    
    let pool = SqlitePool::connect("sqlite:game.db").await?;
    
    // Create tables
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            level INTEGER DEFAULT 1,
            experience INTEGER DEFAULT 0,
            coins INTEGER DEFAULT 0,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(&pool)
    .await?;
    
    Ok(pool)
}
