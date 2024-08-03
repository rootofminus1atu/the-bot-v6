use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use async_trait::async_trait;



#[async_trait]
pub trait BoilerplateForStringListTables: Sized + for<'r> FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin + Serialize + for<'d> Deserialize<'d> {
    const TABLE_NAME: &'static str;
    const FIELD_NAME: &'static str;

    async fn get_all(db: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let query = format!("SELECT * FROM {}", Self::TABLE_NAME);
        let res = sqlx::query_as::<_, Self>(&query)
            .fetch_all(db)
            .await?;

        Ok(res)
    }

    async fn get_random(db: &PgPool) -> Result<Self, sqlx::Error> {
        let query = format!("SELECT * FROM {} ORDER BY RANDOM() LIMIT 1", Self::TABLE_NAME);
        let res = sqlx::query_as::<_, Self>(&query)
            .fetch_one(db)
            .await?;

        Ok(res)
    }

    async fn insert(db: &PgPool, content: String) -> Result<Self, sqlx::Error> {
        let query = format!("INSERT INTO {} ({}) VALUES ($1) RETURNING *", Self::TABLE_NAME, Self::FIELD_NAME);
        let res = sqlx::query_as::<_, Self>(&query)
            .bind(content)
            .fetch_one(db)
            .await?;

        Ok(res)
    }

    async fn delete(db: &PgPool, id: i64) -> Result<Option<Self>, sqlx::Error> {
        let query = format!("DELETE FROM {} WHERE id = $1 RETURNING *", Self::TABLE_NAME);
        let res = sqlx::query_as::<_, Self>(&query)
            .bind(id)
            .fetch_optional(db)
            .await?;
    
        Ok(res)
    }
}
