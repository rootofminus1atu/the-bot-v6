use std::collections::VecDeque;
use std::sync::Arc;
use chrono::{DateTime, Utc};
use serde::Serialize;
use tokio::sync::Mutex as AsyncMutex;
use serde_json::json;
use tracing::info;
use crate::Error;




/// TODO:
/// - fix the double list reversal issue, here and in the API
#[derive(Debug)]
pub struct Cleverbot {
    api_link: String,
    api_key: String,
    cookie: Arc<AsyncMutex<Option<String>>>,  // Arc<AsyncMutex<String>>  if I figure out encoding/decoding issues
    context: Arc<AsyncMutex<MaxQueue<String>>>,
    client: reqwest::Client
}


impl Cleverbot {
    pub fn new(api_key: String, api_link: String, max: usize) -> Self {
        Cleverbot {
            api_link,
            api_key,
            cookie: Arc::new(AsyncMutex::new(None)),  // Arc::new(AsyncMutex::new(Self::generate_cookie().await?)),  // if I figure out encoding
            context: Arc::new(AsyncMutex::new(MaxQueue::new(max))),
            client: reqwest::Client::new()
        }
    }

    pub async fn generate_cookie(&self) -> Result<String, Error> {
        // because of reqwest encoding decoing bs I'm just going with this for now idc what other people think
        // previous: "XVIS=TE1939AFFIAGAYQZD8D31"
        // previous: "XVIS=TE1939AFFIAGAYQZA6731"
        // previous: "XVIS=TE1939AFFIAGAYQZPOO31"
        // previous: "XVIS=TE1939AFFIAGAYQZWEE31"
        let new_cookie = get_cookie().await?;
        let mut l = self.cookie.lock().await;
        *l = Some(new_cookie.clone());
        Ok(new_cookie)
    }

    pub async fn clear_context(&self) {
        self.context.lock().await.clear();
    }

    pub async fn get_context(&self) -> MaxQueue<String> {
        self.context.lock().await.clone()
    }

    pub async fn get_response(&self, stimulus: &str) -> Result<String, Error> {
        let req_params = serde_json::to_string(&json!({
            "stimulus": stimulus,
            "context": self.context.lock().await.list,  // might have to .clone() if some weird buggy behavior occurs
        }))?;

        let cookie = &self.cookie.lock().await.clone().ok_or("Please give me cookies first :(")?;

        let response = self.client
            .post(&self.api_link)
            .header("cookie", cookie)
            .header("clevreq-api-key", &self.api_key)
            .body(req_params)
            .send()
            .await?
            .text()
            .await?;

        let cleared_response = response.trim_matches('"');

        let mut c = self.context.lock().await;
        c.push_front(stimulus.to_string());
        c.push_front(cleared_response.to_string());

        Ok(cleared_response.into())
    }
}



fn get_date() -> String {
    let now: DateTime<Utc> = Utc::now();
    println!("now: {}", now.format("%Y%m%d").to_string());
    now.format("%Y%m%d").to_string()
}

async fn get_cookie() -> Result<String, Error> {
    let url = format!("https://www.cleverbot.com/extras/conversation-social-min.js?{}", get_date());
    let resp = reqwest::get(&url).await?;

    // let mut headers = HeaderMap::new();
    // headers.insert(USER_AGENT, HeaderValue::from_static(""));

    // let resp = reqwest::Client::new()
    //     .get(&url)
    //     .headers(headers)
    //     .send()
    //     .await?;

    let cookie_before = resp.headers()
        .get("set-cookie")
        .and_then(|s| s.to_str().ok())
        .and_then(|s| s.split(';').next());

    let cookie_str = cookie_before
        .map(|s| s.replace("B%", "31"));  // i have no idea why 31 works, but it's the only one that does

    info!("new cookie before: {:?}", cookie_before);
    info!("new cookie after:  {:?}", cookie_str);

    cookie_str.ok_or("No cookie found".into())
}


#[derive(Debug, Clone, Serialize)]
pub struct MaxQueue<T> {
    max: usize,
    pub list: VecDeque<T>,
}

impl<T> MaxQueue<T> {
    pub fn new(max: usize) -> MaxQueue<T> {
        MaxQueue {
            max,
            list: VecDeque::with_capacity(max),
        }
    }

    pub fn push_front(&mut self, item: T) {
        if self.list.len() == self.max {
            self.list.pop_back();
        }
        self.list.push_front(item);
    }

    pub fn push_back(&mut self, item: T) {
        if self.list.len() == self.max {
            self.list.pop_front();
        }
        self.list.push_back(item);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.list.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.list.pop_back()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn get_all(&self) -> &VecDeque<T> {
        &self.list
    }
}