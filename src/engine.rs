pub mod bitboard;

//use super::color::*;
//use bitboard::moves::*;
//use bitboard::BitBoardEngine;
use chessire_utils::color::*;
use chessire_utils::moves::*;
use chessire_utils::*;

pub trait ChessEngine {
    /// Constructor, sets the state according to a ChessGame g
    /// this function should load anything that is needed for the engine to work properly.
    fn new_engine(g: ChessGame) -> Self;
    /// sets position according to Chessgame g
    /// this function is provided to avoid having to recreate any tables or static resources
    /// that the engine needs, by modifying the state of the game in place.
    fn set_position(&mut self, g: ChessGame);
    /// returns a list of the available moves for *side*.
    /// this moves might or might not be all legal moves.
    //TODO: what do we do with illegal moves?
    fn get_moves(&self, side: Color) -> Vec<Move>;
    /// advances the state of the engines internal state according to mov.
    fn make_move(&mut self, mov: Move) -> Result<(), ()>;
    //    /// evaluate move
    //    fn evaluate(&self, mov: Move) -> i32;
    //    /// get best move
    //    fn best_move(&self) -> Move;

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
}
