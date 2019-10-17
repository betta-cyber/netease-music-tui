use super::super::app::App;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        // Key::Char('\n') => {
            // if let (Some(playlist), Some(selected_playlist_index)) =
                // (&app.playlist, &app.selected_playlist_index)
            // {
                // app.track_table.context = Some(TrackTableContext::MyPlaylists);
                // if let Some(selected_playlist) =
                    // playlists.items.get(selected_playlist_index.to_owned())
                // {
                    // let playlist_id = selected_playlist.id.to_owned();
                    // app.get_playlist_tracks(playlist_id);
                // }
            // };
        // }
        _ => {}
    }
}
