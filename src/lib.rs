//! # img_diff
//!
//! `img_diff` is a cmd line tool to diff images in 2 folders
//! you can pass -h to see the help
//!
use image::{DynamicImage, GenericImage, GenericImageView, ImageResult};
use std::fs::{create_dir, read_dir, File};
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

struct DiffImage {
    path: PathBuf,
    image: ImageResult<DynamicImage>,
}

struct Pair<T> {
    src: T,
    dest: T,
}

fn output_diff_file(
    diff_image: DynamicImage,
    diff_value: f64,
    config: &Config,
    src_path: PathBuf,
    dest_path: PathBuf,
) {
    if diff_value != 0.0 {
        if let Some(path) = dest_path.to_str() {
            let diff_file_name = get_diff_file_name_and_validate_path(path, config);
            match diff_file_name {
                Some(diff_file_name) => {
                    // Use another tread to write the files as necessary
                    let file_out = &mut File::create(&Path::new(&diff_file_name)).unwrap();
                    diff_image.write_to(file_out, image::PNG).unwrap();

                    if config.verbose {
                        if let Some(path) = src_path.to_str() {
                            eprintln!("diff found in file: {:?}", String::from(path));
                        } else {
                            eprintln!("failed to convert path to string: {:?}", src_path);
                        }
                    }
                }
                None => {
                    eprintln!("Could not write diff file");
                }
            }
        } else {
            eprintln!("Failed to convert {:?} to string", dest_path);
        }
    }
}

fn subtract_image(a: &DynamicImage, b: &DynamicImage) -> (f64, DynamicImage) {
    let dim = a.dimensions();
    let mut diff_image = DynamicImage::new_rgba8(dim.0, dim.1);
    let max_value: f64 = dim.0 as f64 * dim.1 as f64 * 4.0 * 255.0;
    let mut current_value: f64 = 0.0;
    for ((x, y, pixel_a), (_, _, pixel_b)) in a.pixels().zip(b.pixels()) {
        let r = 255 - subtract_and_prevent_overflow(pixel_a[0], pixel_b[0]);
        let g = 255 - subtract_and_prevent_overflow(pixel_a[1], pixel_b[1]);
        let b = 255 - subtract_and_prevent_overflow(pixel_a[2], pixel_b[2]);
        let a = 255 - subtract_and_prevent_overflow(pixel_a[3], pixel_b[3]);
        current_value += r as f64;
        current_value += g as f64;
        current_value += b as f64;
        current_value += a as f64;
        diff_image.put_pixel(x, y, image::Rgba([r, g, b, a]));
    }
    (100.0 - (max_value / current_value * 100.0), diff_image)
}

fn subtract_and_prevent_overflow(a: u8, b: u8) -> u8 {
    if a > b {
        a - b
    } else {
        b - a
    }
}

/// Diffs all images using a channel to parallelize the file IO and processing.
pub fn do_diff(config: &Config) -> io::Result<()> {
    // Get a full list of all images to load (scr and dest pairs)
    let files_to_load = find_all_files_to_load(config.src_dir.clone(), &config)?;

    // open a channel to load pairs of images from disk
    let (transmitter, receiver) = mpsc::channel();
    thread::spawn(move || {
        for (scr_path, dest_path) in files_to_load {
            if let Err(err) = transmitter.send(Pair {
                src: DiffImage {
                    path: scr_path.clone(),
                    image: image::open(scr_path),
                },
                dest: DiffImage {
                    path: dest_path.clone(),
                    image: image::open(dest_path),
                },
            }) {
                eprintln!("Could not send using channel: {:?}", err);
            };
        }
    });

    // do the comparison in the receiving channel
    for pair in receiver {
        match (pair.src.image, pair.dest.image) {
            (Ok(src_image), Ok(dest_image)) => {
                if src_image.dimensions() != dest_image.dimensions() {
                    print_dimensions_error(config, &pair.src.path);
                } else {
                    let (diff_value, diff_image) = subtract_image(&src_image, &dest_image);
                    print_diff_result(config.verbose, &pair.src.path, diff_value);
                    output_diff_file(
                        diff_image,
                        diff_value,
                        config,
                        pair.src.path,
                        pair.dest.path,
                    );
                }
            }
            (Err(err), _) => eprintln!("Failed to open src img: {:?}", err),
            (_, Err(err)) => eprintln!("Failed to open dest img: {:?}", err),
        };
    }

    Ok(())
}

