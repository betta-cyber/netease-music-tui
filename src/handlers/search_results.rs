use super::super::app::{App, ActiveBlock, RouteId};
use super::common_events;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        // Key::Esc => {
            // app.search_results.selected_block = SearchResultBlock::Empty;
        // }
        // k if common_events::down_event(k) => {
            // if app.search_results.selected_block != SearchResultBlock::Empty {
                // handle_down_press_on_selected_block(app);
            // } else {
                // handle_down_press_on_hovered_block(app);
            // }
        // }
        // k if common_events::up_event(k) => {
            // if app.search_results.selected_block != SearchResultBlock::Empty {
                // handle_up_press_on_selected_block(app);
            // } else {
                // handle_up_press_on_hovered_block(app);
            // }
        // }
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
        _ => {}
    }
}
