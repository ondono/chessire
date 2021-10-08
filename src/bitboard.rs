use crate::*;
use constants::*;
use rand::*;
use termion::color;

#[derive(Debug, Copy, Clone)]
pub struct BitBoard(u64);

impl BitBoard {
    pub fn new(u: u64) -> Self {
        Self(u)
    }
    #[inline]
    pub fn new_single_bit(sq: usize) -> Self {
        let mut x = Self(0);
        x.set_bit(sq);
        x
    }
    #[inline]
    pub fn get(&self) -> u64 {
        self.0
    }
    #[inline]
    pub fn get_bit(&self, square: usize) -> bool {
        (1 << square & self.0) != 0
    }
    #[inline]
    pub fn set_bit(&mut self, square: usize) {
        self.0 |= 1 << square;
    }
    #[inline]
    pub fn reset_bit(&mut self, square: usize) {
        self.0 &= !(1 << square);
    }
    #[inline]
    pub fn set(&mut self, u: u64) {
        self.0 = u;
    }
    #[inline]
    pub fn popcount(&self) -> usize {
        let mut c = self.get();
        let mut count = 0;
        while c != 0 {
            c &= c - 1;
            count += 1;
        }
        count
    }
    #[inline]
    pub fn get_lsb(&self) -> Option<usize> {
        let t = self.get() as i64;
        if self.0 != 0 {
            Some(BitBoard::new(((t & -t) - 1) as u64).popcount())
        } else {
            None
        }
    }
}

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank_range = (0..8).rev().collect::<Vec<usize>>();
        write!(
            f,
            "{}{}\r\n    A  B  C  D  E  F  G  H\r\n",
            color::Fg(color::White),
            color::Bg(color::Reset),
        )?;
        for rank in rank_range {
            // at the start of the rank, set the rank name
            write!(
                f,
                "{}{} {} ",
                color::Fg(color::White),
                color::Bg(color::Reset),
                rank + 1
            )?;
            for file in 0..8 {
                let (tile_color,text_color) =
                    // this sets the tile white or black
                    if (file + rank) & 0x01 == 1 {
                        (color::Rgb(200, 200, 200),
                        color::Rgb(100, 100, 100))
                    } else {
                        (color::Rgb(100, 100, 100),
                        color::Rgb(200, 200, 200))
                    };
                write!(
                    f,
                    "{}{} {} ",
                    color::Bg(tile_color),
                    color::Fg(if self.get_bit(get_index(file, rank)) {
                        color::Rgb(255, 0, 0)
                    } else {
                        text_color
                    }),
                    if self.get_bit(get_index(file, rank)) {
                        1
                    } else {
                        0
                    }
                )?;
            }
            //end of line
            write!(
                f,
                "{}{}\r\n",
                color::Bg(color::Reset),
                color::Fg(color::Reset)
            )?;
        }
        write!(f, "\r\nBitboard:\t{}d", self.0)?;
        write!(f, "\r\nBitboard:\t{:#X}\r\n", self.0)
    }
}

#[inline]
pub fn get_index(file: usize, rank: usize) -> usize {
    file + rank * 8
}

pub fn get_file_rank(index: usize) -> (usize, usize) {
    (index % 8, index / 8)
}

fn generate_king_mask(sq: usize) -> BitBoard {
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
        !BB_FILE_A & !BB_RANK_8, // up right, block left side
        !BB_RANK_1,              // up block down
        !BB_FILE_H & !BB_RANK_8, // up left, block right side
        !BB_FILE_A,              // right, block left side
        !BB_FILE_H,              // left, block right side
        !BB_FILE_A & !BB_RANK_1, // down right, block left side
        !BB_RANK_8,              // down, block up
        !BB_FILE_H & !BB_RANK_1, // down left, block right side
    ];

    for d in 0..8 {
        if directions[d] & masks[d] != 0 {
            attacks.set(attacks.get() | directions[d]);
        }
    }
    attacks
}

fn generate_queen_mask(sq: usize, block: BitBoard) -> BitBoard {
    let mut attacks = BitBoard::new(0);
    attacks.set(generate_rook_mask(sq, block).get() | generate_bishop_mask(sq, block).get());
    attacks.reset_bit(sq);
    attacks
}

