use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufReader, BufRead};

pub type MyResult<T> = Result<T, Box<dyn Error>>;

pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool
}

pub fn get_args() -> MyResult<Config> {
    let matches  = App::new("catr")
        .version("0.1.0")
        .author("Thai Son Tran")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input files")
                .allow_invalid_utf8(true)
                .required(true)
                .min_values(1)
        )
        .arg(
            Arg::with_name("number")
                .help("Number lines")
                .long("number")
                .short('n')
                .takes_value(false)
        )
        .arg(
            Arg::with_name("number_nonblank")
                .help("Number non-blank lines")
                .long("number-nonblank")
                .short('b')
                .takes_value(false)
        )
        .get_matches();


    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank")
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

fn process_file(file: Box<dyn BufRead>, config: &Config) -> MyResult<()> {
    let mut last_num = 0;
    for (line_num, line_result) in file.lines().enumerate() {
        let line = line_result?;
        if config.number_lines {
            println!("{:>6}\t{}", line_num + 1, line);
        }
        else if config.number_nonblank_lines {
            if !line.is_empty() {
                last_num += 1;
                println!("{:>6}\t{}", last_num, line);
            }
            else {
                println!();
            }
        }
        else {
            println!("{}", line);
        }
    }

    Ok(())
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file) => process_file(file, &config)?
        }
    }
    Ok(())
}
