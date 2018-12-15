use img_diff::{do_diff, Config};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    if config.verbose {
        println!("Parsed configs: {:?}", config);
    }

    if config.src_dir.is_some() && config.dest_dir.is_some() && config.diff_dir.is_some() {
        match do_diff(&config) {
            Ok(_) => {
                if config.verbose {
                    println!("Compared everything, process ended with great success!")
                }
            }
            Err(err) => eprintln!("Error occurred: {:?}", err),
        }
    } else if config.help {
        println!(
            "-s to indicate source directory\n-d to indicate destination directory\n-f to indicate diff directory\n-v to toggle verbose mode"
        );
    } else {
        println!("Missing cmd line arguments use img_diff -h to see help");
    }
}

#[cfg(test)]
mod end_to_end {
    use assert_cli;
    use regex;
    use tempdir;

    use self::tempdir::TempDir;
    use std::fs;
    use std::fs::File;

    #[test]
    fn it_works_for_bmp_files() {
        let diff = TempDir::new("it_works_for_bmp_files_diff").unwrap();

        let _result = fs::remove_file(diff.path().join("rustacean-error.bmp"));

        let regex = regex::Regex::new("0\n|0.0\n|0.68007237|3.7269595\n").unwrap();
        assert_cli::Assert::main_binary()
            .with_args(&[
                "-s",
                "tests/it_works_for_bmp_files/it_works_for_bmp_files_src",
                "-d",
                "tests/it_works_for_bmp_files/it_works_for_bmp_files_dest",
                "-f",
                diff.path().to_str().unwrap(),
            ])
            .stdout()
            .satisfies(
                move |x| regex.find_iter(x).count() == 3,
                "different output ",
            )
            .succeeds()
            .unwrap();

        assert!(File::open(diff.path().join("rustacean-error.bmp"),).is_ok());
    }

    #[test]
    fn it_prints_usage_text_when_no_args_are_provided() {
        assert_cli::Assert::main_binary()
            .stdout()
            .is("Missing cmd line arguments use img_diff -h to see help")
            .succeeds()
            .unwrap();
    }

    #[test]
    fn it_prints_help_text_when_help_arg_is_provided() {
        assert_cli::Assert::main_binary()
            .with_args(&["-h"])
            .stdout()
            .contains("-s to indicate source directory\n-d to indicate destination directory\n-f to indicate diff directory\n-v to toggle verbose mode")
            .succeeds()
            .unwrap();
    }

    #[test]
    fn parses_src_dir_param() {
        assert_cli::Assert::main_binary()
            .with_args(&["-v", "-s", "source_dir_param"])
            .stdout()
            .contains("source_dir_param")
            .succeeds()
            .unwrap();
    }

    #[test]
    fn parses_dest_dir_param() {
        assert_cli::Assert::main_binary()
            .with_args(&["-v", "-d", "source_dest_param"])
            .stdout()
            .contains("source_dest_param")
            .succeeds()
            .unwrap();
    }

    #[test]
    fn parses_diff_dir_param() {
        assert_cli::Assert::main_binary()
            .with_args(&["-v", "-f", "source_diff_param"])
            .stdout()
            .contains("source_diff_param")
            .succeeds()
            .unwrap();
    }

    #[test]
    fn it_works_for_equal_images() {
        let diff = TempDir::new("it_works_for_equal_images_diff").unwrap();
        assert_cli::Assert::main_binary()
            .with_args(&[
                "-s",
                "tests/it_works_for_equal_images/it_works_for_equal_images_src",
                "-d",
                "tests/it_works_for_equal_images/it_works_for_equal_images_dest",
                "-f",
                diff.path().to_str().unwrap(),
            ])
            .stdout()
            .is("Dssim(0.0)")
            .succeeds()
            .unwrap();
    }

