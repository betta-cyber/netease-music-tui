mod util;
pub mod circle;

use super::app::{App, ActiveBlock, RouteId, RECOMMEND_OPTIONS, RepeatState};
use tui::{Frame, Terminal};
use tui::widgets::{Widget, Block, Borders, Text, Table, SelectableList, Row, Gauge, Paragraph, Tabs, canvas::Canvas};
use tui::layout::{Layout, Constraint, Direction, Rect};
use tui::style::{Color, Style, Modifier};
use tui::backend::Backend;
use circle::Circle;
use util::{get_color, get_percentage_width, display_track_progress, create_artist_string};

// table item for render
#[derive(Clone, Debug)]
pub struct TableItem {
    id: String,
    format: Vec<String>,
}

pub struct TableHeader<'a> {
    text: &'a str,
    width: u16,
}

pub fn draw_main_layout<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let parent_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(6),
            ]
            .as_ref(),
        )
        .margin(2)
        .split(f.size());

    // Search input and help
    draw_input_and_help_box(f, app, parent_layout[0]);

    // Nested main block with potential routes
    draw_routes(f, app, parent_layout[1]);

    // Currently playing
    draw_playing_block(f, app, parent_layout[2]);
}

pub fn draw_input_and_help_box<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(layout_chunk);

    let current_route = app.get_current_route();

    let highlight_state = (
        current_route.active_block == ActiveBlock::Search,
        current_route.hovered_block == ActiveBlock::Search,
    );

    Paragraph::new([Text::raw(&app.input)].iter())
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Search")
                .title_style(get_color(highlight_state))
                .border_style(get_color(highlight_state)),
        )
        .render(f, chunks[0]);

    let block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray))
        .title_style(Style::default().fg(Color::Gray));

    Paragraph::new([Text::raw("Type ?")].iter())
        .block(block)
        .style(Style::default().fg(Color::Gray))
        .render(f, chunks[1]);
}

pub fn draw_routes<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(layout_chunk);

    draw_user_block(f, app, chunks[0]);

    let current_route = app.get_current_route();

    match current_route.id {
        RouteId::Error => {} // This is handled as a "full screen" route in main.rs
        RouteId::TrackTable => {
            draw_track_table(f, &app, chunks[1]);
        }
        RouteId::Search => {
            draw_search_results(f, app, chunks[1]);
        }
        RouteId::Home => {
            draw_home(f, app, chunks[1]);
        }
        RouteId::PersonalFm => {
            draw_personal_fm(f, &app, chunks[1]);
        }
        // RouteId::Help => {
            // draw_help(f);
        // }
        _ => {
            draw_track_table(f, &app, chunks[1]);
        }
    };
}

// draw track playing block in the bottom
pub fn draw_playing_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .margin(1)
        .split(layout_chunk);

        let state_title = if app.player.is_playing() {
            "Playing"
        } else {
            "Pause"
        };

        let repeat_text = match app.repeat_state {
            RepeatState::Off => "Off",
            RepeatState::Track => "Track",
            RepeatState::All => "All",
            RepeatState::Shuffle => "Shuffle",
        };

        let title = format!("{} | Repeat: {}", state_title, repeat_text);

        let current_route = app.get_current_route();
        let highlight_state = (
            current_route.active_block == ActiveBlock::PlayBar,
            current_route.hovered_block == ActiveBlock::PlayBar,
        );

        let (track_name, artist_name) = match &app.current_playing {
            Some(track) => {
                (
                    track.name.to_owned().unwrap(),
                    match &track.artists {
                        Some(artists) => {
                            create_artist_string(&artists)
                        }
                        None => "Unknown".to_string()
                    }
                )
            }
            None => {
                (
                    String::new(),
                    String::new(),
                )
            }
        };

        Block::default()
            .borders(Borders::ALL)
            .title(&title)
            .title_style(get_color(highlight_state))
            .border_style(get_color(highlight_state))
            .render(f, layout_chunk);

        Paragraph::new(
            [Text::styled(
                artist_name,
                Style::default().fg(Color::White),
            )]
            .iter(),
        )
        .style(Style::default().fg(Color::White))
        .block(
            Block::default().title(&track_name).title_style(
                Style::default()
                    .fg(Color::LightCyan)
                    .modifier(Modifier::BOLD),
            ),
        )
        .render(f, chunks[0]);

        let (perc, label) = match app.duration_ms {
            Some(duration_ms) => {
                (
                    (app.song_progress_ms as f64 / duration_ms as f64) * 100_f64,
                    display_track_progress(app.song_progress_ms, duration_ms)
                )
            }
            None => {
                (0.0_f64, " ".to_string())
            }
        };

        Gauge::default()
            .block(Block::default().title(""))
            .style(
                Style::default()
                    .fg(Color::LightCyan)
                    .bg(Color::Black)
                    .modifier(Modifier::ITALIC | Modifier::BOLD),
            )
            .percent(perc as u16)
            .label(&label)
            .render(f, chunks[1]);
}

