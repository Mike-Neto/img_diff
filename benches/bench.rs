#[macro_use]
extern crate criterion;

use criterion::Criterion;
use img_diff::{do_diff, Config};
use std::path::PathBuf;

fn bmp(c: &mut Criterion) {
    let config: Config = Config {
        src_dir: PathBuf::from("tests/bench_bmp/bench_bmp_src"),
        dest_dir: PathBuf::from("tests/bench_bmp/bench_bmp_dest"),
        diff_dir: PathBuf::from("tests/bench_bmp/bench_bmp_diff"),
        verbose: false,
    };
    c.bench_function("bmp", move |b| {
        b.iter(|| do_diff(&config));
    });
}

criterion_group!(benches, bmp);
criterion_main!(benches);
