use super::artist::Artist;
use super::playlist::Track;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Album {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub size: Option<i32>,
    pub artist: Option<Artist>,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ArtistAlbums {
    pub artist: Option<Artist>,
    pub hotAlbums: Option<Vec<Album>>,
    pub code: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AlbumTrack {
    pub songs: Vec<Track>,
    pub album: Album,
    pub code: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopAlbumRes {
    pub albums: Vec<Album>,
    pub code: Option<i32>,
}
