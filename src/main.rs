#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;
extern crate config;
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate log;
// log panics to find unknown error
extern crate log_panics;

use dirs;
use failure::err_msg;
use log::LevelFilter;
use std::fs;
use std::io;
use std::path::Path;
use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::Terminal;
use util::event::{Event, Events};

mod api;
mod app;
mod handlers;
mod model;
mod player;
mod ui;
mod util;

mod dbus_mpris;

use app::{ActiveBlock, App};

use dbus_mpris::{dbus_mpris_handler, DbusMpris};

const FILE_NAME: &str = "Settings.toml";
const CONFIG_DIR: &str = ".config";
const APP_CONFIG_DIR: &str = "netease-music-tui";

fn main() -> Result<(), failure::Error> {
    let config_file_path = match dirs::home_dir() {
        Some(home) => {
            let path = Path::new(&home);
            let home_config_dir = path.join(CONFIG_DIR);
            let app_config_dir = home_config_dir.join(APP_CONFIG_DIR);

            if !home_config_dir.exists() {
                fs::create_dir(&home_config_dir)?;
            }

            if !app_config_dir.exists() {
                fs::create_dir(&app_config_dir)?;
            }
            let config_file_path = &app_config_dir.join(FILE_NAME);
            config_file_path.to_path_buf()
        }
        None => return Err(err_msg("No $HOME directory found for config")),
    };

    // init application
    let mut settings = config::Config::default();
    let config_string = match fs::read_to_string(&config_file_path) {
        Ok(data) => data,
        Err(_) => return Err(err_msg("Please set your account in config file")),
    };
    settings
        .merge(config::File::from_str(
            &config_string,
            config::FileFormat::Toml,
        ))
        .unwrap();

    match settings.get_bool("debug") {
        Ok(debug) => {
            if debug {
                log_panics::init();
                simple_logging::log_to_file("/var/log/ncmt.log", LevelFilter::Debug)?;
            }
        }
        Err(e) => error!("{}", e),
    }

    info!("start netease cloud music rust client");

    // init application
    let mut app = App::new();
    let mut is_first_render = true;

    let cloud_music = app.cloud_music.to_owned().unwrap();
    let profile = match cloud_music.login_status()? {
        Some(profile) => {
            app.user_id = profile.userId.unwrap();
            profile
        }
        None => {
            // need login
            let username = settings.get::<String>("username")?;
            let password = settings.get::<String>("password")?;
            match cloud_music.login(&username, &password) {
                Ok(profile) => profile,
                Err(_) => return Err(err_msg("Account/Password Error")),
            }
        }
    };

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = termion::input::MouseTerminal::from(stdout);
    let stdout = termion::screen::AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    terminal.hide_cursor()?;

    let events = Events::new();
    let dbus_mpris = DbusMpris::new();

    loop {
        terminal.draw(|mut f| {
            let current_route = app.get_current_route();
            match current_route.active_block {
                ActiveBlock::Help => {
                    ui::draw_help_menu(&mut f);
                }
                ActiveBlock::Msg => {
                    ui::draw_msg(&mut f, &mut app);
                }
                _ => {
                    ui::draw_main_layout(&mut f, &mut app);
                }
            }
        })?;

        // try get dbus cmd
        match dbus_mpris.next() {
            Ok(cmd) => {
                dbus_mpris_handler(cmd, &mut app);
            }
            Err(_) => {}
        }

        match events.next()? {
            Event::Input(input) => {
                match input {
                    Key::Char('q') => {
                        if app.get_current_route().active_block != ActiveBlock::Search {
                            let pop_result = app.pop_navigation_stack();
                            if pop_result.is_none() {
                                break; // Exit application
                            }
                        }
                    }
                    Key::Ctrl('c') => {
                        break;
                    }
                    // means space
                    Key::Char(' ') => {
                        if app.player.is_playing() {
                            app.player.pause();
                        } else {
                            app.player.play();
                        }
                    }
                    _ => {
                        handlers::handle_app(input, &mut app);
                    }
                }
            }
            Event::Tick => {
                app.update_on_tick();
            }
        }

        if is_first_render {
            let cloud_music = app.cloud_music.to_owned().unwrap();
            let playlists = cloud_music.user_playlists(&profile.userId.unwrap().to_string());
            match playlists {
                Ok(p) => {
                    app.playlists = Some(p);
                    app.selected_playlist_index = Some(0);
                }
                Err(e) => {
                    app.handle_error(e);
                }
            };
            is_first_render = false;
        }
    }
    Ok(())
}
