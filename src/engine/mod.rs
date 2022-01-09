pub mod bitboard;

use super::color::*;
use super::game::*;
use bitboard::attacks::*;
use bitboard::BitBoardEngine;
use bitboard::*;

pub struct ChessireEngine {
    pub name: String,
    pub author: String,
    pub bb_engine: BitBoardEngine,
}

impl Default for ChessireEngine {
    fn default() -> Self {
        Self {
            name: "Chessire".to_string(),
            author: "Xavi Ondoño".to_string(),
            bb_engine: BitBoardEngine::new(),
        }
    }
}

impl ChessireEngine {
    pub fn init(&mut self) {
        self.bb_engine.init();
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_position(&mut self, g: ChessGame) {
        // bitboard implementation
        self.bb_engine.set_position(g.board);
    }

    pub fn get_attacked_squares_by(&self, col: Color) -> Vec<usize> {
        let mut ret = vec![];
        // return set of attacked squares
        for sq in 0..64 {
            if self.bb_engine.is_square_attacked_by(sq, col) {
                ret.push(sq)
            };
        }
        ret
    }
}
