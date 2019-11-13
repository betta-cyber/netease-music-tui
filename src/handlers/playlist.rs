use super::super::app::App;
use termion::event::Key;
use super::common_events;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_events::left_event(k) => common_events::handle_left_event(app),
        k if common_events::down_event(k) => {
            if let Some(playlist) = &mut app.playlist_list {
                let next_index = common_events::on_down_press_handler(
                    &playlist.playlists,
                    Some(playlist.selected_index),
                );
                playlist.selected_index = next_index;
            }
        }
        k if common_events::up_event(k) => {
            if let Some(playlist) = &mut app.playlist_list {
                let next_index = common_events::on_up_press_handler(
                    &playlist.playlists,
                    Some(playlist.selected_index),
                );
                playlist.selected_index = next_index;
            }
        }
        Key::Char('\n') => {
            if let Some(playlists) = &app.playlist_list
            {
                if let Some(playlist) =
                    playlists.playlists.get(playlists.selected_index.to_owned())
                {
                    let playlist_id = playlist.id.to_owned().unwrap();
                    app.get_playlist_tracks(playlist_id.to_string());
                }
            };
        }
        _ => {}
    }
}
