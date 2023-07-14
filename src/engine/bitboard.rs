pub mod attacks;
pub mod constants;
pub mod magics;
pub mod moves;
pub mod occupancy;
pub mod tests;
pub mod util;

use super::board::*;
use super::color::*;
use super::piece::*;
use crate::castling::CastlingRights;
use crate::engine::bitboard::moves::*;
use anyhow::Result;
use attacks::*;
use chessire_utils::color::Color::{Black, White};
use chessire_utils::moves::*;
use constants::*;
use occupancy::*;
use util::*;

// Some flags to speed up computation
pub struct PositionFlags {
    pub in_check: bool,
    pub in_double_check: bool,
    pub pins_exist: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct BitBoardState {
    pub current_position: [BitBoard; 12],
    pub occupancies: [BitBoard; 3],
    pub enpassant: Option<usize>,
    pub castling_rights: CastlingRights,
    pub halfmove_clock: u32,
    pub fullmove_clock: u32,
    pub side_to_move: Color,
    //// optimizations
    pub squares_attacked: [BitBoard; 2],
    pub pin_maps: [BitBoard; 2],
    //// they can't never be more than 32 pieces in the board!
    pub white_piece_lists: [(Option<Piece>, usize); 16],
    pub black_piece_lists: [(Option<Piece>, usize); 16],
    //
    //  pub
}

impl Default for BitBoardState {
    fn default() -> Self {
        Self {
            current_position: [BitBoard::new(0); 12],
            occupancies: [BitBoard::new(0); 3],
            enpassant: None,
            castling_rights: CastlingRights::new(),
            halfmove_clock: 0,
            fullmove_clock: 1,
            side_to_move: White,
            squares_attacked: [BitBoard::new(0); 2],
            pin_maps: [BitBoard::new(0); 2],
            white_piece_lists: [(None, 0); 16],
            black_piece_lists: [(None, 0); 16],
        }
    }
}

impl BitBoardState {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_game(&self) -> ChessGame {
        let mut g = ChessGame::new();
        // copy the board back to a chessgame
        g.board.clear();
        for (i, bitboard) in self.current_position.iter().enumerate() {
            use chessire_utils::piece::Piece::*;
            let piece = match i {
                WHITE_KING => King(White),
                WHITE_QUEEN => Queen(White),
                WHITE_ROOK => Rook(White),
                WHITE_BISHOP => Bishop(White),
                WHITE_KNIGHT => Knight(White),
                WHITE_PAWN => Pawn(White),
                BLACK_KING => King(Black),
                BLACK_QUEEN => Queen(Black),
                BLACK_ROOK => Rook(Black),
                BLACK_BISHOP => Bishop(Black),
                BLACK_KNIGHT => Knight(Black),
                BLACK_PAWN => Pawn(Black),
                _ => Pawn(White),
            };
            for sq in bitboard.into_iter() {
                g.set_piece(Coord::from_tile(sq), piece)
            }
        }
        if self.enpassant.is_some() {
            g.enpassant_target_square = Some(Coord::from_tile(self.enpassant.unwrap()));
        } else {
            g.enpassant_target_square = None;
        }
        g.castling_rights = self.castling_rights;
        g.fullmove_clock = self.fullmove_clock;
        g.halfmove_clock = self.halfmove_clock;
        g.side_to_move = self.side_to_move;
        g
    }
}

use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct BitBoardEngine {
    pub attack_tables: Rc<AttackTables>,
    pub state: BitBoardState,
}

impl Default for BitBoardEngine {
    fn default() -> Self {
        let mut engine = Self {
            attack_tables: Rc::new(AttackTables::new()),
            state: BitBoardState::new(),
        };
        engine.init();
        engine
    }
}

use super::ChessEngine;
use chessire_utils::*;

impl ChessEngine for BitBoardEngine {
    fn new_engine(g: ChessGame) -> Self {
        let mut e = BitBoardEngine::new();
        e.set_position(g);
        e
    }
    fn set_position(&mut self, g: ChessGame) {
        self._set_position(g.board);

        self.set_castling_rights(g.castling_rights);
        self.state.halfmove_clock = g.halfmove_clock;
        self.state.fullmove_clock = g.fullmove_clock;
        self.state.side_to_move = g.side_to_move;

        self.state.enpassant = if g.enpassant_target_square.is_some() {
            Some(g.enpassant_target_square.unwrap().to_usize())
        } else {
            None
        };
    }
    fn set_start_position(&mut self) {
        let g = ChessGame::new();
        self.set_position(g);
    }
    fn peek_piece(&self, p: Coord) -> Option<Piece> {
        if self.state.occupancies[BOTH].get_bit(p.to_usize()) {
            use chessire_utils::piece::Piece::*;

            let mut n = 0;
            for (i, bitboard) in self.state.current_position.iter().enumerate() {
                if bitboard.get_bit(p.to_usize()) {
                    n = i;
                }
            }
            let piece = match n {
                WHITE_KING => King(White),
                WHITE_QUEEN => Queen(White),
                WHITE_ROOK => Rook(White),
                WHITE_BISHOP => Bishop(White),
                WHITE_KNIGHT => Knight(White),
                WHITE_PAWN => Pawn(White),
                BLACK_KING => King(Black),
                BLACK_QUEEN => Queen(Black),
                BLACK_ROOK => Rook(Black),
                BLACK_BISHOP => Bishop(Black),
                BLACK_KNIGHT => Knight(Black),
                BLACK_PAWN => Pawn(Black),
                _ => Pawn(White),
            };
            Some(piece)
        } else {
            None
        }
    }
    #[inline]
    fn get_moves(&self, side: Color) -> Vec<Move> {
        self._get_moves(side)
    }

