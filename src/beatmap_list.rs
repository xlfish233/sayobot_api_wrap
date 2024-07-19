use std::string::String;
use std::time::Duration;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::enums::{Class, GameMode, Genre, Language, SubType};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RequestType {
    #[serde(rename = "1")]
    Hot,
    #[serde(rename = "2")]
    New,
    #[serde(rename = "3")]
    Packs,
    #[serde(rename = "4")]
    Search,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Range {
    pub start: f64,
    pub end: f64,
}

impl Range {
    pub fn new(start: f64, end: f64) -> Result<Self> {
        //in this area. range is from 0 to 9999
        if start > end {
            return Err(anyhow!("start must be less than end"));
        }
        if start < 0.0 || end > 9999.0 {
            return Err(anyhow!("start and end must be between 0 and 9999"));
        }
        Ok(Self { start, end })
    }
}

pub struct RequestBuilder {
    params: SearchParams,
    request_timeout: Duration,
}

impl Default for RequestBuilder {
    fn default() -> Self {
        Self {
            params: SearchParams::default(),
            request_timeout: Duration::from_secs(5),
        }
    }
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self {
            params: SearchParams::default(),
            request_timeout: Duration::from_secs(5),
        }
    }
    pub fn set_request_type(mut self, request_type: RequestType) -> Self {
        self.params.request_type = Some(request_type);
        self
    }
    pub fn set_limit(mut self, limit: i32) -> Self {
        self.params.limit = Some(limit);
        self
    }
    pub fn set_offset(mut self, offset: i32) -> Self {
        self.params.offset = Some(offset);
        self
    }
    pub fn set_keyword(mut self, keyword: String) -> Self {
        self.params.keyword = Some(keyword);
        self
    }
    pub fn set_sub_type(mut self, sub_type: SubType) -> Self {
        self.params.sub_type = Some(sub_type.bits());
        self
    }
    pub fn set_mode(mut self, mode: GameMode) -> Self {
        self.params.mode = Some(mode.bits());
        self
    }
    pub fn set_class(mut self, class: Class) -> Self {
        self.params.class = Some(class.bits());
        self
    }
    pub fn set_genre(mut self, genre: Genre) -> Self {
        self.params.genre = Some(genre.bits());
        self
    }
    pub fn set_language(mut self, language: Language) -> Self {
        self.params.language = Some(language.bits());
        self
    }
    pub fn set_starts_range(mut self, range: Range) -> Self {
        self.params.stars = Some(range);
        self
    }
    pub fn set_ar_range(mut self, range: Range) -> Self {
        self.params.ar = Some(range);
        self
    }
    pub fn set_od_range(mut self, range: Range) -> Self {
        self.params.od = Some(range);
        self
    }
    pub fn set_cs_range(mut self, range: Range) -> Self {
        self.params.cs = Some(range);
        self
    }
    pub fn set_hp_range(mut self, range: Range) -> Self {
        self.params.hp = Some(range);
        self
    }
    pub fn set_bpm_range(mut self, range: Range) -> Self {
        self.params.bpm = Some(range);
        self
    }
    pub fn set_length_range(mut self, range: Range) -> Self {
        self.params.length = Some(range);
        self
    }

    pub async fn do_request(mut self) -> Result<SearchResponse> {
        self.build_other_string();

        let request_url = self.params.query_url()?;

        let reqwest_cli = reqwest::Client::new();
        let resp = reqwest_cli
            .get(request_url)
            .timeout(self.request_timeout)
            .send()
            .await
            .expect("reqwest fail");

        if !resp.status().is_success() {
            return Err(anyhow!("request error: {}", resp.status()));
        }
        if resp.status().is_success() {
            let resp_text = resp.text().await?;
            let resp_json: SearchResponse = serde_json::from_str(&resp_text)
                .expect("parse error");
            return Ok(resp_json);
        }
        Err(anyhow!("other error"))
    }

    fn build_other_string(&mut self) {
        let mut other_value = String::new();
        if let Some(r) = self.params.stars {
            other_value += &format!("star:{}~{},", r.start, r.end);
        }
        if let Some(r) = self.params.od {
            other_value += &format!("od:{}~{},", r.start, r.end);
        }
        if let Some(r) = self.params.od {
            other_value += &format!("od:{}~{},", r.start, r.end);
        }
        if let Some(r) = self.params.cs {
            other_value += &format!("cs:{}~{},", r.start, r.end);
        }
        if let Some(r) = self.params.hp {
            other_value += &format!("hp:{}~{},", r.start, r.end);
        }
        if let Some(r) = self.params.bpm {
            other_value += &format!("bpm:{}~{},", r.start, r.end);
        }
        if let Some(r) = self.params.length {
            other_value += &format!("length:{}~{},", r.start, r.end);
        }
        if other_value.is_empty() {
            self.params.other = None;
        } else {
            other_value += "end";
            self.params.other = Some(other_value);
        }
    }
    pub fn set_time_out(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct SearchParams {
    #[serde(rename = "L")]
    limit: Option<i32>,
    #[serde(rename = "O")]
    offset: Option<i32>,
    #[serde(rename = "T")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    request_type: Option<RequestType>,
    #[serde(rename = "K")]
    keyword: Option<String>,
    #[serde(rename = "S")]
    sub_type: Option<u32>,
    #[serde(rename = "M")]
    mode: Option<u32>,
    #[serde(rename = "C")]
    class: Option<u32>,
    #[serde(rename = "G")]
    genre: Option<u32>,
    #[serde(rename = "E")]
    language: Option<u32>,
    #[serde(rename = "R")]
    #[serde(skip_serializing_if = "Option::is_none")]
    other: Option<String>,

    #[serde(skip_serializing)]
    stars: Option<Range>,
    #[serde(skip_serializing)]
    ar: Option<Range>,
    #[serde(skip_serializing)]
    od: Option<Range>,
    #[serde(skip_serializing)]
    cs: Option<Range>,
    #[serde(skip_serializing)]
    hp: Option<Range>,
    #[serde(skip_serializing)]
    length: Option<Range>,
    #[serde(skip_serializing)]
    bpm: Option<Range>,
}

impl SearchParams {
    pub fn query_url(&self) -> Result<String> {
        let url_params =
            serde_url_params::to_string(&self).expect("SearchParams serialize error");
        if url_params.is_empty() {
            return Err(anyhow!("url params is empty"));
        }
        Ok(format!("{}?{}", SearchParams::BASE_URL, url_params))
    }
    const BASE_URL: &'static str = "https://api.sayobot.cn/beatmaplist";
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub approved: i64,
    pub artist: String,
    #[serde(rename = "artistU")]
    pub artist_u: String,
    pub creator: String,
    pub favourite_count: i64,
    pub lastupdate: i64,
    pub modes: i64,
    pub order: f64,
    pub play_count: i64,
    pub sid: i64,
    pub title: String,
    #[serde(rename = "titleU")]
    pub title_u: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResponse {
    pub data: Option<Vec<Data>>,
    pub endid: Option<i64>,
    pub match_artist_results: Option<i64>,
    pub match_creator_results: Option<i64>,
    pub match_tags_results: Option<i64>,
    pub match_title_results: Option<i64>,
    pub match_version_results: Option<i64>,
    pub results: Option<i64>,
    pub status: i64,
    pub time_cost: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_params_default() {
        let search_params = SearchParams::default();
        let url_params = search_params.query_url();
        assert!(url_params.is_err());
    }

    #[test]
    fn test_all_mode() {
        let mut default_params = SearchParams::default();
        default_params.mode = Some(GameMode::all().bits());
        assert_eq!(default_params.mode, Some(1 + 2 + 4 + 8));
    }

    #[tokio::test]
    async fn new_map_test() {
        let mut search_params = SearchParams::default();
        search_params.request_type = Some(RequestType::New);
        let url = search_params.query_url().unwrap();
        let response = reqwest::get(&url).await.unwrap().text().await.unwrap();
        let search_response: SearchResponse = serde_json::from_str(&response).unwrap();
        assert_eq!(search_response.status, 0);
    }

    #[tokio::test]
    async fn search_test() {
        let mut search_params = SearchParams::default();
        search_params.request_type = Some(RequestType::Search);
        search_params.keyword = Some("kano".to_string());
        search_params.limit = Some(20);
        let cc = (Class::LOVED | Class::RANKED_APPROVED | Class::QUALIFIED).bits();
        search_params.class = Some(cc);
        let url = search_params.query_url().unwrap();
        let response = reqwest::get(&url).await.unwrap().text().await.unwrap();
        let search_response: SearchResponse = serde_json::from_str(&response).unwrap();
        assert_eq!(search_response.status, 0);
        println!("{:?}", search_response.data.unwrap());
    }

    #[tokio::test]
    async fn request_builder_test() {
        let builder = RequestBuilder::new()
            .set_time_out(Duration::from_secs(2))
            .set_request_type(RequestType::Search)
            .set_mode(GameMode::STD)
            .set_keyword("kano".to_string())
            .set_limit(20)
            .set_class(Class::LOVED | Class::RANKED_APPROVED | Class::QUALIFIED)
            .set_genre(Genre::ANY)
            .set_language(Language::ANY)
            .set_starts_range(Range::new(0.0, 5.0).unwrap());
        let resp = builder.do_request().await.unwrap();
        assert_eq!(resp.status, 0);
    }
}
