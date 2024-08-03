use serde::Deserialize;
use crate::Error;


#[derive(Debug, Clone, Deserialize)]
pub struct PopeQuote {
    pub id: i64,
    pub quote: String,
    pub translation: String
}

impl PopeQuote {
    pub const API_LINK: &'static str = "https://jp2cenzoapi.onrender.com";

    fn route(path: &str) -> String {
        format!("{}{}", Self::API_LINK, path)
    }

    pub async fn get_random(client: &reqwest::Client) -> Result<Self, Error> {
        let p = client.get(Self::route("/api/quotes/random"))
            .send()
            .await?
            .json::<Self>()
            .await?;

        Ok(p)
    }
}