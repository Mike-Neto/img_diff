//! # img_diff
//!
//! `img_diff` is a cmd line tool to diff images in 2 folders
//! you can pass -h to see the help
//!
use anyhow::Result;
use clap::Parser;
use image::{DynamicImage, GenericImage, GenericImageView, ImageResult};
use log::{info, warn};
use std::cmp;
use std::fs::{create_dir, read_dir, File};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;

#[derive(Debug, Parser)]
/// diff images in 2 structurally similar folders and output diff images
pub struct Config {
    /// the folder to read
    #[arg(short, long)]
    pub src_dir: PathBuf,
    /// the folder to compare the read images
    #[arg(short, long)]
    pub dest_dir: PathBuf,
    /// the folder to output the diff images if a diff is found
    #[arg(short = 'f', long)]
    pub diff_dir: PathBuf,
    /// toggle verbose mode
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

pub struct DiffImage {
    path: PathBuf,
    image: ImageResult<DynamicImage>,
}

fn output_diff_file(
    diff_image: DynamicImage,
    diff_value: f64,
    config: &Config,
    src_path: PathBuf,
    dest_path: PathBuf,
) -> Result<()> {
    if diff_value != 0.0 {
        let diff_file_name =
            get_diff_file_name_and_validate_path(&dest_path.to_string_lossy(), config)?;
        let file_out = &mut File::create(&Path::new(&diff_file_name))?;
        diff_image.write_to(file_out, image::ImageOutputFormat::Png)?;

        if config.verbose.log_level_filter() > log::LevelFilter::Error {
            if let Some(path) = src_path.to_str() {
                eprintln!("diff found in file: {:?}", String::from(path));
            } else {
                eprintln!("failed to convert path to string: {:?}", src_path);
            }
        }
    }
    Ok(())
}

pub fn subtract_image(a: &DynamicImage, b: &DynamicImage) -> (f64, DynamicImage) {
    let (x_dim, y_dim) = a.dimensions();
    let mut diff_image = DynamicImage::new_rgba8(x_dim, y_dim);
    let mut max_value: f64 = 0.0;
    let mut current_value: f64 = 0.0;
    for ((x, y, pixel_a), (_, _, pixel_b)) in a.pixels().zip(b.pixels()) {
        let mut pixel_other: Vec<u8> = vec![0; 4];
        for i in 0..pixel_other.len() {
            max_value += f64::from(cmp::max(pixel_a[i], pixel_b[i]));
            pixel_other[i] = subtract_and_prevent_overflow(pixel_a[i], pixel_b[i]);
            current_value += f64::from(pixel_other[i]);
        }
        diff_image.put_pixel(
            x,
            y,
            image::Rgba([
                255 - pixel_other[0],
                255 - pixel_other[1],
                255 - pixel_other[2],
                255 - pixel_other[3],
            ]),
        );
    }
    (((current_value * 100.0) / max_value), diff_image)
}

fn subtract_and_prevent_overflow<T: Ord + std::ops::Sub<Output = T>>(a: T, b: T) -> T {
    if a > b {
        a - b
    } else {
        b - a
    }
}

/// Diffs all images using a channel to parallelize the file IO and processing.
pub fn do_diff(config: &Config) -> Result<()> {
    // Get a full list of all images to load (scr and dest pairs)
    let files_to_load = find_all_files_to_load(&config.src_dir, &config)?;

    // open a channel to load pairs of images from disk
    let (transmitter, receiver) = mpsc::channel();
    thread::spawn(move || -> Result<()> {
        for (scr_path, dest_path) in files_to_load {
            transmitter.send((
                DiffImage {
                    path: scr_path.clone(),
                    image: image::open(scr_path),
                },
                DiffImage {
                    path: dest_path.clone(),
                    image: image::open(dest_path),
                },
            ))?
        }
        Ok(())
    });

    // do the comparison in the receiving channel
    for (src, dest) in receiver {
        let src_image = src.image?;
        let dest_image = dest.image?;
        if src_image.dimensions() != dest_image.dimensions() {
            print_dimensions_error(config, &src.path)?;
        } else {
            let (diff_value, diff_image) = subtract_image(&src_image, &dest_image);
            print_diff_result(&src.path, diff_value);
            output_diff_file(diff_image, diff_value, config, src.path, dest.path)?
        }
    }

    Ok(())
}

/// Recursively finds all files to compare based on the directory
fn find_all_files_to_load(dir: &PathBuf, config: &Config) -> Result<Vec<(PathBuf, PathBuf)>> {
    let mut files: Vec<(PathBuf, PathBuf)> = vec![];
    let entries = read_dir(dir)?;
    for entry in entries {
        let path = entry?.path();
        if path.is_file() {
            let entry_name = path.to_string_lossy();
            let scr_name = config.src_dir.to_string_lossy();
            let dest_name = config.dest_dir.to_string_lossy();
            let dest_file_name = entry_name.replace(scr_name.as_ref(), dest_name.as_ref());
            let dest_path = PathBuf::from(dest_file_name);
            if dest_path.exists() {
                files.push((path, dest_path));
            }
        } else {
            let child_files = find_all_files_to_load(&path, &config)?;
            files.extend(child_files);
        }
    }

    Ok(files)
}

/// helper to create necessary folders for IO operations to be successful
fn get_diff_file_name_and_validate_path(dest_file_name: &str, config: &Config) -> Result<String> {
    let dest_name = config.dest_dir.to_string_lossy();
    let diff_name = config.diff_dir.to_string_lossy();

    let diff_file_name = dest_file_name.replace(dest_name.as_ref(), diff_name.as_ref());
    let diff_path = Path::new(&diff_file_name);

    if let Some(diff_path_dir) = diff_path.parent() {
        if !diff_path_dir.exists() {
            info!("creating directory: {:?}", diff_path_dir);
            create_path(diff_path)?;
        }
    }
    Ok(diff_file_name)
}

/// print diff result
fn print_diff_result(entry: &PathBuf, diff_value: f64) {
    info!(
        "compared file: {:?} had diff value of: {:?}%",
        entry, diff_value
    );
    println!("{:?}%", diff_value);
}

/// print dimensions errors
fn print_dimensions_error(config: &Config, path: &PathBuf) -> Result<()> {
    warn!("Images have different dimensions, skipping comparison");
    if config.verbose.log_level_filter() > log::LevelFilter::Error {
        let path = path.to_string_lossy();
        eprintln!("diff found in file: {:?}", path);
    }

    Ok(())
}

/// Helper to create folder hierarchies
fn create_path(path: &Path) -> Result<()> {
    let mut buffer = path.to_path_buf();
    if buffer.is_file() {
        buffer.pop();
    }
    create_dir_if_not_there(buffer)?;
    Ok(())
}

/// recursive way to create folders hierarchies
fn create_dir_if_not_there(mut buffer: PathBuf) -> Result<PathBuf> {
    if buffer.pop() {
        create_dir_if_not_there(buffer.clone())?;
        if !buffer.exists() && buffer != Path::new("") {
            create_dir(&buffer)?
        }
    }
    Ok(buffer)
}
