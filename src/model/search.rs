//! All object related to search
use super::album::Album;
use super::artist::Artist;
use super::dj::DjRadio;
use super::playlist::{Playlist, Track};
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchTrackResult {
    pub result: SearchTracks,
    pub code: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchPlaylistResult {
    pub result: SearchPlaylists,
    pub code: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchArtistResult {
    pub result: SearchArtists,
    pub code: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchAlbumResult {
    pub result: SearchAlbums,
    pub code: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchDjradioResult {
    pub result: SearchDjRadios,
    pub code: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchTracks {
    pub songs: Option<Vec<Track>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchPlaylists {
    pub playlists: Option<Vec<Playlist>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchArtists {
    pub artists: Option<Vec<Artist>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchAlbums {
    pub albums: Option<Vec<Album>>,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchDjRadios {
    pub djRadios: Option<Vec<DjRadio>>,
}
