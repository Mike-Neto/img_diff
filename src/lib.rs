//! # img_diff
//!
//! `img_diff` is a cmd line tool to diff images in 2 folders
//! you can pass -h to see the help
//!
use bmp::{open, BmpError, Image, Pixel};
use lodepng::{decode32_file, encode32_file, ffi, Bitmap, RGBA};
use std::fs::{create_dir, read_dir};
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
    BMP(Result<Image, BmpError>),
    PNG(Result<Bitmap<RGBA>, ffi::Error>),
}

struct DiffImage {
    path: PathBuf,
    image: ImageType,
}

struct Pair<T> {
    src: T,
    dest: T,
}

trait DiffImageDimension {
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn has_different_dimensions(&self, other: &Self) -> bool {
        self.width() != other.width() || self.height() != other.height()
    }
}

trait DiffImageOutput {
    fn output_file(&self, file_name: &str, width: Option<usize>, height: Option<usize>);
    fn output_diff_file(
        &self,
        diff_value: f32,
        config: &Config,
        src_path: PathBuf,
        dest_path: PathBuf,
        width: Option<usize>,
        height: Option<usize>,
    ) {
        if diff_value != 0.0 {
            if let Some(path) = dest_path.to_str() {
                let diff_file_name = get_diff_file_name_and_validate_path(path, config);
                match diff_file_name {
                    Some(diff_file_name) => {
                        // Use another tread to write the files as necessary
                        self.output_file(&diff_file_name, width, height);

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
}

impl DiffImageDimension for Image {
    fn height(&self) -> usize {
        self.get_height() as usize
    }

    fn width(&self) -> usize {
        self.get_width() as usize
    }
}

impl DiffImageOutput for Image {
    // @Cleanup
    fn output_file(&self, file_name: &str, width: Option<usize>, height: Option<usize>) {
        output_bmp(&file_name, Some(self));
    }
}

impl DiffImageDimension for Bitmap<RGBA> {
    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }
}

impl DiffImageOutput for Vec<RGBA> {
    fn output_file(&self, file_name: &str, width: Option<usize>, height: Option<usize>) {
        if let Err(err) = encode32_file(file_name, self, width.unwrap(), height.unwrap()) {
            eprintln!("Failed to write file: {:?}", err);
        }
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
            if let Some(extension) = scr_path.extension() {
                if let Some(extension) = extension.to_str() {
                    let extension = extension.to_lowercase();
                    if extension == "bmp" {
                        if let Err(err) = transmitter.send(Pair {
                            src: DiffImage {
                                path: scr_path.clone(),
                                image: ImageType::BMP(open(scr_path)),
                            },
                            dest: DiffImage {
                                path: dest_path.clone(),
                                image: ImageType::BMP(open(dest_path)),
                            },
                        }) {
                            eprintln!("Could not send using channel: {:?}", err);
                        };
                    } else if let Err(err) = transmitter.send(Pair {
                        src: DiffImage {
                            path: scr_path.clone(),
                            image: ImageType::PNG(decode32_file(scr_path)),
                        },
                        dest: DiffImage {
                            path: dest_path.clone(),
                            image: ImageType::PNG(decode32_file(dest_path)),
                        },
                    }) {
                        eprintln!("Could not send using channel: {:?}", err);
                    }
                } else {
                    eprintln!("Could not convert extension to string: {:?}", extension);
                }
            } else {
                eprintln!("Could not get extension from file: {:?}", scr_path);
            }
        }
    });

    // do the comparison in the receiving channel
    for pair in receiver {
        match (pair.src.image, pair.dest.image) {
            (ImageType::BMP(src_image), ImageType::BMP(dest_image)) => {
                match (src_image, dest_image) {
                    (Ok(src_bmp_img), Ok(dest_bmp_img)) => {
                        if src_bmp_img.has_different_dimensions(&dest_bmp_img) {
                            print_dimensions_error(config, pair.src.path);
                        } else {
                            let mut diff_value = 0.0; //TODO(MiguelMendes): Give a meaning to this value
                            let mut diff_image =
                                Image::new(src_bmp_img.get_width(), src_bmp_img.get_height());
                            for (x, y) in src_bmp_img.coordinates() {
                                let dest_pixel = dest_bmp_img.get_pixel(x, y);
                                let src_pixel = src_bmp_img.get_pixel(x, y);
                                let diff_pixel = subtract(src_pixel, dest_pixel);
                                diff_value += interpolate(diff_pixel);
                                diff_image.set_pixel(x, y, diff_pixel);
                            }
                            print_diff_result(config.verbose, &pair.src.path, diff_value);
                            diff_image.output_diff_file(
                                diff_value,
                                config,
                                pair.src.path,
                                pair.dest.path,
                                None,
                                None,
                            );
                        }
                    }
                    (Err(err), _) => eprintln!("Failed to open src img {:?}", err),
                    (_, Err(err)) => eprintln!("Failed to open dest img {:?}", err),
                }
            }
            (ImageType::PNG(src_image), ImageType::PNG(dest_image)) => {
                match (src_image, dest_image) {
                    (Ok(src_png_img), Ok(dest_png_img)) => {
                        if src_png_img.has_different_dimensions(&dest_png_img) {
                            print_dimensions_error(config, pair.src.path);
                        } else {
                            let mut diff_value = 0.0; //TODO(MiguelMendes): Give a meaning to this value
                            let pixels = src_png_img.width * src_png_img.height;
                            let mut diff_img: Vec<RGBA> =
                                Vec::with_capacity(pixels * std::mem::size_of::<RGBA>());
                            for i in 0..pixels {
                                let src_pixel = src_png_img.buffer[i];
                                let dest_pixel = dest_png_img.buffer[i];

                                let diff_pixel = subtract_png(src_pixel, dest_pixel);
                                diff_value += interpolate_png(diff_pixel);
                                diff_img.push(diff_pixel);
                            }
                            print_diff_result(config.verbose, &pair.src.path, diff_value);
                            diff_img.output_diff_file(
                                diff_value,
                                config,
                                pair.src.path,
                                pair.dest.path,
                                Some(src_png_img.width),
                                Some(src_png_img.height),
                            );
                        }
                    }
                    (Err(err), _) => eprintln!("Failed to open src img: {:?}", err),
                    (_, Err(err)) => eprintln!("Failed to open dest img: {:?}", err),
                }
            }
            _ => unreachable!(),
        };
    }

    Ok(())
}

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

/// saves bmp file diff to disk
fn output_bmp(path_name: &str, image: Option<&Image>) {
    if let Some(image) = image {
        if let Err(err) = image.save(&path_name) {
            eprintln!("Failed to save diff_file: {}\nError: {}", path_name, err)
        }
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

fn print_dimensions_error(config: &Config, path: PathBuf) {
    println!("Images have different dimensions, skipping comparison");
    if config.verbose {
        if let Some(path) = path.to_str() {
            eprintln!("diff found in file: {:?}", String::from(path));
        } else {
            eprintln!("failed to convert path to string: {:?}", path);
        }
    }
}

fn subtract(p: Pixel, quantity: Pixel) -> Pixel {
    let r;
    let g;
    let b;

    if p.r >= quantity.r {
        r = p.r - quantity.r;
    } else {
        r = quantity.r - p.r
    }
    if p.g >= quantity.g {
        g = p.g - quantity.g;
    } else {
        g = quantity.g - p.g
    }
    if p.b >= quantity.b {
        b = p.b - quantity.b;
    } else {
        b = quantity.b - p.b
    }

    Pixel { r, g, b }
}

fn interpolate(p: Pixel) -> f32 {
    f32::from((p.r / 3) + (p.g / 3) + (p.b / 3)) / 10_000_000.0
}

fn interpolate_png(p: RGBA) -> f32 {
    f32::from((p.r / 4) + (p.g / 4) + (p.b / 4) + (p.a / 4)) / 10_000_000.0
}

fn subtract_png(p1: RGBA, p2: RGBA) -> RGBA {
    let r;
    let g;
    let b;
    let a;

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
    if p1.a >= p2.a {
        a = p1.a - p2.a;
    } else {
        a = p2.a - p1.a
    }

    RGBA { r, g, b, a }
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
