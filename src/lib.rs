pub mod bitboard;
pub mod bitboard_attacks;
pub mod board;
pub mod constants;
pub mod moves;
pub mod piece;
pub mod test;
pub mod tile;

use board::*;
use constants::*;
use moves::*;
use piece::Color::Black;
use piece::Color::White;
use piece::*;
use std::fmt;

#[derive(Clone, Copy)]
pub struct CastlingRights {
    white_king_side: bool,
    white_queen_side: bool,
    black_king_side: bool,
    black_queen_side: bool,
}

impl CastlingRights {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for CastlingRights {
    fn default() -> CastlingRights {
        Self {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        }
    }
}

impl fmt::Display for CastlingRights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.white_king_side {
            write!(f, "K")?;
        }
        if self.white_queen_side {
            write!(f, "Q")?;
        }
        if self.black_king_side {
            write!(f, "k")?;
        }
        if self.black_queen_side {
            write!(f, "q")?;
        }
        if !self.white_king_side
            && !self.white_queen_side
            && !self.black_king_side
            && !self.black_queen_side
        {
            write!(f, "-")?;
        }
        Ok(())
    }
}

const FEN_STARTING_STRING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Clone)]
pub struct ChessGame {
    pub board: Board,
    pub side_to_move: Color,
    castling_rights: CastlingRights,
    enpassant_target_square: Option<usize>,
    pub halfmove_clock: u32,
    fullmove_clock: u32,
    pub piece_lists: [Vec<(usize, Piece)>; 2],
    pub move_gen: MoveGenerator,
    // History Vectors:
    // used to restore moves
    // history: A vector of the played moves. Last Move can be checked by poping out
    history: Vec<Move>,
    // capture_history: Captured pieces are stored in this vector. This allows us to recover all
    // pieces (if the move is tagged as a capture, the next piece should be popped during unmake)
    capture_history: Vec<(usize, Piece)>,
    // castling_history: Each ply the state of castling moves is pushed on the stack. That way
    // previous castling state can be recovered.
    castling_history: Vec<CastlingRights>,
    // enpassant_history: Each ply the state of the enpassant target square is pushed on the stack.
    // That way previous enpassant squares can be recovered.
    enpassant_history: Vec<Option<usize>>,
    // Optimizers
    //
    white_king_pos: usize,
    black_king_pos: usize,
}

