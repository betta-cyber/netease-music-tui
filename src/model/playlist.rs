use serde::{Serialize, Deserialize};
use serde_json::Value;

use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistRes {
    pub playlist: Vec<Playlist>,
    pub code: Option<i32>,
    pub more: Option<bool>,
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
