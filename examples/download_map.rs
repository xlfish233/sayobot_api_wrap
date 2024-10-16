use sayobot_api_wrap::static_resources::RequestBuilder;
use std::time::Duration;
use sayobot_api_wrap::resource_type::ResourceType;

#[tokio::main]
async fn main() {
    let builder = RequestBuilder::new()
        .set_sid(2045169)
        .set_resource_type(ResourceType::FullSizeMap)
        .set_download_path("./")
        .unwrap()
        .set_timeout(Duration::from_secs(30));
    builder.do_request().await.unwrap();
}
