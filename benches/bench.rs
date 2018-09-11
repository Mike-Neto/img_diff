#[macro_use]
extern crate criterion;
extern crate img_diff;

use criterion::Criterion;
use img_diff::{Config, visit_dirs, do_diff};
use std::path::PathBuf;


fn sync_bmp(c: &mut Criterion) {
    let config: Config = Config {
        src_dir: Some(PathBuf::from("tests/bench_bmp/bench_bmp_src")),
        dest_dir: Some(PathBuf::from("tests/bench_bmp/bench_bmp_dest")),
        diff_dir: Some(PathBuf::from("tests/bench_bmp/bench_bmp_diff")),
        help: false,
        verbose: false,
        async: false,
    };
    c.bench_function("sync_bmp", |b| {
        b.iter(|| visit_dirs(&config.src_dir.clone().unwrap(), &config));
    });
}

fn async_bmp(c: &mut Criterion) {
    let config: Config = Config {
        src_dir: Some(PathBuf::from("tests/bench_bmp/bench_bmp_src")),
        dest_dir: Some(PathBuf::from("tests/bench_bmp/bench_bmp_dest")),
        diff_dir: Some(PathBuf::from("tests/bench_bmp/bench_bmp_diff")),
        help: false,
        verbose: false,
        async: false,
    };
    c.bench_function("async_bmp", |b| { b.iter(|| do_diff(&config)); });
}

criterion_group!(benches, sync_bmp, async_bmp);
criterion_main!(benches);
