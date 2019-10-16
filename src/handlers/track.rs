use super::super::app::App;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Char('\n') => {
            let TrackTable {
                context,
                selected_index,
                tracks,
            } = &app.track_table;
            match &context {
                Some(context) => match context {
                    TrackTableContext::MyPlaylists => {
                        if let Some(_track) = tracks.get(*selected_index) {
                            let context_uri = match (&app.selected_playlist_index, &app.playlists) {
                                (Some(selected_playlist_index), Some(playlists)) => {
                                    if let Some(selected_playlist) =
                                        playlists.items.get(selected_playlist_index.to_owned())
                                    {
                                        Some(selected_playlist.uri.to_owned())
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            };

                            app.start_playback(
                                context_uri,
                                None,
                                Some(app.track_table.selected_index),
                            );
                        };
                    }
                    TrackTableContext::SavedTracks => {
                        if let Some(saved_tracks) = &app.library.saved_tracks.get_results(None) {
                            let track_uris: Vec<String> = saved_tracks
                                .items
                                .iter()
                                .map(|item| item.track.uri.to_owned())
                                .collect();

                            app.start_playback(
                                None,
                                Some(track_uris),
                                Some(app.track_table.selected_index),
                            );
                        };
                    }
                    TrackTableContext::AlbumSearch => {}
                    TrackTableContext::PlaylistSearch => {
                        let TrackTable {
                            selected_index,
                            tracks,
                            ..
                        } = &app.track_table;
                        if let Some(_track) = tracks.get(*selected_index) {
                            let context_uri = match (
                                &app.search_results.selected_playlists_index,
                                &app.search_results.playlists,
                            ) {
                                (Some(selected_playlist_index), Some(playlist_result)) => {
                                    if let Some(selected_playlist) = playlist_result
                                        .playlists
                                        .items
                                        .get(selected_playlist_index.to_owned())
                                    {
                                        Some(selected_playlist.uri.to_owned())
                                    } else {
                                        None
                                    }
                                }
                                _ => None,
                            };

                            app.start_playback(
                                context_uri,
                                None,
                                Some(app.track_table.selected_index),
                            );
                        };
                    }
                },
                None => {}
            };
        }
    }
}
