use sqlx::{Error, PgPool};

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    PgPool::connect(&database_url).await
}

pub async fn ping_db(pool: &PgPool) -> Result<(), Error> {
    sqlx::query("SELECT 1").execute(pool).await?;
    Ok(())
}
