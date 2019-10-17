use super::super::app::App;
use termion::event::Key;
use super::common_events;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_events::down_event(k) => {
            let next_index = common_events::on_down_press_handler(
                &app.track_table.tracks,
                Some(app.track_table.selected_index),
            );
            app.track_table.selected_index = next_index;
        }
        k if common_events::up_event(k) => {
            let next_index = common_events::on_up_press_handler(
                &app.track_table.tracks,
                Some(app.track_table.selected_index),
            );
            app.track_table.selected_index = next_index;
        }
        _ => {}
    }
}
