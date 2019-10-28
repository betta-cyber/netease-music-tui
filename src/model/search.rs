//! All object related to search
use super::playlist::{Track, Playlist};
use serde::{Serialize, Deserialize};

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

#[derive(Debug)]
pub enum SearchResult {
    Track(Vec<Track>),
    Playlist(Vec<Playlist>)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchTracks {
    pub tracks: Vec<Track>,
}