    fn get_name(&self) -> String {
        "Chessire Bitboard implementation".to_string()
    }
    fn get_author(&self) -> String {
        "Xavi Ondono".to_string()
    }
    fn get_internal_position(&self) -> ChessGame {
        self.state.get_game()
    }

    #[inline]
    fn test_move_legality(&self, mov: Move) -> Result<(), ()> {
        self.clone().make_move(mov)
    }

    #[inline]
    fn make_move(&mut self, mov: Move) -> Result<(), ()> {
        // preserve board state
        let backup = self.state;

        let side = mov.piece.get_color();
        let source = mov.source.to_usize();
        let target = mov.target.to_usize();
        let piece_index = get_bb_piece_index(mov.piece);

        // move the piece
        self.state.current_position[piece_index].reset_bit(source);
        self.state.current_position[piece_index].set_bit(target);

        // move the piece in the list
        // if side == White {
        //     // find the moving piece
        //     let index = self
        //         .state
        //         .white_piece_lists
        //         .iter()
        //         .position(|&x| x == (Some(mov.piece), source))
        //         .unwrap();
        //     self.state.white_piece_lists[index] = (Some(mov.piece), target);

        //     if let Some(index) = self
        //         .state
        //         .black_piece_lists
        //         .iter()
        //         .position(|&x| x.1 == (target))
        //     {
        //         self.state.black_piece_lists[index] = (None, target);
        //     }
        // } else {
        //     // find the moving piece
        //     let index = self
        //         .state
        //         .black_piece_lists
        //         .iter()
        //         .position(|&x| x == (Some(mov.piece), source))
        //         .unwrap();
        //     self.state.black_piece_lists[index] = (Some(mov.piece), target);

        //     if let Some(index) = self
        //         .state
        //         .white_piece_lists
        //         .iter()
        //         .position(|&x| x.1 == (target))
        //     {
        //         self.state.white_piece_lists[index] = (None, target);
        //     }
        // }

        // handle capture moves
        if mov.capture {
            for pieces in if side == White {
                BLACK_PIECES
            } else {
                WHITE_PIECES
            } {
                self.state.current_position[pieces].reset_bit(target);
            }
        }

        // promotions
        if mov.promoted_piece.is_some() {
            // remove the pawn from the end position
            self.state.current_position[piece_index].reset_bit(target);
            // and place the promoted piece
            self.state.current_position[get_bb_piece_index(mov.promoted_piece.unwrap())]
                .set_bit(target);
        }

        // en passant captures
        if mov.enpassant {
            // erase enemy pawn
            if side == White {
                self.state.current_position[BLACK_PAWN]
                    .reset_bit(mov.target.next_down().unwrap().to_usize());
            } else {
                {
                    self.state.current_position[WHITE_PAWN]
                        .reset_bit(mov.target.next_up().unwrap().to_usize());
                }
            }
            self.state.enpassant = None;
        }

        if mov.double_push {
            // set enpassant square
            if side == White {
                self.state.enpassant =
                    Some(mov.target.next_down().unwrap_or(mov.target).to_usize());
            } else {
                self.state.enpassant = Some(mov.target.next_up().unwrap_or(mov.target).to_usize());
            }
        } else {
            self.state.enpassant = None;
        }

        // handle castling
        if mov.castling {
            // the king has been already moved, we just need to make sure to move the rook

            // needs a better way of setting this constants that doesn't clash with the bitboard
            // ones!
            match mov.target.to_usize() {
                // C1: White castling queen side
                2 => {
                    self.state.current_position[WHITE_ROOK].reset_bit(index_from_bitmask(A1)); // a1
                    self.state.current_position[WHITE_ROOK].set_bit(index_from_bitmask(D1));
                    self.state.castling_rights.white_queen_side = false;
                }
                // G1: White castling king side
                6 => {
                    self.state.current_position[WHITE_ROOK].reset_bit(index_from_bitmask(H1)); // a1
                    self.state.current_position[WHITE_ROOK].set_bit(index_from_bitmask(F1));
                    self.state.castling_rights.white_king_side = false;
                }
                // C8
                58 => {
                    self.state.current_position[BLACK_ROOK].reset_bit(index_from_bitmask(A8)); // a1
                    self.state.current_position[BLACK_ROOK].set_bit(index_from_bitmask(D8));
                    self.state.castling_rights.black_queen_side = false;
                }
                // G8
                62 => {
                    self.state.current_position[BLACK_ROOK].reset_bit(index_from_bitmask(H8)); // a1
                    self.state.current_position[BLACK_ROOK].set_bit(index_from_bitmask(F8));
                    self.state.castling_rights.black_king_side = false;
                }
                // should never happen!
                _ => panic!("Castling move with wrong target square"),
            }
        }

        // update castling castling rights
        for sq in [source, target] {
            match sq {
                // possibly moving a1 white rook
                0 => self.state.castling_rights.white_queen_side = false,
                // possibly moving h1 white rook
                7 => self.state.castling_rights.white_king_side = false,
                // possibly moving a8 black rook
                56 => self.state.castling_rights.black_queen_side = false,
                // possibly moving h8 black rook
                63 => self.state.castling_rights.black_king_side = false,
                // possibly moving white king
                4 => {
                    self.state.castling_rights.white_king_side = false;
                    self.state.castling_rights.white_queen_side = false;
                }
                // possibly mobing black king
                60 => {
                    self.state.castling_rights.black_king_side = false;
                    self.state.castling_rights.black_queen_side = false;
                }
                // for all other squares, do nothing
                _ => (),
            }
        }

        //update occupancies
        self.state.occupancies[White].clear();
        self.state.occupancies[Black].clear();
        for i in WHITE_PIECES {
            self.state.occupancies[White] =
                self.state.occupancies[White] | self.state.current_position[i];
        }
        for i in BLACK_PIECES {
            self.state.occupancies[Black] =
                self.state.occupancies[Black] | self.state.current_position[i];
        }
        self.state.occupancies[BOTH] =
            self.state.occupancies[White] | self.state.occupancies[Black];

        self.state.side_to_move = self.state.side_to_move.opponent();

        //check legal move
        if let Some(white_king_sq) = self.state.current_position[WHITE_KING].get_lsb() {
            if let Some(black_king_sq) = self.state.current_position[BLACK_KING].get_lsb() {
                // if the king of the moving side is now exposed
                if (side == White && self.is_square_attacked_by(white_king_sq, Black))
                    || (side == Black && self.is_square_attacked_by(black_king_sq, White))
                {
                    // ilegal move, go back
                    self.state = backup;
                    Err(())
                } else {
                    // legal move
                    Ok(())
                }
            } else {
                // black king is gone! illegal move, go back
                self.state = backup;
                Err(())
            }
        } else {
            // white king is gone! illegal move, go back
            self.state = backup;
            Err(())
        }
    }

