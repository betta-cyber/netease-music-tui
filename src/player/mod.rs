extern crate rodio;
extern crate tokio;
extern crate tempfile;

mod player;
mod fetch;
// mod sink;
// mod range_set;
mod track;
// mod fetch_data;

use player::Player;
// use sink::find;
use std::sync::mpsc::Sender;

#[allow(unused)]
pub enum PlayerCommand {
    Play,
    Pause,
    Stop,
    PlayPause,
    Seek(i32),
    Next,
    Previous,
    Load(String),
    Position(i32, u64),
    Metadata(MetaInfo, Sender<String>),
}

#[allow(unused)]
pub enum MetaInfo {
    Volume,
    Shuffle,
    Position,
    LoopStatus,
    Status,
    Info,
}

pub struct Nplayer {
    pub player: player::Player,
}

impl Nplayer {
    pub fn new() -> Nplayer {
        // let backend = find(None).unwrap();
        let mplayer = Player::new();
        debug!("init player");
        Nplayer {
            player: mplayer,
        }
    }

    pub fn play_url(&mut self, url: &str) {
        self.player.load(url.to_owned(), true);
    }

    pub fn is_playing(&mut self) -> bool {
        self.player.status()
    }

    pub fn pause(&mut self) {
        self.player.pause()
    }

    pub fn play(&mut self) {
        self.player.play()
    }

    #[allow(unused)]
    pub fn stop(&self) {
        self.player.stop()
    }

    pub fn get_position(&self) -> Option<u64> {
        match self.player.current.clone() {
            Some(current) => {
                Some(current.elapsed().as_millis() as u64)
            }
            None => { None }
        }
    }

    pub fn get_duration(&self) -> Option<u64> {
        match self.player.current.clone() {
            Some(current) => {
                Some(current.duration.as_millis() as u64)
            }
            None => { None }
        }
    }

    pub fn seek_forwards(&mut self) {
        // let next_duration = self.get_position().unwrap() + 3000;
        // self.player.seek(ClockTime::from_mseconds(next_duration))
    }

    pub fn seek_backwards(&mut self) {
        // let song_progress_ms = self.get_position().unwrap();
        // let next_duration = if song_progress_ms < 3000 {
            // 0
        // } else {
            // song_progress_ms - 3000
        // };
        // self.player.seek(ClockTime::from_mseconds(next_duration))
    }

    #[allow(unused)]
    pub fn seek(&mut self, offset: i32) {
        let next_duration = self.get_position().unwrap() as i32 + (offset * 1000);
        // self.player
            // .seek(ClockTime::from_mseconds(next_duration as u64))
    }

    #[allow(unused)]
    pub fn position(&mut self, position: u64) {
        // self.player.seek(ClockTime::from_mseconds(position))
    }

    pub fn increase_volume(&mut self) {
        let current = self.player.get_volume();
        let volume = if current < 9.9 {
            current + 0.1_f32
        } else {
            10.0_f32
        };
        self.player.set_volume(volume);
    }

    pub fn decrease_volume(&mut self) {
        let current = self.player.get_volume();
        let volume = if current > 0.1 {
            current - 0.1_f32
        } else {
            0.0_f32
        };
        self.player.set_volume(volume);
    }
}
