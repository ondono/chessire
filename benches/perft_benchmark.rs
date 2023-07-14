use chessire::test::perft;
use chessire::BitBoardEngine;
use chessire::ChessEngine;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn perft_benchmark(c: &mut Criterion) {
    c.bench_function("chessire -r perft", |b| {
        b.iter(|| {
            let game = chessire::ChessGame::new();
            let mut engine = BitBoardEngine::new_engine(game);
            perft(black_box(5), black_box(0..5), &mut engine);
        })
    });
}

criterion_group!(benches, perft_benchmark);
criterion_main!(benches);
