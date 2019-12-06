extern crate gstreamer as gst;
extern crate gstreamer_player as gst_player;
use tui::layout::Rect;
use tui::style::Color;
use super::model::playlist::{Playlist, Track};
use super::model::artist::Artist;
use super::model::album::Album;
use super::model::lyric::Lyric;
use super::api::CloudMusic;
use super::ui::circle::{Circle, CIRCLE, CIRCLE_TICK};
use super::handlers::TrackState;
use rand::Rng;
use gst::ClockTime;
use gst::prelude::*;


const DEFAULT_ROUTE: Route = Route {
    id: RouteId::Home,
    active_block: ActiveBlock::Recommend,
    hovered_block: ActiveBlock::Recommend,
};

pub const RECOMMEND_OPTIONS: [&str; 5] = [
    "My Playlist",
    "Discover",
    "Personal FM",
    "Hot Albums",
    "Hot Artists",
];


#[derive(Clone, PartialEq, Debug)]
pub struct Route {
    pub id: RouteId,
    pub active_block: ActiveBlock,
    pub hovered_block: ActiveBlock,
}

#[derive(Clone, PartialEq, Debug)]
pub enum RouteId {
    AlbumTracks,
    AlbumList,
    Playlist,
    Artist,
    ArtistList,
    Error,
    Home,
    Search,
    TrackTable,
    MyPlaylists,
    Artists,
    PersonalFm,
    Playing,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
    AlbumTracks,
    AlbumList,
    Artist,
    ArtistList,
    Playlist,
    Empty,
    Error,
    Help,
    Home,
    Search,
    Recommend,
    MyPlaylists,
    SearchResult,
    TrackTable,
    Artists,
    PlayBar,
    PersonalFm,
    Playing,
}

#[derive(Clone)]
pub struct ArtistAlbums {
    pub artist_name: String,
    pub albums: Vec<Album>,
    pub selected_index: usize,
}

#[derive(Clone)]
pub struct SelectedAlbum {
    pub tracks: Vec<Track>,
    pub selected_index: usize,
    pub album: Album,
}

// playlist list
#[derive(Clone)]
pub struct PlaylistTable {
    pub playlists: Vec<Playlist>,
    pub selected_index: usize,
}

// album list
#[derive(Clone)]
pub struct AlbumsTable {
    pub albums: Vec<Album>,
    pub selected_index: usize,
}

// album list
#[derive(Clone)]
pub struct ArtistsTable {
    pub artists: Vec<Artist>,
    pub selected_index: usize,
}

#[derive(Clone, Debug, Default)]
pub struct TrackTable {
    pub tracks: Vec<Track>,
    pub selected_index: usize,
    pub name: String,
}

#[derive(Clone)]
pub struct Recommend {
    pub selected_index: usize,
}

pub struct SearchResult {
    pub tracks: Option<Vec<Track>>,
    pub playlists: Option<Vec<Playlist>>,
    pub artists: Option<Vec<Artist>>,
    pub albums: Option<Vec<Album>>,

    pub selected_albums_index: usize,
    pub selected_artists_index: usize,
    pub selected_playlists_index: usize,
    pub selected_tracks_index: usize,

    pub selected_albums_page: usize,
    pub selected_artists_page: usize,
    pub selected_playlists_page: usize,
    pub selected_tracks_page: usize,
}

// 顺序播放
// 单曲循环
// 列表循环
// 随机播放
#[derive(Clone, PartialEq, Debug)]
pub enum RepeatState {
    Off,
    Track,
    All,
    Shuffle,
    FM,
}

pub struct TabsState {
    pub titles: Vec<String>,
    pub index: usize,
}
impl TabsState {
    pub fn new(titles: Vec<String>) -> TabsState {
        TabsState { titles, index: 0 }
    }
}


pub struct App {
    navigation_stack: Vec<Route>,
    pub player: gst_player::Player,
    pub size: Rect,
    pub input: String,
    pub input_idx: usize,
    pub input_cursor_position: u16,
    pub song_progress_ms: u64,
    pub current_playing: Option<Track>,
    pub playlists: Option<Vec<Playlist>>,
    pub selected_playlist_index: Option<usize>,
    pub track_table: TrackTable,
    pub cloud_music: Option<CloudMusic>,
    pub recommend: Recommend,
    pub duration_ms: Option<u64>,
    pub my_playlist: TrackTable,
    pub repeat_state: RepeatState,
    pub search_results: SearchResult,
    pub tabs: TabsState,
    pub playing_circle: Circle,
    pub circle_flag: bool,
    pub artist_albums: Option<ArtistAlbums>,
    pub selected_album: Option<SelectedAlbum>,
    pub playlist_list: Option<PlaylistTable>,
    pub album_list: Option<AlbumsTable>,
    pub artist_list: Option<ArtistsTable>,
    pub lyric: Option<Vec<Lyric>>,
    pub error_msg: String,
    pub block_height: usize,
    pub lyric_index: usize,
}