/// Recursively finds all files to compare based on the directory
fn find_all_files_to_load(dir: PathBuf, config: &Config) -> io::Result<Vec<(PathBuf, PathBuf)>> {
    let mut files: Vec<(PathBuf, PathBuf)> = vec![];
    match read_dir(dir) {
        Err(err) => eprintln!("Could not read dir: {:?}", err),
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Err(err) => eprintln!("Error in dir entry: {:?}", err),
                    Ok(entry) => {
                        let entry = entry.path();
                        if entry.is_file() {
                            let entry_name = entry.to_str();
                            let scr_name = config.src_dir.to_str();
                            let dest_name = config.dest_dir.to_str();
                            match (entry_name, scr_name, dest_name) {
                                (Some(entry_name), Some(scr_name), Some(dest_name)) => {
                                    let dest_file_name = entry_name.replace(scr_name, dest_name);
                                    let dest_path = PathBuf::from(dest_file_name);
                                    if dest_path.exists() {
                                        files.push((entry, dest_path));
                                    }
                                }
                                _ => {
                                    eprint!("Failed to convert to path to string: ");
                                    if entry_name.is_none() {
                                        eprintln!("{:?}", entry);
                                    }
                                    if scr_name.is_none() {
                                        eprintln!("{:?}", config.src_dir);
                                    }
                                    if dest_name.is_none() {
                                        eprintln!("{:?}", config.dest_dir);
                                    }
                                }
                            }
                        } else {
                            let child_files = find_all_files_to_load(entry, &config)?;
                            //TODO(MiguelMendes): 1 liner for this? // join vec?
                            for child in child_files {
                                files.push(child);
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(files)
}

/// helper to create necessary folders for IO operations to be successful
fn get_diff_file_name_and_validate_path(dest_file_name: &str, config: &Config) -> Option<String> {
    let scr_name = config.src_dir.to_str();
    let dest_name = config.dest_dir.to_str();
    let diff_name = config.diff_dir.to_str();

    match (dest_name, diff_name) {
        (Some(dest_name), Some(diff_name)) => {
            let diff_file_name = dest_file_name.replace(dest_name, diff_name);
            let diff_path = Path::new(&diff_file_name);

            if let Some(diff_path_dir) = diff_path.parent() {
                if !diff_path_dir.exists() {
                    if config.verbose {
                        println!("creating directory: {:?}", diff_path_dir);
                    }
                    create_path(diff_path);
                }
            }
            Some(diff_file_name)
        }
        _ => {
            eprint!("Failed to convert to path to string: ");
            if scr_name.is_none() {
                eprintln!("{:?}", config.src_dir);
            }
            if dest_name.is_none() {
                eprintln!("{:?}", config.dest_dir);
            }
            if diff_name.is_none() {
                eprintln!("{:?}", config.diff_dir);
            }
            None
        }
    }
}

/// print diff result
fn print_diff_result(verbose: bool, entry: &PathBuf, diff_value: f64) {
    if verbose {
        println!(
            "compared file: {:?} had diff value of: {:?}%",
            entry, diff_value
        );
    } else {
        println!("{:?}%", diff_value);
    }
}

/// print dimensions errors
fn print_dimensions_error(config: &Config, path: &PathBuf) {
    println!("Images have different dimensions, skipping comparison");
    if config.verbose {
        if let Some(path) = path.to_str() {
            eprintln!("diff found in file: {:?}", path);
        } else {
            eprintln!("failed to convert path to string: {:?}", path);
        }
    }
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
            if let Err(err) = create_dir(&buffer) {
                eprintln!("Failed to create directory: {:?}", err);
            }
        }
    }
    buffer
}
