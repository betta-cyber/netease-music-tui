use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LyricRes {
    pub code: i32,
    pub lrc: Lrc,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lrc {
    pub lyric: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lyric {
    pub value: String,
    pub timestamp: String,
}
