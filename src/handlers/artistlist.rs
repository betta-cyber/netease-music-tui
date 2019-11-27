use super::super::app::App;
use termion::event::Key;
use super::common_events;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_events::left_event(k) => common_events::handle_left_event(app),
        k if common_events::down_event(k) => {
            if let Some(artistlist) = &mut app.artist_list {
                let next_index = common_events::on_down_press_handler(
                    &artistlist.artists,
                    Some(artistlist.selected_index),
                );
                artistlist.selected_index = next_index;
            }
        }
        k if common_events::up_event(k) => {
            if let Some(artistlist) = &mut app.artist_list {
                let next_index = common_events::on_up_press_handler(
                    &artistlist.artists,
                    Some(artistlist.selected_index),
                );
                artistlist.selected_index = next_index;
            }
        }
        Key::Char('\n') => {
            if let Some(artistlist) = &app.artist_list
            {
                if let Some(artist) =
                    artistlist.artists.get(artistlist.selected_index.to_owned())
                {
                    let artist_id = artist.id;
                    app.get_artist_albums(artist_id.to_string());
                }
            };
        }
        _ => {}
    }
}
