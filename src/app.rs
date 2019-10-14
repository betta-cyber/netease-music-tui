extern crate gstreamer as gst;
extern crate gstreamer_player as gst_player;
use tui::layout::{Layout, Constraint, Direction, Rect};

use gst::prelude::*;

pub struct App {
    pub player: gst_player::Player,
    pub size: Rect,
    pub input: String,
    pub song_progress_ms: u128,
}

impl App {
    pub fn new() -> App {

        let dispatcher = gst_player::PlayerGMainContextSignalDispatcher::new(None);
        let music_player = gst_player::Player::new(
            None,
            Some(&dispatcher.upcast::<gst_player::PlayerSignalDispatcher>()),
        );

        App {
            player: music_player,
            size: Rect::default(),
            input: String::new(),
            song_progress_ms: 0,
        }
    }
}
