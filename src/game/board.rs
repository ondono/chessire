/***
*** Provides a "board" struct used to represent the state of the board.
*** This state is not designed to be memory efficient or practical for search and evaluation
*** It's only purpose is to simplify the design of interfaces for GUIs or other programs

*** It also provides the main representations of the board for debugging.
***/

use super::color::Color::{self, Black, White};
use super::piece::Piece;
use std::fmt;

/*********
** Tile **
**********/

pub type Tile = usize;

#[derive(Clone, Copy, Debug)]
pub struct Coord {
    file: usize,
    rank: usize,
}

#[inline(always)]
fn to_char(num: usize) -> char {
    (num as u8 + b'A').to_ascii_uppercase() as char
}

impl Coord {
    pub fn new(file: usize, rank: usize) -> Self {
        Self { file, rank }
    }
    pub fn from_tile(t: Tile) -> Self {
        Self {
            file: t % 8,
            rank: t / 8,
        }
    }
    pub fn to_usize(self) -> usize {
        self.file + self.rank * 8
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", to_char(self.file), self.rank + 1)
    }
}

/***************
** Selections **
****************/

#[derive(Clone, Copy, Debug)]
pub struct SelectionColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl SelectionColor {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

#[derive(Clone, Debug)]
pub struct Selection {
    squares: Vec<Tile>,
    color: SelectionColor,
}

impl fmt::Display for Selection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Selection {:?}:",
            termion::color::Rgb(self.color.red, self.color.green, self.color.blue)
        )?;
        for sq in &self.squares {
            write!(f, "{}", sq)?;
        }
        Ok(())
    }
}

impl Selection {
    pub fn new(squares: Vec<Tile>, color: SelectionColor) -> Self {
        Self { squares, color }
    }
}

/**********
** Board **
***********/

#[derive(Clone, Debug)]
pub struct Board {
    pub squares: [Option<Piece>; 64],
    pub selections: Vec<Selection>,
    pub perspective: Color,
}

use core::ops::{Index, IndexMut};

impl Index<Tile> for Board {
    type Output = Option<Piece>;

    fn index(&self, t: Tile) -> &Self::Output {
        &self.squares[t]
    }
}

impl IndexMut<Tile> for Board {
    fn index_mut(&mut self, t: Tile) -> &mut Self::Output {
        &mut self.squares[t]
    }
}

const DEFAULT_PIECE_PLACEMENT: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

impl Default for Board {
    fn default() -> Self {
        let mut ret = Self {
            squares: [None; 64],
            selections: vec![],
            perspective: White,
        };

        ret.set_position_from_fen(DEFAULT_PIECE_PLACEMENT);
        ret
    }
}

impl Index<Coord> for Board {
    type Output = Option<Piece>;

    fn index(&self, t: Coord) -> &Self::Output {
        &self.squares[t.to_usize()]
    }
}
impl IndexMut<Coord> for Board {
    fn index_mut(&mut self, t: Coord) -> &mut Self::Output {
        &mut self.squares[t.to_usize()]
    }
}

impl Board {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn set_position_from_fen(&mut self, piece_placement: &str) -> Result<(), ()> {
        let blocks = piece_placement.split('/');

        if blocks.clone().count() != 8 {
            return Err(());
        }

        for (i, rank_string) in blocks.enumerate() {
            let rank = 7 - i;
            let mut file = 0;

            for c in rank_string.chars() {
                if c.is_ascii_digit() {
                    let space = c.to_digit(10).unwrap() as usize;
                    file += space;
                } else {
                    // set a piece
                    let coord = Coord::new(file, rank);
                    self[coord] = Piece::new_from_fen_char(c);
                    file += 1;
                }
            }
        }
        Ok(())
    }
    pub fn clear(&mut self) {
        self.squares = [None; 64];
        self.selections.clear();
        self.perspective = White;
    }
    pub fn add_selection(&mut self, sel: Selection) {
        self.selections.push(sel);
    }
    pub fn clear_selections(&mut self) {
        self.selections.clear();
    }
    //    pub fn select(&mut self, index: usize) {
    //        self.selected = index;
    //    }
    //    pub fn unselect(&mut self) {
    //        self.selected = 0x78;
    //    }
    //    pub fn add_to_highlighted(&mut self, index: usize) {
    //        self.highlighted.push(index);
    //        self.highlighted.sort_unstable()
    //    }
    //    pub fn remove_from_highlighted(&mut self, index: usize) {
    //        let res = self.highlighted.binary_search(&index);
    //        if let Ok(pos) = res {
    //            self.highlighted.remove(pos);
    //        }
    //    }
    //    pub fn clear_highlighted(&mut self) {
    //        self.highlighted.clear();
    //    }
    //    pub fn add_piece_to(&mut self, position: usize, piece: Piece) {
    //        self.squares[position] = Some(piece);
    //    }
    //    pub fn remove_piece(&mut self, position: usize) {
    //        self.squares[position] = None;
    //    }
    //    // this is not efficient and should only be used when setting a new position
    //    pub fn get_piece_list(&self, color: Color) -> Vec<(usize, Piece)> {
    //        let mut piecelist = vec![];
    //        for (i, sq) in self.squares.iter().enumerate() {
    //            if let Some(p) = sq {
    //                if p.get_color() == color {
    //                    piecelist.push((i, *p))
    //                }
    //            }
    //        }
    //        piecelist
    //    }
    //    pub fn set_perspective(&mut self, color: Color) {
    //        self.perspective = color;
    //    }
}

use termion::color;

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
                let coord = Coord::new(file, rank);
                let sq = self[coord];
                let mut tile_color =
                    // this sets the tile white or black
                    if (file + rank) & 0x01 == 1 {
                        color::Rgb(200, 200, 200)
                    } else {
                        color::Rgb(100, 100, 100)
                    };

                for sel in &self.selections {
                    for selected_square in &sel.squares {
                        if coord.to_usize() == *selected_square {
                            if (file + rank) & 0x01 == 1 {
                                tile_color = color::Rgb(
                                    ((sel.color.red as u16 + 200) / 2) as u8,
                                    ((sel.color.green as u16 + 200) / 2) as u8,
                                    ((sel.color.blue as u16 + 200) / 2) as u8,
                                );
                            } else {
                                tile_color = color::Rgb(
                                    ((sel.color.red as u16 + 100) / 2) as u8,
                                    ((sel.color.green as u16 + 100) / 2) as u8,
                                    ((sel.color.blue as u16 + 100) / 2) as u8,
                                );
                            }
                        }
                    }
                }

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
