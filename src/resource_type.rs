use anyhow::anyhow;

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum ResourceType {
    PreviewImg = 0,
    PreviewAudio = 1,
    FullAudio = 2,
    FullCoverImg = 3,
    Video = 4,
    FullSizeMap = 5,
    NoVideoMap = 6,
    MiniMap = 7,
}

impl PartialEq for ResourceType {
    fn eq(&self, other: &Self) -> bool {
        //if convert to i32 eq just return true
        if *self as i32 == *other as i32 {
            return true;
        }
        false
    }
}

impl ResourceType {
    pub fn get_type_url_format(&self) -> anyhow::Result<String> {
        match self {
            ResourceType::PreviewImg => {
                Ok("https://a.sayobot.cn/beatmaps/${sid}/covers/cover.webp".to_string())
            }
            ResourceType::PreviewAudio => { Ok("https://a.sayobot.cn/preview/${sid}.mp3".to_string()) }

            ResourceType::FullSizeMap => {
                Ok("https://dl.sayobot.cn/beatmaps/download/full/${sid}".to_string())
            }
            ResourceType::NoVideoMap => {
                Ok("https://dl.sayobot.cn/beatmaps/download/novideo/${sid}".to_string())
            }
            ResourceType::MiniMap => {
                Ok("https://dl.sayobot.cn/beatmaps/download/mini/${sid}".to_string())
            }
            _ => {
                Err(anyhow!("not supported url format for {:?}", self))
            }
        }
    }
}
