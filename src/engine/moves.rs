use crate::bitboard::*;
use crate::board::*;
use crate::piece::Piece::*;
use crate::piece::*;
use crate::piece::*;
use crate::*;
use std::fmt;

/****************************/
/****   ADJACENT SQUARES ****/
/****************************/

// First, let's ease the algebra
// let's define the names from white's perspective
// rank 1 = BOTTOM
// rank 8 = TOP
// file A = RIGHT
// file H = LEFT

const VERTICAL_MOVE: usize = 16;
const HORIZONTAL_MOVE: usize = 1;

#[inline(always)]
pub fn next_up(index: usize) -> Option<usize> {
    let x = index + VERTICAL_MOVE;
    validated_position(x)
}

#[inline(always)]
pub fn next_down(index: usize) -> Option<usize> {
    let x = index.checked_sub(VERTICAL_MOVE).unwrap_or(0xff);
    validated_position(x)
}

#[inline(always)]
pub fn next_right(index: usize) -> Option<usize> {
    let x = index.checked_sub(HORIZONTAL_MOVE).unwrap_or(0xff);
    validated_position(x)
}

#[inline(always)]
pub fn next_left(index: usize) -> Option<usize> {
    let x = index + HORIZONTAL_MOVE;
    validated_position(x)
}

#[inline(always)]
pub fn next_up_right(index: usize) -> Option<usize> {
    // we can skip the sub check, because VERTICAL - HORIZONTAL > 0
    let x = index + VERTICAL_MOVE - HORIZONTAL_MOVE;
    validated_position(x)
}

#[inline(always)]
pub fn next_up_left(index: usize) -> Option<usize> {
    let x = index + VERTICAL_MOVE + HORIZONTAL_MOVE;
    validated_position(x)
}
#[inline(always)]
pub fn next_down_right(index: usize) -> Option<usize> {
    let x = index
        .checked_sub(HORIZONTAL_MOVE + VERTICAL_MOVE)
        .unwrap_or(0xff);
    validated_position(x)
}

#[inline(always)]
pub fn next_down_left(index: usize) -> Option<usize> {
    let x = index
        .checked_sub(VERTICAL_MOVE - HORIZONTAL_MOVE)
        .unwrap_or(0xff);
    validated_position(x)
}

/****************************/
/****       MOVES        ****/
/****************************/

// MOVE: For now just keep track of from-to.
// TODO: Add more stuff here for evaluation and search
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    // true if it's a castling move
    pub is_castling: bool,
    // true if it's a pawn push (2 squares)
    pub is_pawn_push: bool,
    // true if it's enpassant move
    pub is_enpassant: bool,
    // true if we are capturing an enemy piece
    pub is_capture: bool,
    // true if we can threat anything in the "to" square
    // mainly to differenciate pawn pushes from pawn captures
    pub is_threat: bool,
    //
    pub promotion: Option<Piece>,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            get_name_from_index(self.from).to_ascii_lowercase(),
            get_name_from_index(self.to).to_ascii_lowercase()
        )?;
        if self.promotion.is_some() {
            write!(
                f,
                "{}",
                self.promotion.unwrap().get_letter().to_ascii_lowercase()
            )?;
        }
        Ok(())
    }
}

impl Move {
    pub fn new(from: usize, to: usize) -> Self {
        Self {
            from,
            to,
            is_castling: false,
            is_pawn_push: false,
            is_enpassant: false,
            is_capture: false,
            is_threat: true,
            promotion: None,
        }
    }
    pub fn new_ep(from: usize, to: usize) -> Self {
        Self {
            from,
            to,
            is_castling: false,
            is_pawn_push: false,
            is_enpassant: true,
            is_capture: true,
            is_threat: true,
            promotion: None,
        }
    }
    pub fn get_from(&self) -> usize {
        self.from
    }

    pub fn get_to(&self) -> usize {
        self.to
    }
}

