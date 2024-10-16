use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use anyhow::{Result, anyhow};

use super::enums::RequestType;
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

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchParams {
    #[serde(rename = "L")]
    pub limit: Option<i32>,
    #[serde(rename = "O")]
    pub offset: Option<i32>,
    #[serde(rename = "T")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(skip_deserializing)]
    pub request_type: Option<RequestType>,
    #[serde(rename = "K")]
    pub keyword: Option<String>,
    #[serde(rename = "S")]
    pub sub_type: Option<u32>,
    #[serde(rename = "M")]
    pub mode: Option<u32>,
    #[serde(rename = "C")]
    pub class: Option<u32>,
    #[serde(rename = "G")]
    pub genre: Option<u32>,
    #[serde(rename = "E")]
    pub language: Option<u32>,
    #[serde(rename = "R")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<String>,

    #[serde(skip_serializing)]
    pub stars: Option<Range>,
    #[serde(skip_serializing)]
    pub ar: Option<Range>,
    #[serde(skip_serializing)]
    pub od: Option<Range>,
    #[serde(skip_serializing)]
    pub cs: Option<Range>,
    #[serde(skip_serializing)]
    pub hp: Option<Range>,
    #[serde(skip_serializing)]
    pub length: Option<Range>,
    #[serde(skip_serializing)]
    pub bpm: Option<Range>,
}

impl SearchParams {
    pub fn query_url(&self) -> Result<String> {
        let url_params =
            serde_url_params::to_string(&self).expect("SearchParams serialize error");
        if url_params.is_empty() {
            return Err(anyhow!("url params is empty"));
        }
        Ok(format!("{}?{}", Self::BASE_URL, url_params))
    }
    const BASE_URL: &'static str = "https://api.sayobot.cn/beatmaplist";
}