pub fn draw_user_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(layout_chunk);

    draw_recommend_block(f, app, chunks[0]);
    draw_playlist_block(f, app, chunks[1]);
}

pub fn draw_recommend_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Recommend,
        current_route.hovered_block == ActiveBlock::Recommend,
    );
    draw_selectable_list(
        f,
        layout_chunk,
        "Recommend",
        &RECOMMEND_OPTIONS,
        highlight_state,
        Some(app.recommend.selected_index),
    );
}

pub fn draw_playlist_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let playlist_items = match &app.playlists {
        Some(p) => p.iter().map(|item| item.name.as_ref().unwrap().to_owned()).collect(),
        None => vec![],
    };

    let current_route = app.get_current_route();

    let highlight_state = (
        current_route.active_block == ActiveBlock::MyPlaylists,
        current_route.hovered_block == ActiveBlock::MyPlaylists,
    );

    draw_selectable_list(
        f,
        layout_chunk,
        "Playlists",
        &playlist_items,
        highlight_state,
        app.selected_playlist_index,
    );
}

// draw selectable list
fn draw_selectable_list<B, S>(
    f: &mut Frame<B>,
    layout_chunk: Rect,
    title: &str,
    items: &[S],
    highlight_state: (bool, bool),
    selected_index: Option<usize>,
) where
    B: Backend,
    S: std::convert::AsRef<str>,
{
    SelectableList::default()
        .block(
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .title_style(get_color(highlight_state))
                .border_style(get_color(highlight_state)),
        )
        .items(items)
        .style(Style::default().fg(Color::White))
        .select(selected_index)
        .highlight_style(get_color(highlight_state).modifier(Modifier::BOLD))
        .render(f, layout_chunk);
}

// draw track table
pub fn draw_track_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{

    let header = [
        TableHeader {
            text: "ID",
            width: get_percentage_width(layout_chunk.width, 0.05),
        },
        TableHeader {
            text: "Title",
            width: get_percentage_width(layout_chunk.width, 0.3),
        },
        TableHeader {
            text: "Artist",
            width: get_percentage_width(layout_chunk.width, 0.3),
        },
        TableHeader {
            text: "Album",
            width: get_percentage_width(layout_chunk.width, 0.3),
        },
    ];

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::TrackTable,
        current_route.hovered_block == ActiveBlock::TrackTable,
    );

    let mut num = 0;
    let items = app
        .track_table
        .tracks
        .iter()
        .map(|item| {
            num += 1;
            TableItem {
                id: item.id.as_ref().unwrap().to_string(),
                format: vec![
                    num.to_string(),
                    item.name.as_ref().unwrap().to_string(),
                    create_artist_string(&item.artists.to_owned().unwrap()),
                    item.album.as_ref().unwrap().name.to_owned(),
                ],
            }
        })
        .collect::<Vec<TableItem>>();

    // draw track table by draw_table function
    draw_table(
        f,
        app,
        layout_chunk,
        (&app.track_table.name, &header),
        &items,
        app.track_table.selected_index,
        highlight_state,
    )
}