pub fn get_piece_pseudolegal_moves(mut g: ChessGame, (i, p): (usize, Piece)) -> Vec<Move> {
    let color = p.get_color();
    let move_gen = MoveGenerator::new();
    let mut moves = vec![];
    if color == g.side_to_move {
        moves = match p {
            King(color) => move_gen.get_king_pseudolegal_moves(
                i,
                &g.board,
                &move_gen.attack[color as usize],
                g.castling_rights,
                color,
            ),
            Queen(color) => move_gen.get_queen_pseudolegal_moves(i, &g.board, color),
            Rook(color) => move_gen.get_rook_pseudolegal_moves(i, &g.board, color),
            Bishop(color) => move_gen.get_bishop_pseudolegal_moves(i, &g.board, color),
            Knight(color) => move_gen.get_knight_pseudolegal_moves(i, &g.board, color),
            Pawn(color) => move_gen.get_pawn_pseudolegal_moves(
                i,
                &g.board,
                color,
                g.enpassant_target_square,
                // we only want the moves
                false,
            ),
        };
    } else {
    }
    moves
        .into_iter()
        .filter(|x| slow_is_legal(&mut g, *x))
        .collect::<Vec<Move>>()
}

pub fn slow_is_legal(g: &mut ChessGame, mov: Move) -> bool {
    let mut legal: bool = true;
    let move_gen = MoveGenerator::new();
    let color = g.side_to_move;
    // make the move in question
    g.make_move(mov);
    // check if the king is in check
    if color == White {
        if move_gen.attack[White as usize].get_value(g.white_king_pos) != 0 {
            legal = false;
        }
    } else {
        if move_gen.attack[Black as usize].get_value(g.black_king_pos) != 0 {
            legal = false;
        }
    }
    g.unmake_move();

    return legal;
}

// a helper struct to contain pregenerated data and persistance
#[derive(Clone)]
pub struct MoveGenerator {
    // some maps to hold the number of squares ahead in every direction
    limit_up: [usize; 128],
    limit_down: [usize; 128],
    limit_right: [usize; 128],
    limit_left: [usize; 128],
    limit_up_left: [usize; 128],
    limit_up_right: [usize; 128],
    limit_down_left: [usize; 128],
    limit_down_right: [usize; 128],

    // holds counters with how many squares are attacked by each side
    // 0: holds the squares White attacks
    // 1: holds the squares Black attacks

    // I'm discarding ATTACK COUNTERS for now. Once we have the bitboards resolved we'll see
    pub attack: [BoardMap; 2],
    // bitboards
    // piece-type bitboards. This bitboards gives us pieces positions
    pub king: [BitBoard; 2],
    pub queen: [BitBoard; 2],
    pub rooks: [BitBoard; 2],
    pub bishops: [BitBoard; 2],
    pub knights: [BitBoard; 2],
    pub pawns: [BitBoard; 2],
    // general use bitboards
    pub all_pieces: BitBoard,
    pub piece_by_color: [BitBoard; 2],

    // precalculated move tables
    // precalculated leaper pieces
    pub pawn_attacks: [[BitBoard; 64]; 2],
    pub knight_attacks: [BitBoard; 64],
    pub king_attacks: [BitBoard; 64],
}

