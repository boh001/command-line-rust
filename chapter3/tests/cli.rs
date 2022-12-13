use assert_cmd::Command;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::fs;

const PRG: &str = "chapter3";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PRG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn skips_bad_file() -> TestResult {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .args(["--files", &bad])
        .assert()
        .success()
        .stderr(predicates::str::is_match(expected)?);
    Ok(())
}

#[test]
fn empty() -> TestResult {
    run(&["--files", EMPTY], "tests/expected/empty.txt.out")
}

#[test]
fn empty_stdin() -> TestResult {
    run_stdin(EMPTY, &["--files", "-"], "tests/expected/empty.txt.out")
}

#[test]
fn fox() -> TestResult {
    run(&["--files", FOX], "tests/expected/fox.txt.out")
}

#[test]
fn fox_stdin() -> TestResult {
    run_stdin(FOX, &["--files", "-"], "tests/expected/fox.txt.out")
}

#[test]
fn spiders() -> TestResult {
    run(&["--files", SPIDERS], "tests/expected/spiders.txt.out")
}

#[test]
fn spiders_stdin() -> TestResult {
    run_stdin(SPIDERS, &["--files", "-"], "tests/expected/spiders.txt.out")
}

#[test]
fn bustle() -> TestResult {
    run(&["--files", BUSTLE], "tests/expected/the-bustle.txt.out")
}

#[test]
fn bustle_stdin() -> TestResult {
    run_stdin(
        BUSTLE,
        &["--files", "-"],
        "tests/expected/the-bustle.txt.out",
    )
}
