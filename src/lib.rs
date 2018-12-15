//! # img_diff
//!
//! `img_diff` is a cmd line tool to diff images in 2 folders
//! you can pass -h to see the help
//!
use bmp;
use dssim;
use dssim::*;
use imgref::*;
use lodepng;
use rayon::prelude::*;
use rgb;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
/// diff images in 2 structurally similar folders and output diff images
pub struct Config {
    /// the folder to read
    #[structopt(parse(from_os_str), short = "s")]
    pub src_dir: PathBuf,
    /// the folder to compare the read images
    #[structopt(parse(from_os_str), short = "d")]
    pub dest_dir: PathBuf,
    /// the folder to output the diff images if a diff is found
    #[structopt(parse(from_os_str), short = "f")]
    pub diff_dir: PathBuf,
    /// toggle verbose mode
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,
}

enum ImageType {
    BMP(Result<bmp::Image, bmp::BmpError>),
    PNG(Result<ImgVec<RGBAPLU>, lodepng::ffi::Error>),
}

struct Image {
    path: PathBuf,
    image: ImageType,
}

/// Diffs all images using a channel to parallelize the file IO and processing.
pub fn do_diff(config: &Config) -> io::Result<()> {
    // Get a full list of all images to load (scr and dest pairs)
    let files_to_load = find_all_files_to_load(config.src_dir.clone(), &config)?;

    // open a channel to load pairs of images from disk
    let (transmitter, receiver) = mpsc::channel();
    thread::spawn(move || {
        for (scr_path, dest_path) in files_to_load {
            let extension = scr_path
                .extension()
                .unwrap()
                .to_str()
                .unwrap()
                .to_lowercase();
            if extension == "bmp" {
                let src_img = Image {
                    path: scr_path.clone(),
                    image: ImageType::BMP(bmp::open(scr_path)),
                };
                let dest_img = Image {
                    path: dest_path.clone(),
                    image: ImageType::BMP(bmp::open(dest_path)),
                };

                transmitter.send((src_img, dest_img)).unwrap();
            } else {
                let src_img = Image {
                    path: scr_path.clone(),
                    image: ImageType::PNG(load(scr_path)),
                };
                let dest_img = Image {
                    path: dest_path.clone(),
                    image: ImageType::PNG(load(dest_path)),
                };

                transmitter.send((src_img, dest_img)).unwrap();
            }
        }
    });

    // do the comparison in the receiving channel
    for (src_img, dest_img) in receiver {
        match src_img.image {
            ImageType::BMP(src_image) => {
                let dest_image = match dest_img.image {
                    ImageType::BMP(dest_image) => dest_image,
                    ImageType::PNG(_) => panic!("Mismatched image types, expected BMP got PNG"),
                };
                if src_image.is_ok() && dest_image.is_ok() {
                    let src_bmp_img = src_image.unwrap();
                    let dest_bmp_img = dest_image.unwrap();
                    let mut diff_value = 0.0; //TODO(MiguelMendes): Give a meaning to this value
                    if src_bmp_img.get_width() != dest_bmp_img.get_width()
                        || src_bmp_img.get_height() != dest_bmp_img.get_height()
                    {
                        diff_value = 1.0; // Any value to flag it to output
                        println!("Images have different dimensions, skipping comparison");
                    } else {
                        let mut diff_image =
                            bmp::Image::new(src_bmp_img.get_width(), src_bmp_img.get_height());
                        for (x, y) in src_bmp_img.coordinates() {
                            let dest_pixel = dest_bmp_img.get_pixel(x, y);
                            let src_pixel = src_bmp_img.get_pixel(x, y);
                            let diff_pixel = subtract(src_pixel, dest_pixel);
                            diff_value += interpolate(diff_pixel);
                            diff_image.set_pixel(x, y, diff_pixel);
                        }
                        let diff_file_name = get_diff_file_name_and_validate_path(
                            dest_img.path.to_str().unwrap(),
                            config,
                        );
                        // Use another tread to write the files as necessary
                        let handle =
                            thread::spawn(move || output_bmp(&diff_file_name, Some(diff_image)));
                        handle.join().unwrap();
                    }
                    print_diff_result(config.verbose, &src_img.path, diff_value);

                    if diff_value != 0.0 && config.verbose {
                        eprintln!(
                            "diff found in file: {:?}",
                            String::from(src_img.path.to_str().unwrap())
                        );
                    }
                }
            }
            // TODO(MiguelMendes): @Cleanup duplicated verbose messages
            ImageType::PNG(src_image) => {
                let dest_image = match dest_img.image {
                    ImageType::PNG(dest_image) => dest_image,
                    ImageType::BMP(_) => panic!("Mismatched image types, expected PNG got BMP"),
                };
                if src_image.is_ok() && dest_image.is_ok() {
                    let src_png_img = src_image.unwrap();
                    let dest_png_img = dest_image.unwrap();

                    if src_png_img.width() != dest_png_img.width()
                        || src_png_img.height() != dest_png_img.height()
                    {
                        println!("Images have different dimensions, skipping comparison");
                        if config.verbose {
                            eprintln!(
                                "diff found in file: {:?}",
                                String::from(src_img.path.to_str().unwrap())
                            );
                        }
                    } else {
                        let mut attr = dssim::Dssim::new();
                        attr.set_save_ssim_maps(1);
                        let (ssim_diff_value, ssim_maps) = attr.compare(
                            &attr.create_image(&src_png_img).unwrap(),
                            attr.create_image(&dest_png_img).unwrap(),
                        );
                        print_diff_result(config.verbose, &src_img.path, ssim_diff_value);
                        if ssim_diff_value != 0.0 {
                            let diff_file_name = get_diff_file_name_and_validate_path(
                                dest_img.path.to_str().unwrap(),
                                config,
                            );
                            output_diff_files(&diff_file_name, &ssim_maps);
                            if config.verbose {
                                eprintln!(
                                    "diff found in file: {:?}",
                                    String::from(src_img.path.to_str().unwrap())
                                );
                            }
                        }
                    }
                }
            }
        };
    }

    Ok(())
}