fn draw_table<B>(
    f: &mut Frame<B>,
    app: &App,
    layout_chunk: Rect,
    table_layout: (&str, &[TableHeader]), // (title, header colums)
    items: &[TableItem], // The nested vector must have the same length as the `header_columns`
    selected_index: usize,
    highlight_state: (bool, bool),
) where
    B: Backend,
{
    let selected_style = get_color(highlight_state).modifier(Modifier::BOLD);

    // caculate index and row
    let interval = (layout_chunk.height / 2) as usize;
    let (row_items, margin) = if !items.is_empty() {
        let count = (layout_chunk.height - 4) as usize;
        let total = items.len();
        if selected_index >= count - interval && total > count {
            if selected_index >= total - interval {
                let margin = total - count;
                (&items[margin..], margin)
            } else {
                let margin = selected_index + interval - count;
                (&items[margin..], margin)
            }
        } else {
            (items, 0)
        }
    } else {
        (items, 0)
    };

    let rows = row_items.iter().enumerate().map(|(i, item)| {
        // Show this ♥ if the song is liked
        let mut formatted_row = item.format.clone();
        let mut style = Style::default().fg(Color::White); // default styling

        // First check if the song should be highlighted because it is currently playing
        match &app.current_playing {
            Some(track) => {
                if item.id == track.id.unwrap().to_string() {
                    formatted_row[0] = format!("|> {}", &formatted_row[0]);
                    style = Style::default().fg(Color::White).modifier(Modifier::BOLD);
                }
            }
            None => {}
        }

        // Next check if the item is under selection
        if i == selected_index - margin {
            style = selected_style;
        }

        // Return row styled data
        Row::StyledData(formatted_row.into_iter(), style)
    });

    let (title, header_columns) = table_layout;

    let widths = header_columns.iter().map(|h| h.width).collect::<Vec<u16>>();

    Table::new(header_columns.iter().map(|h| h.text), rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title(title)
                .title_style(get_color(highlight_state))
                .border_style(get_color(highlight_state)),
        )
        .style(Style::default().fg(Color::White))
        .widths(&widths)
        .render(f, layout_chunk);
}

fn draw_home<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .margin(2)
        .split(layout_chunk);

    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::Home,
        current_route.hovered_block == ActiveBlock::Home,
    );

    Block::default()
        .title("Welcome!")
        .borders(Borders::ALL)
        .title_style(get_color(highlight_state))
        .border_style(get_color(highlight_state))
        .render(f, layout_chunk);

    let top_text = vec![
        Text::styled("网易云音乐", Style::default().fg(Color::LightCyan)),
    ];

    // Contains the banner
    Paragraph::new(top_text.iter())
        .style(Style::default().fg(Color::White))
        .block(Block::default())
        .render(f, chunks[0]);

    // Canvas::default()
        // .block(
            // Block::default()
            // .borders(Borders::ALL)
            // .title("Welcome!")
        // )
        // .paint(|ctx| {
            // ctx.draw(&Circle::default());
        // })
        // .x_bounds([-180.0, 180.0])
        // .y_bounds([-90.0, 90.0])
        // .render(f, layout_chunk);
}

fn draw_personal_fm<B>(
    f: &mut Frame<B>,
    app: &App,
    layout_chunk: Rect,
) where
    B: Backend,
{
    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::PersonalFm,
        current_route.hovered_block == ActiveBlock::PersonalFm,
    );
    let display_block = Block::default()
        .title(&"PERSONAL FM")
        .borders(Borders::ALL)
        .title_style(get_color(highlight_state))
        .border_style(get_color(highlight_state));

    let text = vec![Text::raw("Not implemented yet!")];

    Paragraph::new(text.iter())
        .style(Style::default().fg(Color::White))
        .block(display_block)
        .wrap(true)
        .render(f, layout_chunk);
}

