#[macro_use]
extern crate bencher;

use bencher::Bencher;
extern crate assert_cli;

fn bmp(bench: &mut Bencher) {
    bench.iter(|| {
        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "-s",
                    "tests/bench_bmp/bench_bmp_src",
                    "-d",
                    "tests/bench_bmp/bench_bmp_dest",
                    "-f",
                    "tests/bench_bmp/bench_bmp_diff",
                ],
            )
            .succeeds();
    });
    bench.bytes = 0 as u64;
}

benchmark_group!(benches, bmp);
benchmark_main!(benches);
