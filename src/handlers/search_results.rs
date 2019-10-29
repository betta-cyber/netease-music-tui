use super::super::app::{App, ActiveBlock, RouteId};
use super::common_events;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        // Key::Esc => {
            // app.search_results.selected_block = SearchResultBlock::Empty;
        // }
        k if common_events::down_event(k) => {
            // track tab
            if app.tabs.index == 0 {
                let next_index = common_events::on_down_press_handler(
                    &app.search_results.tracks.as_ref().unwrap(),
                    Some(app.search_results.selected_tracks_index),
                );
                app.search_results.selected_tracks_index = next_index;
            }
        }
        k if common_events::up_event(k) => {
            // track tab
            if app.tabs.index == 0 {
                let next_index = common_events::on_up_press_handler(
                    &app.search_results.tracks.as_ref().unwrap(),
                    Some(app.search_results.selected_tracks_index),
                );
                app.search_results.selected_tracks_index = next_index;
            }
        }
        k if common_events::right_event(k) => {
            let next = (app.tabs.index + 1) % app.tabs.titles.len();
            app.tabs.index = next;
        }
        k if common_events::left_event(k) => {
            let next = if app.tabs.index > 0 {
                app.tabs.index - 1
            } else {
                app.tabs.titles.len() - 1
            };
            app.tabs.index = next;
        }
        Key::Char('\n') => {
            if app.tabs.index == 0 {
                let track_table = &app.search_results.tracks.as_ref().unwrap();
                let track_playing = track_table.get(app.search_results.selected_tracks_index.to_owned()).unwrap().to_owned();
                app.start_playback(track_playing.id.unwrap().to_string());
                app.current_playing = Some(track_playing);
            }
        }
        _ => {}
    }
}
