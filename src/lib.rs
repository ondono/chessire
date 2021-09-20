pub mod board;
pub mod piece;

use board::*;
use piece::Color::Black;
use piece::Color::White;
use piece::*;
use std::fmt;

pub struct CastlingRights {
    white_king_side: bool,
    white_queen_side: bool,
    black_king_side: bool,
    black_queen_side: bool,
}

impl CastlingRights {
    pub fn new() -> Self {
        Self {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        }
    }
}

impl fmt::Display for CastlingRights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.white_king_side {
            write!(f, "K")?;
        }
        if self.white_queen_side {
            write!(f, "Q")?;
        }
        if self.black_king_side {
            write!(f, "k")?;
        }
        if self.black_queen_side {
            write!(f, "q")?;
        }
        if !self.white_king_side
            && !self.white_queen_side
            && !self.black_king_side
            && !self.black_queen_side
        {
            write!(f, "-")?;
        }
        Ok(())
    }
}

const FEN_STARTING_STRING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct ChessGame {
    board: Board,
    side_to_move: Color,
    castling_rights: CastlingRights,
    enpassant_target_square: Option<usize>,
    halfmove_clock: u32,
    fullmove_clock: u32,
}

impl ChessGame {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            side_to_move: White,
            castling_rights: CastlingRights::new(),
            enpassant_target_square: None,
            halfmove_clock: 0,
            fullmove_clock: 1,
        }
    }
    pub fn set_position_from_fen(&mut self, fen_string: &str) {
        let mut fields = fen_string.split_ascii_whitespace();
        // for each field if we can't read it correctly, use default setting
        // piece placement
        let piece_placement = fields
            .next()
            .unwrap_or("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        self.board.set_position_from_fen(piece_placement);

        // side to move
        let side_to_move = fields.next().unwrap_or("w");
        self.side_to_move = match side_to_move {
            "w" | "W" => White,
            "b" | "B" => Black,
            _ => White,
        };

        // Castling rights
        let castl = fields.next().unwrap_or("-");
        self.castling_rights.white_king_side = castl.find('K') != None;
        self.castling_rights.white_queen_side = castl.find('Q') != None;
        self.castling_rights.black_king_side = castl.find('k') != None;
        self.castling_rights.black_queen_side = castl.find('q') != None;

        // en passant target square
        let en_passant = fields.next().unwrap_or("-");
        let mut enpassant_sq = 0;
        let mut enpassant_present = false;
        match en_passant {
            "-" => self.enpassant_target_square = None,
            _ => {
                enpassant_present = true;
                let mut it = en_passant.chars();
                let file = it.next().unwrap_or('-');
                let rank = it.next().unwrap_or('0');
                match file {
                    'a' => enpassant_sq = A,
                    'b' => enpassant_sq = B,
                    'c' => enpassant_sq = C,
                    'd' => enpassant_sq = D,
                    'e' => enpassant_sq = E,
                    'f' => enpassant_sq = F,
                    'g' => enpassant_sq = G,
                    'h' => enpassant_sq = H,
                    _ => (),
                }
                match rank {
                    '3' => enpassant_sq += 0x30,
                    '6' => enpassant_sq += 0x60,
                    _ => enpassant_sq = 0x88,
                }
            }
        }
        if enpassant_present {
            self.enpassant_target_square = Some(enpassant_sq);
        } else {
            self.enpassant_target_square = None;
        }

        let half_move_clock = fields.next().unwrap_or("0");
        self.halfmove_clock = half_move_clock.parse::<u32>().unwrap_or(0);
        let full_move_count = fields.next().unwrap_or("1");
        self.fullmove_clock = full_move_count.parse::<u32>().unwrap_or(0);
    }
    pub fn set_start_positions(&mut self) {
        self.set_position_from_fen(FEN_STARTING_STRING);
    }
}

impl fmt::Display for ChessGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Game status\r\n")?;
        write!(f, "Side to move: {}\r\n", self.side_to_move)?;
        write!(f, "Castling rights: {}\r\n", self.castling_rights)?;
        if self.enpassant_target_square != None {
            if self.enpassant_target_square.unwrap() & 0x88 == 0 {
                write!(
                    f,
                    "enpassant square: {}{}\r\n",
                    file_char_by_index(self.enpassant_target_square.unwrap()),
                    rank_by_index(self.enpassant_target_square.unwrap())
                )?;
            } else {
                write!(f, "enpassant square: None\r\n",)?;
            }
        } else {
            write!(f, "enpassant square: None\r\n",)?;
        }
        write!(f, "Halfmove clock: {}\r\n", self.halfmove_clock)?;
        write!(f, "Fullmove clock: {}\r\n", self.fullmove_clock)?;
        write!(f, "{}", self.board)
    }
}

pub fn test() {
    let mut game = ChessGame::new();
    // clear the terminal
    println!("{}[2J", 27 as char);
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    // now test
    game.board.set_start_position();
    game.set_position_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w Qk f6 10 20");
    //    game.board.select(C6);
    //    game.board.add_to_highlighted(D5);
    //    game.board.add_to_highlighted(E4);
    //    game.board.add_to_highlighted(F3);
    println!("{}", game);
}