    #[test]
    fn it_works_for_equal_images_without_diff_folder_been_created() {
        let temp = TempDir::new("it_works_for_equal_images_without_diff_folder_been_created_diff")
            .unwrap();
        let path = temp
            .path()
            .join("it_works_for_equal_images_without_diff_folder_been_created_diff");
        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "-s",
                    "tests/it_works_for_equal_images_without_diff_folder_been_created/it_works_for_equal_images_without_diff_folder_been_created_scr",
                    "-d",
                    "tests/it_works_for_equal_images_without_diff_folder_been_created/it_works_for_equal_images_without_diff_folder_been_created_dest",
                    "-f",
                    path.to_str().unwrap(),
                ],
            )
            .stdout()
            .is("Dssim(0.0)")
            .succeeds()
            .unwrap();
    }

    #[test]
    fn it_works_for_diffrent_images() {
        let diff = TempDir::new("it_works_for_diffrent_images_diff").unwrap();
        let regex = regex::Regex::new("Dssim[(]4.4469[0-9]{10,11}[)]\n").unwrap();

        assert_cli::Assert::main_binary()
            .with_args(&[
                "-s",
                "tests/it_works_for_diffrent_images/it_works_for_diffrent_images_scr",
                "-d",
                "tests/it_works_for_diffrent_images/it_works_for_diffrent_images_dest",
                "-f",
                diff.path().to_str().unwrap(),
            ])
            .stdout()
            .satisfies(move |x| regex.is_match(x), "wrong format ")
            .succeeds()
            .unwrap();
    }

    #[test]
    fn it_works_for_diffrent_images_and_produces_diff_file() {
        let diff =
            TempDir::new("it_works_for_diffrent_images_and_produces_diff_file_diff").unwrap();
        let regex = regex::Regex::new("Dssim[(]4.4469[0-9]{10,11}[)]\n").unwrap();

        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "-s",
                    "tests/it_works_for_diffrent_images_and_produces_diff_file/it_works_for_diffrent_images_and_produces_diff_file_scr",
                    "-d",
                    "tests/it_works_for_diffrent_images_and_produces_diff_file/it_works_for_diffrent_images_and_produces_diff_file_dest",
                    "-f",
                    diff.path().to_str().unwrap(),
                ],
            )
            .stdout()
            .satisfies(move |x| regex.is_match(x), "wrong format ")
            .succeeds()
            .unwrap();

        assert!(File::open(diff.path().join("rustacean-error.png-0.png"),).is_ok());
    }

    #[test]
    fn it_works_for_nested_folders() {
        let diff = TempDir::new("it_works_for_nested_folders_diff").unwrap();
        let regex_diff = regex::Regex::new("Dssim[(]4.4469[0-9]{10,11}[)]\n").unwrap();
        let regex_equal = regex::Regex::new("Dssim[(]((0[.]0)|(0))+[)]\n").unwrap();
        assert_cli::Assert::main_binary()
            .with_args(&[
                "-s",
                "tests/it_works_for_nested_folders/it_works_for_nested_folders_src",
                "-d",
                "tests/it_works_for_nested_folders/it_works_for_nested_folders_dest",
                "-f",
                diff.path().to_str().unwrap(),
            ])
            .stdout()
            .satisfies(
                move |x| regex_equal.find_iter(x).count() == 2,
                "different output ",
            )
            .and()
            .stdout()
            .satisfies(move |x| regex_diff.is_match(x), "different diff ")
            .succeeds()
            .unwrap();
    }

    #[test]
    fn it_works_for_more_files_in_scr_than_dest() {
        let diff = TempDir::new("it_works_for_more_files_in_scr_than_dest_diff").unwrap();
        let regex_diff = regex::Regex::new("Dssim[(]4.4469[0-9]{10,11}[)]\n").unwrap();
        let regex_equal = regex::Regex::new("Dssim[(]0(.0)*[)]\n").unwrap();

        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "-s",
                    "tests/it_works_for_more_files_in_scr_than_dest/it_works_for_more_files_in_scr_than_dest_src",
                    "-d",
                    "tests/it_works_for_more_files_in_scr_than_dest/it_works_for_more_files_in_scr_than_dest_dest",
                    "-f",
                    diff.path().to_str().unwrap(),
                ],
            )
            .stdout()
            .satisfies(move |x| regex_diff.is_match(x), "different diff ")
            .and()
            .stdout()
            .satisfies(move |x| regex_equal.is_match(x), "different equal ")
            .succeeds()
            .unwrap();
    }

    #[test]
    fn it_works_when_diff_folder_is_not_created() {
        let temp = TempDir::new("it_works_when_diff_folder_is_not_created").unwrap();
        let path = temp
            .path()
            .join("it_works_when_diff_folder_is_not_created_diff");

        let regex_diff = regex::Regex::new("Dssim[(]4.4469[0-9]{10,11}[)]\n").unwrap();
        let regex_equal = regex::Regex::new("Dssim[(]0(.0)*[)]\n").unwrap();

        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "-s",
                    "tests/it_works_when_diff_folder_is_not_created/it_works_when_diff_folder_is_not_created_src",
                    "-d",
                    "tests/it_works_when_diff_folder_is_not_created/it_works_when_diff_folder_is_not_created_dest",
                    "-f",
                    path.to_str().unwrap(),
                ],
            )
            .stdout()
            .satisfies(move |x| regex_diff.is_match(x), "different diff ")
            .and()
            .stdout()
            .satisfies(move |x| regex_equal.is_match(x), "different equal ")
            .succeeds()
            .unwrap();
    }

    #[test]
    fn it_works_when_images_have_different_dimensions() {
        let temp = TempDir::new("it_works_when_images_have_different_dimensions").unwrap();
        let path = temp
            .path()
            .join("it_works_when_images_have_different_dimensions_diff");

        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "-v",
                    "-s",
                    "tests/it_works_when_images_have_different_dimensions/it_works_when_images_have_different_dimensions_src",
                    "-d",
                    "tests/it_works_when_images_have_different_dimensions/it_works_when_images_have_different_dimensions_dest",
                    "-f",
                    path.to_str().unwrap(),
                ],
            )
            .stdout()
            .contains("Images have different dimensions, skipping comparison")
            .and()
            .stderr()
            .contains("rustacean-error.png")
            .and()
            .stderr()
            .contains("MARBLES_01.BMP")
            .succeeds()
            .unwrap();
    }

    #[test]
    fn it_enables_verbose_mode_when_verbose_arg_is_provided() {
        assert_cli::Assert::main_binary()
            .with_args(&["-v"])
            .stdout()
            .contains("verbose: true")
            .succeeds()
            .unwrap();
    }

    #[test]
    fn when_in_verbose_mode_prints_each_file_compare() {
        let diff = TempDir::new("it_works_for_equal_images_diff").unwrap();
        assert_cli::Assert::main_binary()
            .with_args(&[
                "-v",
                "-s",
                "tests/it_works_for_equal_images/it_works_for_equal_images_src",
                "-d",
                "tests/it_works_for_equal_images/it_works_for_equal_images_dest",
                "-f",
                diff.path().to_str().unwrap(),
            ])
            .stdout()
            .contains("compared file:")
            .succeeds()
            .unwrap();
    }

    #[test]
    fn when_in_verbose_mode_prints_each_file_diff_to_stderr() {
        let diff = TempDir::new("it_works_for_diffrent_images_diff").unwrap();
        assert_cli::Assert::main_binary()
            .with_args(&[
                "-v",
                "-s",
                "tests/it_works_for_diffrent_images/it_works_for_diffrent_images_scr",
                "-d",
                "tests/it_works_for_diffrent_images/it_works_for_diffrent_images_dest",
                "-f",
                diff.path().to_str().unwrap(),
            ])
            .stderr()
            .contains("diff found in file:")
            .succeeds()
            .unwrap();
    }
}
