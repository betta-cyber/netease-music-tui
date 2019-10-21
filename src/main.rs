#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;
extern crate gstreamer as gst;
extern crate gstreamer_player as gst_player;
use gst::prelude::*;

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

use app::App;
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

    // init gst
    gst::init()?;

    let mut app = App::new();
    let cloud_music = CloudMusic::default();

    let mut is_first_render = true;

    // let profile = cloud_music.status();
    // println!("{:#?}", profile);

    // let playlist = cloud_music.playlist_detail("2330204571").unwrap().clone();
    // app.track_table.tracks = playlist.tracks.clone();
    // app.get_playlist_tracks("2330204571".to_owned());

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = termion::input::MouseTerminal::from(stdout);
    let stdout = termion::screen::AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.hide_cursor()?;

    let events = Events::new();
    // let mut tui = UI {
        // tabs: TabsState::new(vec!["Tab0", "COOL", "Tab2", "Tab3"]),
        // playlist: playlist
    // };

    loop {
        terminal.draw(|mut f| {
            ui::draw_main_layout(&mut f, &app);
            /* let size = f.size(); */
            // let chunks = Layout::default()
                // .direction(Direction::Vertical)
                // .margin(2)
                // .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                // .split(size);
            // Tabs::default()
                // .block(Block::default().borders(Borders::ALL).title("Tabs"))
                // .titles(&tui.tabs.titles)
                // .select(tui.tabs.index)
                // .style(Style::default().fg(Color::Cyan))
                // .highlight_style(Style::default().fg(Color::Yellow))
                // .render(&mut f, chunks[0]);

            // match tui.tabs.index {
                // 0 => draw_track_table(&mut f, &app, chunks[1]),
                // 1 => draw_first_tab(&mut f, &tui, chunks[1]),
                // 2 => Block::default()
                    // .title("Inner 2")
                    // .borders(Borders::ALL)
                    // .render(&mut f, chunks[1]),
                // 3 => Block::default()
                    // .title("Inner 3")
                    // .borders(Borders::ALL)
                    // .render(&mut f, chunks[1]),
                // _ => {}
            /* } */
        })?;

        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                // means space
                Key::Char(' ') => {
                    if is_playing(&app.player) {
                        app.player.pause();
                    } else {
                        app.player.play();
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
                    app.get_user_playlists()
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


fn draw_music_tab<B>(f: &mut Frame<B>, tui: &UI, area: Rect, app: &App)
where
    B: Backend,
{

    let playlist_items: Vec<_> = tui.playlist.tracks.iter().map(|item| item.name.as_ref().unwrap().to_string()).collect();

    let chunks = SelectableList::default()
        .block(
            Block::default()
                .title("歌曲列表")
                .borders(Borders::ALL)
                .title_style(Style::default().fg(Color::LightCyan))
                .border_style(Style::default().fg(Color::LightCyan))
            )
        .items(&playlist_items)
        .select(Some(app.track_table.selected_index))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::LightCyan))
        .highlight_symbol(">")
        .render(f, area);

}


fn draw_first_tab<B>(f: &mut Frame<B>, tui: &UI, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints(
            [
                Constraint::Length(7),
                Constraint::Min(7),
                Constraint::Length(7),
            ]
            .as_ref(),
        )
        .split(area);
    draw_text(f, chunks[0]);
    draw_text(f, chunks[1]);
    draw_text(f, chunks[2]);
}

fn draw_text<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let text = [
        Text::raw("This is a paragraph with several lines. You can change style your text the way you want.\n\nFox example: "),
        Text::styled("under", Style::default().fg(Color::Red)),
        Text::raw(" "),
        Text::styled("the", Style::default().fg(Color::Green)),
        Text::raw(" "),
        Text::styled("rainbow", Style::default().fg(Color::Blue)),
        Text::raw(".\nOh and if you didn't "),
        Text::styled("notice", Style::default().modifier(Modifier::ITALIC)),
        Text::raw(" you can "),
        Text::styled("automatically", Style::default().modifier(Modifier::BOLD)),
        Text::raw(" "),
        Text::styled("wrap", Style::default().modifier(Modifier::REVERSED)),
        Text::raw(" your "),
        Text::styled("text", Style::default().modifier(Modifier::UNDERLINED)),
        Text::raw(".\nOne more thing is that it should display unicode characters: 10€")
    ];

    Paragraph::new(text.iter())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("TEXT")
                .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::BOLD)),
        )
        .wrap(true)
        .render(f, area);
}


 fn is_playing(player: &gst_player::Player) -> bool {
    let element = player.get_pipeline();
    element.get_state(gst::CLOCK_TIME_NONE).1 == gst::State::Playing
}