fn find_all_files_to_load(dir: PathBuf, config: &Config) -> io::Result<Vec<(PathBuf, PathBuf)>> {
    let mut files: Vec<(PathBuf, PathBuf)> = vec![];
    // TODO(MiguelMendes): Better error msg for when folders are missing
    for entry in fs::read_dir(dir)? {
        let entry = entry.unwrap().path();
        if entry.is_file() {
            //TODO(MiguelMendes): Clone fest @clean-up
            let dest_file_name = entry.to_str().unwrap().replace(
                config.src_dir.clone().to_str().unwrap(),
                config.dest_dir.clone().to_str().unwrap(),
            );
            let dest_path = PathBuf::from(dest_file_name);
            if dest_path.exists() {
                files.push((entry, dest_path));
            }
        } else {
            let child_files = find_all_files_to_load(entry, &config)?;
            //TODO(MiguelMendes): 1 liner for this? // join vec?
            for child in child_files {
                files.push(child);
            }
        }
    }
    Ok(files)
}

/// helper to create necessary folders for IO operations to be successful
fn get_diff_file_name_and_validate_path(dest_file_name: &str, config: &Config) -> String {
    let diff_file_name = dest_file_name.replace(
        config.dest_dir.clone().to_str().unwrap(),
        config.diff_dir.clone().to_str().unwrap(),
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
fn output_bmp(path_name: &str, image: Option<bmp::Image>) {
    if let Some(image) = image {
        image
            .save(&path_name)
            .unwrap_or_else(|e| eprintln!("Failed to save diff_file: {}\nError: {}", path_name, e));
    }
}

/// print diff result
fn print_diff_result<T: std::fmt::Debug>(verbose: bool, entry: &PathBuf, diff_value: T) {
    if verbose {
        println!(
            "compared file: {:?} had diff value of: {:?}",
            entry, diff_value
        );
    } else {
        println!("{:?}", diff_value);
    }
}

fn interpolate(p: bmp::Pixel) -> f32 {
    f32::from((p.r / 3) + (p.g / 3) + (p.b / 3)) / 10_000_000.0
}

fn subtract(p1: bmp::Pixel, p2: bmp::Pixel) -> bmp::Pixel {
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
    if buffer.pop() {
        let temp_buffer = buffer.clone();

        create_dir_if_not_there(temp_buffer);
        if !buffer.exists() && buffer != Path::new("") {
            fs::create_dir(&buffer).unwrap();
        }
    }
    buffer
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
fn output_diff_files(path_name: &str, ssim_maps: &[SsimMap]) {
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
