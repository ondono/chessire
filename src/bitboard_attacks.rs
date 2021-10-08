use crate::bitboard::*;
use crate::constants::*;
use crate::piece::Color;
use crate::*;

// Generating King attacked squares
pub fn generate_king_mask(sq: usize) -> BitBoard {
    let bb = BitBoard::new_single_bit(sq);
    let mut attacks = BitBoard::new(0);

    attacks.set(attacks.get());
    attacks
}

// generate the Queen mask from Bishop and Rook
pub fn generate_queen_mask(sq: usize, block: BitBoard) -> BitBoard {
    let mut bb = BitBoard::new(0);
    bb.set(generate_rook_mask(sq, block).get() | generate_bishop_mask(sq, block).get());
    bb
}

// generates the hole horizontal/vertical mask
pub fn generate_rook_mask(sq: usize, block: BitBoard) -> BitBoard {
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

// generates the hole diagonals mask
fn generate_bishop_mask(sq: usize, block: BitBoard) -> BitBoard {
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

fn generate_knight_mask(sq: usize) -> BitBoard {
    let bb = BitBoard::new_single_bit(sq);
    let mut attacks = BitBoard::new(0);

    if (bb.get() << 17) & !BB_FILE_A != 0 {
        attacks.set(attacks.get() | (bb.get() << 17));
    }
    if (bb.get() >> 17) & !BB_FILE_H != 0 {
        attacks.set(attacks.get() | (bb.get() >> 17));
    }
    if (bb.get() << 15) & !BB_FILE_H != 0 {
        attacks.set(attacks.get() | (bb.get() << 15));
    }
    if (bb.get() >> 15) & !BB_FILE_A != 0 {
        attacks.set(attacks.get() | (bb.get() >> 15));
    }
    if (bb.get() << 10 & (!BB_FILE_A & !BB_FILE_B)) != 0 {
        attacks.set(attacks.get() | (bb.get() << 10));
    }
    if (bb.get() >> 10 & (!BB_FILE_H & !BB_FILE_G)) != 0 {
        attacks.set(attacks.get() | (bb.get() >> 10));
    }
    if (bb.get() << 6 & (!BB_FILE_H & !BB_FILE_G)) != 0 {
        attacks.set(attacks.get() | (bb.get() << 6));
    }
    if (bb.get() >> 6 & (!BB_FILE_A & !BB_FILE_B)) != 0 {
        attacks.set(attacks.get() | (bb.get() >> 6));
    }

    attacks
}

fn generate_pawn_mask(sq: usize, color: Color) -> BitBoard {
    let bb = BitBoard::new_single_bit(sq);
    let mut attacks = BitBoard::new(0);
    let (r, l, mask_r, mask_l) = if color == White {
        (bb.get() << 7, bb.get() << 9, !BB_FILE_H, !BB_FILE_A)
    } else {
        (bb.get() >> 7, bb.get() >> 9, !BB_FILE_A, !BB_FILE_H)
    };
    if r & mask_r != 0 {
        attacks.set(attacks.get() | r);
    }
    if l & mask_l != 0 {
        attacks.set(attacks.get() | l);
    }
    attacks
}
