# img_diff

Rust based Command line tool to diff images in 2 structurally similar folders and output diff images.

Comparison is done using [the SSIM algorithm](https://ece.uwaterloo.ca/~z70wang/research/ssim/) at multiple weighed resolutions and relies on the [dssim](https://crates.io/crates/dssim) crate for the comparisons.

The value returned is 1/SSIM-1, where 0 means identical image, and >0 (unbounded) is amount of difference.

## Future Features

* Support multiple format's of images (jpg, bmp).
* Allow for a threshold to output diff file.

## Usage

    img_diff -s path\to\images -d path\to\images\to\compare -f path\to\output\diff\images

Will go trough all the files in the -s dir and subdir's and compare them to the ones in the -d outputing diff files if a difrence is found to -f dir.

## Build or Download

You need [Rust](https://www.rust-lang.org/en-US/install.html)

    cargo install img_diff
