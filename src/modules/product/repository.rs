use async_trait::async_trait;
use sqlx::{
    PgPool, 
    Error as SqlxError, 
    postgres::PgRow, 
    Row,
};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::{model::Product, dto::UpdateProductDto};

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn create(&self, name: &str, price: f64, stock: i32) -> Result<Product, SqlxError>;
    async fn find_all(&self) -> Result<Vec<Product>, SqlxError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<Product>, SqlxError>;
    async fn update(&self, id: &str, dto: UpdateProductDto) -> Result<Product, SqlxError>;
    async fn delete(&self, id: &str) -> Result<u64, SqlxError>;
}

pub struct ProductRepositoryImpl {
    pool: PgPool,
}

impl ProductRepositoryImpl {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    fn parse_uuid(id: &str) -> Result<Uuid, SqlxError> {
        Uuid::parse_str(id)
            .map_err(|e| SqlxError::Decode(Box::new(e)))
    }

    fn map_row_to_product(row: PgRow) -> Product {
        Product {
            id: row.get("id"),
            name: row.get("name"),
            price: row.get::<f64, _>("price"),
            stock: row.get("stock"),
            created_at: row.get::<DateTime<Utc>, _>("created_at"),
            updated_at: row.get::<DateTime<Utc>, _>("updated_at"),
        }
    }
}

#[async_trait]
impl ProductRepository for ProductRepositoryImpl {
    async fn create(&self, name: &str, price: f64, stock: i32) -> Result<Product, SqlxError> {
        let now = Utc::now();
        
        sqlx::query(
            r#"
            INSERT INTO products (name, price, stock, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(name)
        .bind(price)
        .bind(stock)
        .bind(now)
        .bind(now)
        .map(Self::map_row_to_product)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_all(&self) -> Result<Vec<Product>, SqlxError> {
        sqlx::query("SELECT * FROM products ORDER BY created_at DESC")
            .map(Self::map_row_to_product)
            .fetch_all(&self.pool)
            .await
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Product>, SqlxError> {
        let sqlx_uuid = Self::parse_uuid(id)?;
        
        sqlx::query("SELECT * FROM products WHERE id = $1")
            .bind(sqlx_uuid)
            .map(Self::map_row_to_product)
            .fetch_optional(&self.pool)
            .await
    }

    async fn update(&self, id: &str, dto: UpdateProductDto) -> Result<Product, SqlxError> {
        let sqlx_uuid = Self::parse_uuid(id)?;
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE products 
            SET name = COALESCE($1, name),
                price = COALESCE($2, price),
                stock = COALESCE($3, stock),
                updated_at = $4
            WHERE id = $5
            RETURNING *
            "#,
        )
        .bind(dto.name)
        .bind(dto.price)
        .bind(dto.stock)
        .bind(now)
        .bind(sqlx_uuid)
        .map(Self::map_row_to_product)
        .fetch_one(&self.pool)
        .await
    }

    async fn delete(&self, id: &str) -> Result<u64, SqlxError> {
        let sqlx_uuid = Self::parse_uuid(id)?;
        
        sqlx::query("DELETE FROM products WHERE id = $1")
            .bind(sqlx_uuid)
            .execute(&self.pool)
            .await
            .map(|result| result.rows_affected())
    }
}