impl MoveGenerator {
    pub fn new() -> Self {
        let mut x = Self {
            // decide an ordering an make this a 8 item array
            limit_up: [0; 128],
            limit_down: [0; 128],
            limit_left: [0; 128],
            limit_right: [0; 128],
            limit_up_left: [0; 128],
            limit_up_right: [0; 128],
            limit_down_left: [0; 128],
            limit_down_right: [0; 128],
            // attack maps
            attack: [BoardMap::new(), BoardMap::new()],
            // bitboards
            king: [BitBoard::new(BB_E1), BitBoard::new(BB_E8)],
            queen: [BitBoard::new(BB_D1), BitBoard::new(BB_D8)],
            rooks: [BitBoard::new(BB_A1 | BB_H1), BitBoard::new(BB_A8 | BB_H8)],
            bishops: [BitBoard::new(BB_C1 | BB_F1), BitBoard::new(BB_C8 | BB_F8)],
            knights: [BitBoard::new(BB_B1 | BB_G1), BitBoard::new(BB_B8 | BB_G8)],
            pawns: [
                BitBoard::new(BB_A2 | BB_B2 | BB_C2 | BB_D2 | BB_E2 | BB_F2 | BB_G2 | BB_H2),
                BitBoard::new(BB_A7 | BB_B7 | BB_C7 | BB_D7 | BB_E7 | BB_F7 | BB_G7 | BB_H7),
            ],
            // general bitboards
            all_pieces: BitBoard::new(0),
            piece_by_color: [BitBoard::new(0); 2],
            pawn_attacks: [[BitBoard::new(0); 64]; 2],
            knight_attacks: [BitBoard::new(0); 64],
            king_attacks: [BitBoard::new(0); 64],
        };
        // don't return uninitialized squares please!
        x.init();
        x
    }
    pub fn init(&mut self) {
        for i in 0..120 {
            if !is_off_board(i) {
                // first the 4 cardinals
                self.limit_up[i] = 7 - rank_by_index(i);
                self.limit_down[i] = rank_by_index(i);
                self.limit_left[i] = 7 - file_by_index(i);
                self.limit_right[i] = file_by_index(i);
                //now the diagonals
                self.limit_up_left[i] = usize::min(self.limit_up[i], self.limit_left[i]);
                self.limit_up_right[i] = usize::min(self.limit_up[i], self.limit_right[i]);
                self.limit_down_left[i] = usize::min(self.limit_down[i], self.limit_left[i]);
                self.limit_down_right[i] = usize::min(self.limit_down[i], self.limit_right[i]);
            } else {
                self.limit_up[i] = 0;
                self.limit_down[i] = 0;
                self.limit_left[i] = 0;
                self.limit_right[i] = 0;
            }
        }
        // preload the bitboards with standard position
        self.piece_by_color[0].set(
            self.king[0].get()
                | self.queen[0].get()
                | self.rooks[0].get()
                | self.bishops[0].get()
                | self.knights[0].get()
                | self.pawns[0].get(),
        );
        self.piece_by_color[1].set(
            self.king[1].get()
                | self.queen[1].get()
                | self.rooks[1].get()
                | self.bishops[1].get()
                | self.knights[1].get()
                | self.pawns[1].get(),
        );
        // set the all_pieces bitboard too!
        self.all_pieces
            .set(self.piece_by_color[0].get() | self.piece_by_color[1].get());

        // precalculate the attack tables

        // First calculate leaper pieces
        initialise_pawn_attacks(&mut self.pawn_attacks);
        initialise_knight_attacks(&mut self.knight_attacks);
        initialise_king_attacks(&mut self.king_attacks);

        // Now let's go for the sliders
    }

    pub fn get_limit_up(&self, index: usize) -> usize {
        self.limit_up[index]
    }

    pub fn get_limit_down(&self, index: usize) -> usize {
        self.limit_down[index]
    }

    pub fn get_limit_right(&self, index: usize) -> usize {
        self.limit_right[index]
    }

    pub fn get_limit_left(&self, index: usize) -> usize {
        self.limit_left[index]
    }

    pub fn update_attack_map(
        &mut self,
        board: &Board,
        ally_color: Color,
        piece_list: Vec<(usize, Piece)>,
    ) {
        // for incremental approach to work, we need to keep track of (Blocking, Blocked) pairs,
        // so that we can recompute the effect of friendly pieces clearing blocks over sliding
        // pieces
        //
        // Since this sounds like a PITA, for now we will do a FULL RECOMPUTATION of the attack
        // map.
        // This is clearly not how it is supposed to work, so //TODO!
        if ally_color == White {
            // if we moved a white piece, we need to update blacks map
            self.attack[Black as usize].clear();
        } else {
            self.attack[White as usize].clear();
        }
        for (i, p) in piece_list {
            let moves: Vec<Move> = match p {
                King(_) => self.get_king_pseudolegal_moves(
                    i,
                    &board,
                    &self.attack[ally_color as usize],
                    //TODO: This is a hack!
                    CastlingRights::new(),
                    ally_color,
                ),
                Queen(_) => self.get_queen_pseudolegal_moves(i, &board, ally_color),
                Rook(_) => self.get_rook_pseudolegal_moves(i, &board, ally_color),
                Bishop(_) => self.get_bishop_pseudolegal_moves(i, &board, ally_color),
                Knight(_) => self.get_knight_pseudolegal_moves(i, &board, ally_color),
                Pawn(_) => self.get_pawn_pseudolegal_moves(i, &board, ally_color, None, true),
            };
            for mov in moves {
                if mov.is_threat != true {
                    continue;
                }
                if ally_color == White {
                    // if we moved a white piece, we need to update blacks map
                    self.attack[Black as usize].inc_value(mov.to);
                } else {
                    self.attack[White as usize].inc_value(mov.to);
                }
            }
        }
    }

