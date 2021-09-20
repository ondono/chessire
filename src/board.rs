#[warn(dead_code)]
use crate::piece::*;
use std::fmt;
use termion::color;

// Rank 1
pub const A1: usize = 0x0;
pub const B1: usize = 0x1;
pub const C1: usize = 0x2;
pub const D1: usize = 0x3;
pub const E1: usize = 0x4;
pub const F1: usize = 0x5;
pub const G1: usize = 0x6;
pub const H1: usize = 0x7;

// Rank 2
pub const A2: usize = 0x10;
pub const B2: usize = 0x11;
pub const C2: usize = 0x12;
pub const D2: usize = 0x13;
pub const E2: usize = 0x14;
pub const F2: usize = 0x15;
pub const G2: usize = 0x16;
pub const H2: usize = 0x17;

// Rank 3
pub const A3: usize = 0x20;
pub const B3: usize = 0x21;
pub const C3: usize = 0x22;
pub const D3: usize = 0x23;
pub const E3: usize = 0x24;
pub const F3: usize = 0x25;
pub const G3: usize = 0x26;
pub const H3: usize = 0x27;

// Rank 4
pub const A4: usize = 0x30;
pub const B4: usize = 0x31;
pub const C4: usize = 0x32;
pub const D4: usize = 0x33;
pub const E4: usize = 0x34;
pub const F4: usize = 0x35;
pub const G4: usize = 0x36;
pub const H4: usize = 0x37;

// Rank 5
pub const A5: usize = 0x40;
pub const B5: usize = 0x41;
pub const C5: usize = 0x42;
pub const D5: usize = 0x43;
pub const E5: usize = 0x44;
pub const F5: usize = 0x45;
pub const G5: usize = 0x46;
pub const H5: usize = 0x47;

// Rank 6
pub const A6: usize = 0x50;
pub const B6: usize = 0x51;
pub const C6: usize = 0x52;
pub const D6: usize = 0x53;
pub const E6: usize = 0x54;
pub const F6: usize = 0x55;
pub const G6: usize = 0x56;
pub const H6: usize = 0x57;

// Rank 7
pub const A7: usize = 0x60;
pub const B7: usize = 0x61;
pub const C7: usize = 0x62;
pub const D7: usize = 0x63;
pub const E7: usize = 0x64;
pub const F7: usize = 0x65;
pub const G7: usize = 0x66;
pub const H7: usize = 0x67;

// Rank 8
pub const A8: usize = 0x70;
pub const B8: usize = 0x71;
pub const C8: usize = 0x72;
pub const D8: usize = 0x73;
pub const E8: usize = 0x74;
pub const F8: usize = 0x75;
pub const G8: usize = 0x76;
pub const H8: usize = 0x77;

// rank and file (never use individually)
pub const A: usize = 0x0;
pub const B: usize = 0x1;
pub const C: usize = 0x2;
pub const D: usize = 0x3;
pub const E: usize = 0x4;
pub const F: usize = 0x5;
pub const G: usize = 0x6;
pub const H: usize = 0x7;

#[inline(always)]
pub fn index_by_file_and_rank(rank: usize, file: usize) -> usize {
    16 * rank + file
}

#[inline(always)]
pub fn file_by_index(index: usize) -> usize {
    index & 0x07
}

pub fn file_char_by_index(index: usize) -> char {
    ((index & 0x07) as u8 + 'A' as u8).to_ascii_uppercase() as char
}

#[inline(always)]
pub fn rank_by_index(index: usize) -> usize {
    index >> 4
}

#[inline(always)]
pub fn is_off_board(index: usize) -> bool {
    index & 0x88 != 0
}

const PIECE_PLACEMENT: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

pub struct Board {
    squares: [Option<Piece>; 128],
    selected: usize,
    highlighted: Vec<usize>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            squares: [None; 128],
            selected: 0x78,
            highlighted: vec![],
        }
    }
    pub fn set_position_from_fen(&mut self, piece_placement: &str) {
        for (i, rnk) in piece_placement.split('/').enumerate() {
            let rank = 7 - i;
            let mut file = 1;

            for c in rnk.chars() {
                if c.is_ascii_digit() {
                    let space = c.to_digit(10).unwrap() as usize;
                    file += space;
                } else {
                    // set a piece
                    self.squares[index_by_file_and_rank(rank, file)] = Piece::new_from_fen_char(c);
                    file += 1;
                }
            }
        }
    }
    pub fn set_start_position(&mut self) {
        self.set_position_from_fen(PIECE_PLACEMENT);
    }
    pub fn select(&mut self, index: usize) {
        self.selected = index;
    }
    pub fn add_to_highlighted(&mut self, index: usize) {
        self.highlighted.push(index);
    }
    pub fn remove_from_highlighted(&mut self, index: usize) {
        self.highlighted.remove(index);
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}\r\n    {}  {}  {}  {}  {}  {}  {}  {}",
            color::Fg(color::White),
            color::Bg(color::Reset),
            'A',
            'B',
            'C',
            'D',
            'E',
            'F',
            'G',
            'H'
        )?;
        for (i, sq) in self.squares.iter().skip(1).enumerate() {
            // loop over all squares
            if i & 0x88 == 0 {
                // ignore out of board squares
                let file = file_by_index(i);
                let rank = rank_by_index(i) + 1;

                // if we are at the start of a rank, print the name
                if i & 0x0F == 0 {
                    write!(
                        f,
                        "{}{}\n {} ",
                        color::Fg(color::White),
                        color::Bg(color::Reset),
                        rank
                    )?;
                }
                let tile_color = if i == self.selected {
                    color::Rgb(200, 200, 0)
                } else if self.highlighted.contains(&i) {
                    color::Rgb(200, 0, 0)
                } else {
                    // this sets the tile white or black
                    if (file + rank) & 0x01 == 0 {
                        color::Rgb(200, 200, 200)
                    } else {
                        color::Rgb(100, 100, 100)
                    }
                };

                // print the piece (if any)
                match sq {
                    Some(piece) => write!(f, "{} {} ", color::Bg(tile_color), piece)?,
                    _ => write!(f, "{}   ", color::Bg(tile_color))?,
                };
            }
        }
        // add an empty line and clear all styling
        write!(
            f,
            "{}{}\r\n",
            color::Fg(color::Reset),
            color::Bg(color::Reset)
        )
    }
}
