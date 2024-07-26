use sqlx::query_as;
use sqlx::PgPool;
use super::model_trait::Model;

#[derive(Debug, sqlx::FromRow)]
pub struct PopeQuote {
    pub id: i32,
    pub pl: String,
    pub en: String
}

impl Model for PopeQuote {
    const NAME_PLURAL: &'static str = "popequotes";

    fn stringify(&self) -> String {
        format!("**id: {}** - `pl: {}` - `en: {}`", self.id, self.pl, self.en)
    }
}

impl PopeQuote {
    pub async fn insert_one(pool: &PgPool, pl: &str, en: &str) -> Result<Self, sqlx::Error> {

        let result = query_as::<_, Self>(
            "INSERT INTO popequote (pl, en) VALUES ($1, $2) RETURNING *",
            )
            .bind(pl)
            .bind(en)
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {

        let results = query_as::<_, Self>(
            "SELECT * FROM popequote"
            )
            .fetch_all(pool)
            .await?;

        Ok(results)
    }

    pub async fn get_random(pool: &PgPool) -> Result<Self, sqlx::Error> {
        let result = query_as::<_, Self>(
            "SELECT * FROM popequote ORDER BY RANDOM() LIMIT 1"
            )
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    pub async fn delete_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        let result = query_as::<_, Self>(
            "DELETE FROM popequote WHERE id = $1 RETURNING *"
            )   
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }

    pub async fn edit(pool: &PgPool, id: i32, new_pl: Option<&str>, new_en: Option<&str>) -> Result<Option<Self>, sqlx::Error> {
        let result = query_as::<_, Self>("
            UPDATE popequote
            SET pl = COALESCE($1, pl),
                en = COALESCE($2, en)
            WHERE id = $3
            RETURNING *
            ")
            .bind(new_pl)
            .bind(new_en)
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(result)
    }
}