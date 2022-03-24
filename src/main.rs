use clap::{ArgEnum, Parser};

use chessire_utils::moves::print_movelist;

use chessire::engine::bitboard::BitBoardEngine;
use chessire::engine::ChessEngine;

//use termion::color;
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
const TEST_FEN: &str = "4K2r/8/8/8/8/8/8/4k3 w KQkq - 0 1";

fn main() {
    use Implementation::*;
    use Run::*;

    let args = Args::parse();

    let game = chessire_utils::ChessGame::new();

    //game.apply_fen(TEST_FEN).unwrap();
    if let Some(mut engine) = match args.implementation {
        Mailbox => None,
        Bitboard => Some(BitBoardEngine::new_engine(game)),
    } {
        match args.run {
            Cli => {
                use std::io::stdin;
                println!("Running engine {}", engine.get_name());
                println!("by {}", engine.get_author());
                loop {
                    let mut s = String::new();
                    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                    println!("{}", engine.state.get_game());
                    let moves = engine.get_moves(engine.state.side_to_move);
                    print_movelist(&moves);
                    println!();
                    println!("type your next move:");
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
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            Uci => {}
            Perft => {
                println!("Running perft for engine {}", engine.get_name());
                //perft(4, 0..1, &mut engine,false);
                for i in 1..5 {
                    perft_details(i, &mut engine);
                }
            }
        }
    }
}
