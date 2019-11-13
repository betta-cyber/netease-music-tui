#[allow(non_snake_case)]
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub alias: Option<Vec<String>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopArtistRes {
    pub artists: Vec<Artist>,
    pub code: Option<i32>,
}
