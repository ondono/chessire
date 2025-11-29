pub mod bitboard;

//use super::color::*;
//use bitboard::moves::*;
//use bitboard::BitBoardEngine;
use anyhow::Result;
use chessire_utils::board::*;
use chessire_utils::color::*;
use chessire_utils::moves::MoveRecord;
use chessire_utils::moves::*;
use chessire_utils::piece::*;
use chessire_utils::*;

#[derive(Debug)]
pub enum ChessEngineError {
    IllegalMove,
}

pub trait ChessEngine {
    /// Constructor, sets the state according to a ChessGame g
    /// this function should load anything that is needed for the engine to work properly.
    fn new_engine(g: ChessGame) -> Self;

    /// sets position according to Chessgame g
    /// this function is provided to avoid having to recreate any tables or static resources
    /// that the engine needs, by modifying the state of the game in place.
    fn set_position(&mut self, g: ChessGame);

    /// Sets the normal start position for a chess game
    fn set_start_position(&mut self);

    fn peek_piece(&self, p: Coord) -> Option<Piece>;

    /// returns a list of the available moves for *side*.
    fn get_moves(&self, side: Color) -> Vec<Move>;

    /// advances the state of the engines internal state according to mov.
    fn make_move(&mut self, mov: Move) -> Result<(), ()>;

    /// Evaluate position
    fn evaluate(&self) -> f32;

    /// Test the legality of the given move
    fn test_move_legality(&self, mov: Move) -> Result<(), ()>;

    /// Search for a best move
    fn search_best_move(&self, depth: usize);

    /// Current best move
    fn get_best_move(&self) -> Move;
    //    /// evaluate move
    //    fn evaluate_move(&self, mov: Move) -> i32;
    //    /// get best move
    //    fn best_move(&self) -> Move;

    fn play_best_move(&mut self);
    ///// non fundamental aspects
    /// returns the engines name
 
    fn get_name(&self) -> String;
    /// returns the authors name
    
    fn get_author(&self) -> String;

    //// debugging functions
    /// get current internal state
    fn get_internal_position(&self) -> ChessGame;
    
    /// perft functions
    fn perft(&mut self, depth: usize, node: &mut u128, print_moves: bool);
    
    //
    fn perft_get_records(&mut self, depth: usize, moves: &Vec<String>) -> Result<Vec<MoveRecord>>;
}
