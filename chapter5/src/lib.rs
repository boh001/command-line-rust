use clap::builder::BoolishValueParser;
use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("wcr")
        .version("0.1.0")
        .author("camel")
        .about("Rust wc")
        .arg(Arg::new("files").default_value("-").num_args(0..))
        .arg(
            Arg::new("lines")
                .short('l')
                .default_value("true")
                .default_missing_value("true")
                .require_equals(true)
                .num_args(0..=1)
                .value_parser(BoolishValueParser::new()),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .default_value("true")
                .default_missing_value("true")
                .require_equals(true)
                .num_args(0..=1)
                .value_parser(BoolishValueParser::new()),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .default_value("true")
                .default_missing_value("true")
                .require_equals(true)
                .num_args(0..=1)
                .value_parser(BoolishValueParser::new())
                .conflicts_with("chars"),
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .default_value("true")
                .default_missing_value("true")
                .require_equals(true)
                .num_args(0..=1)
                .value_parser(BoolishValueParser::new()),
        )
        .get_matches();

    let files: Vec<String> = matches
        .get_many("files")
        .expect("files get panic")
        .cloned()
        .collect();
    let lines: bool = *matches.get_one("lines").expect("words get panic");
    let words: bool = *matches.get_one("words").expect("words get panic");
    let bytes: bool = *matches.get_one("bytes").expect("words get panic");
    let chars: bool = *matches.get_one("chars").expect("words get panic");

    Ok(Config {
        files,
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(info.num_lines, config.lines),
                        format_field(info.num_words, config.words),
                        format_field(info.num_bytes, config.bytes),
                        format_field(info.num_chars, config.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", filename)
                        }
                    );

                    total_lines += info.num_lines;
                    total_words += info.num_words;
                    total_bytes += info.num_bytes;
                    total_chars += info.num_chars;
                }
            }
        }
    }

    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, config.lines),
            format_field(total_words, config.words),
            format_field(total_bytes, config.bytes),
            format_field(total_chars, config.chars)
        )
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }

        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_chars,
        num_bytes,
    })
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::format_field;
    use super::{count, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your hafl.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };
        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}
