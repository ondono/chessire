pub mod attacks;
pub mod constants;
pub mod magics;
pub mod occupancy;
pub mod tests;
pub mod util;

use super::ChessGame;
use crate::game::color::Color::{Black, White};
use attacks::*;
use constants::*;
use occupancy::*;
use util::*;

#[derive(Debug, Clone)]
pub struct BitBoardEngine {
    pub attack_tables: AttackTables,
    pub current_position: BitBoard,
}

impl Default for BitBoardEngine {
    fn default() -> Self {
        let mut engine = Self {
            attack_tables: AttackTables::new(),
            current_position: BitBoard::new(0),
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
