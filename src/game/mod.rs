// This module keeps all information of the game in an easy to understand way. It's the source of
// truth for the engine. This representation is never used directly in computing.

// Board holds an easy to use representation of the board, that is not used internally in the
// engine, but helps in several tasks like external representation.
//

pub mod board;
pub mod castling;
pub mod color;
pub mod piece;

use board::*;
use castling::*;
use color::Color::{self, Black, White};
use piece::*;

#[derive(Clone)]
pub struct ChessGame {
    pub board: Board,
    pub castling_rights: CastlingRights,
    pub side_to_move: Color,
    pub enpassant_target_square: Option<Coord>,
    pub halfmove_clock: u32,
    pub fullmove_clock: u32,
}

impl Default for ChessGame {
    fn default() -> Self {
        Self {
            board: Board::new(),
            castling_rights: CastlingRights::new(),
            side_to_move: White,
            enpassant_target_square: None,
            halfmove_clock: 0,
            fullmove_clock: 1,
        }
    }
}

const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

impl ChessGame {
    fn new_empty_board() -> Self {
        Self::default()
    }
    pub fn new() -> Self {
        let mut g = Self::new_empty_board();
        g.set_start_position();
        g
    }

    pub fn new_position(fen: &str) -> Result<Self, ()> {
        let mut game = Self::new_empty_board();
        game.apply_fen(fen)?;
        Ok(game)
    }

    pub fn apply_fen(&mut self, fen: &str) -> Result<(), ()> {
        //TODO: We should be able to feed non-FEN strings and get an error!
        //
        let mut fen_fields = fen.split_ascii_whitespace();
        if fen_fields.clone().count() != 6 {
            return Err(());
        }
        // for each field if we can't read it correctly, use default setting
        // piece placement
        self.board.clear();
        let piece_placement = fen_fields.next().unwrap();
        self.board.set_position_from_fen(piece_placement)?;
        // fill the piece list too!
        // side to move
        let side_to_move = fen_fields.next().unwrap();
        self.side_to_move = match side_to_move {
            "w" | "W" => White,
            "b" | "B" => Black,
            _ => White,
        };

        // Castling rights
        let castl = fen_fields.next().unwrap();
        self.castling_rights.white_king_side = castl.find('K') != None;
        self.castling_rights.white_queen_side = castl.find('Q') != None;
        self.castling_rights.black_king_side = castl.find('k') != None;
        self.castling_rights.black_queen_side = castl.find('q') != None;

        // en passant target square
        let en_passant = fen_fields.next().unwrap();
        let mut enpassant_sq = match en_passant {
            "-" => self.enpassant_target_square = None,
            _ => {
                let mut it = en_passant.chars();
                let file = it.next().unwrap();
                let rank = it.next().unwrap();

                self.enpassant_target_square = Some(Coord::new(
                    match file {
                        'a' => 0,
                        'b' => 1,
                        'c' => 2,
                        'd' => 3,
                        'e' => 4,
                        'f' => 5,
                        'g' => 6,
                        'h' => 7,
                        _ => return Err(()),
                    },
                    match rank {
                        '3' => 3,
                        '6' => 6,
                        _ => return Err(()),
                    },
                ));
            }
        };

        let half_move_clock = fen_fields.next().unwrap_or("0");
        self.halfmove_clock = half_move_clock.parse::<u32>().unwrap_or(0);
        let full_move_count = fen_fields.next().unwrap_or("1");
        self.fullmove_clock = full_move_count.parse::<u32>().unwrap_or(0);

        Ok(())
    }

    fn set_start_position(&mut self) {
        self.apply_fen(STARTING_FEN).ok();
    }
}
