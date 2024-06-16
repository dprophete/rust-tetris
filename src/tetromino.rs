use rand::Rng;
use ruscii::{spatial::Vec2, terminal::Color};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tetromino {
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

    pub fn random() -> Self {
        match rand::thread_rng().gen_range(0..7) {
            0 => Tetromino::I,
            1 => Tetromino::J,
            2 => Tetromino::L,
            3 => Tetromino::O,
            4 => Tetromino::S,
            5 => Tetromino::T,
            _ => Tetromino::Z,
        }
    }

    // using: https://strategywiki.org/wiki/File:Tetris_rotation_super.png
    pub fn cells(&self, mut rot: i32) -> Vec<Vec2> {
        rot = rot % 4;
        match self {
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
                    Vec2::xy(1, 2),
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
