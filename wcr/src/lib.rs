use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type RetType<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

struct Count {
    lines: usize,
    words: usize,
    bytes: usize,
    chars: usize,
}

pub fn get_args() -> RetType<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("Thai Son Tran")
        .about("Rust wc")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .allow_invalid_utf8(true)
                .default_value("-")
                .min_values(1),
        )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .long("chars")
                .short('m')
                .help("Show character count")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("lines")
                .value_name("LINES")
                .long("lines")
                .short('l')
                .help("Show line count")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("words")
                .value_name("WORDS")
                .long("words")
                .short('w')
                .help("Show words count")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("bytes")
                .value_name("BYTES")
                .long("bytes")
                .short('c')
                .help("Show byte count")
                .conflicts_with("chars")
                .takes_value(false),
        )
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let mut chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
        chars = false;
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}

fn open(filename: &str) -> RetType<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

const WIDTH: usize = 8;
fn conditional_print(value: usize, condition: bool) {
    if condition {
        print!("{:WIDTH$}", value);
    }
}

fn print_result(config: &Config, filename: &str, total: &Count) {
    conditional_print(total.lines, config.lines);
    conditional_print(total.words, config.words);
    conditional_print(total.bytes, config.bytes);
    conditional_print(total.chars, config.chars);

    if filename != "-" {
        println!(" {}", filename);
    } else {
        println!();
    }
}

fn file_handle(
    mut buffer: impl BufRead,
    filename: &str,
    config: &Config,
    total: &mut Count,
) -> RetType<()> {
    let mut count = Count {
        words: 0,
        bytes: 0,
        lines: 0,
        chars: 0,
    };

    let mut line_string = String::new();
    let mut read_bytes: usize;
    loop {
        read_bytes = buffer.read_line(&mut line_string)?;
        if read_bytes == 0 {
            break;
        };

        //bytes
        count.bytes += read_bytes;
        //lines
        count.lines += 1;
        //words
        count.words += line_string.split_whitespace().count();
        //chars
        count.chars += line_string.chars().count();

        line_string.clear();
    }

    print_result(
        config,
        filename,
        &count
    );

    total.lines += count.lines;
    total.words += count.words;
    total.chars += count.chars;
    total.bytes += count.bytes;

    Ok(())
}

pub fn run(config: Config) -> RetType<()> {
    let mut total = Count {
        lines: 0,
        words: 0,
        chars: 0,
        bytes: 0,
    };

    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => file_handle(file, filename, &config, &mut total)?,
        }
    }

    if config.files.len() > 1 {
        print_result(&config, "total", &total);
    }
    Ok(())
}
