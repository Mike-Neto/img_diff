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
                    let mut attr = dssim::Dssim::new();
                    let g1 = attr.create_image(&load(entry.path()).unwrap()).unwrap();
                    let g2 = attr.create_image(&load(&dest_file_name).unwrap()).unwrap();
                    attr.set_save_ssim_maps(1);
                    let (diff, ssim_maps) = attr.compare(&g1, g2);
                    if config.verbose {
                        println!(
                            "compared file: {:?} had diff value of: {:?}",
                            entry.path(),
                            diff
                        );
                    } else {
                        println!("{:?}", diff);
                    }
                    if diff != 0.0 {
                        let diff_file_name =
                            dest_file_name.replace(
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
                        output_diff_files(diff_file_name, ssim_maps);
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
