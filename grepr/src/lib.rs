use clap::{App, Arg};
use regex::{Regex, RegexBuilder};
use std::error::Error;

type RetType<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pattern: Regex,
    files: Vec<String>,
    recursive: bool,
    count: bool,
    invert_match: bool,
}

pub fn get_args() -> RetType<Config> {
    let mut matches = App::new("grepr")
        .version("0.1.0")
        .author("Thai Son Tran")
        .about("Rust grep")
        .arg(
            Arg::new("pattern")
                .value_name("PATTERN")
                .takes_value(true)
                .help("Search pattern")
                .required(true)
        )
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .takes_value(true)
                .help("Input file(s)")
                .default_value("-")
                .multiple_values(true)
        )
        .arg(
            Arg::new("count")
                .help("Count occurences")
                .short('c')
                .long("")
                .takes_value(false)
        )
        .arg(
            Arg::new("insensitive")
                .help("Case-insensitive")
                .short('i')
                .long("insensitive")
                .takes_value(false)
        )
        .arg(
            Arg::new("invert")
                .help("Invert match")
                .short('v')
                .long("invert-match")
                .takes_value(false)
        )
        .arg(
            Arg::new("recursive")
                .help("Recursive search")
                .short('r')
                .long("recursive")
                .takes_value(false)
        )
        .get_matches();

    let pattern = matches.remove_one::<String>("pattern").unwrap();
    let files = matches.remove_many::<String>("file").unwrap().collect();
    let insensitive = matches.contains_id("insensitive");
    let invert_match = matches.contains_id("invert");
    let recursive = matches.contains_id("recursive");
    let count = matches.contains_id("count");

    Ok(Config{
        pattern: match RegexBuilder::new(&pattern).case_insensitive(insensitive).build() {
            Ok(pattern) => pattern,
            _ => return Err(From::from(format!("Invalid pattern \"{}\"", pattern)))
        },
        files,
        count,
        invert_match,
        recursive
    })
}

pub fn run(config: Config) -> RetType<()> {
    dbg!(config);
    Ok(())
}
