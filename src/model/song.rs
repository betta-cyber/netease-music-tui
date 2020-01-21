#[allow(non_snake_case)]
use serde_derive::{Deserialize, Serialize};
// use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Songs {
    pub data: Vec<Song>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Song {
    pub id: Option<i32>,
    pub url: Option<String>,
    // pub br: Option<i32>,
}
