//use super::Color;
use super::color::Color;
use termion::color;

/****************************/
/****      PIECE         ****/
/****************************/

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
            color::Rgb(255, 255, 255)
        } else {
            color::Rgb(0, 0, 0)
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
