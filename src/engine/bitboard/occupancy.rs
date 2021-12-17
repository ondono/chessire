use super::*;
// Generate relevant occupancy bits
pub fn generate_bishop_relevant_ocupancy(sq: usize) -> BitBoard {
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

pub fn generate_rook_relevant_ocupancy(sq: usize) -> BitBoard {
    let mut bb = BitBoard::new(0);

    let (file, rank) = get_file_rank(sq);
    for file in file..7 {
        bb.set_bit(get_index(file, rank));
    }
    for file in (1..file).rev() {
        bb.set_bit(get_index(file, rank));
    }
    for rank in rank..7 {
        bb.set_bit(get_index(file, rank));
    }
    for rank in (1..rank).rev() {
        bb.set_bit(get_index(file, rank));
    }
    bb.reset_bit(sq);
    bb
}

pub fn set_occupancy(index: usize, num_bits: usize, attack_mask: &mut BitBoard) -> BitBoard {
    let mut ocuppancy = BitBoard::new(0);

    for i in 0..num_bits {
        if let Some(sq) = attack_mask.get_lsb() {
            if index & (1 << i) != 0 {
                ocuppancy.set_bit(sq);
            }
            attack_mask.reset_bit(sq);
        } else {
            break;
        }
    }
    ocuppancy
}