pub fn draw_search_results<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{


    let current_route = app.get_current_route();
    let highlight_state = (
        current_route.active_block == ActiveBlock::SearchResult,
        current_route.hovered_block == ActiveBlock::SearchResult,
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(layout_chunk);

    {
        let songs = match &app.search_results.tracks {
            Some(r) => r
                .iter()
                .map(|item| format!("{} - {}", item.name.as_ref().unwrap().to_owned(), create_artist_string(&item.artists.as_ref().unwrap())))
                .collect(),
            None => vec![],
        };
        let playlists = match &app.search_results.playlists {
            Some(r) => r
                .iter()
                .map(|item| format!("{} - {}", item.name.as_ref().unwrap().to_owned(), item.creator.as_ref().unwrap().nickname.as_ref().unwrap().to_owned()))
                .collect(),
            None => vec![],
        };
        let artists = match &app.search_results.artists {
            Some(r) => r
                .iter()
                .map(|item| item.name.as_ref().unwrap().to_owned())
                .collect(),
            None => vec![],
        };
        let albums = match &app.search_results.albums {
            Some(r) => r
                .iter()
                .map(|item| item.name.as_ref().unwrap().to_owned())
                .collect(),
            None => vec![],
        };


        Tabs::default()
            .block(
                Block::default()
                .borders(Borders::ALL)
                .title("Search Result")
                .title_style(get_color(highlight_state))
                .border_style(get_color(highlight_state)),
            )
            .titles(&app.tabs.titles)
            .select(app.tabs.index)
            .style(Style::default().fg(Color::Cyan))
            .highlight_style(Style::default().fg(Color::Yellow))
            .render(f, chunks[0]);

        match app.tabs.index {
            0 => draw_selectable_list(
                f,
                chunks[1],
                "Songs",
                &songs,
                highlight_state,
                Some(app.search_results.selected_tracks_index),
            ),
            1 => draw_selectable_list(
                f,
                chunks[1],
                "Artists",
                &artists,
                highlight_state,
                Some(app.search_results.selected_artists_index),
            ),
            2 => draw_selectable_list(
                f,
                chunks[1],
                "Albums",
                &albums,
                highlight_state,
                Some(app.search_results.selected_albums_index),
            ),
            3 => draw_selectable_list(
                f,
                chunks[1],
                "Playlists",
                &playlists,
                highlight_state,
                Some(app.search_results.selected_playlists_index),
            ),
            _ => {}
        }
    }
}


pub fn draw_help_menu<B>(f: &mut Frame<B>)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(2)
        .split(f.size());

    let white = Style::default().fg(Color::White);
    let gray = Style::default().fg(Color::White);
    let header = ["Context", "Event", "Description"];

    let help_docs = vec![
        vec!["General", "a", "Jump to currently playing album"],
    ];

    let rows = help_docs
        .iter()
        .map(|item| Row::StyledData(item.iter(), gray));

    Table::new(header.iter(), rows)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(white)
                .title("Help (press <Esc> to go back)")
                .title_style(gray)
                .border_style(gray),
        )
        .style(Style::default().fg(Color::White))
        .widths(&[20, 40, 50])
        .render(f, chunks[0]);
}

pub fn draw_playing_detail<B>(f: &mut Frame<B>, app: &App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .margin(2)
        .split(f.size());

    Canvas::default()
        // .block(
            // Block::default()
            // // .borders(Borders::ALL)
            // // .title("Playing")
        // )
        .paint(|ctx| {
            ctx.draw(&app.playing_circle);
        })
        .x_bounds([-90.0, 90.0])
        .y_bounds([-90.0, 90.0])
        .render(f, chunks[0]);

    let playlist_items = match &app.playlists {
        Some(p) => p.iter().map(|item| item.name.as_ref().unwrap().to_owned()).collect(),
        None => vec![],
    };
    let selected_index = Some(0);


    SelectableList::default()
        .block(
            Block::default()
        )
        .items(&playlist_items)
        .style(Style::default().fg(Color::White))
        .select(selected_index)
        .render(f, chunks[1]);
}
