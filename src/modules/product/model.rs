use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub name: String,
    pub price: f64,
    pub stock: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