    pub fn get_king_pseudolegal_moves(
        &self,
        position: usize,
        board: &Board,
        attack: &BoardMap,
        castling_rights: CastlingRights,
        ally_color: Color,
    ) -> Vec<Move> {
        // a king has at most 8 moves
        let mut moves = Vec::with_capacity(8);

        let movements = [
            next_up(position),
            next_up_right(position),
            next_right(position),
            next_down_right(position),
            next_down(position),
            next_down_left(position),
            next_left(position),
            next_up_left(position),
        ];

        for m in movements {
            if m.is_some() {
                let end = m.unwrap();
                let mut mov = Move::new(position, end);
                // check the attack map. we can move only if the counter is 0
                if attack.get_value(end) > 0 {
                    continue;
                }
                // if it's not empty
                if board.squares[end].is_some() {
                    let target_color = board.squares[end].unwrap().get_color();
                    // skip if there's a friendly piece
                    if target_color == ally_color {
                        continue;
                    } else {
                        // if the target square has an enemy piece, it's a capture
                        mov.is_capture = true;
                    }
                }
                // after all guards, add to the list
                moves.push(mov);
            }
        }
        //TODO: Add castling moves
        if ally_color == White {
            //white castling
            if position == E1 {
                if castling_rights.white_king_side {
                    //TODO:check the right squares for threats
                    // for now check that the squares are empty
                    if board.squares[F1].is_none() && board.squares[G1].is_none() {
                        if attack.get_value(F1) == 0 && attack.get_value(G1) == 0 {
                            let mut mov = Move::new(E1, G1);
                            mov.is_castling = true;
                            moves.push(mov);
                        }
                    }
                }
                if castling_rights.white_queen_side {
                    //TODO:check the right squares for threats
                    // for now check that the squares are empty
                    if board.squares[D1].is_none()
                        && board.squares[C1].is_none()
                        && board.squares[B1].is_none()
                    {
                        if attack.get_value(D1) == 0
                            && attack.get_value(C1) == 0
                            && attack.get_value(B1) == 0
                        {
                            let mut mov = Move::new(E1, C1);
                            mov.is_castling = true;
                            moves.push(mov);
                        }
                    }
                }
            }
        } else {
            if position == E8 {
                //Black castling
                if castling_rights.black_king_side {
                    //TODO:check the right squares for threats                    // for now check that the squares are empty
                    if board.squares[F8].is_none() && board.squares[G8].is_none() {
                        if attack.get_value(F8) == 0 && attack.get_value(G8) == 0 {
                            let mut mov = Move::new(E8, G8);
                            mov.is_castling = true;
                            moves.push(mov);
                        }
                    }
                }
                if castling_rights.black_queen_side {
                    //TODO:check the right squares for threats                    // for now check that the squares are empty
                    if board.squares[D8].is_none()
                        && board.squares[C8].is_none()
                        && board.squares[B8].is_none()
                    {
                        if attack.get_value(D8) == 0
                            && attack.get_value(C8) == 0
                            && attack.get_value(B8) == 0
                        {
                            let mut mov = Move::new(E8, C8);
                            mov.is_castling = true;
                            moves.push(mov);
                        }
                    }
                }
            }
        }
        moves
    }
    pub fn get_queen_pseudolegal_moves(
        &self,
        position: usize,
        board: &Board,
        ally_color: Color,
    ) -> Vec<Move> {
        let mut moves = self.get_rook_pseudolegal_moves(position, board, ally_color);
        moves.append(&mut self.get_bishop_pseudolegal_moves(position, board, ally_color));
        moves
    }
    pub fn get_rook_pseudolegal_moves(
        &self,
        position: usize,
        board: &Board,
        ally_color: Color,
    ) -> Vec<Move> {
        // a rook has always 14 pseudolegal moves in an empty board
        let mut moves = Vec::with_capacity(14);

        let movements = [next_up, next_down, next_right, next_left];
        let limits = [
            self.limit_up[position],
            self.limit_down[position],
            self.limit_right[position],
            self.limit_left[position],
        ];
        //  println!(
        //      "RookGen start for {} {}",
        //      Rook(ally_color),
        //      get_name_from_index(position)
        //  );
        for i in 0..4 {
            let movement = movements[i];
            let lim = limits[i];
            if lim > 0 {
                let mut last = movement(position).unwrap();
                for _i in 0..lim {
                    //println!("RookGen, evaluating {}", get_name_from_index(last));
                    let sq = board.squares[last];
                    if let Some(p) = sq {
                        if p.get_color() == ally_color {
                            // println!(
                            //     "Rook Gen, found ally piece on {}",
                            //     get_name_from_index(last)
                            // );
                            // if we get an ally piece, we have no more moves
                            break;
                        } else {
                            let mut mov = Move::new(position, last);
                            mov.is_capture = true;
                            mov.is_threat = true;
                            // if we have an enemy piece, this is our last move
                            moves.push(mov);
                            break;
                        }
                    } else {
                        moves.push(Move::new(position, last));
                        last = movement(last).unwrap_or(0x0);
                    }
                }
            }
        }
        moves
    }

