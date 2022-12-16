use assert_cmd::Command;
use std::fs::File;
use std::io::Read;

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let mut file = File::open(expected_file)?;
    let mut buffer = vec![];
    file.read_to_end(&mut buffer)?;
    let expected = String::from_utf8_lossy(&buffer);

    Command::cargo_bin(PRG)?
        .args(args)
        .assert()
        .success()
        .stdout(predicates::eq(&expected.as_bytes() as &[u8]));

    Ok(())
}
