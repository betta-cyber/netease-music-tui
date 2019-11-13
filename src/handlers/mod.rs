mod common_events;
mod my_playlist;
mod playlist;
mod track;
mod recommend;
mod empty;
mod home;
mod search;
mod search_results;
mod artist;
mod album_tracks;

use super::app::{App, ActiveBlock, RouteId};
use termion::event::Key;

pub fn handle_app(key: Key, app: &mut App) {
    match key {
        // Key::Char('a') => {
            // if let Some(current_playback_context) = &app.current_playback_context {
                // if let Some(full_track) = &current_playback_context.item.clone() {
                    // app.get_album_tracks(full_track.album.clone());
                // }
            // };
        // }
        Key::Char('-') => {
            app.decrease_volume();
        }
        Key::Char('+') => {
            app.increase_volume();
        }
        Key::Char('/') => {
            app.set_current_route_state(Some(ActiveBlock::Search), Some(ActiveBlock::Search));
        }
        Key::Char('r') => {
            app.repeat();
        }
        Key::Char('?') => {
            app.set_current_route_state(Some(ActiveBlock::Help), None);
        }
        Key::Char('f') => {
            app.push_navigation_stack(RouteId::Playing, ActiveBlock::Playing);
        }
        Key::Char('a') => {
            let album_id = match &app.current_playing {
                Some(track) => {
                    track.album.to_owned().unwrap().id
                }
                None => None
            };
            app.get_album_tracks(album_id.unwrap().to_string());
        }
        _ => handle_block_events(key, app),
    }
}

// handle current block events
fn handle_block_events(key: Key, app: &mut App) {

    // get current route
    let current_route = app.get_current_route();

    match current_route.active_block {
        ActiveBlock::MyPlaylists => {
            my_playlist::handler(key, app);
        }
        ActiveBlock::TrackTable => {
            track::handler(key, app);
        }
        ActiveBlock::Recommend => {
            recommend::handler(key, app);
        }
        ActiveBlock::Empty => {
            empty::handler(key, app);
        }
        ActiveBlock::Home => {
            home::handler(key, app);
        }
        ActiveBlock::Search => {
            search::handler(key, app);
        }
        ActiveBlock::Artist => {
            artist::handler(key, app);
        }
        ActiveBlock::AlbumTracks => {
            album_tracks::handler(key, app);
        }
        ActiveBlock::Playlist => {
            playlist::handler(key, app);
        }
        ActiveBlock::SearchResult => {
            search_results::handler(key, app);
        }
        _ => {}
    }
}
