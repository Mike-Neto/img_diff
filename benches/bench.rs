use criterion::*;
use img_diff::{do_diff, Config};
use std::path::PathBuf;
use tempdir::TempDir;

fn bench(c: &mut Criterion) {
    let diff = TempDir::new("bench_png_diff").unwrap();

    let config: Config = Config {
        src_dir: PathBuf::from("tests/bench_png/bench_png_src"),
        dest_dir: PathBuf::from("tests/bench_png/bench_png_dest"),
        diff_dir: PathBuf::from(diff.path().to_str().unwrap()),
        verbose: true,
    };

    let mut group = c.benchmark_group("img_diff");
    group.sample_size(20);
    group.bench_function("png", |b| b.iter(|| do_diff(&config)));

    group.bench_function("bmp", |b| b.iter(|| do_diff(&config)));
    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