impl App {
    pub fn new() -> App {

        let dispatcher = gst_player::PlayerGMainContextSignalDispatcher::new(None);
        let music_player = gst_player::Player::new(
            None,
            Some(&dispatcher.upcast::<gst_player::PlayerSignalDispatcher>()),
        );

        App {
            navigation_stack: vec![DEFAULT_ROUTE],
            player: music_player,
            size: Rect::default(),
            input: String::new(),
            input_idx: 0,
            input_cursor_position: 0,
            song_progress_ms: 0,
            current_playing: None,
            duration_ms: None,
            playlists: None,
            selected_playlist_index: None,
            track_table: Default::default(),
            cloud_music: Some(CloudMusic::default()),
            recommend: Recommend {
                selected_index: 0,
            },
            my_playlist: Default::default(),
            repeat_state: RepeatState::All,
            search_results: SearchResult {
                tracks: None,
                playlists: None,
                artists: None,
                albums: None,
                selected_albums_index: 0,
                selected_artists_index: 0,
                selected_playlists_index: 0,
                selected_tracks_index: 0,
                selected_albums_page: 0,
                selected_artists_page: 0,
                selected_playlists_page: 0,
                selected_tracks_page: 0,
            },
            tabs: TabsState::new(vec![
                "Songs".to_string(),
                "Artists".to_string(),
                "Albums".to_string(),
                "Playlists".to_string(),
            ]),
            playing_circle: Circle::default(),
            circle_flag: true,
            artist_albums: None,
            selected_album: None,
            playlist_list: None,
            album_list: None,
            artist_list: None,
            lyric: None,
            error_msg: String::new(),
            block_height: 0,
            lyric_index: 0,
        }
    }

