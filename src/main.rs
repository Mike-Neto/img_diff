extern crate img_diff;

use std::env;
use img_diff::{Config, do_diff};

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
            Err(err) => eprintln!("Error occured: {:?}", err),
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
    extern crate assert_cli;

    use std::fs::File;
    use std::path::Path;
    use std::fs;

    #[test]
    fn it_works_for_bmp_files() {
        let _result = fs::remove_file(
            "tests/it_works_for_bmp_files/it_works_for_bmp_files_diff/rustacean-error.bmp",
        );

        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "-s",
                    "tests/it_works_for_bmp_files/it_works_for_bmp_files_src",
                    "-d",
                    "tests/it_works_for_bmp_files/it_works_for_bmp_files_dest",
                    "-f",
                    "tests/it_works_for_bmp_files/it_works_for_bmp_files_diff",
                ],
            )
            .stdout()
            .contains("3.7269595")
            .and()
            .stdout()
            .contains("0")
            .and()
            .stdout()
            .contains("0.68007237")
            .succeeds()
            .unwrap();

        assert!(
            File::open(
                "tests/it_works_for_bmp_files/it_works_for_bmp_files_diff/rustacean-error.bmp",
            ).is_ok()
        );
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
        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "-s",
                    "tests/it_works_for_equal_images/it_works_for_equal_images_src",
                    "-d",
                    "tests/it_works_for_equal_images/it_works_for_equal_images_dest",
                    "-f",
                    "tests/it_works_for_equal_images/it_works_for_equal_images_diff",
                ],
            )
            .stdout()
            .is("Dssim(0)")
            .succeeds()
            .unwrap();
    }

    #[test]
    fn it_works_for_equal_images_without_diff_folder_been_created() {
        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "-s",
                    "tests/it_works_for_equal_images_without_diff_folder_been_created/it_works_for_equal_images_without_diff_folder_been_created_scr",
                    "-d",
                    "tests/it_works_for_equal_images_without_diff_folder_been_created/it_works_for_equal_images_without_diff_folder_been_created_dest",
                    "-f",
                    "tests/it_works_for_equal_images_without_diff_folder_been_created/it_works_for_equal_images_without_diff_folder_been_created_diff",
                ],
            )
            .stdout()
            .is("Dssim(0)")
            .succeeds()
            .unwrap();
    }

    #[test]
    fn it_works_for_diffrent_images() {
        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "-s",
                    "tests/it_works_for_diffrent_images/it_works_for_diffrent_images_scr",
                    "-d",
                    "tests/it_works_for_diffrent_images/it_works_for_diffrent_images_dest",
                    "-f",
                    "tests/it_works_for_diffrent_images/it_works_for_diffrent_images_diff",
                ],
            )
            .stdout()
            //.is("Dssim(4.44694203487064)")
            .contains("Dssim(4.4469")
            .succeeds()
            .unwrap();
    }

    #[test]
    fn it_works_for_diffrent_images_and_produces_diff_file() {
        let _result = fs::remove_file(
            "tests/it_works_for_diffrent_images_and_produces_diff_file/it_works_for_diffrent_images_and_produces_diff_file_diff/rustacean-error.png-0.png",
        );

        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "-s",
                    "tests/it_works_for_diffrent_images_and_produces_diff_file/it_works_for_diffrent_images_and_produces_diff_file_scr",
                    "-d",
                    "tests/it_works_for_diffrent_images_and_produces_diff_file/it_works_for_diffrent_images_and_produces_diff_file_dest",
                    "-f",
                    "tests/it_works_for_diffrent_images_and_produces_diff_file/it_works_for_diffrent_images_and_produces_diff_file_diff",
                ],
            )
            .stdout()
            //.is("Dssim(4.44694203487064)")
            .contains("Dssim(4.4469")
            .succeeds()
            .unwrap();

        assert!(File::open(
            "tests/it_works_for_diffrent_images_and_produces_diff_file/it_works_for_diffrent_images_and_produces_diff_file_diff/rustacean-error.png-0.png",
        ).is_ok());
    }

    #[test]
    fn it_works_for_nested_folders() {
        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "-s",
                    "tests/it_works_for_nested_folders/it_works_for_nested_folders_src",
                    "-d",
                    "tests/it_works_for_nested_folders/it_works_for_nested_folders_dest",
                    "-f",
                    "tests/it_works_for_nested_folders/it_works_for_nested_folders_diff",
                ],
            )
            .stdout()
            .contains("Dssim(0)")
            .and()
            .stdout()
            .contains("Dssim(4.4469")
            //.is("Dssim(0)\nDssim(4.44694203487064)\nDssim(0)") TODO(MiguelMendes): PR to allow this to be a RegEx
            .succeeds()
            .unwrap();
    }

    /* Linux only bug in /checkout/src/libcore/result.rs:906:4
    #[test]
    fn it_works_for_more_files_in_scr_than_dest() {
        if Path::new("tests/it_works_for_more_files_in_scr_than_dest/it_works_for_more_files_in_scr_than_dest_diff").exists() {
            let _result = fs::remove_dir_all("tests/it_works_for_more_files_in_scr_than_dest/it_works_for_more_files_in_scr_than_dest_diff");
        }

        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "-s",
                    "tests/it_works_for_more_files_in_scr_than_dest/it_works_for_more_files_in_scr_than_dest_src",
                    "-d",
                    "tests/it_works_for_more_files_in_scr_than_dest/it_works_for_more_files_in_scr_than_dest_dest",
                    "-f",
                    "tests/it_works_for_more_files_in_scr_than_dest/it_works_for_more_files_in_scr_than_dest_diff",
                ],
            )
            .stdout()
            //.is("Dssim(0)\nDssim(4.44694203487064)")
            .contains("Dssim(0)")
            .and()
            .stdout()
            .contains("Dssim(4.4469")
            .succeeds()
            .unwrap();
    }
*/
    #[test]
    fn it_works_when_diff_folder_is_not_created() {
        if Path::new("tests/it_works_when_diff_folder_is_not_created/it_works_when_diff_folder_is_not_created_diff").exists() {
            let _result = fs::remove_dir_all("tests/it_works_when_diff_folder_is_not_created/it_works_when_diff_folder_is_not_created_diff");
        }

        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "-s",
                    "tests/it_works_when_diff_folder_is_not_created/it_works_when_diff_folder_is_not_created_src",
                    "-d",
                    "tests/it_works_when_diff_folder_is_not_created/it_works_when_diff_folder_is_not_created_dest",
                    "-f",
                    "tests/it_works_when_diff_folder_is_not_created/it_works_when_diff_folder_is_not_created_diff",
                ],
            )
            .stdout()
            //.is("Dssim(0)\nDssim(4.44694203487064)\nDssim(0)")
            .contains("Dssim(0)")
            .and()
            .stdout()
            .contains("Dssim(4.4469")
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
        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "-v",
                    "-s",
                    "tests/it_works_for_equal_images/it_works_for_equal_images_src",
                    "-d",
                    "tests/it_works_for_equal_images/it_works_for_equal_images_dest",
                    "-f",
                    "tests/it_works_for_equal_images/it_works_for_equal_images_diff",
                ],
            )
            .stdout()
            .contains("compared file:")
            .succeeds()
            .unwrap();
    }

    #[test]
    fn when_in_verbose_mode_prints_each_file_diff_to_stderr() {
        assert_cli::Assert::main_binary()
            .with_args(
                &[
                    "-v",
                    "-s",
                    "tests/it_works_for_diffrent_images/it_works_for_diffrent_images_scr",
                    "-d",
                    "tests/it_works_for_diffrent_images/it_works_for_diffrent_images_dest",
                    "-f",
                    "tests/it_works_for_diffrent_images/it_works_for_diffrent_images_diff",
                ],
            )
            .stderr()
            .contains("diff found in file:")
            .succeeds()
            .unwrap();
    }
}
