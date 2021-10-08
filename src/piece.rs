use std::ops::{Index, IndexMut};

use termion::color;

/****************************/
/****       COLOR        ****/
/****************************/

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    White = 0,
    Black = 1,
}

impl core::fmt::Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        let (letter, col) = if *self == White {
            ("W", color::Rgb(255, 255, 255))
        } else {
            ("B", color::Rgb(0, 0, 0))
        };
        write!(f, "{}{}{}", color::Fg(col), letter, color::Fg(color::Reset))
    }
}

use crate::bitboard::*;
//TODO: find a fix for this!
impl Index<Color> for [BitBoard] {
    type Output = BitBoard;

    fn index(&self, c: Color) -> &Self::Output {
        if c == White {
            &self[White]
        } else {
            &self[Black]
        }
    }
}
impl IndexMut<Color> for [BitBoard] {
    fn index_mut(&mut self, c: Color) -> &mut Self::Output {
        if c == White {
            &mut self[White]
        } else {
            &mut self[Black]
        }
    }
}

/****************************/
/****      PIECE         ****/
/****************************/

const VALUE_KING: i32 = 500;
const VALUE_QUEEN: i32 = 90;
const VALUE_ROOK: i32 = 50;
const VALUE_BISHOP: i32 = 30;
const VALUE_KNIGHT: i32 = 30;
const VALUE_PAWN: i32 = 10;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Piece {
    King(Color),
    Queen(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
    Pawn(Color),
}

use Color::*;
use Piece::*;

impl core::fmt::Display for Piece {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        //let p = self.get_letter(); // enable this one if unicode gives trouble
        let color = if self.get_color() == White {
            color::Rgb(0, 0, 0)
        } else {
            color::Rgb(255, 255, 255)
        };
        write!(
            f,
            "{}{}{}",
            color::Fg(color),
            self.get_symbol(),
            color::Fg(color::Reset)
        )
    }
}

impl Piece {
    pub fn get_symbol(&self) -> &str {
        match self {
            //            King(White) => "♔",
            //            King(Black) => "♚",
            //            Queen(White) => "♕",
            //            Queen(Black) => "♛",
            //            Pawn(White) => "♙",
            //            Pawn(Black) => "♟︎",
            //            Knight(White) => "♘",
            //            Knight(Black) => "♞",
            //            Bishop(White) => "♗",
            //            Bishop(Black) => "♝",
            //            Rook(White) => "♖",
            //            Rook(Black) => "♜",
            King(_c) => "♚",
            Queen(_c) => "♛",
            Pawn(_c) => "♟",
            Knight(_c) => "♞",
            Bishop(_c) => "♝",
            Rook(_c) => "♜",
        }
    }

    pub fn get_unicode(&self) -> &str {
        match self {
            King(White) => "♔",
            King(Black) => "♚",
            Queen(White) => "♕",
            Queen(Black) => "♛",
            Pawn(White) => "♙",
            Pawn(Black) => "♟",
            Knight(White) => "♘",
            Knight(Black) => "♞",
            Bishop(White) => "♗",
            Bishop(Black) => "♝",
            Rook(White) => "♖",
            Rook(Black) => "♜",
        }
    }
    pub fn get_letter(&self) -> &str {
        match self {
            King(_) => "K",
            Queen(_) => "Q",
            Rook(_) => "R",
            Knight(_) => "N",
            Bishop(_) => "B",
            Pawn(_) => "P",
        }
    }
    pub fn get_name(&self) -> &'static str {
        match self {
            King(_) => "king",
            Queen(_) => "queen",
            Rook(_) => "rook",
            Bishop(_) => "bishop",
            Knight(_) => "knight",
            Pawn(_) => "pawn",
        }
    }
    pub fn get_color(&self) -> Color {
        match self {
            King(c) | Queen(c) | Rook(c) | Bishop(c) | Knight(c) | Pawn(c) => *c,
        }
    }

    pub fn get_piece_value(&self) -> i32 {
        match self {
            King(Black) => -VALUE_KING,
            King(White) => VALUE_KING,
            Queen(White) => VALUE_QUEEN,
            Queen(Black) => -VALUE_QUEEN,
            Rook(White) => VALUE_ROOK,
            Rook(Black) => -VALUE_ROOK,
            Bishop(White) => VALUE_BISHOP,
            Bishop(Black) => -VALUE_BISHOP,
            Knight(White) => VALUE_KNIGHT,
            Knight(Black) => -VALUE_KNIGHT,
            Pawn(White) => VALUE_PAWN,
            Pawn(Black) => -VALUE_PAWN,
        }
    }

    pub fn new_from_fen_char(c: char) -> Option<Piece> {
        match c {
            'P' => Some(Self::Pawn(White)),
            'N' => Some(Self::Knight(White)),
            'B' => Some(Self::Bishop(White)),
            'R' => Some(Self::Rook(White)),
            'Q' => Some(Self::Queen(White)),
            'K' => Some(Self::King(White)),
            'p' => Some(Self::Pawn(Black)),
            'n' => Some(Self::Knight(Black)),
            'b' => Some(Self::Bishop(Black)),
            'r' => Some(Self::Rook(Black)),
            'q' => Some(Self::Queen(Black)),
            'k' => Some(Self::King(Black)),
            _ => None,
        }
    }
    pub fn is_sliding_piece(&self) -> bool {
        matches!(self, Queen(_) | Bishop(_) | Rook(_))
    }
}
