use ruscii::spatial::Vec2;

use crate::tetromino::Tetromino;

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub tetromino: Tetromino,
    pub rot: i32,
    pub pos: Vec2,
}

impl Piece {
    pub fn new(tetromino: Tetromino) -> Self {
        Self {
            tetromino,
            rot: 0,
            pos: Vec2::zero(),
        }
    }

    pub fn rotate(&mut self, delta: i32) -> &Self {
        self.rot = (self.rot + delta) % 4;
        self
    }

    pub fn cells(&self) -> Vec<Vec2> {
        self.tetromino.cells(self.rot)
    }
}
