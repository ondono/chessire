use super::*;
use std::io;

pub fn uci_loop(engine: &mut impl ChessEngine) {
    let mut stdin = String::new();

    while let Ok(_n) = io::stdin().read_line(&mut stdin) {
        if let Some(cmd_str) = stdin.trim().to_lowercase().lines().last() {
            let mut cmd = cmd_str.split_ascii_whitespace();
            match cmd.next().unwrap_or("") {
                // debug command from stockfish
                "d" => stockfish_style_debug(engine),
                // written by the order they appear in wbec-ridderker.nl/html/UCIProtocol.html
                "uci" => uci_print_info(),
                "debug" => uci_debug(cmd),
                "isready" => uci_is_ready(),
                "setoption" => uci_set_option(cmd),
                "register" => uci_register(cmd),
                "ucinewgame" => uci_new_game(cmd, engine),
                "position" => uci_position(cmd, engine),
                "go" => uci_go(cmd, engine),
                "stop" => uci_stop(cmd),
                "ponderhit" => uci_ponderhit(cmd),
                "quit" => break,
                _ => uci_unkown_command(cmd),
            }
        }
    }
}

fn stockfish_style_debug(engine: &impl ChessEngine) {
    println!("{}", engine.get_internal_position());
}

fn uci_print_info() {
    //TODO: fix this
    println!("id name {}", "chessire");
    println!("id author Xavi Ondono");
    println!(
        "option name Hash type spin min 1 max {} default 128",
        1024 * 1024
    );
    println!("uciok");
}

fn uci_is_ready() {
    // always ready!
    println!("readyok");
}

fn uci_debug(mut cmd: std::str::SplitAsciiWhitespace) {
    if cmd.next().unwrap_or("") == "on" {
        // enable debug mode
        println!("info debug enabled");
    }
}

fn uci_unkown_command(mut cmd: std::str::SplitAsciiWhitespace) {
    println!("[ERROR] unkown UCI command {}", cmd.next().unwrap_or(""));
}

fn uci_set_option(_cmd: std::str::SplitAsciiWhitespace) {
    // unimplemented!();
}

fn uci_register(_cmd: std::str::SplitAsciiWhitespace) {
    //unimplemented!()
}

fn uci_new_game(_cmd: std::str::SplitAsciiWhitespace, _engine: &mut impl ChessEngine) {
    //engine.set_position();
}

fn uci_position(mut cmd: std::str::SplitAsciiWhitespace, _engine: &mut impl ChessEngine) {
    match cmd.next().unwrap_or("") {
        "startpos" => {
            //board.reset_position();
            // now we need to handle the moves list
            match cmd.next().unwrap_or("") {
                "moves" => {
                    for _m in cmd {
                        //let mov = Move::from_algebraic(board.clone(), m);
                        //board.apply_move(mov);
                    }
                }
                _ => println!("ups!"),
            }
        }
        "fen" => {
            //board.set_position_from_fen(cmd.next().unwrap_or(""));
        }
        _ => println!("info string ERROR: uci_position function failure!"),
    }
}

fn uci_go(mut cmd: std::str::SplitAsciiWhitespace, engine: &mut impl ChessEngine) {
    // ignore parameters for now
    match cmd.next().unwrap_or("") {
        "searchmoves" => (),
        "ponder" => (),
        "wtime" => (),
        "btime" => (),
        "winc" => (),
        "binc" => (),
        "movestogo" => (),
        "depth" => (),
        "nodes" => (),
        "mate" => (),
        "movetime" => (),
        "infinite" => (),
        _ => (),
    };

    // print the best move
    let mov = engine.get_best_move();
    println!("bestmove {}", mov);
}

fn uci_stop(_cmd: std::str::SplitAsciiWhitespace) {
    unimplemented!();
}

fn uci_ponderhit(_cmd: std::str::SplitAsciiWhitespace) {
    unimplemented!();
}
