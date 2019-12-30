extern crate gstreamer as gst;
extern crate gstreamer_player as gst_player;

use gst::prelude::*;
use gst::ClockTime;

// TODO: change gstreamer for more less dependence diy player

#[allow(unused)]
pub enum PlayerCommand {
    Play,
    Pause,
    Stop,
    Seek(u32),
    Next,
    Previous,
    Load(String),
    Status,
    LoopStatus,
    Postion,
    Metadata,
}

pub struct Nplayer {
    pub player: gst_player::Player,
    // pub song_progress_ms: u64,
}

impl Nplayer {
    pub fn new() -> Nplayer {
        let dispatcher = gst_player::PlayerGMainContextSignalDispatcher::new(None);
        let music_player = gst_player::Player::new(
            None,
            Some(&dispatcher.upcast::<gst_player::PlayerSignalDispatcher>()),
        );
        Nplayer {
            player: music_player,
        }
    }

    pub fn play_url(&self, url: &str) {
        self.player.set_uri(&url);
        self.player.play();
    }

    pub fn is_playing(&self) -> bool {
        let player = &self.player;
        let element = player.get_pipeline();
        element.get_state(gst::CLOCK_TIME_NONE).1 == gst::State::Playing
    }

    pub fn pause(&self) {
        self.player.pause()
    }

    pub fn play(&self) {
        self.player.play()
    }

    pub fn get_position(&self) -> Option<u64> {
        self.player.get_position().mseconds()
    }

    pub fn get_duration(&self) -> Option<u64> {
        self.player.get_duration().mseconds()
    }

    pub fn seek_forwards(&mut self) {
        let next_duration = self.get_position().unwrap() + 3000;
        self.player.seek(ClockTime::from_mseconds(next_duration))
    }

    pub fn seek_backwards(&mut self) {
        let song_progress_ms = self.get_position().unwrap();
        let next_duration = if song_progress_ms < 3000 {
            0
        } else {
            song_progress_ms - 3000
        };
        self.player.seek(ClockTime::from_mseconds(next_duration))
    }

    pub fn increase_volume(&mut self) {
        let current = self.player.get_volume();
        let volume = if current < 9.9 {
            current + 0.1_f64
        } else {
            10.0_f64
        };
        self.player.set_volume(volume);
    }

    pub fn decrease_volume(&mut self) {
        let current = self.player.get_volume();
        let volume = if current > 0.1 {
            current - 0.1_f64
        } else {
            0.0_f64
        };
        self.player.set_volume(volume);
    }
}