    fn evaluate(&self) -> f32 {
        let mut position_value = 0.0;
        // for each bitboard
        for (i, piece) in self.state.current_position.iter().enumerate() {
            // loop over all pieces in the current bitboard
            let mut p = *piece;
            while let Some(sq) = p.get_lsb() {
                position_value += match i {
                    // PAWN scores
                    WHITE_PAWN => PAWN_VALUE + WHITE_PAWN_SCORES[sq] as f32,
                    BLACK_PAWN => -PAWN_VALUE - BLACK_PAWN_SCORES[sq] as f32,
                    // KNIGHT scores
                    WHITE_KNIGHT => KNIGHT_VALUE + KNIGHT_SCORES[sq] as f32,
                    BLACK_KNIGHT => -KNIGHT_VALUE - KNIGHT_SCORES[sq] as f32,
                    // BISHOP scores
                    WHITE_BISHOP => BISHOP_VALUE + BISHOP_SCORES[sq] as f32,
                    BLACK_BISHOP => -BISHOP_VALUE - BISHOP_SCORES[sq] as f32,
                    // ROOK scores
                    WHITE_ROOK => ROOK_VALUE + ROOK_SCORES[sq] as f32,
                    BLACK_ROOK => -ROOK_VALUE - ROOK_SCORES[sq] as f32,
                    // QUEEN doesn't get a positional score
                    WHITE_QUEEN => QUEEN_VALUE,
                    BLACK_QUEEN => -QUEEN_VALUE,
                    // KING scores
                    WHITE_KING => KING_VALUE + KING_SCORES[sq] as f32,
                    BLACK_KING => -KING_VALUE - KING_SCORES[sq] as f32,
                    _ => 0.0,
                };
                p.reset_bit(sq);
            }
        }
        position_value
    }

