use super::super::app::{App, ActiveBlock, TrackTable, RouteId};
use super::common_events;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Esc => {
            app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::SearchResult));
        }
        k if common_events::down_event(k) => {
            // track tab
            if app.tabs.index == 0 {
                let next_index = common_events::on_down_press_handler(
                    &app.search_results.tracks.as_ref().unwrap_or(&vec![]),
                    Some(app.search_results.selected_tracks_index),
                );
                app.search_results.selected_tracks_index = next_index;
            } else if app.tabs.index == 1 {
                let next_index = common_events::on_down_press_handler(
                    &app.search_results.artists.as_ref().unwrap_or(&vec![]),
                    Some(app.search_results.selected_artists_index),
                );
                app.search_results.selected_artists_index = next_index;
            } else if app.tabs.index == 2 {
                let next_index = common_events::on_down_press_handler(
                    &app.search_results.albums.as_ref().unwrap_or(&vec![]),
                    Some(app.search_results.selected_albums_index),
                );
                app.search_results.selected_albums_index = next_index;
            } else if app.tabs.index == 3 {
                let next_index = common_events::on_down_press_handler(
                    &app.search_results.playlists.as_ref().unwrap_or(&vec![]),
                    Some(app.search_results.selected_playlists_index),
                );
                app.search_results.selected_playlists_index = next_index;
            } else if app.tabs.index == 4 {
                let next_index = common_events::on_down_press_handler(
                    &app.search_results.djradios.as_ref().unwrap_or(&vec![]),
                    Some(app.search_results.selected_djradio_index),
                );
                app.search_results.selected_djradio_index = next_index;

            }
        }
        k if common_events::up_event(k) => {
            // track tab
            if app.tabs.index == 0 {
                let next_index = common_events::on_up_press_handler(
                    &app.search_results.tracks.as_ref().unwrap_or(&vec![]),
                    Some(app.search_results.selected_tracks_index),
                );
                app.search_results.selected_tracks_index = next_index;
            } else if app.tabs.index == 1 {
                let next_index = common_events::on_up_press_handler(
                    &app.search_results.artists.as_ref().unwrap_or(&vec![]),
                    Some(app.search_results.selected_artists_index),
                );
                app.search_results.selected_artists_index = next_index;
            } else if app.tabs.index == 2 {
                let next_index = common_events::on_up_press_handler(
                    &app.search_results.albums.as_ref().unwrap_or(&vec![]),
                    Some(app.search_results.selected_albums_index),
                );
                app.search_results.selected_albums_index = next_index;
            } else if app.tabs.index == 3 {
                let next_index = common_events::on_up_press_handler(
                    &app.search_results.playlists.as_ref().unwrap_or(&vec![]),
                    Some(app.search_results.selected_playlists_index),
                );
                app.search_results.selected_playlists_index = next_index;
            } else if app.tabs.index == 4 {
                let next_index = common_events::on_up_press_handler(
                    &app.search_results.djradios.as_ref().unwrap_or(&vec![]),
                    Some(app.search_results.selected_djradio_index),
                );
                app.search_results.selected_djradio_index = next_index;
            }
        }
        k if common_events::right_event(k) => {
            let next = (app.tabs.index + 1) % app.tabs.titles.len();
            app.tabs.index = next;
        }
        k if common_events::left_event(k) => {
            let next = if app.tabs.index > 0 {
                app.tabs.index - 1
            } else {
                app.tabs.titles.len() - 1
            };
            app.tabs.index = next;
        }
        Key::Char('\n') => {
            if app.tabs.index == 0 {
                match &app.search_results.tracks.clone() {
                    Some(tracks) => {
                        match tracks.get(app.search_results.selected_tracks_index.to_owned()) {
                            Some(track_playing) => {
                                app.start_playback(track_playing.to_owned());
                                app.fm_state = false;
                                app.my_playlist = TrackTable {
                                    tracks: app.search_results.tracks.to_owned().unwrap(),
                                    selected_index: app.search_results.selected_tracks_index,
                                    name: "search result".to_string()
                                };
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            } else if app.tabs.index == 1 {
                match &app.search_results.artists.clone() {
                    Some(artists) => {
                        match artists.get(app.search_results.selected_artists_index.to_owned()) {
                            Some(artist) => {
                                let artist_id = artist.id.to_owned();
                                app.get_artist_albums(artist_id.to_string());
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            } else if app.tabs.index == 2 {
                match &app.search_results.albums.clone() {
                    Some(albums) => {
                        match albums.get(app.search_results.selected_albums_index.to_owned()) {
                            Some(album) => {
                                let album_id = album.id.to_owned().unwrap();
                                app.get_album_tracks(album_id.to_string());
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            } else if app.tabs.index == 3 {
                match &app.search_results.playlists.clone() {
                    Some(playlists) => {
                        match playlists.get(app.search_results.selected_playlists_index.to_owned()) {
                            Some(playlist) => {
                                let playlist_id = playlist.id.to_owned().unwrap();
                                app.get_playlist_tracks(playlist_id.to_string());
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            } else if app.tabs.index == 4 {
                match &app.search_results.djradios.clone() {
                    Some(djradios) => {
                        match djradios.get(app.search_results.selected_djradio_index.to_owned()) {
                            Some(djradio) => {
                                app.get_djradio_programs(djradio.to_owned(), 500, 0);
                                app.push_navigation_stack(RouteId::DjProgram, ActiveBlock::DjProgram);
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            }
        }
        Key::Ctrl('f') => {
            let limit = (app.block_height - 5) as i32;
            let input: String = app.input.iter().collect();
            if app.tabs.index == 0 {
                let page = app.search_results.selected_tracks_page as i32;
                let next_page = page + 1;
                // search tracks
                match app.cloud_music.as_ref().unwrap().search_track(
                    &input,
                    limit,
                    next_page*limit,
                ) {
                    Ok(result) => {
                        match result.songs {
                            Some(tracks) => {
                                app.search_results.tracks = Some(tracks);
                                app.search_results.selected_tracks_page = next_page as usize;
                            }
                            None => {}
                        }
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }
            }
            if app.tabs.index == 1 {
                let page = app.search_results.selected_artists_page as i32;
                let next_page = page + 1;
                // search artist
                match app.cloud_music.as_ref().unwrap().search_artist(
                    &input,
                    limit,
                    next_page*limit,
                ) {
                    Ok(result) => {
                        match result.artists {
                            Some(artists) => {
                                app.search_results.artists = Some(artists);
                                app.search_results.selected_artists_page = next_page as usize;
                            }
                            None => {}
                        }
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }
            }
            if app.tabs.index == 2 {
                let page = app.search_results.selected_albums_page as i32;
                let next_page = page + 1;
                // search tracks
                match app.cloud_music.as_ref().unwrap().search_album(
                    &input,
                    limit,
                    next_page*limit,
                ) {
                    Ok(result) => {
                        match result.albums {
                            Some(albums) => {
                                app.search_results.albums = Some(albums);
                                app.search_results.selected_albums_page = next_page as usize;
                            }
                            None => {}
                        }
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }
            }
            if app.tabs.index == 3 {
                let page = app.search_results.selected_playlists_page as i32;
                let next_page = page + 1;
                // search playlist
                match app.cloud_music.as_ref().unwrap().search_playlist(
                    &input,
                    limit,
                    next_page*limit,
                ) {
                    Ok(result) => {
                        match result.playlists {
                            Some(playlists) => {
                                app.search_results.playlists = Some(playlists);
                                app.search_results.selected_playlists_page = next_page as usize;
                            }
                            None => {}
                        }
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }
            }
        }
        Key::Ctrl('b') => {
            let limit = (app.block_height - 5) as i32;
            let input: String = app.input.iter().collect();
            if app.tabs.index == 0 {
                let page = app.search_results.selected_tracks_page as i32;
                let next_page = if page < 1 { 0 } else { page - 1 };
                // search tracks
                match app.cloud_music.as_ref().unwrap().search_track(
                    &input,
                    limit,
                    next_page*limit,
                ) {
                    Ok(result) => {
                        match result.songs {
                            Some(tracks) => {
                                app.search_results.tracks = Some(tracks);
                                app.search_results.selected_tracks_page = next_page as usize;
                            }
                            None => {}
                        }
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }
            }
            if app.tabs.index == 1 {
                let page = app.search_results.selected_artists_page as i32;
                let next_page = if page < 1 { 0 } else { page - 1 };
                // search tracks
                match app.cloud_music.as_ref().unwrap().search_artist(
                    &input,
                    limit,
                    next_page*limit,
                ) {
                    Ok(result) => {
                        match result.artists {
                            Some(artists) => {
                                app.search_results.artists = Some(artists);
                                app.search_results.selected_artists_page = next_page as usize;
                            }
                            None => {}
                        }
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }
            }
            if app.tabs.index == 2 {
                let page = app.search_results.selected_albums_page as i32;
                let next_page = if page < 1 { 0 } else { page - 1 };
                // search album
                match app.cloud_music.as_ref().unwrap().search_album(
                    &input,
                    limit,
                    next_page*limit,
                ) {
                    Ok(result) => {
                        app.search_results.albums = Some(result.albums.unwrap_or(vec![]));
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }
                app.search_results.selected_albums_page = next_page as usize;
            }
            if app.tabs.index == 3 {
                let page = app.search_results.selected_playlists_page as i32;
                let next_page = if page < 1 { 0 } else { page - 1 };
                // search playlist
                match app.cloud_music.as_ref().unwrap().search_playlist(
                    &input,
                    limit,
                    next_page*limit,
                ) {
                    Ok(result) => {
                        match result.playlists {
                            Some(playlists) => {
                                app.search_results.playlists = Some(playlists);
                                app.search_results.selected_playlists_page = next_page as usize;
                            }
                            None => {}
                        }
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }
            }
        }
        _ => {}
    }
}
