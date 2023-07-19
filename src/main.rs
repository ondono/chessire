use clap::{ArgEnum, Parser};

use chessire::engine::bitboard::BitBoardEngine;
use chessire::engine::ChessEngine;


//use termion::color;
use chessire::interface::*;
use chessire::test::{perft, perft_details};

#[derive(Parser, Debug, Clone, Copy, ArgEnum)]
enum Run {
    Cli,
    Uci,
    Perft,
}

#[derive(Parser, Debug, Clone, Copy, ArgEnum)]
enum Implementation {
    Mailbox,
    Bitboard,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, arg_enum, default_value_t = Run::Cli)]
    run: Run,
    #[clap(short, long, arg_enum, default_value_t = Implementation::Bitboard)]
    implementation: Implementation,
}

#[allow(dead_code)]
const TEST_FEN: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";

fn main() {
    use Implementation::*;
    use Run::*;

    let args = Args::parse();

    let game = chessire_utils::ChessGame::new();

    if let Some(mut engine) = match args.implementation {
        Mailbox => None,
        Bitboard => Some(BitBoardEngine::new_engine(game)),
    } {
        match args.run {
            Cli => {
                use std::io::stdin;
                println!("Running engine {}", engine.get_name());
                println!("by {}", engine.get_author());
                let g = chessire::ChessGame::new();
                //g.clear();
                //g.apply_fen(TEST_FEN).unwrap();
                engine.set_position(g);

                use chessire_utils::board::*;

                loop {
                    let mut s = String::new();
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    let mut game = engine.state.get_game();
                    for (i, _) in game.board.squares.iter().enumerate() {
                        if engine.is_square_attacked_by(i, game.side_to_move.opponent()) {
                            game.board
                                .selections
                                .push(Selection::new([i].to_vec(), SelectionColor::new(255, 0, 0)));
                        }
                    }
                    println!("{}", game);
                    let moves = engine.get_moves(engine.state.side_to_move);
                    if moves.is_empty() {
                        let sq = engine.state.current_position
                            [chessire::engine::bitboard::constants::WHITE_KING]
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
                    println!("engine evaluation: {}", engine.evaluate());
                    //print_movelist(&moves);
                    println!();
                    //println!("positions:");
                    //for p in engine.state.white_piece_lists {
                    //    if let Some(x) = p.0 {
                    //        println!("{}\t{}", x, p.1);
                    //    }
                    //}
                    //for p in engine.state.black_piece_lists {
                    //    if let Some(x) = p.0 {
                    //        println!("{}\t{}", x, p.1);
                    //    }
                    //}
                    //println!("Total moves: {}", moves.len());
                    println!();
                    println!(
                        "type your next move (q to exit, perft to run perft on this position):"
                    );
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
                            perft_details(n, &mut engine);
                            break;
                        }
                        _ => {
                            for m in moves {
                                if m.to_string().split_whitespace().next().unwrap() == s.trim() {
                                    engine.make_move(m).unwrap_or(());

                                    if engine.get_moves(engine.state.side_to_move).is_empty() {
                                        // find the position of the king
                                        let sq = engine.state.current_position
                                            [chessire::engine::bitboard::constants::BLACK_KING]
                                            .get_lsb()
                                            .unwrap();
                                        // if the king is under attack
                                        if engine.is_square_attacked_by(
                                            sq,
                                            chessire_utils::color::Color::White,
                                        ) {
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
            Uci => {
                uci_loop(&mut engine);
            }
            Perft => {
                println!("Running perft for engine {}", engine.get_name());
                let mut g = chessire::ChessGame::new();
                g.clear();
                g.apply_fen(TEST_FEN).unwrap();
                engine.set_position(g);
                perft(5, 1..3, &mut engine);
            }
        }
    }
}
