#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;
extern crate serde;
extern crate serde_json;
extern crate config;
#[macro_use]
extern crate log;
extern crate gstreamer as gst;
extern crate gstreamer_player as gst_player;
// log panics to find unknown error
// extern crate log_panics;
use gst::prelude::*;

use std::io;
use termion::raw::IntoRawMode;
use tui::Terminal;
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders, Tabs, Text, Paragraph, SelectableList};
use tui::layout::{Layout, Constraint, Direction, Rect};
use tui::style::{Color, Style, Modifier};
use termion::event::Key;
use util::event::{Event, Events};
use log::LevelFilter;

mod util;
mod model;
mod app;
mod api;
mod handlers;
mod ui;

use app::{App, ActiveBlock};

fn main() -> Result<(), failure::Error> {

    simple_logging::log_to_file("test.log", LevelFilter::Trace);

    info!("start netease cloud music rust client");

    gst::init()?;

    // init application
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("Settings")).unwrap();

    let username = settings.get::<String>("username")?;
    let password = settings.get::<String>("password")?;

    let mut app = App::new();
    let mut is_first_render = true;

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = termion::input::MouseTerminal::from(stdout);
    let stdout = termion::screen::AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;


    terminal.hide_cursor()?;

    let events = Events::new();


    loop {
        let current_route = app.get_current_route();
        terminal.draw(|mut f| match current_route.active_block {
            ActiveBlock::Help => {
                ui::draw_help_menu(&mut f);
            }
            // ActiveBlock::Playing => {
                // ui::draw_playing_detail(&mut f, &app);
            // }
            _ => {
                ui::draw_main_layout(&mut f, &app);
            }
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                // means space
                Key::Char(' ') => {
                    if app.is_playing() {
                        app.player.pause();
                    } else {
                        app.player.play();
                    }
                }
                Key::Esc => {
                    if app.get_current_route().active_block != ActiveBlock::Search {
                        // Go back through navigation stack when not in search input mode and exit the app if there are no more places to back to
                        let pop_result = app.pop_navigation_stack();
                        if pop_result.is_none() {
                            break; // Exit application
                        }
                    }
                }
                _ => {
                    handlers::handle_app(input, &mut app);
                }
            },
            Event::Tick => {
                app.update_on_tick();
            }
        }

        if is_first_render {
            let cloud_music = app.cloud_music.to_owned().unwrap();
            let profile = match cloud_music.login_status()? {
                Some(p) => {p}
                None => {
                    // need login
                    cloud_music.phone_login(&username, &password)?
                }
            };

            let playlists = cloud_music.user_playlists(&profile.userId.unwrap().to_string());
            match playlists {
                Ok(p) => {
                    app.playlists = Some(p);
                    app.selected_playlist_index = Some(0);
                }
                Err(e) => {
                    panic!("error {}", e)
                }
            }
            is_first_render = false;
        }
    }
    Ok(())
}
