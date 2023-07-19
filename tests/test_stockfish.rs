#[cfg(test)]
mod test_stockfish {
    use anyhow::{anyhow, Result};
    use chessire::ChessEngine;
    use std::io::Write;
    use std::process::{Command, Stdio};

    use chessire_utils::moves::MoveRecord;
    use std::collections::HashMap;

    #[test]
    fn compare_peft_results_with_stockfish() {
        // set up the positions here
        const FEN: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
        //const FEN: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
        compare_moves(3, vec![], FEN);
    }

    fn compare_moves(depth: usize, moves: Vec<String>, starting_fen: &str) {
        if depth > 0 {
            print!("Examining movelist:");
            for m in &moves {
                print!("{} ", m);
            }
            println!();

            let mut stockfish_records: HashMap<&str, u128> = HashMap::new();

            let stockfish_output = stockfish_perft(depth, &moves, starting_fen).unwrap();

            stockfish_output.iter().for_each(|record| {
                *stockfish_records
                    .entry(&*record.name)
                    .or_insert(record.count) += 0
            });

            use chessire::BitBoardEngine;
            let mut game = chessire::ChessGame::new();
            game.apply_fen(starting_fen).unwrap();
            let mut engine = BitBoardEngine::new_engine(game);

            // create a list of moves with our engine
            let mut chessire_records: HashMap<&str, u128> = HashMap::new();
            let chessire_output = engine.perft_get_records(depth, &moves).unwrap();

            chessire_output.iter().for_each(|record| {
                *chessire_records
                    .entry(&*record.name)
                    .or_insert(record.count) += 0
            });

            use itertools::Itertools;

            for record in &stockfish_records {
                if let Some(chessire_count) = chessire_records.get(&*record.0) {
                    if *chessire_count != *record.1 {
                        print!("> found mismatch on move:\t");
                        // push move to the moves list
                        let mut next_moves = moves.clone();
                        next_moves.push(record.0.to_string());
                        // call this function recursively
                        for m in &next_moves {
                            print!("{}\t", m);
                        }
                        println!();
                        println!("Stockfish:{}\tChessire:{}", record.1, chessire_count);
                        println!("Stockfish next move count: {}", stockfish_records.len());
                        for r in stockfish_records.keys().sorted() {
                            print!("{}\t", r);
                        }
                        println!();
                        println!("Chessire next move count: {}", chessire_records.len());
                        for r in chessire_records.keys().sorted() {
                            print!("{}\t", r);
                        }
                        println!();
                        compare_moves(depth - 1, next_moves, starting_fen);
                    }
                } else {
                    println!("Error: Move missing!");
                    print!("Movelist: ");
                    for m in &moves {
                        print!("{} ", m);
                    }
                    println!();
                    println!("{} was missing!", record.0);
                    panic!();
                }
            }

            for record in &chessire_records {
                if let Some(stockfish_count) = stockfish_records.get(&*record.0) {
                    if *stockfish_count != *record.1 {
                        // push move to the moves list
                        let mut next_moves = moves.clone();
                        next_moves.push(record.0.to_string());
                        // call this function recursively
                        print!("> found mismatch on move:\t");
                        for m in &next_moves {
                            print!("{}\t", m);
                        }
                        println!();
                        println!("Stockfish:{}\tChessire:{}", stockfish_count, record.1);
                        println!("Stockfish next move count: {}", stockfish_records.len());
                        for r in stockfish_records.keys().sorted() {
                            print!("{}\t", r);
                        }
                        println!();
                        println!("Chessire next move count: {}", chessire_records.len());
                        for r in chessire_records.keys().sorted() {
                            print!("{}\t", r);
                        }
                        println!();
                        compare_moves(depth - 1, next_moves, starting_fen);
                    }
                } else {
                    println!("Error: Invalid Move!");
                    print!("Movelist: ");
                    for m in &moves {
                        print!("{} ", m);
                    }
                    println!("{}", record.0);
                    panic!();
                }
            }
        } else {
            println!("ended evaluation of line:");
            for m in &moves {
                print!("{} ", m);
            }
            panic!();
        }
    }

    fn stockfish_perft(
        depth: usize,
        moves: &Vec<String>,
        starting_fen: &str,
    ) -> Result<Vec<MoveRecord>> {
        // call stockfish
        // this technically works with any engine, but the parsing is done specifically for
        // stockfish
        let mut child = Command::new("stockfish")
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        // create the command to run the perft
        let mut command = "position fen ".to_string();

        // apply a nice fen starting position
        command.push_str(starting_fen);

        // if the move string isn't empty, append the list of moves
        if !moves.is_empty() {
            command.push_str(" moves ");
            for m in moves {
                command.push_str(m);
                command.push(' ');
            }
        }
        command.push('\n');
        // Append the perft command
        command.push_str(format!("go perft {}\n", depth).as_str());
        // run the whole thing and capture output
        child
            .stdin
            .as_mut()
            .ok_or("stdin could not be captured")
            .unwrap()
            .write_all(command.as_bytes())
            .unwrap();

        // get the output into a string
        let output = child.wait_with_output().unwrap();

        // if everything works
        if output.status.success() {
            // start to parse the lines of the output
            let raw_output = String::from_utf8(output.stdout).unwrap();
            let moves = raw_output
                .lines()
                // skip the first line, "Stockfish yadayada authors.."
                .skip(1)
                .filter(|x| !x.starts_with("Nodes searched"))
                .filter_map(|m| {
                    let mut s = m.split(':');
                    let name = s.next()?.to_owned();

                    let count = s.next()?.trim_start().parse::<u128>().ok()?;
                    Some(MoveRecord { name, count })
                })
                .collect();

            Ok(moves)
        } else {
            Err(anyhow!("what?"))
        }
    }
}
