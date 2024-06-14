use crate::tetromino::Tetromino;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Tetromino(Tetromino),
    Shadow,
}
