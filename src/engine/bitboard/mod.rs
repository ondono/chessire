pub mod attacks;
pub mod constants;
pub mod magics;
pub mod occupancy;
pub mod tests;
pub mod util;

use super::board::*;
use super::color::*;
use super::piece::*;
use crate::game::color::Color::{Black, White};
use attacks::*;
use constants::*;
use occupancy::*;
use util::*;

// bitboard state indexes

pub const WHITE_PAWN: usize = 0;
pub const WHITE_KNIGHT: usize = 1;
pub const WHITE_BISHOP: usize = 2;
pub const WHITE_ROOK: usize = 3;
pub const WHITE_QUEEN: usize = 4;
pub const WHITE_KING: usize = 5;

pub const BLACK_PAWN: usize = 6;
pub const BLACK_KNIGHT: usize = 7;
pub const BLACK_BISHOP: usize = 8;
pub const BLACK_ROOK: usize = 9;
pub const BLACK_QUEEN: usize = 10;
pub const BLACK_KING: usize = 11;

pub const BOTH: usize = 2;

#[derive(Debug, Clone)]
pub struct BitBoardEngine {
    pub attack_tables: AttackTables,
    pub current_position: [BitBoard; 12],
    pub occupancies: [BitBoard; 3],
}

impl Default for BitBoardEngine {
    fn default() -> Self {
        let mut engine = Self {
            attack_tables: AttackTables::new(),
            current_position: [BitBoard::new(0); 12],
            occupancies: [BitBoard::new(0xFFFFFFFFFFFFFFFF); 3],
        };
        engine.init();
        engine
    }
}

impl BitBoardEngine {
    pub fn init(&mut self) {
        self.attack_tables.init();
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_position(&mut self, b: Board) {
        for (sq, p) in b.squares.iter().enumerate() {
            if p.is_some() {
                match p.unwrap() {
                    Piece::King(White) => self.current_position[WHITE_KING].set_bit(sq),
                    Piece::Queen(White) => self.current_position[WHITE_QUEEN].set_bit(sq),
                    Piece::Rook(White) => self.current_position[WHITE_ROOK].set_bit(sq),
                    Piece::Bishop(White) => self.current_position[WHITE_BISHOP].set_bit(sq),
                    Piece::Knight(White) => self.current_position[WHITE_KNIGHT].set_bit(sq),
                    Piece::Pawn(White) => self.current_position[WHITE_PAWN].set_bit(sq),

                    Piece::King(Black) => self.current_position[BLACK_KING].set_bit(sq),
                    Piece::Queen(Black) => self.current_position[BLACK_QUEEN].set_bit(sq),
                    Piece::Rook(Black) => self.current_position[BLACK_ROOK].set_bit(sq),
                    Piece::Bishop(Black) => self.current_position[BLACK_BISHOP].set_bit(sq),
                    Piece::Knight(Black) => self.current_position[BLACK_KNIGHT].set_bit(sq),
                    Piece::Pawn(Black) => self.current_position[BLACK_PAWN].set_bit(sq),
                }
            }
        }
        for i in WHITE_PAWN..BLACK_PAWN {
            self.occupancies[White as usize]
                .set(self.occupancies[White as usize].get() | self.current_position[i].get());
        }
        for i in BLACK_PAWN..(BLACK_KING + 1) {
            self.occupancies[Black as usize]
                .set(self.occupancies[Black as usize].get() | self.current_position[i].get());
        }
        self.occupancies[BOTH]
            .set(self.occupancies[White as usize].get() | self.occupancies[Black as usize].get());
    }

    #[inline]
    pub fn is_square_attacked_by(&self, sq: usize, col: Color) -> bool {
        match col {
            White => {
                let pawn = self.current_position[WHITE_PAWN].get()
                    & self.attack_tables.pawn_attacks[Black as usize][sq].get()
                    != 0;

                let knight = self.current_position[WHITE_KNIGHT].get()
                    & self.attack_tables.knight_attacks[sq].get()
                    != 0;

                let bishop = self.current_position[WHITE_BISHOP].get()
                    & get_bishop_attack(&self.attack_tables, sq, self.occupancies[BOTH]).get()
                    != 0;

                let rook = self.current_position[WHITE_ROOK].get()
                    & get_rook_attack(&self.attack_tables, sq, self.occupancies[BOTH]).get()
                    != 0;

                let queen = self.current_position[WHITE_QUEEN].get()
                    & get_queen_attack(&self.attack_tables, sq, self.occupancies[BOTH]).get()
                    != 0;
                let king = false;

                pawn || knight || bishop || rook || queen || king
            }
            Black => {
                let pawn = self.current_position[BLACK_PAWN].get()
                    & self.attack_tables.pawn_attacks[White as usize][sq].get()
                    != 0;

                let knight = self.current_position[BLACK_KNIGHT].get()
                    & self.attack_tables.knight_attacks[sq].get()
                    != 0;

                let bishop = self.current_position[BLACK_BISHOP].get()
                    & get_bishop_attack(&self.attack_tables, sq, self.occupancies[BOTH]).get()
                    != 0;

                let rook = self.current_position[BLACK_ROOK].get()
                    & get_rook_attack(&self.attack_tables, sq, self.occupancies[BOTH]).get()
                    != 0;

                let queen = self.current_position[BLACK_QUEEN].get()
                    & get_queen_attack(&self.attack_tables, sq, self.occupancies[0]).get()
                    != 0;
                let king = false;

                pawn || knight || bishop || rook || queen || king
            }
        }
    }
    #[inline]
    pub fn get_moves() {
        //
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
