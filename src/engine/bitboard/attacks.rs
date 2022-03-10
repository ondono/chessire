use super::constants::*;
use super::*;
use chessire_utils::color::Color::{self, White};

/////***************************/////
/////*****  ATTACK GETTERS *****/////
/////***************************/////

// These functions return the attacked squares for each piece type. They are only defined for
// sliding pieces, since are the only ones requiring of Magics. For non slider pieces a single
// lookup at the corresponding attack table is sufficient

#[inline]
pub fn get_queen_attack(at: &AttackTables, sq: usize, occupied_positions: BitBoard) -> BitBoard {
    BitBoard::new(
        get_rook_attack(at, sq, occupied_positions).get()
            | get_bishop_attack(at, sq, occupied_positions).get(),
    )
}
#[inline]
pub fn get_rook_attack(at: &AttackTables, sq: usize, occupied_positions: BitBoard) -> BitBoard {
    // get bishop attacks assuming current board occupancy
    let mut occupancy = occupied_positions.get() & at.rook_masks[sq].get();
    occupancy = occupancy.overflowing_mul(ROOK_MAGIC_NUMBERS[sq]).0;
    occupancy >>= 64 - ROOK_REL_BITS[sq];

    at.rook_attacks[sq][occupancy as usize]
}

#[inline]
pub fn get_bishop_attack(at: &AttackTables, sq: usize, occupied_positions: BitBoard) -> BitBoard {
    // get bishop attacks assuming current board occupancy
    let mut occupancy = occupied_positions.get() & at.bishop_masks[sq].get();
    occupancy = occupancy.overflowing_mul(BISHOP_MAGIC_NUMBERS[sq]).0;
    occupancy >>= 64 - BISHOP_REL_BITS[sq];

    at.bishop_attacks[sq][occupancy as usize]
}

/////***************************/////
/////**** ATTACK GENERATORS ****/////
/////***************************/////

// These functions generate attack bitboards for sliding pieces, they should not be used
// during move computing, they're use is limited to the seeding of attack tables during
// the loading phase

// generates the hole horizontal/vertical attacks given a blockers bitboard
pub fn rook_attacks_on_the_fly(sq: usize, block: BitBoard) -> BitBoard {
    let mut bb = BitBoard::new(0);
    let (file, rank) = get_file_rank(sq);
    for file in file + 1..8 {
        bb.set_bit(get_index(file, rank));
        if block.get_bit(get_index(file, rank)) {
            break;
        }
    }
    for file in (0..file).rev() {
        bb.set_bit(get_index(file, rank));
        if block.get_bit(get_index(file, rank)) {
            break;
        }
    }
    for rank in rank + 1..8 {
        bb.set_bit(get_index(file, rank));
        if block.get_bit(get_index(file, rank)) {
            break;
        }
    }
    for rank in (0..rank).rev() {
        bb.set_bit(get_index(file, rank));
        if block.get_bit(get_index(file, rank)) {
            break;
        }
    }
    bb
}

// generates the hole diagonals attacks given a blockers bitboard
pub fn bishop_attacks_on_the_fly(sq: usize, block: BitBoard) -> BitBoard {
    let mut bb = BitBoard::new(0);
    let (file, rank) = get_file_rank(sq);
    for (file, rank) in (file..8).zip(rank..8).skip(1) {
        bb.set_bit(get_index(file, rank));
        bb.set_bit(get_index(file, rank));
        if block.get_bit(get_index(file, rank)) {
            break;
        }
    }
    for (file, rank) in (0..file).rev().zip((0..rank).rev()) {
        bb.set_bit(get_index(file, rank));
        bb.set_bit(get_index(file, rank));
        if block.get_bit(get_index(file, rank)) {
            break;
        }
    }
    for (file, rank) in (file..8).skip(1).zip((0..rank).rev()) {
        bb.set_bit(get_index(file, rank));
        bb.set_bit(get_index(file, rank));
        if block.get_bit(get_index(file, rank)) {
            break;
        }
    }
    for (file, rank) in (0..file).rev().zip((rank..8).skip(1)) {
        bb.set_bit(get_index(file, rank));
        bb.set_bit(get_index(file, rank));
        if block.get_bit(get_index(file, rank)) {
            break;
        }
    }
    bb
}

/////***************************/////
/////***** MASK GENERATORS *****/////
/////***************************/////

