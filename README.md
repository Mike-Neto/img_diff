# img_diff

[![Linux/OSX Build Status](https://travis-ci.org/Mike-Neto/img_diff.svg?branch=master)](https://travis-ci.org/Mike-Neto/img_diff)
[![Windows Build Status](https://ci.appveyor.com/api/projects/status/afjuww52fyb2bd3g?svg=true)](https://ci.appveyor.com/project/Mike-Neto/img-diff)
[![Current Version](https://img.shields.io/crates/v/img_diff.svg)](https://crates.io/crates/img_diff)
[![License: MIT](https://img.shields.io/crates/l/img_diff.svg)](#license)

Rust based Command line tool to diff images in 2 structurally similar folders and output diff images.

Comparison is done using [the SSIM algorithm](https://ece.uwaterloo.ca/~z70wang/research/ssim/) at multiple weighed resolutions and relies on the [dssim](https://crates.io/crates/dssim) crate for the comparisons of png images.

BMP files are compared using a by pixel sample algorithm and the output is the MOD of the difference between each of the
pixel components (rgb)

The value returned is 1/SSIM-1, where 0 means identical image, and >0 (unbounded) is amount of difference for PNG.

The value returned for bmp images 0 if images are equal and a positive number that scales with the amount of differences.

## Future Features

* Support multiple format's of images (JPEG).
* Allow for a threshold to output diff file.
* Provide a single unit of difference.

* Revise stdout & stderr efficiency.
* Add logging (log crate)
* Add Progress bar (indicatif crate)

## Usage

    img_diff -s path\to\images -d path\to\images\to\compare -f path\to\output\diff\images

Will go trough all the files in the -s dir and subdirectories and compare them to the ones in the -d outputting diff files if a difference is found to -f dir.

	-v

enables verbose mode and output to stderr.


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
	cargo build && cargo test && cargo bench --no-run
    cargo +beta build && cargo +beta test && cargo +beta bench --no-run
	cargo +nightly build && cargo +nightly test && cargo +nightly bench --no-run

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

## Changelog

### From 2.1.0
Removed Multi-threaded flag making that the default.

Upgraded to Rust Edition 2018

## License

Copyright 2018 Miguel Mendes

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.