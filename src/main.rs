use chessire::cli::cli_loop;
use clap::{ArgEnum, Parser};

use chessire::engine::bitboard::BitBoardEngine;
use chessire::engine::ChessEngine;

use chessire::interface::*;
use chessire::test::perft;

#[derive(Parser, Debug, Clone, Copy, ArgEnum)]
enum Run {
    Cli,
    Uci,
    Perft,
}

#[derive(Parser, Debug, Clone, Copy, ArgEnum)]
enum Implementation {
    Bitboard,
    MailBox,
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

    let args = Args::parse();
    let game = chessire_utils::ChessGame::new();

    use Implementation::*;
    let engine = match args.implementation {
        Bitboard => Some(BitBoardEngine::new_engine(game)),
        MailBox => None,
    };

    if let Some(mut engine) = engine
    {
        use Run::*;
        match args.run {
            Cli => {
                cli_loop(&mut engine);
            }
            Uci => {
                uci_loop(&mut engine);
            }
            Perft => {
                // set up the position
                let mut g = chessire::ChessGame::new();
                g.clear();
                g.apply_fen(TEST_FEN).unwrap();
                engine.set_position(g);
                // run perft
                println!("Running perft for engine {}", engine.get_name());
                perft(5, 1..3, &mut engine);
            }
        }
    }
    else{
        println!("Implementation not found!");
    }
}
