use chessire_utils::{
    board::{Selection, SelectionColor},
    moves::print_movelist,
};

use crate::test::perft_details;

use super::*;
use std::io::stdin;

// DEBUGGING FLAGS
const SELECTION: bool = false;
const EVAL: bool = false;
const MOVELIST: bool = false;
const PIECELISTS: bool = false;

pub fn print_status(engine: &mut BitBoardEngine) {
    // Clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    // Engine info
    println!("Running engine {}", engine.get_name());
    println!("by {}", engine.get_author());

    let mut game = engine.state.get_game();

    // build selections
    // this section is built for debugging purposes
    if SELECTION {
        for (i, _) in game.board.squares.iter().enumerate() {
            if engine.is_square_attacked_by(i, game.side_to_move.opponent()) {
                game.board
                    .selections
                    .push(Selection::new([i].to_vec(), SelectionColor::new(255, 0, 0)));
            }
        }
    }

    // print the game
    println!("{}", game);

    if EVAL {
        println!("engine evaluation: {}\n", engine.evaluate());
    }

    if MOVELIST {
        let moves = engine.get_moves(engine.state.side_to_move);
        print_movelist(&moves);
        println!("\nTotal moves: {}\n", moves.len());
    }

    if PIECELISTS {
        println!("positions:\n");
        for p in engine.state.white_piece_lists {
            if let Some(x) = p.0 {
                println!("{}\t{}", x, p.1);
            }
        }
        for p in engine.state.black_piece_lists {
            if let Some(x) = p.0 {
                println!("{}\t{}", x, p.1);
            }
        }
        println!();
    }
    println!("type your next move (q to exit, perft to run perft on this position):");
}

pub fn cli_loop(engine: &mut BitBoardEngine) {
    // set a new game
    let g = ChessGame::new();
    engine.set_position(g);

    loop {
        print_status(engine);

        let moves = engine.get_moves(engine.state.side_to_move);

        if moves.is_empty() {
            let sq = engine.state.current_position[engine::bitboard::constants::WHITE_KING]
                .get_lsb()
                .unwrap();
            // if the king is under attack
            if engine.is_square_attacked_by(sq, chessire_utils::color::Color::Black) {
                println!("Checkmate! you lost!");
            } else {
                println!("Stalemate!");
            }
            return;
        }
        // interactive prompt
        let mut s = String::new();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
        match s.trim() {
            "q" => break,
            "perft" => {
                println!("enter number of plys");
                let mut x = String::new();
                stdin()
                    .read_line(&mut x)
                    .expect("Did not enter a correct string");
                let n = x.trim().parse::<usize>().unwrap();
                perft_details::<_>(n, engine);
                break;
            }
            _ => {
                // check if it's a valid move
                for m in moves {
                    if m.to_string().split_whitespace().next().unwrap() == s.trim() {
                        engine.make_move(m).unwrap_or(());

                        if engine.get_moves(engine.state.side_to_move).is_empty() {
                            // find the position of the king
                            let sq = engine.state.current_position
                                [engine::bitboard::constants::BLACK_KING]
                                .get_lsb()
                                .unwrap();
                            // if the king is under attack
                            if engine.is_square_attacked_by(sq, chessire_utils::color::Color::White)
                            {
                                println!("Checkmate! you win!");
                            } else {
                                println!("Stalemate!");
                            }
                            return;
                        }
                        engine.play_best_move();
                        break;
                    }
                }
            }
        }
    }
}
