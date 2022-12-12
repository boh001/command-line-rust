use clap::builder::BoolishValueParser;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("camel")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .num_args(0..)
                .required(true)
                .help("Input text"),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .default_value("false")
                .value_parser(BoolishValueParser::new()),
        )
        .get_matches();

    let text: Vec<String> = matches
        .get_many("text")
        .expect("text is required")
        .cloned()
        .collect();
    let omit_newline: bool = *matches.get_one("omit_newline").expect("omit_newline error");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
