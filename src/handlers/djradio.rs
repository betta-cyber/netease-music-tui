use super::super::app::{App, RouteId, ActiveBlock};
use termion::event::Key;
use super::common_events;

pub fn handler(key: Key, app: &mut App) {
    match key {
        k if common_events::left_event(k) => common_events::handle_left_event(app),
        k if common_events::down_event(k) => {
            if let Some(djradio_list) = &mut app.djradio_list {
                let next_index = common_events::on_down_press_handler(
                    &djradio_list.djradios,
                    Some(djradio_list.selected_index),
                );
                djradio_list.selected_index = next_index;
            }
        }
        k if common_events::up_event(k) => {
            if let Some(djradio_list) = &mut app.djradio_list {
                let next_index = common_events::on_up_press_handler(
                    &djradio_list.djradios,
                    Some(djradio_list.selected_index),
                );
                djradio_list.selected_index = next_index;
            }
        }
        Key::Char('\n') => {
            if let Some(djradio_list) = &app.djradio_list.clone()
            {
                let limit = (app.block_height - 4) as i32;
                if let Some(djradio) =
                    djradio_list.djradios.get(djradio_list.selected_index.to_owned())
                {

                    app.get_djradio_programs(djradio.to_owned(), limit, 0);
                    app.push_navigation_stack(RouteId::DjProgram, ActiveBlock::DjProgram);
                }
            };
        }
        _ => {}
    }
}
