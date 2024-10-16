use serde::{Deserialize, Serialize};

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
