use super::constants::*;
use super::BitBoard;
use super::Color;
use super::Piece;
use std::fmt;
use std::fmt::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Move {
    source: usize,
    target: usize,
    piece: Piece,
    promoted_piece: Option<Piece>,
    capture: bool,
    double_push: bool,
    enpassant: bool,
    castling: bool,
}

pub fn print_movelist(movelist: &[Move]) {
    println!("move\tpiece\tprom.\tcapture\tdouble\tenpass.\tcastling\n\r");
    for m in movelist {
        println!("{}", m);
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.promoted_piece.is_some() {
            write!(
                f,
                "{}{}\t{}\t{}\t{}\t{}\t{}\t{}",
                SQUARE_NAMES[self.source],
                SQUARE_NAMES[self.target],
                self.piece,
                self.promoted_piece.unwrap(),
                self.capture,
                self.double_push,
                self.enpassant,
                self.castling
            )
        } else {
            write!(
                f,
                "{}{}\t{}\tNone\t{}\t{}\t{}\t{}",
                SQUARE_NAMES[self.source],
                SQUARE_NAMES[self.target],
                self.piece,
                self.capture,
                self.double_push,
                self.enpassant,
                self.castling
            )
        }
    }
}
#[allow(clippy::too_many_arguments)]
impl Move {
    pub fn new(
        source: usize,
        target: usize,
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
    pub fn set_promoted(&mut self, prom: Option<Piece>) {
        self.promoted_piece = prom;
    }
    pub fn new_pawn_double_push(color: Color, source: usize) -> Self {
        Self::new(
            source,
            if color == White {
                source + 16
            } else {
                source - 16
            },
            Pawn(color),
            None,
            false,
            true,
            false,
            false,
        )
    }
    pub fn new_pawn_push(color: Color, source: usize) -> Self {
        Self::new(
            source,
            if color == White {
                source + 8
            } else {
                source - 8
            },
            Pawn(color),
            None,
            false,
            false,
            false,
            false,
        )
    }
    pub fn new_promotion(color: Color, source: usize, piece: Piece) -> Self {
        Move::new(
            source,
            if color == White {
                source + 8
            } else {
                source - 8
            },
            Pawn(color),
            Some(piece),
            false,
            false,
            false,
            false,
        )
    }
    pub fn new_knight_move(source: usize, target: usize, color: Color, capture: bool) -> Self {
        Move::new(
            source,
            target,
            Knight(color),
            None,
            capture,
            false,
            false,
            false,
        )
    }
    pub fn new_bishop_move(source: usize, target: usize, color: Color, capture: bool) -> Self {
        Move::new(
            source,
            target,
            Bishop(color),
            None,
            capture,
            false,
            false,
            false,
        )
    }
    pub fn new_rook_move(source: usize, target: usize, color: Color, capture: bool) -> Self {
        Move::new(
            source,
            target,
            Rook(color),
            None,
            capture,
            false,
            false,
            false,
        )
    }
}

use super::BitBoardEngine;
use super::Color::*;
use super::Piece::*;

pub fn get_pawn_moves(bb: &BitBoardEngine, source_square: usize, color: Color) -> Vec<Move> {
    let mut moves = vec![];

    let is_start = is_start_rank(source_square, color);
    let is_promotion = is_promotion_rank(source_square, color);
    let single_push_target = get_single_push_target(source_square, color);

    let side = color as usize;

    //*** QUIET MOVES ***//

    // double push first
    if is_start {
        let double_push_target = get_double_push_target(source_square, color);
        // if both squares are empty
        if is_empty(bb, single_push_target) && is_empty(bb, double_push_target) {
            moves.push(Move::new_pawn_double_push(color, source_square));
        }
    }
    // promotions
    // If we are on the last rank and the next square is empty
    if is_promotion && is_empty(bb, single_push_target) {
        moves.push(Move::new_promotion(color, source_square, Queen(color)));
        moves.push(Move::new_promotion(color, source_square, Rook(color)));
        moves.push(Move::new_promotion(color, source_square, Bishop(color)));
        moves.push(Move::new_promotion(color, source_square, Knight(color)));
    }

    // single push
    // if the next square is empty, the promotion rank should be ignored to avoid dupes
    if !is_promotion && is_empty(bb, single_push_target) {
        moves.push(Move::new_pawn_push(color, source_square));
    }
    //*** CAPTURES ***//

    //TODO: en passant!
    if let Some(_enpassant_target) = bb.enpassant {
        println!("enpassant!");
    }
    // time to consider attacks
    // get valid attacks where there's an enemy piece
    let mut attacks = bb.attack_tables.pawn_attacks[side][source_square]
        & bb.occupancies[color.opponent() as usize];

    // loop over all set bits
    while let Some(target_square) = attacks.get_lsb() {
        // promotion attacks
        if is_promotion {
            let mut m = Move::new(
                source_square,
                target_square,
                Pawn(color),
                Some(Queen(color)),
                true,
                false,
                false,
                false,
            );
            moves.push(m);
            m.promoted_piece = Some(Rook(color));
            moves.push(m);
            m.promoted_piece = Some(Bishop(color));
            moves.push(m);
            m.promoted_piece = Some(Knight(color));
            moves.push(m);
        } else {
            // non promotion attacks
            moves.push(Move::new(
                source_square,
                target_square,
                Pawn(color),
                None,
                true,
                false,
                false,
                false,
            ));
        }
        attacks.reset_bit(target_square);
    }
    moves
}

pub fn get_knight_moves(bb: &BitBoardEngine, source_square: usize, color: Color) -> Vec<Move> {
    let free_squares = bb.attack_tables.knight_attacks[source_square] & !bb.occupancies[BOTH];
    let enemy_squares =
        bb.attack_tables.knight_attacks[source_square] & bb.occupancies[color.opponent() as usize];

    enemy_squares
        .into_iter()
        .map(|target| Move::new_knight_move(source_square, target, color, true))
        .chain(
            free_squares
                .into_iter()
                .map(|target| Move::new_knight_move(source_square, target, color, false)),
        )
        .collect()
}

use super::attacks::*;

//TODO: this can probably be refactor into something better
pub fn get_bishop_moves(bb: &BitBoardEngine, source_square: usize, color: Color) -> Vec<Move> {
    let bishop_attacks = get_bishop_attack(&bb.attack_tables, source_square, bb.occupancies[BOTH]);

    let free_squares = bishop_attacks & !bb.occupancies[BOTH];
    let enemy_squares = bishop_attacks & bb.occupancies[color.opponent() as usize];

    fill_movelist(
        Bishop(color),
        free_squares,
        enemy_squares,
        source_square,
        color,
    )
}
pub fn get_rook_moves(bb: &BitBoardEngine, source_square: usize, color: Color) -> Vec<Move> {
    let attacks = get_rook_attack(&bb.attack_tables, source_square, bb.occupancies[BOTH]);

    let free_squares = attacks & !bb.occupancies[BOTH];
    let enemy_squares = attacks & bb.occupancies[color.opponent() as usize];

    fill_movelist(
        Rook(color),
        free_squares,
        enemy_squares,
        source_square,
        color,
    )
}
pub fn get_queen_moves(bb: &BitBoardEngine, source_square: usize, color: Color) -> Vec<Move> {
    let attacks = get_queen_attack(&bb.attack_tables, source_square, bb.occupancies[BOTH]);

    let free_squares = attacks & !bb.occupancies[BOTH];
    let enemy_squares = attacks & bb.occupancies[color.opponent() as usize];

    fill_movelist(
        Queen(White),
        free_squares,
        enemy_squares,
        source_square,
        color,
    )
}
pub fn get_king_moves(bb: &BitBoardEngine, source_square: usize, color: Color) -> Vec<Move> {
    //TODO: Add castling moves
    use super::index_from_bitmask;
    use super::*;

    if color == White {
        if bb.castling_rights.white_king_side
            && is_empty(bb, index_from_bitmask(F1))
            && is_empty(bb, index_from_bitmask(G1))
            && !bb.is_square_attacked_by(index_from_bitmask(F1), color.opponent())
            && !bb.is_square_attacked_by(index_from_bitmask(G1), color.opponent())
        {
            println!("white king side castling");
        }
        if bb.castling_rights.white_queen_side
            && is_empty(bb, index_from_bitmask(B1))
            && is_empty(bb, index_from_bitmask(C1))
            && is_empty(bb, index_from_bitmask(D1))
            && !bb.is_square_attacked_by(index_from_bitmask(B1), color.opponent())
            && !bb.is_square_attacked_by(index_from_bitmask(C1), color.opponent())
            && !bb.is_square_attacked_by(index_from_bitmask(D1), color.opponent())
        {
            println!("white queen side castling");
        }
    } else {
        if bb.castling_rights.black_king_side {}
        if bb.castling_rights.black_queen_side {}
    }
    // First regular moves
    let attacks = bb.attack_tables.king_attacks[source_square];

    let free_squares = attacks & !bb.occupancies[BOTH];
    let enemy_squares = attacks & bb.occupancies[color.opponent() as usize];

    fill_movelist(
        King(White),
        free_squares,
        enemy_squares,
        source_square,
        color,
    )
}

// helper functions
//
fn is_promotion_rank(square: usize, color: Color) -> bool {
    if color == White {
        (48..56).contains(&square)
    } else {
        (8..16).contains(&square)
    }
}
fn is_start_rank(square: usize, color: Color) -> bool {
    if color == White {
        (8..16).contains(&square)
    } else {
        (48..56).contains(&square)
    }
}
fn get_single_push_target(square: usize, color: Color) -> usize {
    if color == White {
        square + 8
    } else {
        square - 8
    }
}

fn get_double_push_target(square: usize, color: Color) -> usize {
    if color == White {
        square + 16
    } else {
        square - 16
    }
}

fn is_empty(bb: &BitBoardEngine, square: usize) -> bool {
    !bb.occupancies[BOTH].get_bit(square)
}

fn fill_movelist(
    piece: Piece,
    free_squares: BitBoard,
    enemy_squares: BitBoard,
    source_square: usize,
    _color: Color,
) -> Vec<Move> {
    enemy_squares
        .into_iter()
        .map(|target| {
            Move::new(
                source_square,
                target,
                piece,
                None,
                true,
                false,
                false,
                false,
            )
        })
        .chain(free_squares.into_iter().map(|target| {
            Move::new(
                source_square,
                target,
                piece,
                None,
                false,
                false,
                false,
                false,
            )
        }))
        .collect()
}
