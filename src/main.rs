use std::cell::Cell;

use gamestate::GameState;
use ruscii::app::{App, State};
use ruscii::drawing::Pencil;
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::Window;

mod gamestate;
mod tetromino;

fn main() {
    let mut fps_counter = FPSCounter::default();
    let mut app = App::default();
    let win_size = app.window().size();
    let mut state = GameState::new((win_size * 5) / 5);

    state.init_with_all_pieces();

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        for key_down in app_state.keyboard().get_keys_down() {
            match key_down {
                Key::A => state.upd_gpos(Vec2::xy(-1, 0)),
                Key::D => state.upd_gpos(Vec2::xy(1, 0)),
                Key::W => state.upd_gpos(Vec2::xy(0, -1)),
                Key::S => state.upd_gpos(Vec2::xy(0, 1)),
                _ => (),
            }
        }

        fps_counter.update();
        if app_state.step() % 2 == 0 {
            state.update();
        }

        let mut pencil = Pencil::new(window.canvas_mut());
        pencil.draw_text(&format!("FPS: {}", fps_counter.count()), Vec2::xy(1, 1));
        pencil.draw_text(&format!("gridpos: {}", state.gpos), Vec2::xy(1, 2));
        state.draw(&mut pencil);
    });
}
