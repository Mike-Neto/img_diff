//! # img_diff
//!
//! `img_diff` is a cmd line tool to diff images in 2 folders
//! you can pass -h to see the help
//!
extern crate getopts;
extern crate dssim;
extern crate rayon;
extern crate rgb;
extern crate lodepng;
extern crate imgref;
extern crate bmp;

use std::path::{Path, PathBuf};
use getopts::Options;
use std::fs;
use std::io;
use dssim::*;
use rayon::prelude::*;
use imgref::*;

/// Config includes all the variables in this application
#[derive(Debug)]
pub struct Config {
    /// the folder to read
    pub src_dir: Option<PathBuf>,
    /// the folder to compare the read images
    pub dest_dir: Option<PathBuf>,
    /// the folder to output the diff images if a diff is found
    pub diff_dir: Option<PathBuf>,
    /// toogle verbose mode
    pub verbose: bool,
    /// toogle help mode
    pub help: bool,
}

impl Config {
    /// Config contructor, takes the env args as string vec
    pub fn new(args: &[String]) -> Config {
        let mut opts = Options::new();
        opts.optopt("s", "srcDir", "set source dir name", "");
        opts.optopt("d", "destDir", "set output dir name", "");
        opts.optopt("f", "diffDir", "set diff dir name", "");
        opts.optflag("v", "verbose", "toogle verbose mode");
        opts.optflag("h", "help", "print this help menu");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => panic!(f.to_string()),
        };

        let verbose = matches.opt_present("v");

        let help = matches.opt_present("h");

        let src_dir: Option<PathBuf> = match matches.opt_str("s") {
            Some(string) => Some(PathBuf::from(string)),
            None => None,
        };

        let dest_dir: Option<PathBuf> = match matches.opt_str("d") {
            Some(string) => Some(PathBuf::from(string)),
            None => None,
        };

        let diff_dir: Option<PathBuf> = match matches.opt_str("f") {
            Some(string) => Some(PathBuf::from(string)),
            None => None,
        };

        Config {
            src_dir,
            dest_dir,
            diff_dir,
            verbose,
            help,
        }
    }
}

/// Recursive method that does the comparison for each image in each folder
pub fn visit_dirs(dir: &PathBuf, config: &Config) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, &config)?;
            } else {
                let dest_file_name =
                    entry.path().to_str().unwrap().replace(
                        config.src_dir.clone().unwrap().to_str().unwrap(),
                        config
                            .dest_dir
                            .clone()
                            .unwrap()
                            .to_str()
                            .unwrap(),
                    );

                if Path::new(&dest_file_name).exists() {
                    let file_extension = entry
                        .path()
                        .extension()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_lowercase();
                    if file_extension == "bmp" {
                        let (diff_value, diff_image) = compare_bmp(&entry, &dest_file_name);
                        print_diff_result(config.verbose, &entry, diff_value);
                        if diff_value != 0.0 {
                            let diff_file_name =
                                get_diff_file_name_and_validate_path(dest_file_name, config);
                            output_bmp(diff_file_name, diff_image);
                            if config.verbose {
                                eprintln!("diff found in file: {:?}", entry.path());
                            }
                        }
                    } else {
                        let mut attr = dssim::Dssim::new();
                        let g1 = attr.create_image(&load(entry.path()).unwrap()).unwrap();
                        let g2 = attr.create_image(&load(&dest_file_name).unwrap()).unwrap();
                        attr.set_save_ssim_maps(1);
                        let (ssim_diff_value, ssim_maps) = attr.compare(&g1, g2);
                        print_diff_result(config.verbose, &entry, ssim_diff_value);
                        if ssim_diff_value != 0.0 {
                            let diff_file_name =
                                get_diff_file_name_and_validate_path(dest_file_name, config);
                            output_diff_files(diff_file_name, ssim_maps);
                        }
                        if config.verbose {
                            eprintln!("diff found in file: {:?}", entry.path());
                        }
                    }
                }
            }
        }
    }
    Ok(())
}


/// Parallel and diffrent algorithm implementation of visit_dirs
pub fn do_diff(config: &Config) -> io::Result<()> {
    // Get a full list of all images to load (scr and dest pairs)
    let files_to_load = find_all_files_to_load(&config);
    // open a channel for pairs of loaded images
    // do the comparison in the recivieng channel
    // send to another channel to write the diff file if necessary

    Ok(())
}

fn find_all_files_to_load(config: &Config) {
    unimplemented!();
}

/// helper to create necessary folders for IO operations to be successfull
fn get_diff_file_name_and_validate_path(dest_file_name: String, config: &Config) -> String {
    let diff_file_name = dest_file_name.replace(
        config.dest_dir.clone().unwrap().to_str().unwrap(),
        config.diff_dir.clone().unwrap().to_str().unwrap(),
    );
    {

        let diff_path = Path::new(&diff_file_name);
        let diff_path_dir = diff_path.parent().unwrap();
        if !diff_path_dir.exists() {
            if config.verbose {
                println!("creating directory: {:?}", diff_path_dir);
            }
            create_path(diff_path);
        }
    }
    diff_file_name
}