impl ChessGame {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            side_to_move: White,
            castling_rights: CastlingRights::new(),
            enpassant_target_square: None,
            halfmove_clock: 0,
            fullmove_clock: 1,
            piece_lists: [vec![], vec![]],
            move_gen: MoveGenerator::new(),
            capture_history: vec![],
            history: Vec::with_capacity(100),
            castling_history: vec![CastlingRights::new()],
            enpassant_history: vec![None],
            white_king_pos: E1,
            black_king_pos: E8,
        }
    }
    pub fn set_position_from_fen(&mut self, fen_string: &str) {
        let mut fields = fen_string.split_ascii_whitespace();
        // for each field if we can't read it correctly, use default setting
        // piece placement
        self.board.clear();
        let piece_placement = fields
            .next()
            .unwrap_or("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        self.board.set_position_from_fen(piece_placement);
        // fill the piece list too!
        // side to move
        let side_to_move = fields.next().unwrap_or("w");
        self.side_to_move = match side_to_move {
            "w" | "W" => White,
            "b" | "B" => Black,
            _ => White,
        };

        // Castling rights
        let castl = fields.next().unwrap_or("-");
        self.castling_rights.white_king_side = castl.find('K') != None;
        self.castling_rights.white_queen_side = castl.find('Q') != None;
        self.castling_rights.black_king_side = castl.find('k') != None;
        self.castling_rights.black_queen_side = castl.find('q') != None;

        // en passant target square
        let en_passant = fields.next().unwrap_or("-");
        let mut enpassant_sq = 0;
        let mut enpassant_present = false;
        match en_passant {
            "-" => self.enpassant_target_square = None,
            _ => {
                enpassant_present = true;
                let mut it = en_passant.chars();
                let file = it.next().unwrap_or('-');
                let rank = it.next().unwrap_or('0');
                match file {
                    'a' => enpassant_sq = A,
                    'b' => enpassant_sq = B,
                    'c' => enpassant_sq = C,
                    'd' => enpassant_sq = D,
                    'e' => enpassant_sq = E,
                    'f' => enpassant_sq = F,
                    'g' => enpassant_sq = G,
                    'h' => enpassant_sq = H,
                    _ => (),
                }
                match rank {
                    '3' => enpassant_sq += 0x30,
                    '6' => enpassant_sq += 0x60,
                    _ => enpassant_sq = 0x88,
                }
            }
        }
        if enpassant_present {
            self.enpassant_target_square = Some(enpassant_sq);
        } else {
            self.enpassant_target_square = None;
        }
        let half_move_clock = fields.next().unwrap_or("0");
        self.halfmove_clock = half_move_clock.parse::<u32>().unwrap_or(0);
        let full_move_count = fields.next().unwrap_or("1");
        self.fullmove_clock = full_move_count.parse::<u32>().unwrap_or(0);
        // update the piece list
        self.piece_lists[White as usize].clear();
        self.piece_lists[Black as usize].clear();
        self.piece_lists[White as usize].append(&mut self.board.get_piece_list(White).clone());
        self.piece_lists[Black as usize].append(&mut self.board.get_piece_list(Black).clone());

        self.history.clear();

        self.move_gen.attack[0]
            .initialise_attack_maps(&self.board, self.piece_lists[Black as usize].clone());
        self.move_gen.attack[1]
            .initialise_attack_maps(&self.board, self.piece_lists[White as usize].clone());
    }
    pub fn set_start_positions(&mut self) {
        self.set_position_from_fen(FEN_STARTING_STRING);
    }
    pub fn add_piece(&mut self, position: usize, piece: Piece) {
        self.board.add_piece_to(position, piece);
        let color = piece.get_color();
        let index = self.piece_lists[color as usize]
            .clone()
            .iter()
            .position(|(x, _p)| (*x == position));
        if let Some(i) = index {
            //self.piece_lists[color as usize].remove(i);
        }
        self.piece_lists[color as usize].push((position, piece));
    }
    pub fn remove_piece(&mut self, position: usize) {
        // crashes if the square is empty!
        let color = self.board.squares[position].unwrap().get_color() as usize;
        let index = self.piece_lists[color]
            .clone()
            .iter()
            .position(|(x, _p)| (*x == position));
        if let Some(i) = index {
            self.piece_lists[color].remove(i);
        }
        self.board.remove_piece(position);
    }
    /// executes without checking for validity!!
    pub fn make_move(&mut self, m: Move) {
        //println!("Making move {}", m);

        // these need to be recorded BEFORE modified

        self.castling_history.push(self.castling_rights);
        self.enpassant_history.push(self.enpassant_target_square);

        if m.is_pawn_push {
            if self.side_to_move == White {
                self.enpassant_target_square = next_down(m.to);
            } else {
                self.enpassant_target_square = next_up(m.to);
            }
        } else {
            // clear the target square if the move is not a pawn push!
            self.enpassant_target_square = None;
        }

        let piece = self.board.squares[m.from];
        //TODO: Castling
        if m.is_castling {
            // disable castling rights for the future
            if self.side_to_move == White {
                self.castling_rights.white_king_side = false;
                self.castling_rights.white_queen_side = false;
            } else {
                self.castling_rights.black_king_side = false;
                self.castling_rights.black_queen_side = false;
            }
            match (m.from, m.to) {
                // white kingside castling
                (E1, G1) => {
                    // remove the rook from H1
                    self.remove_piece(H1);
                    // add the rook to F1
                    self.add_piece(F1, Piece::Rook(White));
                }
                (E1, C1) => {
                    // remove the rook from A1
                    self.remove_piece(A1);
                    // add the rook to D1
                    self.add_piece(D1, Piece::Rook(White));
                }
                (E8, G8) => {
                    // remove the rook from H8
                    self.remove_piece(H8);
                    // add the rook to F8
                    self.add_piece(F8, Piece::Rook(Black));
                }
                (E8, C8) => {
                    // remove the rook from A1
                    self.remove_piece(A8);
                    // add the rook to F1
                    self.add_piece(D8, Piece::Rook(Black));
                }
                _ => unimplemented!("a wrong move was marked as castling!"),
            }
        } else {
            if piece == Some(Piece::King(White)) {
                // the king is moving and is not castling, take his castling rights!
                self.castling_rights.white_king_side = false;
                self.castling_rights.white_queen_side = false;
            }
            if piece == Some(Piece::King(Black)) {
                // the king is moving and is not castling, take his castling rights!
                self.castling_rights.black_king_side = false;
                self.castling_rights.black_queen_side = false;
            }
        }
        //castling rights
        if self.side_to_move == White {
            if m.from == H1 {
                self.castling_rights.white_king_side = false;
            }
            if m.from == A1 {
                self.castling_rights.white_queen_side = false;
            }
        } else {
            if m.from == H8 {
                self.castling_rights.black_king_side = false;
            }
            if m.from == A8 {
                self.castling_rights.black_queen_side = false;
            }
        }

        self.halfmove_clock += 1;
        if self.side_to_move == Black {
            self.fullmove_clock += 1;
        }

        if piece.is_none() {
            println!("Making move error Report:");
            println!("{}", self);
            println!("Error: I tried the move {}", m);
            println!("Move History");
            for h in &self.history {
                println!("{}", h);
            }
            return;
        }
        // remove our piece from origin
        self.remove_piece(m.from);

        if m.is_capture {
            // save the captured piece. If it's enpassant, we need to get it from another place!
            let captured_piece = if m.is_enpassant {
                let enemy_sq: usize;
                if self.side_to_move == White {
                    enemy_sq = next_down(m.to).unwrap();
                } else {
                    enemy_sq = next_up(m.to).unwrap();
                }
                self.board.squares[enemy_sq]
            } else {
                // if it's not enpassant it's our destinations
                self.board.squares[m.to]
            };

            if captured_piece.is_some() {
                if m.is_enpassant {
                    let end: usize;
                    if self.side_to_move == White {
                        end = next_down(m.to).unwrap();
                        self.capture_history.push((end, captured_piece.unwrap()));
                    } else {
                        end = next_up(m.to).unwrap();
                        self.capture_history.push((end, captured_piece.unwrap()));
                    }
                    self.remove_piece(end);
                } else {
                    self.remove_piece(m.to);
                    self.capture_history.push((m.to, captured_piece.unwrap()));
                }
            } else {
                println!(
                    "Error!: A move I was making was tagged as a capture, but there was no enemy there!"
                );
                println!("move: {}", m);
                println!("{}", self);
                println!("Move History");
                for h in self.history.iter() {
                    println!("{}", h);
                }
            }
        }

        if piece == Some(Piece::King(White)) {
            self.white_king_pos = m.to;
        }

        if piece == Some(Piece::King(Black)) {
            self.black_king_pos = m.to;
        }
        // add our piece to destination
        self.add_piece(m.to, piece.unwrap());
        // update the history
        self.history.push(m);

        self.move_gen.update_attack_map(
            &self.board,
            White,
            self.piece_lists[White as usize].clone(),
        );
        self.move_gen.update_attack_map(
            &self.board,
            Black,
            self.piece_lists[Black as usize].clone(),
        );
        // switch who's turn it is
        if self.side_to_move == White {
            self.side_to_move = Black
        } else {
            self.side_to_move = White
        }
    }

    pub fn unmake_move(&mut self) {
        self.halfmove_clock -= 1;
        if self.side_to_move == White {
            self.fullmove_clock -= 1;
        }
        // backroll the history
        let m = self.history.pop().unwrap();
        self.castling_rights = self.castling_history.pop().unwrap();
        self.enpassant_target_square = self.enpassant_history.pop().unwrap();

        //TODO: Castling
        if m.is_castling {
            match (m.from, m.to) {
                // white kingside castling
                (E1, G1) => {
                    self.remove_piece(F1);
                    self.add_piece(H1, Piece::Rook(White));
                }
                (E1, C1) => {
                    self.remove_piece(D1);
                    self.add_piece(A1, Piece::Rook(White));
                }
                (E8, G8) => {
                    self.remove_piece(F8);
                    self.add_piece(H8, Piece::Rook(Black));
                }
                (E8, C8) => {
                    self.remove_piece(D8);
                    self.add_piece(A8, Piece::Rook(Black));
                }
                _ => unimplemented!("a wrong move was marked as castling!"),
            }
        }
        //println!("unmaking move {}", m);
        let piece = self.board.squares[m.to];
        if piece.is_none() {
            println!("{}", self);
            println!("Error: I tried to unmove {}", m);
            println!("is capture: {}", m.is_capture);
            println!("Move History");
            for h in self.history.iter() {
                println!("{}", h);
            }

            use std::io::stdin;
            let mut input_string = String::new();

            stdin()
                .read_line(&mut input_string)
                .expect("Failed to read line");
            return;
        }
        if m.is_pawn_push {
            //TODO:What if we do 2 enpassant moves in a sequence?
            self.enpassant_target_square = None;
        }
        self.add_piece(m.from, piece.unwrap());
        self.remove_piece(m.to);
        if m.is_capture {
            let captured_piece = self.capture_history.pop();
            if captured_piece.is_some() {
                self.add_piece(captured_piece.unwrap().0, captured_piece.unwrap().1);
            } else {
                println!(
                    "A move I was unmaking was tagged as a capture, but there was no enemy there!"
                );
                println!("move: {}", m);
                println!("{}", self);
                println!("Move History");
                for h in self.history.iter() {
                    println!("{}", h);
                }

                use std::io::stdin;
                let mut input_string = String::new();

                stdin()
                    .read_line(&mut input_string)
                    .expect("Failed to read line");
                return;
            }
        }

        self.move_gen.update_attack_map(
            &self.board,
            White,
            self.piece_lists[White as usize].clone(),
        );

        self.move_gen.update_attack_map(
            &self.board,
            Black,
            self.piece_lists[Black as usize].clone(),
        );
        // switch who's turn it is
        if self.side_to_move == White {
            self.side_to_move = Black
        } else {
            self.side_to_move = White
        }
        //  println!("{}", self);
        //  println!("Move history:");
        //  for h in self.history.iter() {
        //      println!("{}", h);
        //  }
    }

    pub fn get_rand_move(&self) -> Move {
        let mut moves = Vec::with_capacity(1000);
        for piece_entry in &self.piece_lists[self.side_to_move as usize] {
            moves.append(&mut get_piece_pseudolegal_moves(
                //TODO: this cloning is probably a bad idea I need to solve!
                self.clone(),
                *piece_entry,
            ));
        }

        use rand::*;

        let movecount = moves.len();

        let mut rng = thread_rng();
        let i = rng.gen_range(0..movecount);
        let mov = moves.get(i).unwrap();

        //    *moves.get(i).unwrap()
        *mov
    }
    pub fn move_testing(&self, depth: usize) -> u128 {
        if depth == 0 {
            return 1;
        }
        // first make a copy of the game statd
        let mut saved_state = self.clone();
        let mut num_moves: u128 = 0;
        let mut trace = vec![];
        // then simulate forwards and evaluate
        let mut moves = Vec::with_capacity(1000);
        for piece_entry in &saved_state.piece_lists[saved_state.side_to_move as usize] {
            moves.append(&mut get_piece_pseudolegal_moves(
                //TODO: this cloning is probably a bad idea I need to solve!
                saved_state.clone(),
                *piece_entry,
            ));
        }

        for mov in moves {
            saved_state.make_move(mov);
            let phase_moves = saved_state.move_testing(depth - 1);
            trace.push((depth, mov, phase_moves));
            num_moves += phase_moves;
            saved_state.unmake_move();
        }
        num_moves
    }
}

impl fmt::Display for ChessGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Side to move: {}\r\n", self.side_to_move)?;
        write!(f, "Castling rights: {}\r\n", self.castling_rights)?;
        if self.enpassant_target_square != None {
            if self.enpassant_target_square.unwrap() & 0x88 == 0 {
                write!(
                    f,
                    "enpassant square: {}\r\n",
                    get_name_from_index(self.enpassant_target_square.unwrap()),
                )?;
            }
        } else {
            write!(f, "enpassant square: None\r\n",)?;
        }
        write!(f, "Halfmove clock: {}\r\n", self.halfmove_clock)?;
        write!(f, "Fullmove clock: {}\r\n", self.fullmove_clock)?;
        write!(f, "{}", self.board)
    }
}
