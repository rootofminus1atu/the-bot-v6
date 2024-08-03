use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};

// #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
// pub struct PopeMsgLocation {
//     pub guild_id: i64,
//     pub channel_id: i64
// }

// impl PopeMsgLocation {
//     const TABLE_NAME: &'static str = "testing.pope_msg_location";

//     pub async fn get_all(db: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
//         let query = format!("SELECT * FROM {}", Self::TABLE_NAME);
//         let res = sqlx::query_as::<_, Self>(&query)
//             .fetch_all(db)
//             .await?;

//         Ok(res)
//     }

//     pub async fn insert(db: &PgPool, guild_id: i64, channel_id: i64) -> Result<Self, sqlx::Error> {
//         let query = format!("INSERT INTO {} (guild_id, channel_id) VALUES ($1, $2) RETURNING *", Self::TABLE_NAME);
//         let res = sqlx::query_as::<_, Self>(&query)
//             .bind(guild_id)
//             .bind(channel_id)
//             .fetch_one(db)
//             .await?;

//         Ok(res)
//     }

//     pub async fn delete(db: &PgPool, guild_id: i64, channel_id: i64) -> Result<Option<Self>, sqlx::Error> {
//         let query = format!("DELETE FROM {} WHERE guild_id = $1 AND channel_id = $2 RETURNING *", Self::TABLE_NAME);
//         let res = sqlx::query_as::<_, Self>(&query)
//             .bind(guild_id)
//             .bind(channel_id)
//             .fetch_optional(db)
//             .await?;
    
//         Ok(res)
//     }
// }







pub trait LocationContext {
    const TABLE_NAME: &'static str;
}

pub struct PopeMsgCtx;
impl LocationContext for PopeMsgCtx {
    const TABLE_NAME: &'static str = "testing.pope_msg_location";
}

pub struct ClairvoyanceCtx;
impl LocationContext for ClairvoyanceCtx {
    const TABLE_NAME: &'static str = "testing.clairvoyance_location";
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Location {
    pub guild_id: i64,
    pub channel_id: i64,
}

impl Location {
    pub async fn get_all<C: LocationContext>(db: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        let query = format!("SELECT guild_id, channel_id FROM {}", C::TABLE_NAME);
        let res = sqlx::query_as::<_, Self>(&query)
            .fetch_all(db)
            .await?;

        Ok(res)
    }

    pub async fn insert<C: LocationContext>(db: &PgPool, guild_id: i64, channel_id: i64) -> Result<Self, sqlx::Error> {
        let query = format!("INSERT INTO {} (guild_id, channel_id) VALUES ($1, $2) RETURNING *", C::TABLE_NAME);
        let res = sqlx::query_as::<_, Self>(&query)
            .bind(guild_id)
            .bind(channel_id)
            .fetch_one(db)
            .await?;

        Ok(res)
    }

    pub async fn delete<C: LocationContext>(db: &PgPool, guild_id: i64, channel_id: i64) -> Result<Option<Self>, sqlx::Error> {
        let query = format!("DELETE FROM {} WHERE guild_id = $1 AND channel_id = $2 RETURNING *", C::TABLE_NAME);
        let res = sqlx::query_as::<_, Self>(&query)
            .bind(guild_id)
            .bind(channel_id)
            .fetch_optional(db)
            .await?;
    
        Ok(res)
    }
}

