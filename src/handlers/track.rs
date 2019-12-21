use super::super::app::App;
use super::common_events;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_events::left_event(k) => common_events::handle_left_event(app),
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
        Key::Char('\n') => {
            #[allow(non_snake_case)]
            let TrackTable = &app.track_table;
            let track_playing = TrackTable
                .tracks
                .get(TrackTable.selected_index.to_owned())
                .unwrap()
                .to_owned();
            app.my_playlist = TrackTable.to_owned();

            app.start_playback(track_playing);
            app.fm_state = false;
        }
        _ => {}
    }
}
