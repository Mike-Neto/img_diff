# img_diff

[![Linux/OSX Build Status](https://travis-ci.org/Mike-Neto/img_diff.svg?branch=master)](https://travis-ci.org/Mike-Neto/img_diff)
[![Windows Build Status](https://ci.appveyor.com/api/projects/status/afjuww52fyb2bd3g?svg=true)](https://ci.appveyor.com/project/Mike-Neto/img-diff)
[![Current Version](https://img.shields.io/crates/v/img_diff.svg)](https://crates.io/crates/img_diff)
[![License: MIT](https://img.shields.io/crates/l/img_diff.svg)](#license)

Rust based Command line tool to diff images in 2 structurally similar folders and output diff images.

The value outputted represent a percentage of how much the amount of pixel data differs from the hightest possible value for that given image in each pixel.

## Future Features

- Support multiple format's of images (JPEG).
- Allow for a threshold to output diff file.

### From the CLI WG

- Revise stdout & stderr efficiency (avoid flush by using a stream).
- Add logging (log crate)
- Add Progress bar (indicatif crate)
- Add types of output (convey crate)

## Usage

    img_diff -s path\to\images -d path\to\images\to\compare -f path\to\output\diff\images

Will go trough all the files in the -s dir and subdirectories and compare them to the ones in the -d outputting diff files if a difference is found to -f dir.

    -v

enables verbose mode and output to stderr in case of a difference found.

## Usage in CI

    img_diff -s path\to\images -d path\to\images\to\compare -f path\to\output\diff\images -v 2> results/output.txt

This will enable verbose output and enable the results of failed comparisons to be put into output.txt
We can use this to enable CI with
if [[ -s results/output.txt ]]; then exit 1; else exit 0; fi

## Compile from source

     git clone https://github.com/Mike-Neto/img_diff.git
     cd img_diff
     cargo build --release

## Build all files

    cargo build && cargo test && cargo test --benches
    cargo +beta build && cargo +beta test && cargo +beta test --benches
    cargo +nightly build && cargo +nightly test && cargo +nightly test --benches

## Test

    cargo test

## Docs

    cargo doc --open

## Crate

[Crates.io](https://crates.io/crates/img_diff)

## Download

You need [Rust](https://www.rust-lang.org)

    cargo install img_diff

You can also download a binary release for your platform on [github releases](https://github.com/Mike-Neto/img_diff/releases/latest)

## Publish process

    cargo bump (major|minor|patch)
    git commit -m "v4.0.0"
    git tag -a v4.0.0 -m "v4.0.0"
    git push --tags
    cargo publish

## Benchmarking and Profiling

### Dependencies

    sudo apt install valgrind kcachegrind

### Local Benchmarks

Will run all all benchmarks against local data.

    cargo clean
    git checkout master
    cargo bench
    git checkout $CURRENT_WORKING_BRANCH
    cargo bench

### Profiling changes

    cargo bench img_diff/subtract
    # Check for the binary name which will then be used as input bellow
    valgrind --tool=callgrind --dump-instr=yes --collect-jumps=yes --simulate-cache=yes target/release/deps/bench-f72b65412859cf2f --bench img_diff/subtract
    # Check for the above output file name
    kcachegrind callgrind.out.20282

## License

Copyright 2018 Miguel Mendes

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
