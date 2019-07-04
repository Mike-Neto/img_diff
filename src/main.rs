use human_panic::*;
use img_diff::{do_diff, Config};
use structopt::StructOpt;

fn main() {
    let config = Config::from_args();
    setup_panic!();

    if config.verbose {
        println!("Parsed configs: {:?}", config);
    }

    match do_diff(&config) {
        Ok(_) => {
            if config.verbose {
                println!("Compared everything, process ended with great success!")
            }
        }
        Err(err) => eprintln!("Error occurred: {:?}", err),
    }
}

#[cfg(test)]
mod end_to_end {
    use assert_cmd::prelude::*;
    use predicates::prelude::*;
    use std::fs::{remove_file, File};
    use std::process::Command;
    use tempdir::TempDir;

    #[test]
    fn it_works_for_bmp_files() {
        let diff = TempDir::new("it_works_for_bmp_files_diff").unwrap();
        let _ = remove_file(diff.path().join("rustacean-error.bmp"));
        Command::cargo_bin("img_diff")
            .unwrap()
            .args(&[
                "-s",
                "tests/it_works_for_bmp_files/it_works_for_bmp_files_src",
                "-d",
                "tests/it_works_for_bmp_files/it_works_for_bmp_files_dest",
                "-f",
                diff.path().to_str().unwrap(),
            ])
            .assert()
            .stdout(
                predicate::str::is_match("0\n|0.0\n|0.68007237|3.7269595\n")
                    .unwrap()
                    .count(3),
            )
            .success();
        assert!(File::open(diff.path().join("rustacean-error.bmp"),).is_ok());
    }
    #[test]
    fn it_prints_usage_text_when_no_args_are_provided() {
        Command::cargo_bin("img_diff")
            .unwrap()
            .assert()
            .stdout(predicates::str::is_empty())
            .stderr(predicates::str::is_empty().not())
            .failure();
    }

    #[test]
    fn it_prints_help_text_when_help_arg_is_provided() {
        Command::cargo_bin("img_diff")
            .unwrap()
            .args(&["-h"])
            .assert()
            .stdout(predicates::str::is_empty().not())
            .success();
        Command::cargo_bin("img_diff")
            .unwrap()
            .args(&["--help"])
            .assert()
            .stdout(predicates::str::is_empty().not())
            .success();
    }

    #[test]
    fn it_fails_when_path_is_provided_but_are_not_there() {
        Command::cargo_bin("img_diff")
            .unwrap()
            .args(&[
                "-s",
                "fake_test/it_works_for_equal_images/it_works_for_equal_images_src",
                "-d",
                "fake_test/it_works_for_equal_images/it_works_for_equal_images_dest",
                "-f",
                "fake_test/it_works_for_equal_images/it_works_for_equal_images_diff",
            ])
            .assert()
            .stdout(predicate::str::is_empty())
            .stderr(predicate::str::is_empty().not())
            .success();
    }

    #[test]
    fn it_works_for_equal_images() {
        let diff = TempDir::new("it_works_for_equal_images_diff").unwrap();
        Command::cargo_bin("img_diff")
            .unwrap()
            .args(&[
                "-s",
                "tests/it_works_for_equal_images/it_works_for_equal_images_src",
                "-d",
                "tests/it_works_for_equal_images/it_works_for_equal_images_dest",
                "-f",
                diff.path().to_str().unwrap(),
            ])
            .assert()
            .stdout(predicate::str::similar("0.0%\n"))
            .stderr(predicate::str::is_empty())
            .success();
    }
    #[test]
    fn it_works_for_equal_images_without_diff_folder_been_created() {
        let temp = TempDir::new("it_works_for_equal_images_without_diff_folder_been_created_diff")
            .unwrap();
        let path = temp
            .path()
            .join("it_works_for_equal_images_without_diff_folder_been_created_diff");
        Command::cargo_bin("img_diff")
                .unwrap()
                .args(
                    &[
                        "-s",
                        "tests/it_works_for_equal_images_without_diff_folder_been_created/it_works_for_equal_images_without_diff_folder_been_created_scr",
                        "-d",
                        "tests/it_works_for_equal_images_without_diff_folder_been_created/it_works_for_equal_images_without_diff_folder_been_created_dest",
                        "-f",
                        path.to_str().unwrap(),
                    ],
                )
                .assert()
                .stdout(predicate::str::similar("0.0%\n"))
                .stderr(predicate::str::is_empty())
                .success();
    }

    #[test]
    fn it_works_for_different_images() {
        let diff = TempDir::new("it_works_for_different_images").unwrap();

        Command::cargo_bin("img_diff")
            .unwrap()
            .args(&[
                "-s",
                "tests/it_works_for_different_images/it_works_for_different_images_scr",
                "-d",
                "tests/it_works_for_different_images/it_works_for_different_images_dest",
                "-f",
                diff.path().to_str().unwrap(),
            ])
            .assert()
            .stdout(predicate::str::is_match("2.089595\n").unwrap())
            .stderr(predicate::str::is_empty())
            .success();
    }

