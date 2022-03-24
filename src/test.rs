use crate::*;
use chessire_utils::*;
use itertools::Itertools;
use std::ops::Range;

pub fn perft<T>(depth: usize, tests_range: Range<usize>, engine: &mut T)
where
    T: ChessEngine,
{
    let positions = vec![POSITION1, POSITION2, POSITION3, POSITION4, POSITION5];
    let pos_res = vec![
        POS1_PERFT_RESULTS,
        POS2_PERFT_RESULTS,
        POS3_PERFT_RESULTS,
        POS4_PERFT_RESULTS,
        POS5_PERFT_RESULTS,
    ];

    let count_pos = positions.len();
    let count_pos_res = pos_res.len();

    let count = usize::min(count_pos, count_pos_res);

    for test_case in tests_range {
        if test_case > count - 1 {
            continue;
        }
        println!(
            "Starting validation of test position {}: {}",
            test_case + 1,
            positions[test_case]
        );

        let mut game = ChessGame::new();
        game.apply_fen(positions[test_case])
            .unwrap_or_else(|_| panic!("error while parsing FEN string {}", positions[test_case]));
        for ply in 1..depth + 1 {
            use std::time::Instant;
            use termion::color;
            let now = Instant::now();
            let mut nodes = 0;
            engine.perft(ply, &mut nodes, false);
            let elapsed = now.elapsed().as_millis();
            let expected_res = (pos_res[test_case])[ply - 1].get_nodes();

            if nodes == expected_res {
                println!(
                    "> {}Depth:{}{} ply\tNumber of positions:{}{:>9}{}\ttime:{:>6}ms",
                    color::Fg(color::Green),
                    color::Fg(color::Reset),
                    ply,
                    color::Fg(color::Red),
                    nodes,
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
                    nodes,
                    color::Fg(color::Reset),
                    color::Fg(color::Red),
                    expected_res,
                    color::Fg(color::Reset),
                    nodes as i128 - expected_res as i128,
                    elapsed,
                );
            }
        }
    }
}

pub fn perft_details<T>(depth: usize, engine: &mut T)
where
    T: ChessEngine,
{
    let mut nodes = 0;
    engine.perft(depth, &mut nodes, true);
    println!("Nodes searched: {}", nodes);
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
static POS1_PERFT_RESULTS: &[PerfResults] = &[
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

static POS2_PERFT_RESULTS: &[PerfResults] = &[
    PerfResults::new(1, 48, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(2, 2039, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(3, 97862, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(4, 4085603, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(5, 193690690, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(6, 8031647685, 0, 0, 0, 0, 0, 0, 0, 0),
];

static POS3_PERFT_RESULTS: &[PerfResults] = &[
    PerfResults::new(1, 14, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(2, 191, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(3, 2812, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(4, 43238, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(5, 43238, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(6, 11030083, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(7, 178633661, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(8, 3009794393, 0, 0, 0, 0, 0, 0, 0, 0),
];

static POS4_PERFT_RESULTS: &[PerfResults] = &[
    PerfResults::new(1, 6, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(2, 264, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(3, 9467, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(4, 422333, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(5, 15833292, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(6, 15833292, 0, 0, 0, 0, 0, 0, 0, 0),
];

static POS5_PERFT_RESULTS: &[PerfResults] = &[
    PerfResults::new(1, 44, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(2, 1486, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(3, 62379, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(4, 2103487, 0, 0, 0, 0, 0, 0, 0, 0),
    PerfResults::new(5, 89941194, 0, 0, 0, 0, 0, 0, 0, 0),
];
