use super::super::app::{ActiveBlock, App, RouteId, RECOMMEND_OPTIONS};
use super::common_events;
use termion::event::Key;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_events::right_event(k) => common_events::handle_right_event(app),
        k if common_events::down_event(k) => {
            let next_index = common_events::on_down_press_handler(
                &RECOMMEND_OPTIONS,
                Some(app.recommend.selected_index),
            );
            app.recommend.selected_index = next_index;
        }
        k if common_events::up_event(k) => {
            let next_index = common_events::on_up_press_handler(
                &RECOMMEND_OPTIONS,
                Some(app.recommend.selected_index),
            );
            app.recommend.selected_index = next_index;
        }
        // recommend list
        // you can go Discover music
        // you can go Personal FM
        // you can go you playlists
        Key::Char('\n') => {
            let limit = (app.block_height - 4) as i32;
            match app.recommend.selected_index {
                0 => app.push_navigation_stack(RouteId::MyPlaylists, ActiveBlock::MyPlaylists),
                1 => {
                    app.get_top_playlist(limit, 0);
                    app.push_navigation_stack(RouteId::Playlist, ActiveBlock::Playlist)
                }
                2 => app.set_fm_mode(),
                3 => {
                    app.get_top_albums(limit, 0);
                    app.push_navigation_stack(RouteId::AlbumList, ActiveBlock::AlbumList);
                }
                4 => {
                    app.get_top_artists(limit, 0);
                    app.push_navigation_stack(RouteId::ArtistList, ActiveBlock::ArtistList);
                }
                5 => {
                    app.get_sub_dj_radio(limit, 0);
                    app.push_navigation_stack(RouteId::DjRadio, ActiveBlock::DjRadio);
                }
                _ => {}
            }
        }
        _ => {}
    }
}
