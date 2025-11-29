use super::constants::*;
use super::BitBoard;
use super::Color;
use super::Piece;
use chessire_utils::board::*;
use chessire_utils::moves::*;

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
            moves.push(Move::new_pawn_double_push(
                color,
                Coord::from_tile(source_square),
            ));
        }
    }
    let source_square = Coord::from_tile(source_square);
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
    if let Some(enpassant_target) = bb.state.enpassant {
        // and with the attack tables of the current pawn
        if (BitBoard::new_single_bit(enpassant_target)
            & bb.attack_tables.pawn_attacks[side][source_square.to_usize()])
        .get()
            != 0
        {
            //add enpassant move
            let m = Move::new(
                source_square,
                Coord::from_tile(enpassant_target),
                Pawn(color),
                None,
            )
            .capture(true)
            .enpassant(true)
            .castling(false)
            .double_push(false);

            moves.push(m);
        }
    }
    // time to consider attacks
    // get valid attacks where there's an enemy piece
    let mut attacks = bb.attack_tables.pawn_attacks[side][source_square.to_usize()]
        & bb.state.occupancies[color.opponent() as usize];

    // loop over all set bits
    while let Some(target_square) = attacks.get_lsb() {
        let target_square = Coord::from_tile(target_square);
        // promotion attacks
        if is_promotion {
            let mut m = Move::new(
                source_square,
                target_square,
                Pawn(color),
                Some(Queen(color)),
            )
            .capture(true);
            moves.push(m);
            m.set_promotion(Some(Rook(color)));
            moves.push(m);
            m.set_promotion(Some(Bishop(color)));
            moves.push(m);
            m.set_promotion(Some(Knight(color)));
            moves.push(m);
        } else {
            // non promotion attacks
            moves.push(Move::new(source_square, target_square, Pawn(color), None).capture(true));
        }
        attacks.reset_bit(target_square.to_usize());
    }
    moves
}

pub fn get_knight_moves(bb: &BitBoardEngine, source_square: usize, color: Color) -> Vec<Move> {
    let free_squares = bb.attack_tables.knight_attacks[source_square] & !bb.state.occupancies[BOTH];
    let enemy_squares = bb.attack_tables.knight_attacks[source_square]
        & bb.state.occupancies[color.opponent() as usize];

    enemy_squares
        .into_iter()
        .map(|target| {
            Move::new_knight_move(
                Coord::from_tile(source_square),
                Coord::from_tile(target),
                color,
                true,
            )
        })
        .chain(free_squares.into_iter().map(|target| {
            Move::new_knight_move(
                Coord::from_tile(source_square),
                Coord::from_tile(target),
                color,
                false,
            )
        }))
        .collect()
}

use super::attacks::*;