/// saves bmp file diff to disk
fn output_bmp(path_name: String, image: Option<bmp::Image>) {
    match image {
        Some(image) => {
            let _ = image.save(&path_name).unwrap_or_else(|e| {
                eprintln!("Failed to save diff_file: {}\nError: {}", path_name, e)
            });
        }
        None => (),
    }
}

/// print diff result
fn print_diff_result<T: std::fmt::Debug>(verbose: bool, entry: &fs::DirEntry, diff_value: T) {
    if verbose {
        println!(
            "compared file: {:?} had diff value of: {:?}",
            entry.path(),
            diff_value
        );
    } else {
        println!("{:?}", diff_value);
    }
}

/// load and compare bmp files
fn compare_bmp(entry: &fs::DirEntry, dest_file_name: &String) -> (f32, Option<bmp::Image>) {
    let src_img = bmp::open(entry.path());
    let dest_img = bmp::open(dest_file_name);
    let mut diff_value = 0.0;

    if src_img.is_ok() && dest_img.is_ok() {
        let src_img = src_img.unwrap();
        let dest_img = dest_img.unwrap();

        let mut diff_image = bmp::Image::new(src_img.get_width(), src_img.get_height());
        //TODO(MiguelMendes): Thread this
        for (x, y) in src_img.coordinates() {
            let dest_pixel = dest_img.get_pixel(x, y);
            let src_pixel = src_img.get_pixel(x, y);
            let diff_pixel = subtract(&src_pixel, &dest_pixel);
            diff_value += interpolate(&diff_pixel);
            diff_image.set_pixel(x, y, diff_pixel);
        }
        return (diff_value, Some(diff_image));
    }

    (diff_value, None)
}

fn interpolate(p: &bmp::Pixel) -> f32 {
    ((p.r / 3) + (p.g / 3) + (p.b / 3)) as f32 / 10000000.0
}

fn subtract(p1: &bmp::Pixel, p2: &bmp::Pixel) -> bmp::Pixel {
    let r;
    let g;
    let b;

    if p1.r >= p2.r {
        r = p1.r - p2.r;
    } else {
        r = p2.r - p1.r
    }
    if p1.g >= p2.g {
        g = p1.g - p2.g;
    } else {
        g = p2.g - p1.g
    }
    if p1.b >= p2.b {
        b = p1.b - p2.b;
    } else {
        b = p2.b - p1.b
    }

    bmp::Pixel { r, g, b }
}

/// Helper to create folder hierarchies
fn create_path(path: &Path) {
    let mut buffer = path.to_path_buf();
    if buffer.is_file() {
        buffer.pop();
    }
    create_dir_if_not_there(buffer);
}

/// recursive way to create folders hierarchies
fn create_dir_if_not_there(mut buffer: PathBuf) -> PathBuf {
    if buffer == Path::new("") {
        buffer
    } else {
        buffer.pop();
        let temp_buffer = buffer.clone();

        create_dir_if_not_there(temp_buffer);
        if !buffer.exists() && buffer != Path::new("") {
            fs::create_dir(&buffer).unwrap();
        }
        buffer
    }
}

/// Helper float to byte
fn to_byte(i: f32) -> u8 {
    if i <= 0.0 {
        0
    } else if i >= 255.0 / 256.0 {
        255
    } else {
        (i * 256.0) as u8
    }
}

/// Creates a saves a png with the img diff to config.diff folder
fn output_diff_files(path_name: String, ssim_maps: Vec<SsimMap>) {
    ssim_maps.par_iter().enumerate().for_each(|(n, map_meta)| {
        let avgssim = map_meta.ssim as f32;
        let out: Vec<_> = map_meta
            .map
            .pixels()
            .map(|ssim| {
                let max = 1_f32 - ssim;
                let maxsq = max * max;
                rgb::RGBA8 {
                    r: to_byte(maxsq * 16.0),
                    g: to_byte(max * 3.0),
                    b: to_byte(max / ((1_f32 - avgssim) * 4_f32)),
                    a: 255,
                }
            })
            .collect();

        let write_res = lodepng::encode32_file(
            format!("{}-{}.png", path_name, n),
            &out,
            map_meta.map.width(),
            map_meta.map.height(),
        );
        if write_res.is_err() {
            eprintln!("Can't write {}: {:?}", path_name, write_res);
            std::process::exit(1);
        }
    });
}

/// Helper load images as png from path
fn load<P: AsRef<Path>>(path: P) -> Result<ImgVec<RGBAPLU>, lodepng::Error> {
    let image = lodepng::decode32_file(path.as_ref())?;
    Ok(Img::new(
        image.buffer.to_rgbaplu(),
        image.width,
        image.height,
    ))
}
