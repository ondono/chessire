use chessire::ChessGame;
use chessire::ChessireEngine;
use std::io;
//
//const ENGINE_NAME: &'static str = "chers";
//
pub fn main() {
    //    let mut game = ChessGame::new();
    //    let mut engine = ChessireEngine::new();
    //    loop {
    //        uci_loop(&mut game,&mut engine);
    //    }
}
//
//pub fn uci_loop(game: &mut ChessGame) {
//    // init engine
//    //
//    let mut stdin = String::new();
//    loop {
//        match io::stdin().read_line(&mut stdin) {
//            Ok(_n) => {
//                let command_string = stdin.trim().to_lowercase().clone();
//                let mut cmd = command_string.split_ascii_whitespace();
//                match cmd.next().unwrap_or("") {
//                    // written by the order they appear in wbec-ridderker.nl/html/UCIProtocol.html
//                    "uci" => uci_print_info(),
//                    "debug" => uci_debug(cmd),
//                    "isready" => uci_is_ready(),
//                    "setoption" => uci_set_option(cmd),
//                    "register" => uci_register(cmd),
//                    "ucinewgame" => uci_new_game(cmd, game),
//                    "position" => uci_position(cmd, game),
//                    "go" => uci_go(cmd, game),
//                    "stop" => uci_stop(cmd),
//                    "ponderhit" => uci_ponderhit(cmd),
//                    "quit" => break,
//                    _ => uci_unkown_command(cmd),
//                }
//                stdin.clear();
//            }
//            _ => println!("[ERROR] unkown error on stdin"),
//        }
//    }
//}
//
//fn uci_print_info() {
//    println!("id name {}", ENGINE_NAME);
//    println!("id author Xavi Ondono");
//    println!(
//        "option name Hash type spin min 1 max {} default 128",
//        1024 * 1024
//    );
//    println!("uciok");
//}
//
//fn uci_is_ready() {
//    // always ready!
//    println!("readyok");
//}
//
//fn uci_debug(mut cmd: std::str::SplitAsciiWhitespace) {
//    if cmd.next().unwrap_or("") == "on" {
//        // enable debug mode
//        println!("info debug enabled");
//    }
//}
//
//fn uci_unkown_command(mut cmd: std::str::SplitAsciiWhitespace) {
//    println!("[ERROR] unkown UCI command {}", cmd.next().unwrap_or(""));
//}
//
//fn uci_set_option(mut cmd: std::str::SplitAsciiWhitespace) {
//    // unimplemented!();
//}
//
//fn uci_register(mut cmd: std::str::SplitAsciiWhitespace) {
//    //unimplemented!()
//}
//
//fn uci_new_game(mut cmd: std::str::SplitAsciiWhitespace, game: &mut ChessGame) {
//    game.set_start_positions();
//}
//
//fn uci_position(mut cmd: std::str::SplitAsciiWhitespace, game: &mut ChessGame) {
//    match cmd.next().unwrap_or("") {
//        "startpos" => {
//            game.set_start_positions();
//            // now we need to handle the moves list
//            match cmd.next().unwrap_or("") {
//                "moves" => {
//                    for m in cmd {
//                        let mut moves = Vec::with_capacity(1000);
//                        for piece_entry in &game.piece_lists[game.side_to_move as usize] {
//                            moves.append(&mut get_piece_pseudolegal_moves(
//                                //TODO: this cloning is probably a bad idea I need to solve!
//                                game.clone(),
//                                *piece_entry,
//                            ));
//                        }
//                        for mov in moves {
//                            if mov.to_string() == m {
//                                game.make_move(mov);
//                            }
//                        }
//                    }
//                }
//                _ => (),
//            }
//        }
//        "fen" => {
//            game.set_position_from_fen(cmd.next().unwrap_or(""));
//        }
//        _ => println!("info string ERROR: uci_position function failure!"),
//    }
//}
//
//fn uci_go(mut cmd: std::str::SplitAsciiWhitespace, game: &mut ChessGame) {
//    // ignore parameters for now
//    match cmd.next().unwrap_or("") {
//        "searchmoves" => (),
//        "ponder" => (),
//        "wtime" => (),
//        "btime" => (),
//        "winc" => (),
//        "binc" => (),
//        "movestogo" => (),
//        "depth" => (),
//        "nodes" => (),
//        "mate" => (),
//        "movetime" => (),
//        "infinite" => (),
//        _ => (),
//    };
//
//    // print the best move
//    let mov = game.get_rand_move();
//    println!("bestmove {}", mov);
//}
//
//fn uci_stop(mut cmd: std::str::SplitAsciiWhitespace) {
//    unimplemented!();
//}
//
//fn uci_ponderhit(mut cmd: std::str::SplitAsciiWhitespace) {
//    unimplemented!();
//}
