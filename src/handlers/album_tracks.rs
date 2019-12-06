use super::super::app::{App, TrackTable};
use termion::event::Key;
use super::common_events;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_events::left_event(k) => common_events::handle_left_event(app),
        k if common_events::down_event(k) => {
            if let Some(selected_album) = &mut app.selected_album {
                let next_index = common_events::on_down_press_handler(
                    &selected_album.tracks,
                    Some(selected_album.selected_index),
                );
                selected_album.selected_index = next_index;
            }
        }
        k if common_events::up_event(k) => {
            if let Some(selected_album) = &mut app.selected_album {
                let next_index = common_events::on_up_press_handler(
                    &selected_album.tracks,
                    Some(selected_album.selected_index),
                );
                selected_album.selected_index = next_index;
            }
        }
        Key::Char('\n') => {
            if let Some(selected_album) = &mut app.selected_album {
                app.my_playlist = TrackTable {
                    tracks: selected_album.tracks.to_owned(),
                    selected_index: selected_album.selected_index,
                    name: selected_album.album.name.to_owned().unwrap()
                };
                if let Some(selected_track) = selected_album.tracks
                    .get(selected_album.selected_index)
                    .cloned()
                {
                    app.start_playback(selected_track.id.unwrap().to_string());
                    app.current_playing = Some(selected_track);
                    app.fm_state = false;
                }
            };
        }
        _ => {}
    }
}
