use super::super::app::{App, TrackTable};
use super::super::model::artist::Artist;
use super::super::model::playlist::Track;
use super::common_events;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_events::left_event(k) => common_events::handle_left_event(app),
        k if common_events::down_event(k) => {
            if let Some(djprogram_list) = &mut app.program_list {
                let next_index = common_events::on_down_press_handler(
                    &djprogram_list.dj_programs,
                    Some(djprogram_list.selected_index),
                );
                djprogram_list.selected_index = next_index;
            }
        }
        k if common_events::up_event(k) => {
            if let Some(djprogram_list) = &mut app.program_list {
                let next_index = common_events::on_up_press_handler(
                    &djprogram_list.dj_programs,
                    Some(djprogram_list.selected_index),
                );
                djprogram_list.selected_index = next_index;
            }
        }
        Key::Char('\n') => {
            if let Some(djprogram_list) = &app.program_list.clone() {
                // convert djprogram to tracks
                let track_list = djprogram_list
                    .dj_programs
                    .iter()
                    .map(|item| {
                        let artist = Artist {
                            id: item.radio.id as i32,
                            name: item.radio.name.to_string(),
                            alias: None,
                        };
                        Track {
                            name: Some(item.mainSong.name.to_string()),
                            fee: Some(item.mainSong.fee as i64),
                            id: Some(item.mainSong.id as i64),
                            artists: Some(vec![artist]),
                            album: None,
                        }
                    })
                    .collect::<Vec<Track>>();
                if let Some(djprogram) = track_list.get(djprogram_list.selected_index.to_owned()) {
                    app.my_playlist = TrackTable {
                        tracks: track_list.to_owned(),
                        selected_index: djprogram_list.selected_index,
                        name: "dj program".to_string(),
                    };
                    app.start_playback(djprogram.to_owned());
                    app.fm_state = false;
                }
            };
        }
        _ => {}
    }
}
