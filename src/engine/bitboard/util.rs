use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
        let t = self.get();
        if t != 0 {
            Some(BitBoard::new(((t & t.overflowing_neg().0) - 1) as u64).popcount())
        } else {
            None
        }
    }
}

use std::ops::{BitAnd, BitOr, Not};

impl BitAnd for BitBoard {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard::new(self.get() & rhs.get())
    }
}

impl BitOr for BitBoard {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard::new(self.get() | rhs.get())
    }
}

impl Not for BitBoard {
    type Output = Self;
    fn not(self) -> Self::Output {
        BitBoard::new(!self.get())
    }
}

impl Iterator for BitBoard {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let lsb = self.get_lsb()?;
        self.reset_bit(lsb);
        Some(lsb)
    }
}

use super::Color;
use std::ops::Index;

impl Index<Color> for [BitBoard] {
    type Output = BitBoard;

    fn index(&self, color: Color) -> &Self::Output {
        &self[color as usize]
    }
}

use termion::color;

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
pub fn index_from_bitmask(num: u64) -> usize {
    let mut count = 0;
    while (num & (1 << count) == 0) && (count < 64) {
        count += 1;
    }
    count
}
#[inline]
pub fn get_index(file: usize, rank: usize) -> usize {
    file + rank * 8
}

#[inline]
pub fn get_file_rank(index: usize) -> (usize, usize) {
    (index % 8, index / 8)
}
