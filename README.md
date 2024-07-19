# Sayobot API Wrapper

这是一个用于封装 Sayobot API 的 Rust 库，旨在提供简单、高效的方式来访问 Sayobot 提供的 osu! 游戏数据。

## 特性

- 异步支持：基于 `tokio` 和 `reqwest`，提供完全的异步 API 调用。
- 序列化/反序列化：利用 `serde` 和相关库，轻松处理 JSON 数据。
- 错误处理：通过 `anyhow` 库，简化错误处理流程。

## 依赖

本项目依赖于以下主要的 crates：

- `tokio`：异步运行时。
- `reqwest`：异步 HTTP 客户端。
- `serde`、`serde_json`：序列化和反序列化 JSON 数据。
- `anyhow`：简化错误处理。

## 使用 

首先，确保你的 Rust 版本是 2021 edition 或更高。然后，将此库作为依赖项添加到你的 `Cargo.toml` 文件中：

```toml
[dependencies]
sayobot_api_wrap = { git = "https://github.com/your-repo/sayobot_api_wrap.git", branch = "main" }

use sayobot_api_wrap::beatmap_info_v2::BeatmapInfoV2;



#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let beatmap_info = BeatmapInfoV2::new(beatmap_id).await?;
    println!("{:?}", beatmap_info);
    Ok(())
}use sayobot_api_wrap::beatmap_info_v2::BeatmapInfoV2;
