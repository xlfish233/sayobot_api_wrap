use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use futures::stream::StreamExt;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::resource_type::ResourceType;

struct QueryParams {
    sid: Option<i64>,
    resource_type: Option<ResourceType>,
    download_path: Option<PathBuf>,
}

pub struct RequestBuilder {
    params: QueryParams,
    request_timeout: Duration,
}

impl Default for RequestBuilder {
    fn default() -> Self {
        Self {
            params: QueryParams {
                sid: None,
                resource_type: None,
                download_path: Some(PathBuf::from(".")),
            },
            request_timeout: Duration::from_secs(30),
        }
    }
}


impl RequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_sid(mut self, sid: i64) -> Self {
        self.params.sid = Some(sid);
        self
    }
    pub fn set_resource_type(mut self, resource_type: ResourceType) -> Self {
        self.params.resource_type = Some(resource_type);
        self
    }
    pub fn set_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = timeout;
        self
    }
    pub fn set_download_path<P: AsRef<Path>>(mut self, path: P) -> Result<Self> {
        if Path::new(&path.as_ref()).exists() {
            self.params.download_path = Some(path.as_ref().to_path_buf());
            Ok(self)
        } else {
            Err(anyhow::anyhow!("path not exist"))
        }
    }

    pub async fn do_request(self) -> Result<String> {
        let url = self.get_url().await?;
        let client = reqwest::Client::builder()
            .timeout(self.request_timeout)
            .build()?;

        let response = client.get(&url).send().await.expect("reqwest fail");

        if response.status().is_success() {
            let content_disposition = response.headers()
                .get("Content-Disposition")
                .and_then(|value| value.to_str().ok())
                .and_then(|content| content.split("filename*=utf-8''").nth(1));
            let file_name = match content_disposition {
                Some(name) => urlencoding::decode(name)?.to_string(),
                None => return Err(anyhow!("can't get file name from response header")),
            };

            let file_path = Path::new(self.params.download_path.as_ref().unwrap())
                .join(file_name.clone());
            let mut file = File::create(&file_path).await.expect("file create fail");

            let mut stream = response.bytes_stream();
            while let Some(chunk_result) = stream.next().await {
                let chunk = chunk_result.expect("read chunk fail");
                file.write_all(&chunk).await.expect("write to file fail");
            }
            Ok(file_name)
        } else {
            Err(anyhow!("http status not success {}", response.status()))
        }
    }

    async fn get_url(&self) -> Result<String> {
        let sid = self.params.sid.with_context(|| "sid not set")?;
        let res_type = self.params.resource_type.with_context(|| "res_type not set")?;

        if let ResourceType::PreviewImg | ResourceType::PreviewAudio | ResourceType::FullSizeMap | ResourceType::NoVideoMap | ResourceType::MiniMap = res_type {
            let url_format = res_type.get_type_url_format()?;
            return Ok(url_format.replace("${sid}", &sid.to_string()));
        }

        let v2_map_info = crate::beatmap_info_v2::RequestBuilder::new()
            .set_key("sid".to_string())
            .set_timeout(self.request_timeout)
            .do_request()
            .await
            .expect("get beatmap info failed");

        let matched_map = v2_map_info.data.bid_data
            .iter()
            .find(|map| map.bid == sid)
            .expect("map not found");

        match res_type {
            ResourceType::FullAudio => {
                let audio_file_name = matched_map.audio.clone();
                build_file_url(sid, audio_file_name)
            }
            ResourceType::FullCoverImg => {
                let cover_file_name = matched_map.bg.clone();
                build_file_url(sid, cover_file_name)
            }
            ResourceType::Video => Err(anyhow!("video not supported")),

            _ => unreachable!(),
        }
    }
}

fn build_file_url(sid: i64, file_name: String) -> Result<String> {
    Ok(format!(
        "https://dl.sayobot.cn/beatmaps/files/{}/{}",
        sid, file_name
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download_mini_map() {
        let builder = RequestBuilder::new()
        .set_sid(2045169
        ).set_resource_type(ResourceType::MiniMap)
        .set_download_path("./").unwrap()
        .set_timeout(Duration::from_secs(100));
        let result = builder.do_request().await;
        assert!(result.is_ok());
        let file_name = result.unwrap();
        if Path::new(&file_name).exists() {
            std::fs::remove_file(&file_name).unwrap();
        }
    }
}