// Generating King attacked squares
pub fn generate_king_mask(sq: usize) -> BitBoard {
    let bb = BitBoard::new_single_bit(sq);
    let mut attacks = BitBoard::new(0);
    let directions = [
        bb.get() << 9, // up right
        bb.get() << 8, // up
        bb.get() << 7, // up left
        bb.get() << 1, // right
        bb.get() >> 1, // left
        bb.get() >> 7, // down right
        bb.get() >> 8, // down
        bb.get() >> 9, // down left
    ];

    let masks = [
        !FILE_A & !RANK_1, // up right, block left side
        !RANK_1,           // up block down
        !FILE_H & !RANK_1, // up left, block right side
        !FILE_A,           // right, block left side
        !FILE_H,           // left, block right side
        !FILE_A & !RANK_8, // down right, block left side
        !RANK_8,           // down, block up
        !FILE_H & !RANK_8, // down left, block right side
    ];

    for d in 0..8 {
        if directions[d] & masks[d] != 0 {
            attacks.set(attacks.get() | directions[d]);
        }
    }
    attacks
}

pub fn generate_rook_mask(sq: usize) -> BitBoard {
    let mut bb = BitBoard::new(0);
    let (file, rank) = get_file_rank(sq);

    for file in file + 1..7 {
        bb.set_bit(get_index(file, rank));
    }
    for file in (1..file).rev() {
        bb.set_bit(get_index(file, rank));
    }
    for rank in rank + 1..7 {
        bb.set_bit(get_index(file, rank));
    }
    for rank in (1..rank).rev() {
        bb.set_bit(get_index(file, rank));
    }
    bb
}

pub fn generate_bishop_mask(sq: usize) -> BitBoard {
    let mut bb = BitBoard::new(0);
    let (file, rank) = get_file_rank(sq);

    for (file, rank) in (file..7).zip(rank..7).skip(1) {
        bb.set_bit(get_index(file, rank));
        bb.set_bit(get_index(file, rank));
    }
    for (file, rank) in (1..file).rev().zip((1..rank).rev()) {
        bb.set_bit(get_index(file, rank));
        bb.set_bit(get_index(file, rank));
    }
    for (file, rank) in (file..7).skip(1).zip((1..rank).rev()) {
        bb.set_bit(get_index(file, rank));
        bb.set_bit(get_index(file, rank));
    }
    for (file, rank) in (1..file).rev().zip((rank..7).skip(1)) {
        bb.set_bit(get_index(file, rank));
        bb.set_bit(get_index(file, rank));
    }
    bb
}

pub fn generate_knight_mask(sq: usize) -> BitBoard {
    let bb = BitBoard::new_single_bit(sq);
    let mut attacks = BitBoard::new(0);

    if (bb.get() << 17) & !FILE_A != 0 {
        attacks.set(attacks.get() | (bb.get() << 17));
    }
    if (bb.get() >> 17) & !FILE_H != 0 {
        attacks.set(attacks.get() | (bb.get() >> 17));
    }
    if (bb.get() << 15) & !FILE_H != 0 {
        attacks.set(attacks.get() | (bb.get() << 15));
    }
    if (bb.get() >> 15) & !FILE_A != 0 {
        attacks.set(attacks.get() | (bb.get() >> 15));
    }
    if (bb.get() << 10 & (!FILE_A & !FILE_B)) != 0 {
        attacks.set(attacks.get() | (bb.get() << 10));
    }
    if (bb.get() >> 10 & (!FILE_H & !FILE_G)) != 0 {
        attacks.set(attacks.get() | (bb.get() >> 10));
    }
    if (bb.get() << 6 & (!FILE_H & !FILE_G)) != 0 {
        attacks.set(attacks.get() | (bb.get() << 6));
    }
    if (bb.get() >> 6 & (!FILE_A & !FILE_B)) != 0 {
        attacks.set(attacks.get() | (bb.get() >> 6));
    }

    attacks
}

pub fn generate_pawn_mask(sq: usize, color: Color) -> BitBoard {
    let bb = BitBoard::new_single_bit(sq);
    let mut attacks = BitBoard::new(0);
    let (r, l, mask_r, mask_l) = if color == White {
        (bb.get() << 7, bb.get() << 9, !FILE_H, !FILE_A)
    } else {
        (bb.get() >> 7, bb.get() >> 9, !FILE_A, !FILE_H)
    };
    if r & mask_r != 0 {
        attacks.set(attacks.get() | r);
    }
    if l & mask_l != 0 {
        attacks.set(attacks.get() | l);
    }
    attacks
}
