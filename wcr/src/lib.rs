use clap::{App, Arg};
use std::error::Error;

type RetType<T> = Result<T, Box <dyn Error>>;

#[derive(Debug)]
pub struct Config {
    file: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn get_args() -> RetType<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Thai Son Tran")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
            .value_name("FILES")
            .help("Input files")
            .allow_invalid_utf8(true)
            .default_value("-")
        )
        .arg(
            Arg::with_name("chars")
            .long("chars")
            .short('m')
            .help("Show character count")
            .takes_value(false)
        )
        .arg(
            Arg::with_name("lines")
            .long("lines")
            .short('l')
            .help("Show line count")
            .takes_value(false)
        )
        .arg(
            Arg::with_name("words")
            .long("words")
            .short('w')
            .help("Show words count")
            .takes_value(false)
        )
        .arg(
            Arg::with_name("bytes")
            .long("bytes")
            .short('c')
            .help("Show byte count")
            .takes_value(false)
        )
        .get_matches();

    Ok(Config {
        file: matches.values_of_lossy("files").unwrap(),
        lines: matches.is_present("line"),
        words: matches.is_present("words"),
        bytes: matches.is_present("bytes"),
        chars: matches.is_present("chars")
    })
}

pub fn run (config: Config) -> RetType<()> {
    Ok(())
}
