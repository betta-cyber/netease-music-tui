use serde_derive::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LyricRes {
    pub code: i32,
    pub lrc: Lrc,
    pub tlyric: Lrc
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lrc {
    pub lyric: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lyric {
    pub value: String,
    pub timeline: Duration,
}
