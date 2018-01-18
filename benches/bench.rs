#[macro_use]
extern crate criterion;
extern crate img_diff;

use criterion::Criterion;
use img_diff::{Config, visit_dirs};
use std::path::PathBuf;


fn bmp(c: &mut Criterion) {
    let config: Config = Config {
        src_dir: Some(PathBuf::from("tests/bench_bmp/bench_bmp_src")),
        dest_dir: Some(PathBuf::from("tests/bench_bmp/bench_bmp_dest")),
        diff_dir: Some(PathBuf::from("tests/bench_bmp/bench_bmp_diff")),
        help: false,
        verbose: false,
    };
    c.bench_function("sync_bmp", |b| {
        b.iter(|| visit_dirs(&config.src_dir.clone().unwrap(), &config));
    });
}

criterion_group!(benches, bmp);
criterion_main!(benches);
