use crate::*;
use core::ops::Range;

pub fn perft(depth: usize, range: Range<usize>) {
    let mut game = ChessGame::new();

    let positions = vec![POSITION1, POSITION2, POSITION3, POSITION4, POSITION5];
    let pos_res = vec![
        POS1_PERFT_RESULTS,
        POS2_PERFT_RESULTS,
        POS3_PERFT_RESULTS,
        POS4_PERFT_RESULTS,
        POS5_PERFT_RESULTS,
    ];

    let count_pos = positions.iter().count();
    let count_pos_res = pos_res.iter().count();

    let count = usize::min(count_pos, count_pos_res);

    for test_case in range {
        if test_case > count - 1 {
            continue;
        }
        println!("Starting validation of test position {}", test_case + 1);
        game.set_position_from_fen(positions[test_case]);
        for ply in 1..depth + 1 {
            use std::time::Instant;
            use termion::color;

            let now = Instant::now();
            let test_results = game.move_testing(ply);
            let elapsed = now.elapsed().as_millis();
            let expected_res = (pos_res[test_case])[ply - 1].get_nodes();
            if test_results == expected_res {
                println!(
                    "> {}Depth:{}{} ply\tNumber of positions:{}{:>9}{}\ttime:{:>6}ms",
                    color::Fg(color::Green),
                    color::Fg(color::Reset),
                    ply,
                    color::Fg(color::Red),
                    test_results,
                    color::Fg(color::Reset),
                    elapsed,
                );
            } else {
                println!(
                    "> {}Error at Depth:{}{} ply\tNumber of positions:{}{:>9}{}\tExpected:{}{}{}\tDiff:{}\ttime:{:>6}ms",
                    color::Fg(color::Red),
                    color::Fg(color::Reset),
                    ply,
                    color::Fg(color::Red),
                    test_results,
                    color::Fg(color::Reset),
                    color::Fg(color::Red),
                    expected_res,
                    color::Fg(color::Reset),
                    test_results as i128 - expected_res as i128,
                    elapsed,
                );
            }
        }
    }
}

pub fn perft_search(game: &mut ChessGame, depth: usize) -> Vec<(Move, u128)> {
    // first make a copy of the game state
    let mut saved_state = game.clone();
    let mut trace = vec![];
    // then simulate forwards and evaluate
    let mut moves = Vec::with_capacity(1000);
    for piece_entry in &saved_state.piece_lists[saved_state.side_to_move as usize] {
        moves.append(&mut get_piece_pseudolegal_moves(
            //TODO: this cloning is probably a bad idea I need to solve!
            saved_state.clone(),
            *piece_entry,
        ));
    }

    for mov in moves {
        saved_state.make_move(mov);
        let phase_moves = saved_state.move_testing(depth - 1);
        trace.push((mov, phase_moves));
        saved_state.unmake_move();
    }
    trace
}

pub fn perft_moves(game: &mut ChessGame, depth: usize) {
    let test_results = perft_search(game, depth);

    let mut a = 0;
    for (m, i) in test_results.clone() {
        a += i;
        println!("{}: {}", m, i);
    }

    println!("Nodes searched: {}", a);
}

pub struct PerfResults {
    depth: u32,
    nodes: u128,
    captures: u128,
    enpassant: u128,
    castles: u128,
    promotions: u128,
    checks: u128,
    discovery_checks: u128,
    double_checks: u128,
    checkmates: u128,
}

impl PerfResults {
    pub const fn new(
        depth: u32,
        nodes: u128,
        captures: u128,
        enpassant: u128,
        castles: u128,
        promotions: u128,
        checks: u128,
        discovery_checks: u128,
        double_checks: u128,
        checkmates: u128,
    ) -> Self {
        Self {
            depth,
            nodes,
            captures,
            enpassant,
            castles,
            promotions,
            checks,
            discovery_checks,
            double_checks,
            checkmates,
        }
    }
    pub fn get_nodes(&self) -> u128 {
        self.nodes
    }
}
/* PERFT CONSTANTS */

pub const POSITION1: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ";
pub const POSITION2: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -";
pub const POSITION3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - ";
pub const POSITION4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
pub const POSITION5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8  ";

/** Depth Nodes Captures Enpassant Castles Promotions Checks DiscoveryChecks DoubleChecks Checkmates **/
static POS1_PERFT_RESULTS: &'static [PerfResults] = &[
    PerfResults::new(1, 20, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(2, 400, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(3, 8902, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(4, 197281, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(5, 4865609, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(6, 119060324, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(7, 3195901860, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(8, 84998978956, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(9, 2439530234167, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(10, 2439530234167, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(11, 2439530234167, 0, 0, 0, 0, 0, 0, 0, 0),
];

static POS2_PERFT_RESULTS: &'static [PerfResults] = &[
    PerfResults::new(1, 48, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(2, 2039, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(3, 97862, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(4, 4085603, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(5, 193690690, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(6, 8031647685, 0, 0, 0, 0, 0, 0, 0, 0),
];

static POS3_PERFT_RESULTS: &'static [PerfResults] = &[
    PerfResults::new(1, 14, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(2, 191, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(3, 2812, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(4, 43238, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(5, 43238, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(6, 11030083, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(7, 178633661, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(8, 3009794393, 0, 0, 0, 0, 0, 0, 0, 0),
];

static POS4_PERFT_RESULTS: &'static [PerfResults] = &[
    PerfResults::new(1, 6, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(2, 264, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(3, 9467, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(4, 422333, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(5, 15833292, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(6, 15833292, 0, 0, 0, 0, 0, 0, 0, 0),
];

static POS5_PERFT_RESULTS: &'static [PerfResults] = &[
    PerfResults::new(1, 44, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(2, 1486, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(3, 62379, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(4, 2103487, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(5, 89941194, 0, 0, 0, 0, 0, 0, 0, 0),
];