    pub fn get_bishop_pseudolegal_moves(
        &self,
        position: usize,
        board: &Board,
        ally_color: Color,
    ) -> Vec<Move> {
        // a bishop has always 14 pseudolegal moves in an empty board
        let mut moves = Vec::with_capacity(14);
        let movements = [next_up_right, next_down_right, next_up_left, next_down_left];
        let limits = [
            self.limit_up_right[position],
            self.limit_down_right[position],
            self.limit_up_left[position],
            self.limit_down_left[position],
        ];

        for i in 0..4 {
            let movement = movements[i];
            let lim = limits[i];

            if lim > 0 {
                let mut last = movement(position).unwrap();
                for _i in 0..lim {
                    let sq = board.squares[last];
                    if let Some(p) = sq {
                        if p.get_color() == ally_color {
                            // if we get an ally piece, we have no more moves
                            break;
                        } else {
                            let mut mov = Move::new(position, last);
                            mov.is_capture = true;
                            // if we have an enemy piece, this is our last move
                            moves.push(mov);
                            break;
                        }
                    }
                    moves.push(Move::new(position, last));
                    last = movement(last).unwrap_or(0x0);
                }
            }
        }
        moves
    }

    pub fn get_knight_pseudolegal_moves(
        &self,
        position: usize,
        board: &Board,
        ally_color: Color,
    ) -> Vec<Move> {
        // a knight has always 8 pseudolegal moves in an empty board
        let mut moves = Vec::with_capacity(8);
        let movements = [33, 14, 18, 33, 31, 18, 14, 31];
        let op = [1, 1, -1, -1, -1, 1, -1, 1];
        let mut end: usize;
        // starting from 2 up 1 right in clockwise order
        for i in 0..8 {
            if op[i] > 0 {
                end = position + movements[i];
            } else {
                end = position.checked_sub(movements[i]).unwrap_or(0xff);
                // if the number is negative, send it outside of the board
            }
            if !is_off_board(end) {
                if board.squares[end].is_some() {
                    if board.squares[end].unwrap().get_color() == ally_color {
                        continue;
                    } else {
                        let mut mov = Move::new(position, end);
                        mov.is_capture = true;
                        // if we have an enemy piece, this is our last move
                        moves.push(mov);
                    }
                } else {
                    moves.push(Move::new(position, end));
                }
            }
        }
        moves
    }
    pub fn get_pawn_pseudolegal_moves(
        &self,
        position: usize,
        board: &Board,
        ally_color: Color,
        enpassant: Option<usize>,
        generate_attacked_squares: bool,
    ) -> Vec<Move> {
        // a pawn has at most 2 movements + 2 captures (first move + pieces at both sides)
        // enpassant implies not being the 1st movement.
        let mut moves = Vec::with_capacity(4);
        let end: usize;
        let d_end: usize;
        let cap_r: usize;
        let cap_l: usize;
        let starting_rank: usize;
        let ending_rank: usize;
        // first the pawn push
        if ally_color == White {
            starting_rank = 1;
            end = next_up(position).unwrap_or(0xff);
            d_end = next_up(next_up(position).unwrap_or(0xff)).unwrap_or(0xff);
            cap_r = next_right(next_up(position).unwrap_or(0xff)).unwrap_or(0xff);
            cap_l = next_left(next_up(position).unwrap_or(0xff)).unwrap_or(0xff);
            ending_rank = 7;
        } else {
            starting_rank = 6;
            end = next_down(position).unwrap_or(0xff);
            d_end = next_down(next_down(position).unwrap_or(0xff)).unwrap_or(0xff);
            cap_r = next_right(next_down(position).unwrap_or(0xff)).unwrap_or(0xff);
            cap_l = next_left(next_down(position).unwrap_or(0xff)).unwrap_or(0xff);
            ending_rank = 0;
        }
        if !is_off_board(end) {
            if board.squares[end].is_none() {
                // emtpy squares, free to push
                let mut mov = Move::new(position, end);
                mov.is_threat = false;
                if rank_by_index(end) == ending_rank {
                    // if it's a promotion, we need to add all of them
                    mov.promotion = Some(Queen(ally_color));
                    moves.push(mov);
                    mov.promotion = Some(Rook(ally_color));
                    moves.push(mov);
                    mov.promotion = Some(Bishop(ally_color));
                    moves.push(mov);
                    mov.promotion = Some(Knight(ally_color));
                    moves.push(mov);
                } else {
                    moves.push(mov);
                }
            }
        }
        if rank_by_index(position) == starting_rank {
            // first movement, add double push
            //TODO: This should always be ok!
            if !is_off_board(d_end) {
                if board.squares[d_end].is_none() && board.squares[end].is_none() {
                    // emtpy squares, free to push
                    let mut mov = Move::new(position, d_end);
                    mov.is_threat = false;
                    mov.is_pawn_push = true;
                    moves.push(mov);
                }
            }
        }
        //captures
        if !is_off_board(cap_r) {
            // here we need to check if there's an enemy
            if board.squares[cap_r].is_some() {
                if board.squares[cap_r].unwrap().get_color() != ally_color {
                    let mut mov = Move::new(position, cap_r);
                    mov.is_capture = true;
                    if rank_by_index(end) == ending_rank {
                        // if it's a promotion, we need to add all of them
                        mov.promotion = Some(Queen(ally_color));
                        moves.push(mov);
                        mov.promotion = Some(Rook(ally_color));
                        moves.push(mov);
                        mov.promotion = Some(Bishop(ally_color));
                        moves.push(mov);
                        mov.promotion = Some(Knight(ally_color));
                        moves.push(mov);
                    } else {
                        moves.push(mov);
                    }
                }
            } else {
                if generate_attacked_squares {
                    // if this flag is true, we want to generate a move even if no enemy piece is
                    // present. This is used to evaluate forbidden squares (attacks)
                    let mov = Move::new(position, cap_r);
                    moves.push(mov);
                }
            }
        }
        if !is_off_board(cap_l) {
            if board.squares[cap_l].is_some() {
                if board.squares[cap_l].unwrap().get_color() != ally_color {
                    let mut mov = Move::new(position, cap_l);
                    mov.is_capture = true;
                    moves.push(mov);
                }
            } else {
                if generate_attacked_squares {
                    // if this flag is true, we want to generate a move even if no enemy piece is
                    // present. This is used to evaluate forbidden squares (attacks)
                    let mov = Move::new(position, cap_l);
                    moves.push(mov);
                }
            }
        }

        //TODO: check and test enpassant
        if enpassant.is_some() {
            // opponent did a double push last turn
            let ep = enpassant.unwrap();
            if cap_r == ep {
                let mut mov = Move::new(position, cap_r);
                mov.is_capture = true;
                mov.is_enpassant = true;
                moves.push(mov);
            }
            if cap_l == ep {
                let mut mov = Move::new(position, cap_l);
                mov.is_capture = true;
                mov.is_enpassant = true;
                moves.push(mov);
            }
        }
        moves
    }
}

