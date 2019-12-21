use super::super::app::{Action, App};
use super::common_events;
use termion::event::Key;

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
            if let Some(playlists) = &app.playlist_list {
                if let Some(playlist) = playlists.playlists.get(playlists.selected_index.to_owned())
                {
                    let playlist_id = playlist.id.to_owned().unwrap();
                    app.get_playlist_tracks(playlist_id.to_string());
                }
            };
        }
        Key::Ctrl('f') => {
            let limit = (app.block_height - 4) as i32;
            if let Some(playlists) = &app.playlist_list {
                let page = playlists.selected_page;
                let next_page = (page + 1) as i32;
                app.get_top_playlist(limit, next_page)
            };
        }
        Key::Ctrl('b') => {
            let limit = (app.block_height - 4) as i32;
            if let Some(playlists) = &app.playlist_list {
                let page = playlists.selected_page;
                let next_page = if page < 1 { 0 } else { page - 1 } as i32;
                app.get_top_playlist(limit, next_page)
            };
        }
        Key::Alt('s') => match &app.playlist_list.clone() {
            Some(playlists) => {
                if let Some(playlist) = playlists.playlists.get(playlists.selected_index.to_owned())
                {
                    app.subscribe_playlist(playlist.to_owned(), Action::Subscribe);
                }
            }
            None => {}
        },
        Key::Alt('d') => match &app.playlist_list.clone() {
            Some(playlists) => {
                if let Some(playlist) = playlists.playlists.get(playlists.selected_index.to_owned())
                {
                    app.subscribe_playlist(playlist.to_owned(), Action::Unsubscribe);
                }
            }
            None => {}
        },
        _ => {}
    }
}
