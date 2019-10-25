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
            | ActiveBlock::Artists
            | ActiveBlock::Home
            | ActiveBlock::RecentlyPlayed
            | ActiveBlock::TrackTable => {
                app.set_current_route_state(None, Some(ActiveBlock::Recommend));
            }
            _ => {}
        },
        _ => {}
    }
}
