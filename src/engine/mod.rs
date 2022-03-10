pub mod bitboard;

use super::color::*;
use bitboard::moves::*;
use bitboard::BitBoardEngine;
use bitboard::*;
use chessire_utils::*;

// THIS WHOLE THING SHOULD BE CONVERTED INTO A TRAIT

//pub struct ChessireEngine {
//    // required fields for an engine
//    pub name: String,
//    pub author: String,
//    pub bb_engine: BitBoardEngine,
//}
//
//impl Default for ChessireEngine {
//    fn default() -> Self {
//        Self {
//            name: "Chessire".to_string(),
//            author: "Xavi Ondoño".to_string(),
//            bb_engine: BitBoardEngine::new(),
//        }
//    }
//}
//
//impl ChessireEngine {
//    pub fn init(&mut self) {
//        self.bb_engine.init();
//    }
//
//    pub fn new() -> Self {
//        Self::default()
//    }
//
//    pub fn set_position(&mut self, g: ChessGame) {
//        // bitboard implementation
//        // set board position
//        self.bb_engine.set_position(g.board);
//        // copy enpassant square
//        if let Some(coord) = g.enpassant_target_square {
//            self.bb_engine.set_enpassant(Some(coord.to_usize()));
//        } else {
//            self.bb_engine.set_enpassant(None);
//        }
//        // copy castling_rights
//        self.bb_engine.set_castling_rights(g.castling_rights);
//        // I probably need to copy the counters too
//    }
//
//    pub fn get_attacked_squares_by(&self, side: Color) -> Vec<usize> {
//        let mut ret = vec![];
//        // return set of attacked squares
//        for sq in 0..64 {
//            if self.bb_engine.is_square_attacked_by(sq, side) {
//                ret.push(sq)
//            };
//        }
//        ret
//    }
//    pub fn get_moves(&self, side: Color) -> Vec<Move> {
//        self.bb_engine.get_moves(side)
//    }
//}

// new implementation
trait ChessEngine {
    //    /// Constructor, sets the state according to a ChessGame g
    //    fn new(g: ChessGame) -> Self;
    //    /// sets position according to Chessgame g
    //    fn set_position(&mut self, g: ChessGame);
    //    /// get_moves
    //    fn get_moves(&self, side: Color) -> Vec<Move>;
    //    /// get best move
    //    fn best_move(&self) -> Move;
    //    /// make move
    //    fn make_move(&mut self, mov: Move) -> Result<(), ()>;
    //    /// unmake last move
    //    fn unmake_move(&mut self);
    ///// non fundamental aspects
    /// returns the engines name
    fn get_name() -> String;
    /// returns the authors name
    fn get_author() -> String;
}
