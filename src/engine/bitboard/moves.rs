use super::BitBoard;
use super::Color;
use super::Piece;
use std::fmt;

// MOVE ENCODING DESCRIPTION
// source square    [0..63] bits 0-5
// target square    [0..63] bits 6-11
// piece            [0..7]  bits 12-15
// promoted piece   [0..7]  bits 16-19
// capture flag             bit 20
// double push flag         bit 21
// enpassant flag           bit 22
// castling flag            bit 23

const SOURCE_MASK: u32 = 0x00003F;
const TARGET_MASK: u32 = 0x000FC0;
const PIECE_MASK: u32 = 0x00F000;
const PROMOTED_MASK: u32 = 0x0F0000;
const CAPTURE_FLAG: u32 = 0x100000;
const DOUBLE_PUSH_FLAG: u32 = 0x200000;
const ENPASSANT_FLAG: u32 = 0x400000;
const CASTLING_FLAG: u32 = 0x800000;

// piece ENCODING
const PIECE_KING: u8 = 0x00;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Move {
    source: u8,
    target: u8,
    piece: Piece,
    promoted_piece: Option<Piece>,
    capture: bool,
    double_push: bool,
    enpassant: bool,
    castling: bool,
}

impl Move {
    pub fn new(
        source: u8,
        target: u8,
        piece: Piece,
        promoted_piece: Option<Piece>,
        capture: bool,
        double_push: bool,
        enpassant: bool,
        castling: bool,
    ) -> Self {
        Self {
            source,
            target,
            piece,
            promoted_piece,
            capture,
            double_push,
            enpassant,
            castling,
        }
    }
}
