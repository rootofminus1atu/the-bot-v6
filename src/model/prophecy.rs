use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Prophecy {
    pub id: i64,
    pub content: String
}

impl Prophecy {
    const TABLE_NAME: &'static str = "testing.prophecy";

    pub async fn get_all(db: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let query = format!("SELECT * FROM {}", Self::TABLE_NAME);
        let res = sqlx::query_as::<_, Self>(&query)
            .fetch_all(db)
            .await?;

        Ok(res)
    }

    pub async fn get_random(db: &PgPool) -> Result<Self, sqlx::Error> {
        let query = format!("SELECT * FROM {} ORDER BY RANDOM() LIMIT 1", Self::TABLE_NAME);
        let res = sqlx::query_as::<_, Self>(&query)
            .fetch_one(db)
            .await?;

        Ok(res)
    }

    pub async fn insert(db: &PgPool, prophecy: String) -> Result<Self, sqlx::Error> {
        let query = format!("INSERT INTO {} (prophecy) VALUES ($1) RETURNING *", Self::TABLE_NAME);
        let res = sqlx::query_as::<_, Self>(&query)
            .bind(prophecy)
            .fetch_one(db)
            .await?;

        Ok(res)
    }

    pub async fn delete(db: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        let query = format!("DELETE FROM {} WHERE id = $1 RETURNING *", Self::TABLE_NAME);
        let res = sqlx::query_as::<_, Self>(&query)
            .bind(id)
            .fetch_optional(db)
            .await?;
    
        Ok(res)
    }

    // update here in the future
}