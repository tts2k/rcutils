use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Write}
};

pub type RetType<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn get_args() -> RetType<Config> {
    let matches = App::new("uniq")
        .version("0.1.0")
        .author("Thai Son Tran")
        .about("Rust uniq")
        .arg(
            Arg::with_name("in_file")
            .value_name("FILE")
            .help("Input file")
            .default_value("-"),
        )
        .arg(
            Arg::with_name("out_file")
                .value_name("FILE")
                .help("Output file")
        )
        .arg(
            Arg::with_name("count")
                .long("count")
                .short('c')
                .help("show counts")
                .takes_value(false)
        )
        .get_matches();

    Ok(Config {
        in_file: matches.value_of("in_file").map(str::to_string).unwrap(),
        out_file: matches.value_of("out_file").map(String::from),
        count: matches.is_present("count")
    })
}

fn open(filename: &str) -> RetType<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn read_line(config: &Config) -> RetType<()> {
    let mut file = open(&config.in_file)
        .map_err(|e| format!("{} {}", config.in_file, e))?;

    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_name) => Box::new(File::create(&out_name)?),
        _ => Box::new(io::stdout())
    };

    let mut print = |count: &u64, line: &String| -> RetType<()>{
        if count > &0 {
            if config.count {
                write!(out_file, "{:>4} {}", &count, &line)?;
            }
            else {
                write!(out_file, "{}", &line)?;
            }
        }

        Ok(())
    };

    let mut line = String::new();
    let mut prev_line = String::new();
    let mut count: u64 = 0;
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != prev_line.trim_end() {
            print(&count, &prev_line)?;
            prev_line = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }
    if !count > 1 {
        print(&count, &prev_line)?;
    }

    Ok(())
}

pub fn run(config: Config) -> RetType<()> {
    read_line(&config)?;
    Ok(())
}
