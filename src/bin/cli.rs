use chessire::game::board::*;
use chessire::game::color::Color::{Black, White};
use chessire::ChessGame;
use chessire::ChessireEngine;
use std::io;
use std::io::stdin;

use chessire::engine::bitboard::attacks::{get_bishop_attack, get_queen_attack, get_rook_attack};
use chessire::engine::bitboard::constants::*;
use chessire::engine::bitboard::util::*;

use chessire::engine::bitboard::attacks::{bishop_attacks_on_the_fly, rook_attacks_on_the_fly};

pub fn main() {
    terminal_clear();
    println!("Running Chessire test CLI");
    let mut game = ChessGame::new();
    let mut engine = ChessireEngine::new();

    println!("{}", game.board);

    let mut input_string: String = "".to_string();
    input_string.clear();

    let block_const = A1 | D4;

    for sq in 0..64 {
        let blocker = BitBoard::new(block_const);

        let on_the_fly = rook_attacks_on_the_fly(sq, blocker);
        let bitboarded = get_rook_attack(&engine.bb_engine.attack_tables, sq, blocker);

        println!(
            "{} {} {}",
            blocker,
            get_bishop_attack(&engine.bb_engine.attack_tables, sq, blocker),
            get_rook_attack(&engine.bb_engine.attack_tables, sq, blocker)
        );
        // if on_the_fly != bitboarded {
        //     println!(
        //         "square:{} blocker:{} on the fly:{} bitboard:{}",
        //         sq, blocker, on_the_fly, bitboarded
        //     );
        stdin()
            .read_line(&mut input_string)
            .expect("failed to get depth");
        input_string.clear();
        // }
    }
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
