# img_diff

[![Linux/OSX Build Status](https://travis-ci.org/Mike-Neto/img_diff.svg?branch=master)](https://travis-ci.org/Mike-Neto/img_diff)
[![Windows Build Status](https://ci.appveyor.com/api/projects/status/afjuww52fyb2bd3g?svg=true)](https://ci.appveyor.com/project/Mike-Neto/img-diff)
[![Current Version](https://img.shields.io/crates/v/img_diff.svg)](https://crates.io/crates/img_diff)
[![dependency status](https://deps.rs/repo/github/Mike-Neto/img_diff/status.svg)](https://deps.rs/repo/github/Mike-Neto/img_diff)
[![License: MIT](https://img.shields.io/crates/l/img_diff.svg)](#license)

Rust based Command line tool to diff images in 2 structurally similar folders and output diff images.

TODO REDO ALL THIS DOCS
BMP files are compared using a by pixel sample algorithm and the output is the MOD of the difference between each of the
pixel components (rgb)

The value returned for bmp images 0 if images are equal and a positive number that scales with the amount of differences.
END TODO 

## Future Features
* Support multiple format's of images (JPEG).
* Allow for a threshold to output diff file.

### From the CLI WG
* Revise stdout & stderr efficiency (avoid flush by using a stream).
* Add logging (log crate)
* Add Progress bar (indicatif crate)
* Add types of output (convey crate)
* Generate a man page (clap crate)

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

### From 3.0.2
Removed Dssim and a bunch of other dependencies.

Now the value shown is a percentage. More details above.

Uniform value no matter the image type.

### From 3.0.1
Fixed some issues and migrated to using tools as recommended by the CLI WG
* Migrated to StructOpt
* Migrated to assert_cmd
* Added human friendly panic

Removed all unwraps and provide error messages.

Updated dependencies.

More typo fixes.

Updated future features with things from the CLI WG suggestions.

### From 3.0.0
Formatted using cargo fmt.

Fixed clippy issues.

Fixed typos and updated docs.

Updated dependencies.

### From 2.1.0
Removed Multi-threaded flag making that the default.

Upgraded to Rust Edition 2018

## License

Copyright 2018 Miguel Mendes

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.