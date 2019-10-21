mod util;

use super::app::{App, ActiveBlock, RouteId, RECOMMEND_OPTIONS};
use tui::{Frame, Terminal};
use tui::backend::TermionBackend;
use tui::widgets::{Widget, Block, Borders, Text, Table, SelectableList, Row, Gauge, Paragraph};
use tui::layout::{Layout, Constraint, Direction, Rect};
use tui::style::{Color, Style, Modifier};
use termion::event::Key;
use tui::backend::Backend;
use util::{get_color, get_percentage_width};

// table item for render
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
        current_route.active_block == ActiveBlock::Input,
        current_route.hovered_block == ActiveBlock::Input,
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
        _ => {
            draw_track_table(f, &app, chunks[1]);
        }
    };
}

pub fn draw_playing_block<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .margin(1)
        .split(layout_chunk);

    // If no track is playing, render paragraph showing which device is selected, if no selected
    // give hint to choose a device
    // if let Some(current_playback_context) = &app.current_playback_context {
        // if let Some(track_item) = &current_playback_context.item {
            // let play_title = if current_playback_context.is_playing {
                // "Playing"
            // } else {
                // "Paused"
            // };

            // let shuffle_text = if current_playback_context.shuffle_state {
                // "On"
            // } else {
                // "Off"
            // };

            // let repeat_text = match current_playback_context.repeat_state {
                // RepeatState::Off => "Off",
                // RepeatState::Track => "Track",
                // RepeatState::Context => "All",
            // };

       /* let title = format!( */
           // "{} ({} | Shuffle: {} | Repeat: {} | Volume: {}%)",
           // play_title,
           // current_playback_context.device.name,
           // shuffle_text,
           // repeat_text,
           // current_playback_context.device.volume_percent
       /* ); */
        let title = "1111".to_string();

        Block::default()
            .borders(Borders::ALL)
            .title(&title)
            .title_style(Style::default().fg(Color::Gray))
            .border_style(Style::default().fg(Color::Gray))
            .render(f, layout_chunk);

        Paragraph::new(
            [Text::styled(
                "111".to_string(),
                Style::default().fg(Color::White),
            )]
            .iter(),
        )
        .style(Style::default().fg(Color::White))
        .block(
            Block::default().title("ddd").title_style(
                Style::default()
                    .fg(Color::LightCyan)
                    .modifier(Modifier::BOLD),
            ),
        )
        .render(f, chunks[0]);

        // let perc = (app.song_progress_ms as f64 / f64::from(track_item.duration_ms)) * 100_f64;

        Gauge::default()
            .block(Block::default().title(""))
            .style(
                Style::default()
                    .fg(Color::LightCyan)
                    .bg(Color::Black)
                    .modifier(Modifier::ITALIC | Modifier::BOLD),
            )
            .percent(0.3_f64 as u16)
            .label(&"1:00".to_string())
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
        "recommend",
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

// fn draw_table<B>(
    // f: &mut Frame<B>,
    // app: &App,
    // layout_chunk: Rect,
    // table_layout: (&str, &[TableHeader]), // (title, header colums)
    // items: &[TableItem], // The nested vector must have the same length as the `header_columns`
    // selected_index: usize,
    // highlight_state: (bool, bool),
// ) where
    // B: Backend,
// {
    // let selected_style = get_color(highlight_state).modifier(Modifier::BOLD);

    // let track_playing_index = match &app.current_playback_context {
        // Some(ctx) => items.iter().position(|t| match &ctx.item {
            // Some(item) => Some(t.id.to_owned()) == item.id,
            // None => false,
        // }),
        // None => None,
    // };

    // let rows = items.iter().enumerate().map(|(i, item)| {
        // // Show this ♥ if the song is liked
        // let mut formatted_row = item.format.clone();
        // let mut style = Style::default().fg(Color::White); // default styling

        // // First check if the song should be highlighted because it is currently playing
        // if let Some(_track_playing_index) = track_playing_index {
            // if i == _track_playing_index {
                // formatted_row[0] = format!("|> {}", &formatted_row[0]);
                // style = Style::default().fg(Color::Cyan).modifier(Modifier::BOLD);
            // }
        // }

        // // Next check if the item is under selection
        // if i == selected_index {
            // style = selected_style;
        // }

        // // Return row styled data
        // Row::StyledData(formatted_row.into_iter(), style)
    // });

    // let (title, header_columns) = table_layout;

    // let widths = header_columns.iter().map(|h| h.width).collect::<Vec<u16>>();

    // Table::new(header_columns.iter().map(|h| h.text), rows)
        // .block(
            // Block::default()
                // .borders(Borders::ALL)
                // .style(Style::default().fg(Color::White))
                // .title(title)
                // .title_style(get_color(highlight_state))
                // .border_style(get_color(highlight_state)),
        // )
        // .style(Style::default().fg(Color::White))
        // .widths(&widths)
        // .render(f, layout_chunk);
// }

pub fn draw_track_table<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{

    let playlist_items: Vec<_> = app.track_table.tracks.iter().map(|item| item.name.as_ref().unwrap().to_string()).collect();

    let chunks = SelectableList::default()
        .block(
            Block::default()
                .title("Songs")
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


// pub fn draw_song_table<B>(f: &mut Frame<B>, app: &App, layout_chunk: Rect)
// where
    // B: Backend,
// {
    // let header = [
        // TableHeader {
            // text: "Title",
            // width: get_percentage_width(layout_chunk.width, 0.3),
        // },
        // TableHeader {
            // text: "Artist",
            // width: get_percentage_width(layout_chunk.width, 0.3),
        // },
        // TableHeader {
            // text: "AlbumTracks",
            // width: get_percentage_width(layout_chunk.width, 0.3),
        // },
        // TableHeader {
            // text: "Length",
            // width: get_percentage_width(layout_chunk.width, 0.1),
        // },
    // ];

    // let current_route = app.get_current_route();
    // let highlight_state = (
        // current_route.active_block == ActiveBlock::TrackTable,
        // current_route.hovered_block == ActiveBlock::TrackTable,
    // );

    // let items = app
        // .track_table
        // .tracks
        // .iter()
        // .map(|item| TableItem {
            // id: item.id.clone().unwrap_or_else(|| "".to_string()),
            // format: vec![
                // item.name.to_owned(),
                // create_artist_string(&item.artists),
                // item.album.name.to_owned(),
                // millis_to_minutes(u128::from(item.duration_ms)),
            // ],
        // })
        // .collect::<Vec<TableItem>>();

    // draw_table(
        // f,
        // app,
        // layout_chunk,
        // ("Songs", &header),
        // &items,
        // app.track_table.selected_index,
        // highlight_state,
    // )
// }
