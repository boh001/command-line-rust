use clap::builder::BoolishValueParser;
use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => {
                let mut last_num = 0;
                for (line_num, line_result) in file.lines().enumerate() {
                    let line = line_result?;
                    if config.number_lines {
                        println!("{:>6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, line);
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("camel")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .long("files")
                .default_value("-")
                .num_args(0..),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .default_value("false")
                .default_missing_value("false")
                .value_parser(BoolishValueParser::new())
                .conflicts_with("number_nonblank_lines"),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .default_value("false")
                .default_missing_value("false")
                .value_parser(BoolishValueParser::new()),
        )
        .get_matches();

    let files: Vec<String> = matches
        .get_many("files")
        .expect("files is required")
        .cloned()
        .collect();
    let number_lines = *matches
        .get_one("number_lines")
        .expect("number_lines is required");
    let number_nonblank_lines = *matches
        .get_one("number_nonblank_lines")
        .expect("number_lines is required");

    Ok(Config {
        files,
        number_lines,
        number_nonblank_lines,
    })
}
