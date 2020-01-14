extern crate rodio;
extern crate tokio;
extern crate tempfile;

mod player;
mod fetch;
mod sink;
mod range_set;
// mod fetch_data;

use player::Player;
use sink::find;
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
    // pub song_progress_ms: u64,
}

impl Nplayer {
    pub fn new() -> Nplayer {
        let backend = find(None).unwrap();
        let (mplayer, _) = Player::new(move || (backend)(None));
        debug!("init player");
        Nplayer {
            player: mplayer,
        }
    }

    pub fn play_url(&self, url: &str) {
        self.player.load(&url, true);
    }

    pub fn is_playing(&self) -> bool {
        // let player = &self.player;
        true
        // let element = player.get_pipeline();
        // element.get_state(gst::CLOCK_TIME_NONE).1 == gst::State::Playing
    }

    pub fn pause(&self) {
        self.player.pause()
    }

    pub fn play(&self) {
        self.player.play()
    }

    #[allow(unused)]
    pub fn stop(&self) {
        self.player.stop()
    }

    pub fn get_position(&self) -> Option<u64> {
        // self.player.get_position().mseconds()
        Some(1000_u64)
    }

    pub fn get_duration(&self) -> Option<u64> {
        // self.player.sink.total_duration()
        Some(100000000_u64)
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
        // let current = self.player.get_volume();
        // let volume = if current < 9.9 {
            // current + 0.1_f64
        // } else {
            // 10.0_f64
        // };
        // self.player.set_volume(volume);
    }

    pub fn decrease_volume(&mut self) {
        // let current = self.player.get_volume();
        // let volume = if current > 0.1 {
            // current - 0.1_f64
        // } else {
            // 0.0_f64
        // };
        // self.player.set_volume(volume);
    }
}
