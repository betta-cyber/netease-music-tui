extern crate vlc;
use vlc::{Instance, Media, MediaPlayer, MediaPlayerAudioEx, State};
use tui::layout::Rect;
use tui::style::Color;
use super::model::playlist::{Playlist, Track};
use super::model::artist::Artist;
use super::model::album::Album;
use super::api::CloudMusic;
use super::ui::circle::{Circle, CIRCLE, CIRCLE_TICK};
use rand::Rng;

const DEFAULT_ROUTE: Route = Route {
    id: RouteId::Home,
    active_block: ActiveBlock::Empty,
    hovered_block: ActiveBlock::Recommend,
};

pub const RECOMMEND_OPTIONS: [&str; 5] = [
    "My Playlist",
    "Discover",
    "Personal FM",
    "Albums",
    "Artists",
];

pub struct Route {
    pub id: RouteId,
    pub active_block: ActiveBlock,
    pub hovered_block: ActiveBlock,
}

#[derive(Clone, PartialEq, Debug)]
pub enum RouteId {
    AlbumTracks,
    AlbumList,
    Artist,
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
    pub player: MediaPlayer,
    pub vlc_instance: Instance,
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
}

impl App {
    pub fn new() -> App {

        let instance = Instance::new().unwrap();
        let music_player = MediaPlayer::new(&instance).unwrap();

        App {
            navigation_stack: vec![DEFAULT_ROUTE],
            player: music_player,
            vlc_instance: instance,
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
        }
    }

    // update app every tick
    pub fn update_on_tick(&mut self) {
        if self.player.is_playing() {
            self.song_progress_ms = self.player.get_time().unwrap() as u64;

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
                        color: Color::Reset,
                    }
                }
                self.circle_flag = !self.circle_flag;
            }
        } else if self.player.state() == State::Ended {
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
                    );
                    list.selected_index = next_index;
                    // println!("{:#?}", next_index);

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
                _ => {}
            }
        }
    }

    pub fn next_index<T>(selection_data: &[T], selection_index: Option<usize>) -> usize {
        match selection_index {
            Some(selection_index) => {
                if !selection_data.is_empty() {
                    let next_index = selection_index + 1;
                    if next_index > selection_data.len() - 1 {
                        return 0;
                    } else {
                        return next_index;
                    }
                }
                0
            }
            None => 0,
        }
    }

    pub fn increase_volume(&mut self) {
        let current = self.player.get_volume();
        let volume = current + 10_i32;
        self.player.set_volume(volume).unwrap();
    }

    pub fn decrease_volume(&mut self) {
        let current = self.player.get_volume();
        let volume = current - 10_i32;
        self.player.set_volume(volume).unwrap();
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
        };
        self.repeat_state = next_repeat_state;
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
            None => {
                panic!("get playlist track error");
            }
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
            None => {
                panic!("get artist albums error");
            }
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
            None => {
                panic!("get artist albums error");
            }
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

                let md = Media::new_location(&self.vlc_instance, &url).unwrap();
                self.player.set_media(&md);
                self.player.play().unwrap();

                let mut flag = false;
                while !flag {
                    if md.state() == State::Playing {
                        self.duration_ms = Some(md.duration().unwrap() as u64);
                        flag = true;
                    }
                }
            }
            None => {}
        }
    }
}
