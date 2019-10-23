#[allow(non_snake_case)]
use serde::{Serialize, Deserialize};
use serde_json::Value;

use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistRes {
    pub playlist: Vec<Playlist>,
    pub code: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Playlist {
    pub name: Option<String>,
    pub userId: Option<i64>,
    pub id: Option<i64>,
    pub creator: Option<Creator>,
    pub trackCount: Option<i32>,
    pub description: Option<String>,
    pub privacy: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Creator {
    pub nickname: Option<String>,
    pub signature: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistDetailRes {
    pub playlist: Option<PlaylistDetail>,
    pub code: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistDetail {
    pub name: Option<String>,
    pub description: Option<String>,
    pub id: Option<i64>,
    pub playCount: Option<i32>,
    pub creator: Option<Creator>,
    pub tracks: Vec<Track>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Track {
    pub name: Option<String>,
    pub id: Option<i64>,
    pub ar: Option<Vec<Artist>>,
    pub al: Option<Album>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artist {
    pub name: String,
    pub id: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Album {
    pub name: String,
    pub id: i64,
}