#[derive(Clone)]
pub struct BoardMap {
    map: [i32; 128],
}

impl BoardMap {
    pub fn new() -> Self {
        Self { map: [0; 128] }
    }
    pub fn get_value(&self, index: usize) -> i32 {
        return self.map[index];
    }
    pub fn inc_value(&mut self, index: usize) {
        self.map[index] += 1;
    }
    pub fn dec_value(&mut self, index: usize) {
        self.map[index] -= 1;
    }
    pub fn clear(&mut self) {
        self.map = [0; 128];
    }
    pub fn initialise_attack_maps(&mut self, board: &Board, piece_list: Vec<(usize, Piece)>) {
        // clear the map first
        self.map = [0; 128];
        // create an empty move gen
        let mg = MoveGenerator::new();
        // go piece by piece and create the attack map
        for (i, p) in piece_list {
            let moves = match p {
                //TODO: Castling rights required here!
                King(c) => mg.get_king_pseudolegal_moves(i, board, self, CastlingRights::new(), c),
                Queen(c) => mg.get_queen_pseudolegal_moves(i, board, c),
                //TODO: how to get enpassant square here
                Pawn(c) => mg.get_pawn_pseudolegal_moves(i, board, c, None, true),
                _ => vec![],
            };
            for i in moves.iter().filter(|m| m.is_threat).map(|m| m.get_to()) {
                self.inc_value(i);
            }
        }
    }
    pub fn get_squares(&self) -> Vec<usize> {
        let mut v: Vec<usize> = vec![];
        for (i, sq) in self.map.iter().enumerate() {
            if *sq > 0 {
                v.push(i);
            }
        }
        v
    }
}
