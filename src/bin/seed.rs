use axum_sqlx_postgresql_v1::seed::seed_products;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    let pool = axum_sqlx_postgresql_v1::core::db::connect().await?;
    seed_products(&pool).await?;
    println!("🌱 Seeder berhasil dijalankan!");
    Ok(())
} 