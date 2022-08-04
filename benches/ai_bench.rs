use criterion::{black_box, criterion_group, criterion_main, Criterion};
use shellfishlib::game::Board;
use shellfishlib::opponent::engine::eval;

use criterion::BenchmarkId;
use shellfishlib::consts::position_consts::BASE_POS;
use shellfishlib::mv::Move;

pub fn criterion_benchmark(c: &mut Criterion) {
    let board = Board::from_fen(String::from("rn2kb1r/p3qppp/2p2n2/1N2p1B1/2B1P3/1Q6/PPP2PPP/R3K2R b KQkq - 0 10"));
    let base = Board::from_fen(String::from(BASE_POS)).make_move(&Move::new_move(12, 28, false));
    let mut complicted = Board::from_fen(String::from("r2r2k1/pp3pp1/1qnpbn1p/1Bb1p3/4P3/1PNQ1N1P/PBP2PP1/R2R2K1 b - - 6 12"));
    complicted.castle_rights = [false, false, false, false];
    let mut group = c.benchmark_group("eval");
    // Configure Criterion.rs to detect smaller differences and increase sample size to improve
    // precision and counteract the resulting noise.
    group.significance_level(0.05).sample_size(10);
    group.bench_function("morphy-duke", |b| b.iter(|| eval(board.clone(), 4)));
    group.bench_function("base-pos", |b| b.iter(|| eval(base.clone(), 4)));
    group.bench_function("complicated", |b| b.iter(|| eval(complicted.clone(), 4)));

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);