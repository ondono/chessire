#[cfg(test)]
mod test_stockfish {
    use anyhow::{anyhow, Result};
    use std::io::Write;
    use std::process::{Command, Stdio};

    use std::collections::HashSet;

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    pub struct MoveRecord {
        name: String,
        count: u128,
    }

    #[test]
    fn compare_peft_results_with_stockfish() {
        let depth = 3;
        let moves = vec![];

        // get the list from stockfish
        let stockfish_records: HashSet<MoveRecord> = if let Ok(x) = stockfish_perft(depth, moves) {
            x.into_iter().collect()
        } else {
            HashSet::new()
        };

        // create a list of moves with the engine
        //let chessire_records: HashSet<MoveRecord> = vec![].into_iter().collect();
        let missing_move = stockfish_records.clone().drain().next().unwrap();
        let mut chessire_records = stockfish_records.clone();

        chessire_records.remove(&missing_move);

        let missing = stockfish_records
            .difference(&chessire_records)
            .collect::<HashSet<&MoveRecord>>();

        let incorrect = chessire_records
            .difference(&stockfish_records)
            .collect::<HashSet<&MoveRecord>>();

        println!("Missing moves:");
        for record in &missing {
            println!("{:?}", record);
        }

        println!("Erroneous moves:");
        for record in &incorrect {
            println!("{:?}", record);
        }
        println!("Everything was okay?");
        assert!(missing.is_empty() && incorrect.is_empty());
    }

    fn stockfish_perft(depth: usize, moves: Vec<String>) -> Result<Vec<MoveRecord>> {
        // call stockfish
        let mut child = Command::new("stockfish")
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        // create the command to run the perft
        let mut command = "position startpos\n".to_string();

        for m in moves {
            command.push_str(&m);
        }

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