    fn search_best_move(&self, depth: usize) {
        // time to minimax
    }

    fn get_best_move(&self) -> Move {
        //placeholder
        Move::new(
            Coord::from_tile(index_from_bitmask(E2)),
            Coord::from_tile(index_from_bitmask(E4)),
            Piece::Pawn(White),
            None,
        )
    }

    fn play_best_move(&mut self) {
        let mut move_scores: Vec<(Move, f32)> = Vec::with_capacity(30);

        let move_list = self.get_moves(self.state.side_to_move);
        for m in move_list {
            let state = self.state;
            if self.make_move(m).is_ok() {
                move_scores.push((m, minimax_search(self, 3)));
            }
            self.state = state;
        }

        let mut best = move_scores[0];

        for (m, s) in move_scores {
            println!("{}\t{}", m, s);
            // black minimizes
            if s < best.1 {
                best = (m, s)
            }
        }
        if self.make_move(best.0).is_err() {
            println!("You won!");
        }
    }

    fn perft(&mut self, depth: usize, nodes: &mut u128, print_moves: bool) {
        if depth != 0 {
            // generate the move list for the current position
            let move_list = self.get_moves(self.state.side_to_move);
            for mov in move_list {
                let move_name: String = mov
                    .to_string()
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .to_string();
                let state = self.state;
                if self.make_move(mov).is_ok() {
                    let mut move_nodes = 0;
                    if depth > 1 {
                        self.perft(depth - 1, &mut move_nodes, false);
                    } else {
                        move_nodes = 1;
                    }
                    if print_moves {
                        println!("{}:{}", move_name, move_nodes);
                    }
                    *nodes += move_nodes;
                    self.state = state;
                }
            }
        } else {
            *nodes += 1;
        }
    }

