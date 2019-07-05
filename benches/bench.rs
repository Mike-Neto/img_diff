#[macro_use]
extern crate criterion;

use criterion::Criterion;
use img_diff::{do_diff, Config};
use std::path::PathBuf;
use tempdir::TempDir;

fn bmp(c: &mut Criterion) {
    let diff = TempDir::new("bench_bmp_diff").unwrap();

    let config: Config = Config {
        src_dir: PathBuf::from("tests/bench_bmp/bench_bmp_src"),
        dest_dir: PathBuf::from("tests/bench_bmp/bench_bmp_dest"),
        diff_dir: PathBuf::from(diff.path().to_str().unwrap()),
        verbose: true,
    };
    c.bench_function("end_to_end_bmp", move |b| {
        b.iter(|| do_diff(&config));
    });
}

fn png(c: &mut Criterion) {
    let diff = TempDir::new("bench_png_diff").unwrap();

    let config: Config = Config {
        src_dir: PathBuf::from("tests/bench_png/bench_png_src"),
        dest_dir: PathBuf::from("tests/bench_png/bench_png_dest"),
        diff_dir: PathBuf::from(diff.path().to_str().unwrap()),
        verbose: true,
    };
    c.bench_function("end_to_end_png", move |b| {
        b.iter(|| do_diff(&config));
    });
}

criterion_group!(benches, bmp, png);
criterion_main!(benches);
