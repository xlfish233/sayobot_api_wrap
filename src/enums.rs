use serde::{Deserialize, Serialize};
use bitflags::bitflags;

bitflags! {
    pub struct Language: u32 {
        const ANY = 0b00000001;
        const OTHER = 0b00000010;
        const ENGLISH = 0b00000100;
        const JAPANESE = 0b00001000;
        const CHINESE = 0b00010000;
        const INSTRUMENTAL = 0b00100000;
        const KOREAN = 0b01000000;
        const FRENCH = 0b10000000;
        const GERMAN = 0b100000000;
        const SWEDISH = 0b1000000000;
        const SPANISH = 0b10000000000;
        const ITALIAN = 0b100000000000;
    }
}
bitflags! {
    pub struct Genre: u32 {
        const ANY = 0b00000001;
        const UNSPECIFIED = 0b00000010;
        const VIDEO_GAME = 0b00000100;
        const ANIME = 0b00001000;
        const ROCK = 0b00010000;
        const POP = 0b00100000;
        const OTHER = 0b01000000;
        const NOVELTY = 0b10000000;
        const HIPHOP = 0b100000000;
        const ELECTRONIC = 0b1000000000;
    }
}
bitflags! {
    pub struct Class: u32 {
        const RANKED_APPROVED = 0b0001;
        const QUALIFIED = 0b0010;
        const LOVED = 0b0100;
        const PENDING_WIP = 0b1000;
        const GRAVEYARD = 0b10000;
    }
}
bitflags! {
     pub struct GameMode: u32 {
        const STD = 0b0001;
        const TAIKO = 0b0010;
        const CTB = 0b0100;
        const MANIA = 0b1000;
    }
}
bitflags! {
    pub struct SubType: u32 {
        const TITLE = 0b00000001;
        const ARTIST = 0b00000010;
        const CREATOR = 0b00000100;
        const VERSION = 0b00001000;
        const TAGS = 0b00010000;
        const SOURCE = 0b00100000;
    }
}

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
