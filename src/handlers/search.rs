extern crate unicode_width;

use super::super::app::{App, ActiveBlock, RouteId};
use termion::event::Key;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};
use std::convert::TryInto;

// Handle events when the search input block is active
pub fn handler(key: Key, app: &mut App) {
    match key {
        Key::Ctrl('u') => {
            app.input = vec![];
            app.input_idx = 0;
            app.input_cursor_position = 0;
        }
        Key::Ctrl('e') => {
            app.input_idx = app.input.len();
            let input_string: String = app.input.iter().collect();
            app.input_cursor_position = UnicodeWidthStr::width(input_string.as_str())
                .try_into()
                .unwrap();
        }
        Key::Ctrl('a') => {
            app.input_idx = 0;
            app.input_cursor_position = 0;
        }
        Key::Left => {
            if !app.input.is_empty() && app.input_idx > 0 {
                let last_c = app.input[app.input_idx - 1];
                app.input_idx -= 1;
                let width: u16 = UnicodeWidthChar::width(last_c).unwrap().try_into().unwrap();
                app.input_cursor_position -= width;
            }
        }
        Key::Right => {
            if app.input_idx < app.input.len() {
                let next_c = app.input[app.input_idx];
                app.input_idx += 1;
                let width: u16 = UnicodeWidthChar::width(next_c).unwrap().try_into().unwrap();
                app.input_cursor_position += width;
            }
        }
        Key::Esc => {
            app.set_current_route_state(Some(ActiveBlock::Empty), Some(ActiveBlock::Search));
        }
        Key::Backspace => {
            if !app.input.is_empty() && app.input_idx > 0 {
                let last_c = app.input.remove(app.input_idx - 1);
                app.input_idx -= 1;
                let width: u16 = UnicodeWidthChar::width(last_c).unwrap() as u16;
                app.input_cursor_position -= width;
            }
        }
        Key::Delete => {
            if !app.input.is_empty() && app.input_idx < app.input.len() {
                app.input.remove(app.input_idx);
            }
        }
        Key::Char('\n') => {
            let limit = (app.block_height - 5) as i32;
            // no input no search
            if app.input.len() > 0 {
                // search tracks
                let input: String = app.input.iter().collect();
                match app.cloud_music.as_ref().unwrap().search_track(
                    &input,
                    limit,
                    0
                ) {
                    Ok(result) => {
                        app.search_results.tracks = Some(result.songs.unwrap_or(vec![]));
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }
                match app.cloud_music.as_ref().unwrap().search_playlist(
                    &input,
                    limit,
                    0
                ) {
                    Ok(result) => {
                        app.search_results.playlists = Some(result.playlists.unwrap_or(vec![]));
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }
                match app.cloud_music.as_ref().unwrap().search_artist(
                    &input,
                    limit,
                    0
                ) {
                    Ok(result) => {
                        app.search_results.artists = Some(result.artists.unwrap_or(vec![]));
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }
                match app.cloud_music.as_ref().unwrap().search_album(
                    &input,
                    limit,
                    0
                ) {
                    Ok(result) => {
                        app.search_results.albums = Some(result.albums.unwrap_or(vec![]));
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }
                match app.cloud_music.as_ref().unwrap().search_djradio(
                    &input,
                    limit,
                    0
                ) {
                    Ok(result) => {
                        app.search_results.djradios = Some(result.djRadios.unwrap_or(vec![]));
                    }
                    Err(e) => {
                        app.handle_error(e);
                    }
                }
                app.selected_playlist_index = None;
                app.push_navigation_stack(RouteId::Search, ActiveBlock::SearchResult);
            }
        }
        // search input
        Key::Char(c) => {
            app.input.insert(app.input_idx, c);
            app.input_idx += 1;
            let width: u16 = UnicodeWidthChar::width(c).unwrap().try_into().unwrap();
            app.input_cursor_position += width;
        }
        _ => {}
    }
}