    #[test]
    fn it_works_for_different_images_and_produces_diff_file() {
        let diff =
            TempDir::new("it_works_for_different_images_and_produces_diff_file_diff").unwrap();

        Command::cargo_bin("img_diff")
            .unwrap()
            .args(
                &[
                    "-s",
                    "tests/it_works_for_different_images_and_produces_diff_file/it_works_for_different_images_and_produces_diff_file_scr",
                    "-d",
                    "tests/it_works_for_different_images_and_produces_diff_file/it_works_for_different_images_and_produces_diff_file_dest",
                    "-f",
                    diff.path().to_str().unwrap(),
                ],
            )
            .assert()
            .stdout(predicate::str::is_match("2.089595\n").unwrap())
            .stderr(predicate::str::is_empty())
            .success();

        assert!(File::open(diff.path().join("rustacean-error.png"),).is_ok());
    }

    #[test]
    fn it_works_for_nested_folders() {
        let diff = TempDir::new("it_works_for_nested_folders_diff").unwrap();
        Command::cargo_bin("img_diff")
            .unwrap()
            .args(&[
                "-s",
                "tests/it_works_for_nested_folders/it_works_for_nested_folders_src",
                "-d",
                "tests/it_works_for_nested_folders/it_works_for_nested_folders_dest",
                "-f",
                diff.path().to_str().unwrap(),
            ])
            .assert()
            .stdout(
                predicate::str::is_match("2.089595\n")
                    .unwrap()
                    .and(predicate::str::is_match("2.089595\n").unwrap()),
            )
            .stderr(predicate::str::is_empty())
            .success();
    }

    #[test]
    fn it_works_for_more_files_in_scr_than_dest() {
        let diff = TempDir::new("it_works_for_more_files_in_scr_than_dest_diff").unwrap();

        Command::cargo_bin("img_diff")
            .unwrap()
            .args(
                &[
                    "-s",
                    "tests/it_works_for_more_files_in_scr_than_dest/it_works_for_more_files_in_scr_than_dest_src",
                    "-d",
                    "tests/it_works_for_more_files_in_scr_than_dest/it_works_for_more_files_in_scr_than_dest_dest",
                    "-f",
                    diff.path().to_str().unwrap(),
                ],
            )
            .assert()
            .stdout(
                predicate::str::is_match("2.089595\n")
                    .unwrap()
                    .and(predicate::str::is_match("2.089595\n").unwrap()),
            )
            .stderr(predicate::str::is_empty())
            .success();
    }

    #[test]
    fn it_works_when_diff_folder_is_not_created() {
        let temp = TempDir::new("it_works_when_diff_folder_is_not_created").unwrap();
        let path = temp
            .path()
            .join("it_works_when_diff_folder_is_not_created_diff");

        Command::cargo_bin("img_diff")
            .unwrap()
            .args(
                &[
                    "-s",
                    "tests/it_works_when_diff_folder_is_not_created/it_works_when_diff_folder_is_not_created_src",
                    "-d",
                    "tests/it_works_when_diff_folder_is_not_created/it_works_when_diff_folder_is_not_created_dest",
                    "-f",
                    path.to_str().unwrap(),
                ],
            )
            .assert()
            .stdout(
                predicate::str::is_match("2.089595\n")
                    .unwrap()
                    .and(predicate::str::is_match("2.089595\n").unwrap()),
            )
            .stderr(predicate::str::is_empty())
            .success();
    }

    #[test]
    fn it_works_when_images_have_different_dimensions() {
        let temp = TempDir::new("it_works_when_images_have_different_dimensions").unwrap();
        let path = temp
            .path()
            .join("it_works_when_images_have_different_dimensions_diff");

        Command::cargo_bin("img_diff")
            .unwrap()
            .args(
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
            .assert()
            .stdout(
                predicate::str::contains("Images have different dimensions, skipping comparison"),
            )
            .stderr(predicate::str::contains("rustacean-error.png").and(
                predicate::str::contains("MARBLES_01.BMP")
            ))
            .success();
    }
    #[test]
    fn when_in_verbose_mode_prints_each_file_compare() {
        let diff = TempDir::new("it_works_for_equal_images_diff").unwrap();
        Command::cargo_bin("img_diff")
            .unwrap()
            .args(&[
                "-v",
                "-s",
                "tests/it_works_for_equal_images/it_works_for_equal_images_src",
                "-d",
                "tests/it_works_for_equal_images/it_works_for_equal_images_dest",
                "-f",
                diff.path().to_str().unwrap(),
            ])
            .assert()
            .stdout(predicate::str::contains("compared file:"))
            .stderr(predicate::str::is_empty())
            .success();
    }

    #[test]
    fn when_in_verbose_mode_prints_each_file_diff_to_stderr() {
        let diff = TempDir::new("it_works_for_different_images_diff").unwrap();
        Command::cargo_bin("img_diff")
            .unwrap()
            .args(&[
                "-v",
                "-s",
                "tests/it_works_for_different_images/it_works_for_different_images_scr",
                "-d",
                "tests/it_works_for_different_images/it_works_for_different_images_dest",
                "-f",
                diff.path().to_str().unwrap(),
            ])
            .assert()
            .stdout(predicate::str::is_empty().not())
            .stderr(predicate::str::contains("diff found in file:"))
            .success();
    }
}
