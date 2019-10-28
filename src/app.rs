extern crate vlc;
use vlc::{Instance, Media, MediaPlayer, MediaPlayerAudioEx, State};
use tui::layout::{Layout, Constraint, Direction, Rect};
use super::model::playlist::{Playlist, Track};
use super::api::CloudMusic;
use std::time::Instant;

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
    RecentlyPlayed,
    Search,
    SelectedDevice,
    TrackTable,
    MadeForYou,
    Artists,
    Podcasts,
    PersonalFm,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ActiveBlock {
    AlbumTracks,
    AlbumList,
    Artist,
    Empty,
    Error,
    HelpMenu,
    Home,
    Search,
    Recommend,
    MyPlaylists,
    Podcasts,
    RecentlyPlayed,
    SearchResultBlock,
    SelectDevice,
    TrackTable,
    Artists,
    PlayBar,
    PersonalFm,
}

// #[derive(Default)]
#[derive(Clone, Debug, Default)]
pub struct TrackTable {
    pub tracks: Vec<Track>,
    pub selected_index: usize,
}

#[derive(Clone)]
pub struct Recommend {
    pub selected_index: usize,
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
    pub my_playlist: Option<TrackTable>,
    pub repeat_state: RepeatState,
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
            my_playlist: None,
            repeat_state: RepeatState::All,
        }
    }

    // update app every tick
    pub fn update_on_tick(&mut self) {
        if self.player.is_playing() {
            self.song_progress_ms = self.player.get_time().unwrap() as u64;
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
                    match &self.my_playlist {
                        Some(list) => {
                            let mut list = list.to_owned();
                            // catulate next index
                            let mut next_index = list.selected_index + 1;
                            if next_index > list.tracks.len() - 1 {
                                next_index = 0;
                            }
                            let track_playing = list.tracks.get(next_index.to_owned()).unwrap().to_owned();
                            self.start_playback(track_playing.id.unwrap().to_string());
                            self.current_playing = Some(track_playing);

                            list.selected_index = next_index;
                            println!("{:#?}", next_index);
                        }
                        None => {}
                    }
                }
                _ => {}
            }
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

    pub fn get_playlist_tracks(&mut self, playlist_id: String) {
        match &self.cloud_music {
            Some(api) => {
                if let Ok(playlist_tracks) = api.playlist_detail(&playlist_id) {
                    self.track_table.tracks = playlist_tracks.tracks;
                }
            }
            None => {
                panic!("get playlist track error");
            }
        }
        self.push_navigation_stack(RouteId::TrackTable, ActiveBlock::TrackTable)
    }

    pub fn start_playback(
        &mut self,
        id: String,
    ) {
        match &self.cloud_music {
            Some(api) => {
                let song = api.song(&id).unwrap();
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
