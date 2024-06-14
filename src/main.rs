use gamestate::{GameState, Piece};
use ruscii::app::{App, State};
use ruscii::drawing::Pencil;
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::Window;
use tetromino::Tetromino;

mod gamestate;
mod tetromino;

fn main() {
    let mut fps_counter = FPSCounter::default();
    let mut app = App::default();
    let win_size = app.window().size();
    let mut state = GameState::new((win_size * 5) / 5);

    // state.init_with_all_pieces();
    state.pick_current_piece();

    app.run(|app_state: &mut State, window: &mut Window| {
        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::Q) => app_state.stop(),
                _ => (),
            }
        }

        fps_counter.update();
        let step = app_state.step();
        let mut pencil = Pencil::new(window.canvas_mut());

        for key_down in app_state.keyboard().get_keys_down() {
            state.handle_key_down(step, key_down);
        }
        state.update(step);
        state.draw(&mut pencil);
    });
}
