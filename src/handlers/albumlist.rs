use super::super::app::App;
use termion::event::Key;
use super::common_events;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_events::left_event(k) => common_events::handle_left_event(app),
        k if common_events::down_event(k) => {
            if let Some(albumlist) = &mut app.album_list {
                let next_index = common_events::on_down_press_handler(
                    &albumlist.albums,
                    Some(albumlist.selected_index),
                );
                albumlist.selected_index = next_index;
            }
        }
        k if common_events::up_event(k) => {
            if let Some(albumlist) = &mut app.album_list {
                let next_index = common_events::on_up_press_handler(
                    &albumlist.albums,
                    Some(albumlist.selected_index),
                );
                albumlist.selected_index = next_index;
            }
        }
        Key::Char('\n') => {
            if let Some(albumlist) = &app.album_list
            {
                if let Some(album) =
                    albumlist.albums.get(albumlist.selected_index.to_owned())
                {
                    let album_id = album.id.to_owned().unwrap();
                    app.get_album_tracks(album_id.to_string());
                }
            };
        }
        _ => {}
    }
}
