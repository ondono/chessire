use chessire::bitboard::*;
use chessire::board::*;
use chessire::moves::*;
use chessire::piece::Color::*;
use chessire::test;
use chessire::*;

use test::perft;
use test::perft_moves;

pub fn main() {
    let mut game = ChessGame::new();
    game.set_start_positions();
    game.move_gen.init();
    // clear the terminal
    println!("{}[2J", 27 as char);
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    // now test
    //let start = 0;
    //let stop = 5;
    //perft(2, start..stop);
    //game.set_position_from_fen(test::POSITION5);

    println!("{}", game);
    //game.set_position_from_fen("8/P7/8/8/8/8/8/8");
    use std::io::stdin;

    let mut input_string = String::new();

    loop {
        // clear the string before processing the next line
        input_string.clear();
        println!("{}[2J", 27 as char);
        println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        // update the state
        game.board.clear_highlighted();
        let squares = game.move_gen.attack[Black as usize].clone().get_squares();
        for sq in squares {
            game.board.add_to_highlighted(sq);
        }

        println!("{}", game);
        test_bitboard();

        // generate all valid moves
        let mut moves = Vec::with_capacity(1000);
        for piece_entry in &game.piece_lists[game.side_to_move as usize] {
            moves.append(&mut get_piece_pseudolegal_moves(
                //TODO: this cloning is probably a bad idea I need to solve!
                game.clone(),
                *piece_entry,
            ));
        }
        // get the input from terminal
        stdin()
            .read_line(&mut input_string)
            .ok()
            .expect("Failed to read line");
        // take the return
        input_string = input_string.trim().to_string();

        match input_string.as_str() {
            "q" => break,
            "t" => {
                perft(2, 0..5);
                break;
            }
            "p" => {
                let mut num = String::new();
                stdin()
                    .read_line(&mut num)
                    .ok()
                    .expect("failed to get depth");

                perft_moves(&mut game, num.trim().parse::<usize>().unwrap());
                stdin().read_line(&mut num).ok();
            }
            "s" => game.set_start_positions(),
            "u" => {
                if game.halfmove_clock != 0 {
                    game.unmake_move();
                }
            }
            _ => {
                for mov in moves {
                    let movr = mov.to_string().to_ascii_lowercase();
                    if movr == input_string {
                        game.make_move(mov);
                    }
                }
            }
        }
    }
}
