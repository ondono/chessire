use chessire::game::board::*;
use chessire::game::color::Color::{Black, White};
use chessire::ChessGame;
use chessire::ChessireEngine;
use std::io;
use std::io::stdin;

use chessire::engine::bitboard::constants::*;
use chessire::engine::bitboard::util::*;

use chessire::game::*;

//const TEST_FEN: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
const TEST_FEN: &str = "r7/8/8/4K3/8/8/8/7B w KQkq - 0 1";

pub fn main() {
    terminal_clear();
    println!("Running Chessire test CLI");
    let mut game = ChessGame::new();
    let mut engine = ChessireEngine::new();

    //game.apply_fen("8/8/3p4/8/8/3P4/8/8 w KQkq - 0 1");

    let mut input_string: String = "".to_string();
    input_string.clear();

    game.apply_fen(TEST_FEN).ok();
    engine.set_position(game.clone());

    let mut attacked_by_white = vec![];
    for sq in engine.get_attacked_squares_by(White) {
        attacked_by_white.push(sq);
    }

    let mut attacked_by_black = vec![];
    for sq in engine.get_attacked_squares_by(Black) {
        attacked_by_black.push(sq);
    }

    // add selections
    let white_sel = Selection::new(attacked_by_white, SelectionColor::new(200, 0, 0));
    let black_sel = Selection::new(attacked_by_black, SelectionColor::new(0, 200, 0));

    game.board.add_selection(white_sel);
    game.board.add_selection(black_sel);

    engine.get_moves(White);
    //engine.get_moves(Black);
    //print board
    println!("{}", game.board);
}

fn terminal_clear() {
    print!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

//pub fn main() {
//    let mut game = ChessGame::new();
//    game.set_start_positions();
//    // clear the terminal
//    println!("{}[2J", 27 as char);
//    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
//    // now test
//    //let start = 0;
//    //let stop = 5;
//    //perft(2, start..stop);
//    //game.set_position_from_fen(test::POSITION5);
//
//    println!("{}", game);
//    //game.set_position_from_fen("8/P7/8/8/8/8/8/8");
//    use std::io::stdin;
//
//
//    loop {
//        // clear the string before processing the next line
//        input_string.clear();
//        println!("{}[2J", 27 as char);
//        println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
//        // update the state
//        game.board.clear_highlighted();
//
//        println!("{}", game);
//        test_bitboard();
//
//        // get the input from terminal
//        let moves: Vec<Move> = vec![];
//  //        // take the return
//        input_string = input_string.trim().to_string();
//
//        match input_string.as_str() {
//            "q" => break,
//            "t" => {
//                perft(2, 0..5);
//                break;
//            }
//            "p" => {
//                let mut num = String::new();
//                stdin().read_line(&mut num).expect("failed to get depth");
//
//                perft_moves(&mut game, num.trim().parse::<usize>().unwrap());
//                stdin().read_line(&mut num).ok();
//            }
//            "s" => game.set_start_positions(),
//            "u" => {
//                if game.halfmove_clock != 0 {
//                    game.unmake_move();
//                }
//            }
//            _ => {
//                for mov in moves {
//                    let movr = mov.to_string().to_ascii_lowercase();
//                    if movr == input_string {
//                        game.make_move(mov);
//                    }
//                }
//            }
//        }
//    }
//}
