use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PopeMsgLocation {
    pub guild_id: i64,
    pub channel_id: i64
}

impl PopeMsgLocation {
    const TABLE_NAME: &str = "testing.pope_msg_location";

    pub async fn get_all(db: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let query = format!("SELECT * FROM {}", Self::TABLE_NAME);
        let res = sqlx::query_as::<_, Self>(&query)
            .fetch_all(db)
            .await?;

        Ok(res)
    }

    pub async fn insert(db: &PgPool, guild_id: i64, channel_id: i64) -> Result<Self, sqlx::Error> {
        let query = format!("INSERT INTO {} (guild_id, channel_id) VALUES ($1, $2) RETURNING *", Self::TABLE_NAME);
        let res = sqlx::query_as::<_, Self>(&query)
            .bind(guild_id)
            .bind(channel_id)
            .fetch_one(db)
            .await?;

        Ok(res)
    }

    pub async fn delete(db: &PgPool, guild_id: i64, channel_id: i64) -> Result<Option<Self>, sqlx::Error> {
        let query = format!("DELETE FROM {} WHERE guild_id = $1 AND channel_id = $2 RETURNING *", Self::TABLE_NAME);
        let res = sqlx::query_as::<_, Self>(&query)
            .bind(guild_id)
            .bind(channel_id)
            .fetch_optional(db)
            .await?;
    
        Ok(res)
    }
}