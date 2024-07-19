use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

pub struct RequestBuilder {
    params: Request,
    request_timeout: Duration,
}

impl Default for RequestBuilder {
    fn default() -> Self {
        RequestBuilder {
            params: Request::default(),
            request_timeout: Duration::from_secs(5),
        }
    }
}

impl RequestBuilder {
    pub fn new() -> Self {
        RequestBuilder {
            params: Request::default(),
            request_timeout: Duration::from_secs(10),
        }
    }
    pub fn set_key(mut self, key: String) -> Self {
        self.params.key = Some(key);
        self
    }
    pub fn set_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }
    pub fn set_match_mode(mut self, match_mode: i32) -> Self {
        self.params.match_mode = Some(match_mode);
        self
    }

    pub async fn do_request(self) -> Result<Response> {
        let url = self.params.query_url()?;
        let reqwest_cli = reqwest::Client::new();
        let reqwest_response = reqwest_cli
            .get(url)
            .timeout(self.request_timeout)
            .send()
            .await
            .expect("reqwest fail");
        let text = reqwest_response
            .text()
            .await
            .expect("reqwest response text fail");
        serde_json::from_str(&text).with_context(|| "parse error")
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
struct Request {
    #[serde(rename = "0")]
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "1")]
    match_mode: Option<i32>,
}

impl Request {
    const BASE_URL: &'static str = "https://api.sayobot.cn/v2/beatmapinfo";
    pub fn query_url(&self) -> Result<String> {
        let url_params = serde_url_params::to_string(&self)?;
        if url_params.is_empty() {
            return Err(anyhow!("url params is empty"));
        }
        Ok(format!("{}?{}", Request::BASE_URL, url_params))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildInfo {
    #[serde(rename = "AR")]
    pub ar: f64,
    #[serde(rename = "CS")]
    pub cs: f64,
    #[serde(rename = "HP")]
    pub hp: f64,
    #[serde(rename = "OD")]
    pub od: f64,
    pub aim: f64,
    pub audio: String,
    pub bg: String,
    pub bid: i64,
    pub circles: i64,
    pub hit300window: i64,
    pub img: String,
    pub length: i64,
    pub maxcombo: i64,
    pub mode: i64,
    pub passcount: i64,
    pub playcount: i64,
    pub pp: f64,
    pub pp_acc: f64,
    pub pp_aim: f64,
    pub pp_speed: f64,
    pub sliders: i64,
    pub speed: f64,
    pub spinners: i64,
    pub star: f64,
    pub strain_aim: String,
    pub strain_speed: String,
    pub version: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseData {
    pub approved: Option<i64>,

    pub approved_date: Option<i64>,

    pub artist: Option<String>,
    #[serde(rename = "artistU")]
    pub artist_u: Option<String>,
    pub bid_data: Vec<BuildInfo>,
    pub bids_amount: i64,
    pub bpm: f64,
    pub creator: String,
    pub creator_id: i64,
    pub favourite_count: i64,
    pub genre: i32,
    pub language: i32,
    pub last_update: i64,
    pub local_update: i64,
    pub preview: i64,
    pub sid: i64,
    pub source: String,
    pub storyboard: i64,
    pub tags: String,
    pub title: String,
    #[serde(rename = "titleU")]
    pub title_u: String,
    pub video: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub data: ResponseData,
    pub status: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn new_map_test() {
        // get 2035712' info to test.
        let mut search_params = Request::default();
        search_params.key = Some("2035712".to_string());
        let url = Request::query_url(&search_params).unwrap();
        let response = reqwest::get(&url).await.unwrap().text().await.unwrap();
        let search_response: Response = serde_json::from_str(&response).unwrap();
        assert_eq!(search_response.status, 0);
        assert_eq!(
            search_response.data.bids_amount as usize,
            search_response.data.bid_data.len()
        );
    }

    #[tokio::test]
    async fn test_request_builder() {
        let search_response = RequestBuilder::new()
            .set_key("2035712".to_string())
            .do_request()
            .await
            .unwrap();
        assert_eq!(search_response.status, 0);
        assert_eq!(
            search_response.data.bids_amount as usize,
            search_response.data.bid_data.len()
        );
    }
}
