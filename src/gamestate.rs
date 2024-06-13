use std::cmp::{max, min};

use ruscii::{drawing::Pencil, spatial::Vec2, terminal::Color};

use crate::tetromino::Tetromino;

const GRID_WIDTH: i32 = 20;
const GRID_HEIGHT: i32 = 40;

pub struct GameState {
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

    fn place_piece(&mut self, piece: &Piece) {
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

    pub fn upd_gpos(&mut self, delta: Vec2) {
        self.gpos += delta;
        self.gpos.x = max(0, self.gpos.x);
        self.gpos.y = max(0, self.gpos.y);
        self.gpos.x = min(self.dimension.x - GRID_WIDTH * 2, self.gpos.x);
        self.gpos.y = min(self.dimension.y - GRID_WIDTH * 2, self.gpos.y);
    }

    pub fn draw(&mut self, pencil: &mut Pencil) {
        for (y, row) in self.grid.iter().enumerate() {
            let y = y as i32;
            for (x, cell) in row.iter().enumerate() {
                let x = x as i32 * 2;
                let pos = Vec2::xy(x + self.gpos.x, y + self.gpos.y);
                match cell {
                    Cell::Empty => pencil.set_background(Color::Black).draw_text("..", pos),
                    Cell::Tetromino(tetromino) => pencil
                        .set_background(tetromino.color())
                        .draw_text("  ", pos),
                };
            }
        }
    }

    pub fn init_with_all_pieces(&mut self) {
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
                self.pieces.push(piece);
            }
        }
    }
}

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

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Tetromino(Tetromino),
}
