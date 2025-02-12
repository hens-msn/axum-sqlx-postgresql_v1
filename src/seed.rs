use sqlx::{PgPool, Error};
use serde::Deserialize;
use include_dir::{include_dir, Dir};

static DATA_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/data");

#[derive(Debug, Deserialize)]
struct SeedProduct {
    name: String,
    price: f64,
    stock: i32,
}

pub async fn seed_products(pool: &PgPool) -> Result<(), Error> {
    // Baca file JSON
    let json_file = DATA_DIR.get_file("products.json")
        .ok_or_else(|| Error::Configuration("File products.json tidak ditemukan 😱".into()))?;
    
    let products: Vec<SeedProduct> = serde_json::from_slice(json_file.contents())
        .map_err(|e| Error::Configuration(format!("Gagal parse JSON: {}", e).into()))?;

    // Bersihin data lama
    sqlx::query("TRUNCATE TABLE products RESTART IDENTITY CASCADE")
        .execute(pool)
        .await?;

    // Insert data baru
    for product in products {
        sqlx::query!(
            r#"
            INSERT INTO products (name, price, stock)
            VALUES ($1, $2, $3)
            "#,
            product.name,
            product.price,
            product.stock
        )
        .execute(pool)
        .await?;
    }

    println!("✅ Seeder dari JSON berhasil!");
    Ok(())
} 