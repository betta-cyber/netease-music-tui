use super::super::app::{App, RECOMMEND_OPTIONS, RouteId, ActiveBlock};
use termion::event::Key;
use super::common_events;

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
            match app.recommend.selected_index {
                0 => {
                    app.push_navigation_stack(RouteId::MyPlaylists, ActiveBlock::MyPlaylists)
                }
                1 => {
                    app.get_top_playlist()
                }
                2 => {
                    app.set_fm_mode()
                }
                _ => {}
            }
        }
        _ => {}
    }
}
