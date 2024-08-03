use sqlx::{FromRow, PgPool, Type};
use serde::{Serialize, Deserialize};


// location for timed messages, such as the timed 2137 pope message, or randomly timed prophecies


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Location {
    pub guild_id: i64,
    pub channel_id: i64,
    pub kind: LocationKind
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "location_kind")]
#[sqlx(rename_all = "snake_case")] 
pub enum LocationKind {
    PopeMsg,
    Clairvoyance
}

impl Location {
    pub const TABLE_NAME: &'static str = "testing.location_v2";

    pub async fn get_all(db: &PgPool, kind: LocationKind) -> Result<Vec<Self>, sqlx::Error> {
        let query = format!(
            "SELECT * FROM {} WHERE kind = $1", 
            Self::TABLE_NAME
        );
        
        let res = sqlx::query_as::<_, Self>(&query)
            .bind(kind)
            .fetch_all(db)
            .await?;

        Ok(res)
    }

    pub async fn insert(db: &PgPool, location: &Location) -> Result<Self, sqlx::Error> {
        let query = format!(
            "INSERT INTO {} (guild_id, channel_id, kind) VALUES ($1, $2, $3) RETURNING *",
            Self::TABLE_NAME
        );

        let res = sqlx::query_as::<_, Self>(&query)
            .bind(location.guild_id)
            .bind(location.channel_id)
            .bind(&location.kind)
            .fetch_one(db)
            .await?;

        Ok(res)
    }

    pub async fn delete(db: &PgPool, location: &Location) -> Result<Option<Self>, sqlx::Error> {
        let query = format!(
            "DELETE FROM {} WHERE guild_id = $1 AND channel_id = $2 AND kind = $3 RETURNING *",
            Self::TABLE_NAME
        );

        let res = sqlx::query_as::<_, Self>(&query)
            .bind(location.guild_id)
            .bind(location.channel_id)
            .bind(&location.kind)
            .fetch_optional(db)
            .await?;

        Ok(res)
    }

    pub async fn toggle(db: &PgPool, location: &Location) -> Result<(ToggleAction, Self), sqlx::Error> {
        let query = format!(
            "SELECT * FROM {} WHERE guild_id = $1 AND channel_id = $2 AND kind = $3", 
            Self::TABLE_NAME
        );

        let existing_location = sqlx::query_as::<_, Self>(&query)
            .bind(location.guild_id)
            .bind(location.channel_id)
            .bind(&location.kind)
            .fetch_optional(db)
            .await?;

        if let Some(_) = existing_location {
            let delete_query = format!(
                "DELETE FROM {} WHERE guild_id = $1 AND channel_id = $2 AND kind = $3 RETURNING *",
                Self::TABLE_NAME
            );

            let res = sqlx::query_as::<_, Self>(&delete_query)
                .bind(location.guild_id)
                .bind(location.channel_id)
                .bind(&location.kind)
                .fetch_one(db)
                .await?;

            Ok((ToggleAction::Deleted, res))
        } else {
            let insert_query = format!(
                "INSERT INTO {} (guild_id, channel_id, kind) VALUES ($1, $2, $3) RETURNING *",
                Self::TABLE_NAME
            );

            let res = sqlx::query_as::<_, Self>(&insert_query)
                .bind(location.guild_id)
                .bind(location.channel_id)
                .bind(&location.kind)
                .fetch_one(db)
                .await?;

            Ok((ToggleAction::Inserted, res))
        }
    }
}


pub enum ToggleAction {
    Inserted,
    Deleted,
}