//TODO: this can probably be refactor into something better
pub fn get_bishop_moves(bb: &BitBoardEngine, source_square: usize, color: Color) -> Vec<Move> {
    let bishop_attacks =
        get_bishop_attack(&bb.attack_tables, source_square, bb.state.occupancies[BOTH]);

    let free_squares = bishop_attacks & !bb.state.occupancies[BOTH];
    let enemy_squares = bishop_attacks & bb.state.occupancies[color.opponent() as usize];

    fill_movelist(
        Bishop(color),
        free_squares,
        enemy_squares,
        source_square,
        color,
    )
}
pub fn get_rook_moves(bb: &BitBoardEngine, source_square: usize, color: Color) -> Vec<Move> {
    let attacks = get_rook_attack(&bb.attack_tables, source_square, bb.state.occupancies[BOTH]);

    let free_squares = attacks & !bb.state.occupancies[BOTH];
    let enemy_squares = attacks & bb.state.occupancies[color.opponent() as usize];

    fill_movelist(
        Rook(color),
        free_squares,
        enemy_squares,
        source_square,
        color,
    )
}
pub fn get_queen_moves(bb: &BitBoardEngine, source_square: usize, color: Color) -> Vec<Move> {
    let attacks = get_queen_attack(&bb.attack_tables, source_square, bb.state.occupancies[BOTH]);

    let free_squares = attacks & !bb.state.occupancies[BOTH];
    let enemy_squares = attacks & bb.state.occupancies[color.opponent() as usize];

    fill_movelist(
        Queen(color),
        free_squares,
        enemy_squares,
        source_square,
        color,
    )
}
pub fn get_king_moves(bb: &BitBoardEngine, source_square: usize, color: Color) -> Vec<Move> {
    use super::*;

    // First regular moves
    let attacks = bb.attack_tables.king_attacks[source_square];

    let free_squares = attacks & !bb.state.occupancies[BOTH];
    let enemy_squares = attacks & bb.state.occupancies[color.opponent() as usize];

    let mut moves = fill_movelist(
        King(color),
        free_squares,
        enemy_squares,
        source_square,
        color,
    );

    if color == White {
        // White castling moves
        if bb.state.castling_rights.white_king_side
            && is_empty(bb, index_from_bitmask(F1))
            && is_empty(bb, index_from_bitmask(G1))
            && !bb.is_square_attacked_by(index_from_bitmask(F1), color.opponent())
            && !bb.is_square_attacked_by(index_from_bitmask(G1), color.opponent())
            // if the king is in check, no castling!
            && !bb.is_square_attacked_by(index_from_bitmask(E1), color.opponent())
        {
            let mov = Move::new_castling(
                Coord::from_tile(index_from_bitmask(E1)),
                Coord::from_tile(index_from_bitmask(G1)),
                color,
            );
            moves.push(mov);
        }
        if bb.state.castling_rights.white_queen_side
            && is_empty(bb, index_from_bitmask(B1))
            && is_empty(bb, index_from_bitmask(C1))
            && is_empty(bb, index_from_bitmask(D1))
//            && !bb.is_square_attacked_by(index_from_bitmask(B1), color.opponent())
            && !bb.is_square_attacked_by(index_from_bitmask(C1), color.opponent())
            && !bb.is_square_attacked_by(index_from_bitmask(D1), color.opponent()) 
            // if the king is in check, no castling!
            && !bb.is_square_attacked_by(index_from_bitmask(E1), color.opponent())
        {
            let mov = Move::new_castling(
                Coord::from_tile(index_from_bitmask(E1)),
                Coord::from_tile(index_from_bitmask(C1)),
                color,
            );
            moves.push(mov);
        }
    } else {
        if bb.state.castling_rights.black_king_side
            && is_empty(bb, index_from_bitmask(F8))
            && is_empty(bb, index_from_bitmask(G8))
            && !bb.is_square_attacked_by(index_from_bitmask(F8), color.opponent())
            && !bb.is_square_attacked_by(index_from_bitmask(G8), color.opponent())
            // if the king is in check, no castling!
            && !bb.is_square_attacked_by(index_from_bitmask(E8), color.opponent())
 
        {
            //CHECK: manual move from e8 to g8
            let mov = Move::new_castling(
                Coord::from_tile(index_from_bitmask(E8)),
                Coord::from_tile(index_from_bitmask(G8)),
                color,
            );
            moves.push(mov);
        }
        if bb.state.castling_rights.black_queen_side
            && is_empty(bb, index_from_bitmask(B8))
            && is_empty(bb, index_from_bitmask(C8))
            && is_empty(bb, index_from_bitmask(D8))
//            && !bb.is_square_attacked_by(index_from_bitmask(B8), color.opponent())
            && !bb.is_square_attacked_by(index_from_bitmask(C8), color.opponent())
            && !bb.is_square_attacked_by(index_from_bitmask(D8), color.opponent())
            // if the king is in check, no castling!
            && !bb.is_square_attacked_by(index_from_bitmask(E8), color.opponent())
 
        {
            let mov = Move::new_castling(
                Coord::from_tile(index_from_bitmask(E8)),
                Coord::from_tile(index_from_bitmask(C8)),
                color,
            );
            moves.push(mov);
        }
    }

    moves
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
    !bb.state.occupancies[BOTH].get_bit(square)
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
                Coord::from_tile(source_square),
                Coord::from_tile(target),
                piece,
                None,
            )
            .capture(true)
            .enpassant(false)
            .castling(false)
            .double_push(false)
        })
        .chain(free_squares.into_iter().map(|target| {
            Move::new(
                Coord::from_tile(source_square),
                Coord::from_tile(target),
                piece,
                None,
            )
            .capture(false)
            .double_push(false)
            .castling(false)
            .enpassant(false)
        }))
        .collect()
}
