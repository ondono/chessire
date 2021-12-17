pub mod bitboard;

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
        let mut e = Self {
            name: "Chessire".to_string(),
            author: "Xavi Ondoño".to_string(),
            bb_engine: BitBoardEngine::new(),
        };
        e
    }
}

impl ChessireEngine {
    pub fn init(&mut self) {
        self.bb_engine.init();
    }

    pub fn new() -> Self {
        Self::default()
    }
}
