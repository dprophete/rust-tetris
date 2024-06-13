use ruscii::app::{App, State};
use ruscii::drawing::Pencil;
use ruscii::gui::FPSCounter;
use ruscii::keyboard::{Key, KeyEvent};
use ruscii::spatial::Vec2;
use ruscii::terminal::{Color, Window};

const GRID_WIDTH: i32 = 20;
const GRID_HEIGHT: i32 = 40;

#[derive(Debug, Clone, Copy)]
enum Tetromino {
    // X
    // X
    // X
    // X
    I,
    //   X
    //   X
    // X X
    J,
    // X
    // X
    // X X
    L,
    // X X
    // X X
    O,
    //   X X
    // X X
    S,
    // X X X
    //   X
    T,
    // X X
    //   X X
    Z,
}

impl Tetromino {
    pub fn color(&self) -> Color {
        match self {
            Tetromino::O => Color::Yellow,
            Tetromino::I => Color::Cyan,
            Tetromino::S => Color::Green,
            Tetromino::Z => Color::Red,
            Tetromino::L => Color::Xterm(208),
            Tetromino::J => Color::Blue,
            Tetromino::T => Color::Magenta,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Piece {
    pub tetromino: Tetromino,
    pub rotation: i32,
    pub pos: Vec2,
}

impl Piece {
    pub fn new(tetromino: Tetromino) -> Self {
        Self {
            tetromino,
            rotation: 0,
            pos: Vec2::zero(),
        }
    }

    pub fn rotate(&mut self, delta: i32) -> &Self {
        self.rotation = (self.rotation + delta) % 4;
        self
    }

    // using: https://strategywiki.org/wiki/File:Tetris_rotation_super.png
    pub fn cells(&self) -> Vec<Vec2> {
        let rot = self.rotation % 4;
        match self.tetromino {
            Tetromino::O => vec![
                Vec2::xy(1, 0),
                Vec2::xy(2, 0),
                Vec2::xy(1, 1),
                Vec2::xy(2, 1),
            ],

            Tetromino::I => match rot {
                0 => vec![
                    Vec2::xy(0, 1),
                    Vec2::xy(1, 1),
                    Vec2::xy(2, 1),
                    Vec2::xy(3, 1),
                ],
                1 => vec![
                    Vec2::xy(2, 0),
                    Vec2::xy(2, 1),
                    Vec2::xy(2, 2),
                    Vec2::xy(2, 3),
                ],
                2 => vec![
                    Vec2::xy(0, 2),
                    Vec2::xy(1, 2),
                    Vec2::xy(2, 2),
                    Vec2::xy(3, 2),
                ],
                _ => vec![
                    Vec2::xy(1, 0),
                    Vec2::xy(1, 1),
                    Vec2::xy(1, 2),
                    Vec2::xy(1, 3),
                ],
            },
            Tetromino::S => match rot {
                0 => vec![
                    Vec2::xy(1, 0),
                    Vec2::xy(2, 0),
                    Vec2::xy(0, 1),
                    Vec2::xy(1, 1),
                ],
                1 => vec![
                    Vec2::xy(1, 0),
                    Vec2::xy(1, 1),
                    Vec2::xy(2, 1),
                    Vec2::xy(2, 2),
                ],
                2 => vec![
                    Vec2::xy(1, 1),
                    Vec2::xy(2, 1),
                    Vec2::xy(0, 2),
                    Vec2::xy(1, 2),
                ],
                _ => vec![
                    Vec2::xy(0, 0),
                    Vec2::xy(0, 1),
                    Vec2::xy(1, 1),
                    Vec2::xy(1, 2),
                ],
            },
            Tetromino::Z => match rot {
                0 => vec![
                    Vec2::xy(0, 0),
                    Vec2::xy(1, 0),
                    Vec2::xy(1, 1),
                    Vec2::xy(2, 1),
                ],
                1 => vec![
                    Vec2::xy(2, 0),
                    Vec2::xy(1, 1),
                    Vec2::xy(2, 1),
                    Vec2::xy(1, 2),
                ],
                2 => vec![
                    Vec2::xy(0, 1),
                    Vec2::xy(1, 1),
                    Vec2::xy(1, 2),
                    Vec2::xy(2, 2),
                ],
                _ => vec![
                    Vec2::xy(1, 0),
                    Vec2::xy(0, 1),
                    Vec2::xy(1, 1),
                    Vec2::xy(0, 2),
                ],
            },
            Tetromino::L => match rot {
                0 => vec![
                    Vec2::xy(2, 0),
                    Vec2::xy(0, 1),
                    Vec2::xy(1, 1),
                    Vec2::xy(2, 1),
                ],
                1 => vec![
                    Vec2::xy(1, 0),
                    Vec2::xy(1, 1),
                    Vec2::xy(1, 2),
                    Vec2::xy(2, 2),
                ],
                2 => vec![
                    Vec2::xy(0, 1),
                    Vec2::xy(1, 1),
                    Vec2::xy(2, 1),
                    Vec2::xy(0, 2),
                ],
                _ => vec![
                    Vec2::xy(0, 0),
                    Vec2::xy(1, 0),
                    Vec2::xy(1, 1),
                    Vec2::xy(1, 2),
                ],
            },
            Tetromino::J => match rot {
                0 => vec![
                    Vec2::xy(0, 0),
                    Vec2::xy(0, 1),
                    Vec2::xy(1, 1),
                    Vec2::xy(2, 1),
                ],
                1 => vec![
                    Vec2::xy(1, 0),
                    Vec2::xy(2, 0),
                    Vec2::xy(1, 1),
                    Vec2::xy(1, 2),
                ],
                2 => vec![
                    Vec2::xy(0, 1),
                    Vec2::xy(1, 1),
                    Vec2::xy(2, 1),
                    Vec2::xy(2, 2),
                ],
                _ => vec![
                    Vec2::xy(1, 0),
                    Vec2::xy(1, 1),
                    Vec2::xy(0, 2),
                    Vec2::xy(1, 2),
                ],
            },
            Tetromino::T => match rot {
                0 => vec![
                    Vec2::xy(1, 0),
                    Vec2::xy(0, 1),
                    Vec2::xy(1, 1),
                    Vec2::xy(2, 1),
                ],
                1 => vec![
                    Vec2::xy(1, 0),
                    Vec2::xy(1, 1),
                    Vec2::xy(2, 1),
                    Vec2::xy(1, 2),
                ],
                2 => vec![
                    Vec2::xy(0, 1),
                    Vec2::xy(1, 1),
                    Vec2::xy(2, 1),
                    Vec2::xy(2, 2),
                ],
                _ => vec![
                    Vec2::xy(1, 0),
                    Vec2::xy(0, 1),
                    Vec2::xy(1, 1),
                    Vec2::xy(1, 2),
                ],
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Tetromino(Tetromino),
}

// grid size: 10x20
struct GameState {
    pub dimension: Vec2,
    pub grid: [[Cell; GRID_WIDTH as usize]; GRID_HEIGHT as usize],
    pub gpos: Vec2,
    pub pieces: Vec<Piece>,
    pub current_piece: Option<Piece>,
}

impl GameState {
    pub fn new(dim: Vec2) -> Self {
        Self {
            dimension: dim,
            grid: [[Cell::Empty; GRID_WIDTH as usize]; GRID_HEIGHT as usize],
            gpos: Vec2::xy((dim.x - GRID_WIDTH * 2) / 2, (dim.y - GRID_HEIGHT) / 2),
            pieces: vec![],
            current_piece: None,
        }
    }

    pub fn place_piece(&mut self, piece: &Piece) {
        for cell in piece.cells().iter() {
            let x = piece.pos.x + cell.x;
            let y = piece.pos.y + cell.y;
            if x >= 0 && x < GRID_WIDTH && y >= 0 && y < GRID_HEIGHT {
                self.grid[y as usize][x as usize] = Cell::Tetromino(piece.tetromino);
            }
        }
    }

    pub fn update(&mut self) {
        self.grid = [[Cell::Empty; GRID_WIDTH as usize]; GRID_HEIGHT as usize];

        // static pieces
        let pieces = self.pieces.clone();
        for piece in pieces.iter() {
            self.place_piece(piece);
        }

        // moving piece
        if let Some(piece) = self.current_piece {
            self.place_piece(&piece);
        }
    }
}

fn main() {
    let mut fps_counter = FPSCounter::default();
    let mut app = App::default();
    let win_size = app.window().size();
    let mut state = GameState::new((win_size * 5) / 5);

    for (t_nb, t) in [
        Tetromino::I,
        Tetromino::J,
        Tetromino::L,
        Tetromino::O,
        Tetromino::S,
        Tetromino::T,
        Tetromino::Z,
    ]
    .iter()
    .enumerate()
    {
        for rot in 0..=3 {
            let mut piece = Piece::new(*t);
            piece.pos = Vec2::xy(rot * 5, t_nb as i32 * 5);
            piece.rotate(rot);
            state.pieces.push(piece);
        }
    }

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
                Key::A => state.gpos.x -= 1,
                Key::S => state.gpos.x += 1,
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

        for (y, row) in state.grid.iter().enumerate() {
            let y = y as i32;
            for (x, cell) in row.iter().enumerate() {
                let x = x as i32 * 2;
                let pos = Vec2::xy(x + state.gpos.x, y + state.gpos.y);
                match cell {
                    Cell::Empty => pencil.set_background(Color::Black).draw_text("..", pos),
                    Cell::Tetromino(tetromino) => pencil
                        .set_background(tetromino.color())
                        .draw_text("  ", pos),
                };
            }
        }
    });
}