    fn perft_get_records(&mut self, depth: usize, moves: &Vec<String>) -> Result<Vec<MoveRecord>> {
        let mut records = vec![];
        if depth != 0 {
            //FIXME: bools have not been set here!
            // This will break when the moves require appropiate flags!
            for m in moves {
                let (start, end) = m.split_at(2);

                let source = start.parse::<Coord>().unwrap();
                let target = end.parse::<Coord>().unwrap();

                if m.len() > 4 {
                    //TODO: PARSE PROMOTIONS
                    let (_, promotion) = end.split_at(2);
                    println!("FIXME: end was {}", end);
                    println!("promotion:{}", promotion);
                }
                let mov = Move::new(source, target, self.peek_piece(source).unwrap(), None);
                self.make_move(mov).unwrap();
            }

            // generate the move list for the current position
            let move_list = self.get_moves(self.state.side_to_move);

            for mov in move_list {
                let mut move_name: String = mov
                    .to_string()
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .to_string();

                if let Some(prom) = mov.promoted_piece {
                    match prom {
                        Piece::Queen(_) => move_name.push('q'),
                        Piece::Rook(_) => move_name.push('r'),
                        Piece::Knight(_) => move_name.push('n'),
                        Piece::Bishop(_) => move_name.push('b'),
                        _ => (),
                    };
                }

                let state = self.state;

                if self.make_move(mov).is_ok() {
                    let mut move_nodes = 0;
                    if depth > 1 {
                        self.perft(depth - 1, &mut move_nodes, false);
                    } else {
                        move_nodes = 1;
                    }
                    records.push(MoveRecord {
                        name: move_name,
                        count: move_nodes,
                    });
                    self.state = state;
                }
            }
            Ok(records)
        } else {
            Ok(vec![])
        }
    }
}

impl BitBoardEngine {
    pub fn init(&mut self) {
        let x = Rc::get_mut(&mut self.attack_tables).unwrap();
        x.init();
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn _set_position(&mut self, b: Board) {
        for i in [WHITE_PIECES, BLACK_PIECES].concat() {
            self.state.current_position[i].clear();
        }
        for i in 0..3 {
            self.state.occupancies[i].clear();
        }
        // clear the piece lists
        self.state.white_piece_lists = [(None, 0); 16];
        self.state.black_piece_lists = [(None, 0); 16];

        let mut white_piece_counter = 0;
        let mut black_piece_counter = 0;
        for (sq, p) in b.squares.iter().enumerate() {
            if p.is_some() {
                let piece = p.unwrap();
                match piece {
                    Piece::King(White) => self.state.current_position[WHITE_KING].set_bit(sq),
                    Piece::Queen(White) => self.state.current_position[WHITE_QUEEN].set_bit(sq),
                    Piece::Rook(White) => self.state.current_position[WHITE_ROOK].set_bit(sq),
                    Piece::Bishop(White) => self.state.current_position[WHITE_BISHOP].set_bit(sq),
                    Piece::Knight(White) => self.state.current_position[WHITE_KNIGHT].set_bit(sq),
                    Piece::Pawn(White) => self.state.current_position[WHITE_PAWN].set_bit(sq),

                    Piece::King(Black) => self.state.current_position[BLACK_KING].set_bit(sq),
                    Piece::Queen(Black) => self.state.current_position[BLACK_QUEEN].set_bit(sq),
                    Piece::Rook(Black) => self.state.current_position[BLACK_ROOK].set_bit(sq),
                    Piece::Bishop(Black) => self.state.current_position[BLACK_BISHOP].set_bit(sq),
                    Piece::Knight(Black) => self.state.current_position[BLACK_KNIGHT].set_bit(sq),
                    Piece::Pawn(Black) => self.state.current_position[BLACK_PAWN].set_bit(sq),
                }
                if piece.get_color() == White {
                    self.state.white_piece_lists[white_piece_counter] = (Some(piece), sq);
                    white_piece_counter += 1;
                } else {
                    self.state.black_piece_lists[black_piece_counter] = (Some(piece), sq);
                    black_piece_counter += 1;
                }
            }
        }

        for i in WHITE_PAWN..BLACK_PAWN {
            self.state.occupancies[White as usize]
                .set(self.state.occupancies[White].get() | self.state.current_position[i].get());
        }
        for i in BLACK_PAWN..(BLACK_KING + 1) {
            self.state.occupancies[Black as usize]
                .set(self.state.occupancies[Black].get() | self.state.current_position[i].get());
        }
        self.state.occupancies[BOTH]
            .set(self.state.occupancies[White].get() | self.state.occupancies[Black].get());
    }
    pub fn set_enpassant(&mut self, x: Option<usize>) {
        self.state.enpassant = x;
    }

    pub fn set_castling_rights(&mut self, cr: CastlingRights) {
        self.state.castling_rights = cr;
    }
    #[inline]
    pub fn is_square_attacked_by(&self, sq: usize, col: Color) -> bool {
        match col {
            White => {
                let pawn = self.state.current_position[WHITE_PAWN].get()
                    & self.attack_tables.pawn_attacks[Black as usize][sq].get()
                    != 0;

                let knight = self.state.current_position[WHITE_KNIGHT].get()
                    & self.attack_tables.knight_attacks[sq].get()
                    != 0;

                let bishop = self.state.current_position[WHITE_BISHOP].get()
                    & get_bishop_attack(&self.attack_tables, sq, self.state.occupancies[BOTH])
                        .get()
                    != 0;

                let rook = self.state.current_position[WHITE_ROOK].get()
                    & get_rook_attack(&self.attack_tables, sq, self.state.occupancies[BOTH]).get()
                    != 0;

                let queen = self.state.current_position[WHITE_QUEEN].get()
                    & get_queen_attack(&self.attack_tables, sq, self.state.occupancies[BOTH]).get()
                    != 0;

                let king = self.state.current_position[WHITE_KING].get()
                    & self.attack_tables.king_attacks[sq].get()
                    != 0;

                pawn || knight || bishop || rook || queen || king
            }
            Black => {
                let pawn = self.state.current_position[BLACK_PAWN].get()
                    & self.attack_tables.pawn_attacks[White as usize][sq].get()
                    != 0;

                let knight = self.state.current_position[BLACK_KNIGHT].get()
                    & self.attack_tables.knight_attacks[sq].get()
                    != 0;

                let bishop = self.state.current_position[BLACK_BISHOP].get()
                    & get_bishop_attack(&self.attack_tables, sq, self.state.occupancies[BOTH])
                        .get()
                    != 0;

                let rook = self.state.current_position[BLACK_ROOK].get()
                    & get_rook_attack(&self.attack_tables, sq, self.state.occupancies[BOTH]).get()
                    != 0;

                let queen = self.state.current_position[BLACK_QUEEN].get()
                    & get_queen_attack(&self.attack_tables, sq, self.state.occupancies[BOTH]).get()
                    != 0;
                let king = self.state.current_position[BLACK_KING].get()
                    & self.attack_tables.king_attacks[sq].get()
                    != 0;

                pawn || knight || bishop || rook || queen || king
            }
        }
    }
    #[inline]
    fn _get_moves(&self, side: Color) -> Vec<Move> {
        let mut moves = Vec::with_capacity(30);

        let piece_lists = match side {
            White => WHITE_PIECES,
            Black => BLACK_PIECES,
        };

        for pieces in piece_lists {
            let mut positions = self.state.current_position[pieces];
            while positions.get() != 0 {
                if let Some(source_square) = positions.get_lsb() {
                    let mut m = match pieces {
                        WHITE_PAWN | BLACK_PAWN => get_pawn_moves(self, source_square, side),
                        WHITE_KNIGHT | BLACK_KNIGHT => get_knight_moves(self, source_square, side),
                        WHITE_BISHOP | BLACK_BISHOP => get_bishop_moves(self, source_square, side),
                        WHITE_ROOK | BLACK_ROOK => get_rook_moves(self, source_square, side),
                        WHITE_QUEEN | BLACK_QUEEN => get_queen_moves(self, source_square, side),
                        WHITE_KING | BLACK_KING => get_king_moves(self, source_square, side),
                        _ => vec![],
                    };
                    moves.append(&mut m);
                    positions.reset_bit(source_square);
                }
            }
        }

        moves
            .into_iter()
            .filter(|m| self.test_move_legality(*m).is_ok())
            .collect()
    }
}

// preloaded attack tables
#[derive(Debug, Clone)]
pub struct AttackTables {
    pub king_attacks: Vec<BitBoard>,
    // Queen attacks are built with rook_attacks + bishop_attacks
    pub rook_attacks: Vec<Vec<BitBoard>>,   // 4096 x 64
    pub bishop_attacks: Vec<Vec<BitBoard>>, // 512 x 64
    pub knight_attacks: Vec<BitBoard>,      // 64
    pub pawn_attacks: Vec<Vec<BitBoard>>,   // 64 x 2
    // masks tables (required for the magic bitboards implementation)
    pub rook_masks: Vec<BitBoard>,   // 64
    pub bishop_masks: Vec<BitBoard>, // 64
}

impl Default for AttackTables {
    fn default() -> Self {
        // create empty attack tables
        Self {
            king_attacks: [BitBoard::new(0); 64].to_vec(),
            rook_attacks: vec![vec![BitBoard::new(0); 4096]; 64], //[[BitBoard::new(0); 4096].to_vec(); 64].to_vec(),
            bishop_attacks: vec![vec![BitBoard::new(0); 512]; 64],
            knight_attacks: [BitBoard::new(0); 64].to_vec(),
            pawn_attacks: vec![vec![BitBoard::new(0); 64]; 2],
            // masks
            rook_masks: [BitBoard::new(0); 64].to_vec(),
            bishop_masks: [BitBoard::new(0); 64].to_vec(),
        }
    }
}

impl AttackTables {
    pub fn init(&mut self) {
        // initialise the leaping pieces first
        initialise_king_attacks(&mut self.king_attacks);
        initialise_knight_attacks(&mut self.knight_attacks);
        initialise_pawn_attacks(&mut self.pawn_attacks);

        //initialise sliders
        initialise_bishop_attacks(&mut self.bishop_masks, &mut self.bishop_attacks);
        initialise_rook_attacks(&mut self.rook_masks, &mut self.rook_attacks);
    }