    // update app every tick
    pub fn update_on_tick(&mut self) {
        // debug!("{:#?}", self.get_state());
        if self.is_playing() {
            self.song_progress_ms = match self.player.get_position().mseconds() {
                Some(ms) => ms,
                None => 0 as u64
            };
            let current_route = self.get_current_route();
            if current_route.active_block == ActiveBlock::Playing {
                if self.circle_flag {
                    self.playing_circle = Circle {
                        circle: &CIRCLE,
                        color: Color::Reset,
                    }
                } else {
                    self.playing_circle = Circle {
                        circle: &CIRCLE_TICK,
                        color: Color::Cyan,
                    }
                }
                self.circle_flag = !self.circle_flag;
                match &self.lyric {
                    Some(lyrics) => {
                        let next_lyric = lyrics.get(self.lyric_index + 1);
                        // check current ms and lyric timeline
                        match next_lyric {
                            Some(next_lyric) => {
                                if self.song_progress_ms as u128 >= next_lyric.timeline.as_millis() {
                                    self.lyric_index += 1;
                                }
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            }

            // check progress duration
            match self.duration_ms {
                Some(duration_ms) => {
                    if self.song_progress_ms >= duration_ms {
                        // log track
                        self.log_track();
                        self.skip_track(TrackState::Forword)
                    }
                }
                None => {}
            }
        }
    }

    pub fn next_index<T>(selection_data: &[T], selection_index: Option<usize>, state: TrackState) -> usize {
        match selection_index {
            Some(selection_index) => {
                if !selection_data.is_empty() {
                    match state {
                        TrackState::Forword => {
                            let next_index = selection_index + 1;
                            if next_index > selection_data.len() - 1 {
                                return 0;
                            } else {
                                return next_index;
                            }
                        }
                        TrackState::Backword => {
                            if selection_index <= 1 {
                                return 0;
                            } else {
                                return selection_index - 1;
                            }
                        }
                    }
                }
                0
            }
            None => 0,
        }
    }

    pub fn skip_track(&mut self, state: TrackState) {
        match self.repeat_state {
            RepeatState::Track => {
                // loop current song
                match &self.current_playing {
                    Some(track) => {
                        self.start_playback(track.id.unwrap().to_string());
                    }
                    None => {
                        panic!("error");
                    }
                };
            }
            RepeatState::All => {
                // loop current my playlist
                let list = &mut self.my_playlist;
                let next_index = App::next_index(
                    &list.tracks,
                    Some(list.selected_index),
                    state,
                );
                list.selected_index = next_index;

                let track_playing = list.tracks.get(next_index.to_owned()).unwrap().to_owned();
                self.start_playback(track_playing.id.unwrap().to_string());
                self.current_playing = Some(track_playing);

            }
            RepeatState::Shuffle => {
                let list = &mut self.my_playlist;
                let mut rng = rand::thread_rng();
                let next_index = rng.gen_range(0, list.tracks.len());
                list.selected_index = next_index;

                let track_playing = list.tracks.get(next_index.to_owned()).unwrap().to_owned();
                self.start_playback(track_playing.id.unwrap().to_string());
                self.current_playing = Some(track_playing);
            }
            RepeatState::FM => {
                // use my playlist for play personal fm
                let list = &mut self.my_playlist;
                let next_index = App::next_index(
                    &list.tracks,
                    Some(list.selected_index),
                    state,
                );
                if next_index == 0 {
                    if let Ok(tracks) = self.cloud_music.as_ref().unwrap().personal_fm() {
                        list.tracks = tracks.to_owned();
                    }
                }
                list.selected_index = next_index;

                let track_playing = list.tracks.get(next_index.to_owned()).unwrap().to_owned();
                self.start_playback(track_playing.id.unwrap().to_string());
                self.current_playing = Some(track_playing);
            }
            _ => {}
        }
    }

    pub fn increase_volume(&mut self) {
        let current = self.player.get_volume();
        let volume = current + 0.1_f64;
        self.player.set_volume(volume);
    }

    pub fn decrease_volume(&mut self) {
        let current = self.player.get_volume();
        let volume = current - 0.1_f64;
        self.player.set_volume(volume);
    }

    pub fn seek_forwards(&mut self) {
        let next_duration = self.song_progress_ms + 3000;
        self.player.seek(ClockTime::from_mseconds(next_duration))
    }

    pub fn seek_backwards(&mut self) {
        let next_duration = if self.song_progress_ms < 3000 {
            0
        } else {
            self.song_progress_ms - 3000
        };
        self.player.seek(ClockTime::from_mseconds(next_duration))
    }

    pub fn get_current_route(&self) -> &Route {
        match self.navigation_stack.last() {
            Some(route) => route,
            None => &DEFAULT_ROUTE, // if for some reason there is no route return the default
        }
    }

    fn get_current_route_mut(&mut self) -> &mut Route {
        self.navigation_stack.last_mut().unwrap()
    }

    pub fn push_navigation_stack(
        &mut self,
        next_route_id: RouteId,
        next_active_block: ActiveBlock,
    ) {
        self.navigation_stack.push(Route {
            id: next_route_id,
            active_block: next_active_block,
            hovered_block: next_active_block,
        });
    }

    pub fn pop_navigation_stack(&mut self) -> Option<Route> {
        if self.navigation_stack.len() == 1 {
            None
        } else {
            self.navigation_stack.pop()
        }
    }

    // set current route
    pub fn set_current_route_state(
        &mut self,
        active_block: Option<ActiveBlock>,
        hovered_block: Option<ActiveBlock>,
    ) {
        let mut current_route = self.get_current_route_mut();
        if let Some(active_block) = active_block {
            current_route.active_block = active_block;
        }
        if let Some(hovered_block) = hovered_block {
            current_route.hovered_block = hovered_block;
        }
    }

    // shuffle
    pub fn repeat(&mut self) {
        let next_repeat_state = match self.repeat_state {
            RepeatState::All => RepeatState::Off,
            RepeatState::Off => RepeatState::Track,
            RepeatState::Track => RepeatState::Shuffle,
            RepeatState::Shuffle => RepeatState::All,
            RepeatState::FM => RepeatState::FM,
        };
        self.repeat_state = next_repeat_state;
    }

    // handle error
    pub fn handle_error(&mut self, e: failure::Error) {
        self.push_navigation_stack(RouteId::Error, ActiveBlock::Error);
        self.error_msg = e.to_string();
    }

    pub fn get_playlist_tracks(&mut self, playlist_id: String) {
        match &self.cloud_music {
            Some(api) => {
                if let Ok(playlist_tracks) = api.playlist_detail(&playlist_id) {
                    let tracks = playlist_tracks.tracks
                        .into_iter()
                        .map(|t| {
                            Track{
                                name: t.name,
                                id: t.id,
                                artists: t.ar,
                                album: t.al,
                            }
                        })
                        .collect();
                    self.track_table.tracks = tracks;
                    self.track_table.name = playlist_tracks.name.unwrap();
                }
            }
            None => {}
        }
        self.push_navigation_stack(RouteId::TrackTable, ActiveBlock::TrackTable)
    }

    pub fn get_artist_albums(&mut self, artist_id: String) {
        match &self.cloud_music {
            Some(api) => {
                if let Ok(albums) = api.artist_albums(&artist_id) {
                    self.artist_albums = Some(ArtistAlbums {
                        artist_name: String::new(),
                        albums: albums,
                        selected_index: 0
                    })
                }
                self.push_navigation_stack(RouteId::Artist, ActiveBlock::Artist);
            }
            None => {}
        }
    }

    pub fn get_album_tracks(&mut self, album_id: String) {
        match &self.cloud_music {
            Some(api) => {
                if let Ok(album_track) = api.album_track(&album_id) {
                    self.selected_album = Some(SelectedAlbum {
                        tracks: album_track.songs,
                        album: album_track.album,
                        selected_index: 0
                    })
                }
                self.push_navigation_stack(RouteId::AlbumTracks, ActiveBlock::AlbumTracks);
            }
            None => {}
        }
    }

    pub fn get_top_playlist(&mut self) {
        match &self.cloud_music {
            Some(api) => {
                if let Ok(playlists) = api.top_playlists() {
                    self.playlist_list = Some(PlaylistTable {
                        playlists: playlists,
                        selected_index: 0
                    })
                }
                self.push_navigation_stack(RouteId::Playlist, ActiveBlock::Playlist);
            }
            None => {}
        }
    }


    pub fn get_top_albums(&mut self) {
        match &self.cloud_music {
            Some(api) => {
                if let Ok(albums) = api.top_albums() {
                    self.album_list = Some(AlbumsTable {
                        albums: albums,
                        selected_index: 0,
                    })
                }
                self.push_navigation_stack(RouteId::AlbumList, ActiveBlock::AlbumList);
            }
            None => {}
        }
    }

    // top artist
    pub fn get_top_artists(&mut self) {
        match &self.cloud_music {
            Some(api) => {
                if let Ok(artists) = api.top_artists() {
                    self.artist_list = Some(ArtistsTable {
                        artists: artists,
                        selected_index: 0,
                    })
                }
                self.push_navigation_stack(RouteId::ArtistList, ActiveBlock::ArtistList);
            }
            None => {}
        }
    }

    pub fn follow_current(&self) {
        let track_id = &self.current_playing.clone().unwrap().id.unwrap().to_string();
        match &self.cloud_music {
            Some(api) => {
                api.like(&track_id, true);
            }
            None => {}
        }
    }

    // unfollow
    pub fn unfollow_current(&self) {
        let track_id = &self.current_playing.clone().unwrap().id.unwrap().to_string();
        match &self.cloud_music {
            Some(api) => {
                api.like(&track_id, false);
            }
            None => {}
        }
    }

    pub fn start_playback(
        &mut self,
        id: String,
    ) {
        match &self.cloud_music {
            Some(api) => {
                let song = api.get_song_url(&id).unwrap();
                let url = song.url.unwrap().to_string();

                // init play state
                self.duration_ms = None;
                self.song_progress_ms = 0;
                self.lyric_index = 0;
                self.player.set_uri(&url);
                self.lyric = Some(api.lyric(&id).unwrap());
                self.player.play();

                let mut flag = false;
                while !flag {
                    if self.is_playing() {
                        self.duration_ms = self.player.get_duration().mseconds();
                        flag = true;
                    }
                }

                // exit fm mode
                if self.get_current_route().active_block != ActiveBlock::PersonalFm {
                    self.repeat_state = RepeatState::All;
                }
            }
            None => {}
        }
    }

    pub fn log_track(&mut self) {
        match &self.cloud_music {
            Some(api) => {
                match &self.current_playing {
                    Some(track) => {
                        api.log_track(&track.id.unwrap().to_string());
                    }
                    None => {}
                }
            }
            None => {}
        }
    }

    pub fn set_fm_mode(&mut self) {
        match &self.cloud_music {
            Some(api) => {
                if let Ok(tracks) = api.personal_fm() {
                    self.my_playlist = TrackTable {
                        tracks: tracks,
                        selected_index: 0,
                        name: "personal fm".to_string()
                    }
                }

                let track_playing = self.my_playlist.tracks.get(0).unwrap().to_owned();

                self.start_playback(track_playing.id.unwrap().to_string());
                self.current_playing = Some(track_playing);

                self.push_navigation_stack(RouteId::PersonalFm, ActiveBlock::PersonalFm);
                self.repeat_state = RepeatState::FM;
            }
            None => {}
        }
    }

    // set hover mode
    pub fn hover_mode(&mut self) {
        let current_route = &self.get_current_route();
        self.set_current_route_state(Some(ActiveBlock::Empty), Some(current_route.hovered_block))
    }


    pub fn is_playing(&self) -> bool {
        let player = &self.player;
        let element = player.get_pipeline();
        element.get_state(gst::CLOCK_TIME_NONE).1 == gst::State::Playing
    }

    pub fn get_state(&self) -> gst::State {
        let player = &self.player;
        let element = player.get_pipeline();
        element.get_state(gst::CLOCK_TIME_NONE).1
    }
}