// generates the hole horizontal/vertical mask
fn generate_rook_mask(sq: usize, block: BitBoard) -> BitBoard {
    let mut bb = BitBoard::new(0);

    let (file, rank) = get_file_rank(sq);
    for file in file..8 {
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
    for rank in rank..8 {
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
    bb.reset_bit(sq);
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

// Generate relevant occupancy bits
//
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

pub fn initialise_pawn_attacks(table: &mut [[BitBoard; 64]; 2]) {
    for i in 0..64 {
        table[White as usize][i] = generate_pawn_mask(i, White);
        table[Black as usize][i] = generate_pawn_mask(i, Black);
    }
}

pub fn initialise_knight_attacks(table: &mut [BitBoard; 64]) {
    for (i, t) in table.iter_mut().enumerate() {
        *t = generate_knight_mask(i);
    }
}

pub fn initialise_king_attacks(table: &mut [BitBoard; 64]) {
    for (i, t) in table.iter_mut().enumerate() {
        *t = generate_king_mask(i);
    }
}

pub fn test_bitboard() {
    //let block = BitBoard::new(BB_RANK_2 | BB_RANK_7 | BB_FILE_B | BB_FILE_G);
    let block = BitBoard::new(0);

    let king_attacks = [BitBoard::new(0); 64];

    init_magics();
    //initialise_king_attacks(&mut king_attacks);
    //
    //let mut attack_mask = generate_bishop_relevant_ocupancy(LUT_0X88_TO_MAILBOX[D4]);

    //let occupancy = set_occupancy(0, attack_mask.popcount(), &mut attack_mask);
    //println!("{}", occupancy);

    //for sq in [A1, D4, A1, H8, H1, A8] {
    //println!("{}", king_attacks[LUT_0X88_TO_MAILBOX[sq]]);
    //println!("{}", get_name_from_index(sq));
    //println!("{}", generate_king_mask(LUT_0X88_TO_MAILBOX[sq]));
    //        println!("{}", generate_rook_mask(LUT_0X88_TO_MAILBOX[sq], block));
    //        println!("{}", generate_queen_mask(LUT_0X88_TO_MAILBOX[sq], block));
    //}
    //
}

fn set_occupancy(index: usize, num_bits: usize, attack_mask: &mut BitBoard) -> BitBoard {
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

fn generate_random_number() -> u64 {
    let mut rng = thread_rng();
    rng.gen_range(0..(u64::MAX & 0xFF00000000000000))
}

fn find_magic_number(sq: usize, num_bits: usize, bishop: bool) -> u64 {
    let mut occupancies = [BitBoard::new(0); 4096];
    let mut attacks = [BitBoard::new(0); 4096];

    let attack_mask = if bishop {
        generate_bishop_relevant_ocupancy(sq)
    } else {
        generate_rook_relevant_ocupancy(sq)
    };

    let oc_indexes: usize = 1 << num_bits;

    for index in 0..oc_indexes {
        occupancies[index] = set_occupancy(index, num_bits, &mut attack_mask.clone());
        attacks[index] = if bishop {
            generate_bishop_mask(sq, occupancies[index])
        } else {
            generate_rook_mask(sq, occupancies[index])
        };
    }

    // test magic numbers
    //
    for _random_count in 0..100000000 {
        let candidate = generate_random_number();

        //println!("Testing candidate:{}", candidate);
        if BitBoard::new(candidate).popcount() < 6 {
            //println!("Candidate had too low popcount");
            continue;
        }

        let c = candidate as u128;
        let a = attack_mask.get() as u128;

        if ((c * a) & 0xFF00000000000000) != 0 {
            continue;
        }

        // init used attacks
        let mut used_attacks = [BitBoard::new(0); 4096];

        let mut fail = false;

        // test magic index loop
        for index in 0..oc_indexes {
            if fail {
                break;
            }
            // init magic index
            let magic_index = ((occupancies[index].get().checked_mul(candidate).unwrap())
                >> (64 - num_bits)) as usize;

            // if magic index works
            if used_attacks[magic_index].get() == 0 {
                // initialize used attacks
                used_attacks[magic_index] = attacks[index];
            } else {
                if used_attacks[magic_index].get() != attacks[index].get() {
                    // magic index doesn't work
                    fail = true;
                }
            }
            if !fail {
                return candidate;
            }
        }
    }
    0
}

fn init_magics() {
    for sq in 0..64 {
        println!(
            "Bishop:\t{:#X}",
            find_magic_number(sq, BB_BISHOP_REL_BITS[sq], true)
        );
        println!(
            "Rook:\t{:#X}",
            find_magic_number(sq, BB_ROOK_REL_BITS[sq], false)
        );
    }
}
