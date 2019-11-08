use super::super::app::App;
use termion::event::Key;
use super::common_events;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_events::left_event(k) => common_events::handle_left_event(app),
        k if common_events::down_event(k) => {
            if let Some(artist_albums) = &mut app.artist_albums {
                let next_index = common_events::on_down_press_handler(
                    &artist_albums.albums,
                    Some(artist_albums.selected_index),
                );
                artist_albums.selected_index = next_index;
            }
        }
        k if common_events::up_event(k) => {
            if let Some(artist_albums) = &mut app.artist_albums {
                let next_index = common_events::on_up_press_handler(
                    &artist_albums.albums,
                    Some(artist_albums.selected_index),
                );
                artist_albums.selected_index = next_index;
            }
        }
        Key::Char('\n') => {
            if let Some(artist_albums) = &mut app.artist_albums {
                if let Some(selected_album) = artist_albums.albums
                    .get(artist_albums.selected_index)
                    .cloned()
                {
                    app.get_album_tracks(selected_album.id.unwrap().to_string());
                }
            };
        }
        _ => {}
    }
}
