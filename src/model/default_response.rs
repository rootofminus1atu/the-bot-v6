use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use super::boilerplate::BoilerplateForStringListTables;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DefaultResponse {
    pub id: i64,
    pub content: String
}

impl BoilerplateForStringListTables for DefaultResponse {
    const TABLE_NAME: &'static str = "testing.prophecy";
    const FIELD_NAME: &'static str = "content";
}
