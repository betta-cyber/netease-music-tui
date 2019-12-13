use super::super::app::{App, ActiveBlock};
use termion::event::Key;
use super::common_events;

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Char('\n') => {
            let current_hovered = app.get_current_route().hovered_block;
            app.set_current_route_state(Some(current_hovered), None);
        }
        k if common_events::left_event(k) => match app.get_current_route().hovered_block {
            ActiveBlock::Artist
            | ActiveBlock::AlbumList
            | ActiveBlock::AlbumTracks
            | ActiveBlock::Home
            | ActiveBlock::SearchResult
            | ActiveBlock::Playlist
            | ActiveBlock::PersonalFm
            | ActiveBlock::Playing
            | ActiveBlock::TrackTable => {
                app.set_current_route_state(None, Some(ActiveBlock::Recommend));
            }
            _ => {}
        },
        k if common_events::up_event(k) => match app.get_current_route().hovered_block {
            ActiveBlock::MyPlaylists => {
                app.set_current_route_state(None, Some(ActiveBlock::Recommend));
            }
            // ActiveBlock::PlayBar => {
                // app.set_current_route_state(None, Some(ActiveBlock::MyPlaylists));
            // }
            ActiveBlock::Recommend => {
                app.set_current_route_state(None, Some(ActiveBlock::Search));
            }
            ActiveBlock::Artist
            | ActiveBlock::AlbumList
            | ActiveBlock::AlbumTracks
            | ActiveBlock::Home
            | ActiveBlock::SearchResult
            | ActiveBlock::Playlist
            | ActiveBlock::PersonalFm
            | ActiveBlock::TrackTable => {
                app.set_current_route_state(None, Some(ActiveBlock::Search));
            }
            _ => {}
        },
        k if common_events::down_event(k) => match app.get_current_route().hovered_block {
            ActiveBlock::Recommend => {
                app.set_current_route_state(None, Some(ActiveBlock::MyPlaylists));
            }
            ActiveBlock::Search => {
                app.set_current_route_state(None, Some(ActiveBlock::Recommend));
            }
            _ => {}
        },
        k if common_events::right_event(k) => common_events::handle_right_event(app),
        _ => {}
    }
}
