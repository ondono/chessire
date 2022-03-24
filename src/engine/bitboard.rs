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
use attacks::*;
use chessire_utils::color::Color::{Black, White};
use chessire_utils::moves::*;
use constants::*;
use occupancy::*;
use util::*;

#[derive(Debug, Copy, Clone)]
pub struct BitBoardState {
    pub current_position: [BitBoard; 12],
    pub occupancies: [BitBoard; 3],
    pub enpassant: Option<usize>,
    pub castling_rights: CastlingRights,
    pub halfmove_clock: u32,
    pub fullmove_clock: u32,
    pub side_to_move: Color,
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

#[derive(Debug, Clone)]
pub struct BitBoardEngine {
    pub attack_tables: AttackTables,
    pub state: BitBoardState,
}

impl Default for BitBoardEngine {
    fn default() -> Self {
        let mut engine = Self {
            attack_tables: AttackTables::new(),
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
        ChessGame::new()
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
                self.state.enpassant = Some(mov.target.next_up().unwrap_or(mov.target).to_usize());
            } else {
                self.state.enpassant =
                    Some(mov.target.next_down().unwrap_or(mov.target).to_usize());
            }
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
                    self.state.current_position[WHITE_ROOK].reset_bit(index_from_bitmask(D1));
                }
                // G1
                6 => {
                    self.state.current_position[WHITE_ROOK].reset_bit(index_from_bitmask(H1)); // a1
                    self.state.current_position[WHITE_ROOK].reset_bit(index_from_bitmask(F1));
                }
                // C8
                58 => {
                    self.state.current_position[WHITE_ROOK].reset_bit(index_from_bitmask(A8)); // a1
                    self.state.current_position[WHITE_ROOK].reset_bit(index_from_bitmask(D8));
                }
                // G8
                62 => {
                    self.state.current_position[WHITE_ROOK].reset_bit(index_from_bitmask(H8)); // a1
                    self.state.current_position[WHITE_ROOK].reset_bit(index_from_bitmask(F8));
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
                    self.state.castling_rights.white_queen_side = false;
                    self.state.castling_rights.white_queen_side = false;
                }
                // possibly mobing black king
                60 => {
                    self.state.castling_rights.black_queen_side = false;
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
                    //self.unmake_move();
                    self.state = state;
                }
            }
        } else {
            *nodes += 1;
        }
    }
}

impl BitBoardEngine {
    pub fn init(&mut self) {
        self.attack_tables.init();
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn _set_position(&mut self, b: Board) {
        for (sq, p) in b.squares.iter().enumerate() {
            if p.is_some() {
                match p.unwrap() {
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
                let king = false;

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
                    & get_queen_attack(&self.attack_tables, sq, self.state.occupancies[0]).get()
                    != 0;
                let king = false;

                pawn || knight || bishop || rook || queen || king
            }
        }
    }
    #[inline]
    fn _get_moves(&self, side: Color) -> Vec<Move> {
        let mut moves = vec![];

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

fn initialise_knight_attacks(table: &mut Vec<BitBoard>) {
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
