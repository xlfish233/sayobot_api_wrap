
#[cfg(test)]
mod tests {
    use crate::enums::*;
    use crate::beatmap_params::*;
    use crate::beatmap_request::*;
    use crate::beatmap_response::*;
    use tokio;
    use reqwest;
    use std::time::Duration;

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
