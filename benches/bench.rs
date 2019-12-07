use criterion::*;
use img_diff::{do_diff, subtract_image, Config};
use std::path::PathBuf;
use tempfile::tempdir;

fn end_to_end(c: &mut Criterion) {
    let diff = tempdir().unwrap();

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

fn diff(c: &mut Criterion) {
    let src_image = image::open("tests/bench_png/bench_png_src/rustacean-error.png").unwrap();
    let dest_image = image::open("tests/bench_png/bench_png_dest/rustacean-error.png").unwrap();

    let mut group = c.benchmark_group("img_diff");
    group.sample_size(30);
    group.bench_function("subtract", |b| {
        b.iter(|| {
            let (_diff_value, _diff_image) = subtract_image(&src_image, &dest_image);
        })
    });
    group.finish();
}

criterion_group!(benches, end_to_end, diff);
criterion_main!(benches);