    pub fn new() -> Self {
        Self::default()
    }
}

/// Initialisators
/// This functions initialise the attack tables when an engine is created

fn initialise_king_attacks(table: &mut Vec<BitBoard>) {
    for (i, t) in table.iter_mut().enumerate() {
        *t = generate_king_mask(i);
    }
}

fn initialise_rook_attacks(rook_masks: &mut Vec<BitBoard>, table: &mut Vec<Vec<BitBoard>>) {
    for sq in 0..64 {
        // init masks
        rook_masks[sq] = generate_rook_mask(sq);

        // init current mask
        let attack_mask = rook_masks[sq];

        // relevant bit count
        let relevant_bit_count = attack_mask.popcount();

        // init occupancy index count
        let oc_index_count = 1u64 << relevant_bit_count;

        for occupancy_index in 0..oc_index_count {
            // rebuild the mask just in case!
            let mut attack_mask = rook_masks[sq];

            // init current occupancy
            let occupancy = set_occupancy(
                occupancy_index as usize,
                relevant_bit_count,
                &mut attack_mask,
            );

            // init magic index
            let magic_index = (occupancy.get().overflowing_mul(ROOK_MAGIC_NUMBERS[sq]).0)
                >> (64 - ROOK_REL_BITS[sq]);

            table[sq][magic_index as usize] = rook_attacks_on_the_fly(sq, occupancy);
        }
    }
}

fn initialise_bishop_attacks(bishop_masks: &mut Vec<BitBoard>, table: &mut Vec<Vec<BitBoard>>) {
    for sq in 0..64 {
        // init masks
        bishop_masks[sq] = generate_bishop_mask(sq);

        // init current mask
        let attack_mask = bishop_masks[sq];

        // relevant bit count
        let relevant_bit_count = attack_mask.popcount();

        // init occupancy index count
        let oc_index_count = 1u64 << relevant_bit_count;

        for occupancy_index in 0..oc_index_count {
            // rebuild the mask just in case!
            let mut attack_mask = bishop_masks[sq];

            // init current occupancy
            let occupancy = set_occupancy(
                occupancy_index as usize,
                relevant_bit_count,
                &mut attack_mask,
            );

            // init magic index
            let magic_index = (occupancy.get().overflowing_mul(BISHOP_MAGIC_NUMBERS[sq]).0)
                >> (64 - BISHOP_REL_BITS[sq]);

            table[sq][magic_index as usize] = bishop_attacks_on_the_fly(sq, occupancy);
        }
    }
}

fn initialise_knight_attacks(table: &mut [BitBoard]) {
    for (i, t) in table.iter_mut().enumerate() {
        *t = generate_knight_mask(i);
    }
}

fn initialise_pawn_attacks(table: &mut Vec<Vec<BitBoard>>) {
    for i in 0..64 {
        table[White as usize][i] = generate_pawn_mask(i, White);
        table[Black as usize][i] = generate_pawn_mask(i, Black);
    }
}

fn minimax_search(engine: &mut BitBoardEngine, depth: usize) -> f32 {
    if depth == 0 {
        return engine.evaluate();
    }
    let player = engine.state.side_to_move;
    if player == White {
        let mut max_eval: f32 = std::f32::MIN;
        // generate the move list for the current position
        let move_list = engine.get_moves(player);
        for mov in move_list {
            // store current state
            let state = engine.state;
            //make move and evaluate
            if engine.make_move(mov).is_ok() {
                max_eval = f32::max(minimax_search(engine, depth - 1), max_eval);
            }
            //restore state
            engine.state = state;
        }
        max_eval
    } else {
        let mut min_eval: f32 = std::f32::MAX;
        // generate the move list for the current position
        let move_list = engine.get_moves(player);

        for mov in move_list {
            let state = engine.state;

            if engine.make_move(mov).is_ok() {
                min_eval = f32::min(minimax_search(engine, depth - 1), min_eval);
            }
            engine.state = state;
        }
        min_eval
    }
}
