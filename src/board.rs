use crate::constants::*;
use crate::piece::*;
use std::fmt;
use termion::color;

#[warn(dead_code)]
#[inline(always)]
pub fn index_by_file_and_rank(rank: usize, file: usize) -> usize {
    16 * rank + file
}

#[inline(always)]
pub fn file_by_index(index: usize) -> usize {
    index & 0x07
}

#[inline(always)]
pub fn file_char_by_index(index: usize) -> char {
    ((index & 0x07) as u8 + b'A').to_ascii_uppercase() as char
}

#[inline(always)]
pub fn rank_by_index(index: usize) -> usize {
    index >> 4
}

#[inline(always)]
pub fn is_off_board(index: usize) -> bool {
    index & 0x88 != 0
}

pub fn validated_position(index: usize) -> Option<usize> {
    if is_off_board(index) {
        None
    } else {
        Some(index)
    }
}

pub fn get_name_from_index(index: usize) -> String {
    let mut s = String::from("");
    s.push(file_char_by_index(index));
    s.push_str(&(rank_by_index(index) + 1).to_string());
    s
}

const PIECE_PLACEMENT: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

#[derive(Clone)]
pub struct Board {
    pub squares: [Option<Piece>; 128],
    selected: usize,
    highlighted: Vec<usize>,
    perspective: Color,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            squares: [None; 128],
            selected: 0x78,
            highlighted: vec![],
            perspective: Color::White,
        }
    }
}

impl Board {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_position_from_fen(&mut self, piece_placement: &str) {
        for (i, rnk) in piece_placement.split('/').enumerate() {
            let rank = 7 - i;
            let mut file = 0;

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
    pub fn clear(&mut self) {
        self.squares = [None; 128];
        self.selected = 0x78;
        self.highlighted.clear();
        self.perspective = Color::White;
    }
    pub fn set_start_position(&mut self) {
        self.set_position_from_fen(PIECE_PLACEMENT);
    }
    pub fn select(&mut self, index: usize) {
        self.selected = index;
    }
    pub fn unselect(&mut self) {
        self.selected = 0x78;
    }
    pub fn add_to_highlighted(&mut self, index: usize) {
        self.highlighted.push(index);
        self.highlighted.sort_unstable()
    }
    pub fn remove_from_highlighted(&mut self, index: usize) {
        let res = self.highlighted.binary_search(&index);
        if let Ok(pos) = res {
            self.highlighted.remove(pos);
        }
    }
    pub fn clear_highlighted(&mut self) {
        self.highlighted.clear();
    }
    pub fn add_piece_to(&mut self, position: usize, piece: Piece) {
        self.squares[position] = Some(piece);
    }
    pub fn remove_piece(&mut self, position: usize) {
        self.squares[position] = None;
    }
    // this is not efficient and should only be used when setting a new position
    pub fn get_piece_list(&self, color: Color) -> Vec<(usize, Piece)> {
        let mut piecelist = vec![];
        for (i, sq) in self.squares.iter().enumerate() {
            if let Some(p) = sq {
                if p.get_color() == color {
                    piecelist.push((i, *p))
                }
            }
        }
        piecelist
    }
    pub fn set_perspective(&mut self, color: Color) {
        self.perspective = color;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // print the board from white's perspective
        let rank_range = if self.perspective == Color::White {
            (0..8).rev().collect::<Vec<usize>>()
        } else {
            (0..8).collect::<Vec<usize>>()
        };
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
                let index = index_by_file_and_rank(rank, file);
                let sq = self.squares[index];
                let tile_color = if index == self.selected {
                    color::Rgb(200, 200, 0)
                } else if self.highlighted.contains(&index) {
                    color::Rgb(200, 0, 0)
                } else {
                    // this sets the tile white or black
                    if (file + rank) & 0x01 == 1 {
                        color::Rgb(200, 200, 200)
                    } else {
                        color::Rgb(100, 100, 100)
                    }
                };
                match sq {
                    Some(piece) => write!(f, "{} {} ", color::Bg(tile_color), piece)?,
                    _ => write!(f, "{}   ", color::Bg(tile_color))?,
                };
            }
            //end of line
            write!(f, "{}\r\n", color::Bg(color::Reset))?;
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
