use super::album::Album;
use super::artist::Artist;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistRes {
    pub playlist: Vec<Playlist>,
    pub code: Option<i32>,
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Playlist {
    pub name: Option<String>,
    pub userId: Option<i64>,
    pub id: Option<i64>,
    pub creator: Option<Creator>,
    pub trackCount: Option<i32>,
    pub description: Option<String>,
    pub privacy: Option<i32>,
    pub tags: Option<Vec<String>>,
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

#[allow(non_snake_case)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistDetail {
    pub name: Option<String>,
    pub description: Option<String>,
    pub id: Option<i64>,
    pub playCount: Option<i32>,
    pub creator: Option<Creator>,
    pub tracks: Vec<PlaylistTrack>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Track {
    pub name: Option<String>,
    pub fee: Option<i64>,
    pub id: Option<i64>,
    pub artists: Option<Vec<Artist>>,
    pub album: Option<Album>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaylistTrack {
    pub name: Option<String>,
    pub fee: Option<i64>,
    pub id: Option<i64>,
    pub ar: Option<Vec<Artist>>,
    pub al: Option<Album>,
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Track {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PersonalFmRes {
    pub data: Vec<Track>,
    pub code: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TopPlaylistRes {
    pub playlists: Vec<Playlist>,
    pub code: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UidPlaylistRes {
    pub playlist: Vec<Playlist>,
    pub code: Option<i32>,
}
