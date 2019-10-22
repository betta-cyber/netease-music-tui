#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;
extern crate serde;
extern crate serde_json;


use std::io;
use termion::raw::IntoRawMode;
use tui::{Frame, Terminal};
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders, Tabs, Text, Paragraph, SelectableList};
use tui::layout::{Layout, Constraint, Direction, Rect};
use tui::style::{Color, Style, Modifier};
use termion::event::Key;
use util::event::{Event, Events};
use tui::backend::Backend;

mod util;
mod model;
mod app;
mod api;
mod handlers;
mod ui;

use app::{App, ActiveBlock};
use api::CloudMusic;
use model::playlist::PlaylistDetail;
use ui::{draw_track_table, draw_main_layout};

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

struct UI<'a> {
    tabs: TabsState<'a>,
    playlist: PlaylistDetail,
}

fn main() -> Result<(), failure::Error> {

    // init application
    let mut app = App::new();
    let cloud_music = CloudMusic::default();

    let mut is_first_render = true;

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = termion::input::MouseTerminal::from(stdout);
    let stdout = termion::screen::AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.hide_cursor()?;

    let events = Events::new();

    loop {
        terminal.draw(|mut f| {
            ui::draw_main_layout(&mut f, &app);
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                // means space
                Key::Char(' ') => {
                    if app.player.is_playing() {
                        app.player.pause();
                    } else {
                        app.player.play().unwrap();
                    }
                }
                // Key::Right => tui.tabs.next(),
                // Key::Left => tui.tabs.previous(),
                _ => {
                    handlers::handle_app(input, &mut app);
                }
            },
            _ => {}
        }

        if is_first_render {
            let playlists = cloud_music.current_user_playlists();
            match playlists {
                Ok(p) => {
                    app.playlists = Some(p);
                    app.selected_playlist_index = Some(0);
                    app.set_current_route_state(Some(ActiveBlock::Recommend), None);
                }
                Err(e) => {
                    panic!("error")
                }
            }
            is_first_render = false;
        }
    }
    Ok(())
}
