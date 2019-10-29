//! All object related to search
use super::playlist::{Track, Playlist};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct SearchPlaylists {
    // pub playlists: Page<SimplifiedPlaylist>,
// }
// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct SearchAlbums {
    // pub albums: Page<SimplifiedAlbum>,
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct SearchArtists {
    // pub artists: Page<FullArtist>,
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct SearchTracks {
    // pub tracks: Page<FullTrack>,
// }
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchTrackResult {
    pub result: Option<SearchTracks>,
    pub code: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchTracks {
    pub songs: Vec<Track>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchPlaylists {
    pub playlist: Vec<Playlist>,
}
