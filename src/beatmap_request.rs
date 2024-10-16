use std::time::Duration;

use anyhow::{anyhow, Result};
use reqwest;

use super::{beatmap_params::{SearchParams, Range}, beatmap_response::SearchResponse, enums::{Class, GameMode, Genre, Language, SubType, RequestType}};


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
