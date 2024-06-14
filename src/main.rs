use gamestate::GameState;
use ruscii::app::{App, State};
use ruscii::drawing::Pencil;
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::terminal::Window;

mod cell;
mod gamestate;
mod piece;
mod tetromino;

fn main() {
    let mut fps_counter = FPSCounter::default();
    let mut app = App::default();
    let win_size = app.window().size();
    let mut state = GameState::new((win_size * 5) / 5);
    state.init();

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

        state.handle_keys_down(app_state.keyboard().get_keys_down(), step);
        state.update(step);
        state.draw(&mut pencil, step);
    });
}
