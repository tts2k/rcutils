use std::error::Error;
use std::io::{BufRead, BufReader, stdin, Read};
use std::fs::File;
use clap::{App, Arg};

pub type RetType<T> = Result<T, Box<dyn Error>>;

pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

fn parse_positive_int(val: &str) -> RetType<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

pub fn get_args() -> RetType<Config> {
    let matches = App::new("headr")
        .author("Thai Son Tran")
        .version("0.1.0")
        .about("Rust head")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input files")
                .allow_invalid_utf8(true)
                .min_values(1)
                .default_value("-")
        )
        .arg(
            Arg::with_name("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .help("print the first K bytes of each file")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("lines")
                .short('n')
                .long("lines")
                .value_name("LINES")
                .help("print the first K lines of each file")
                .takes_value(true)
                .default_value("10")
                .conflicts_with("bytes")
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes
    })
}

fn read_lines(mut buffer: Box<dyn BufRead>, config: &Config) -> RetType<()> {
    let mut line = String::new();
    for _ in 0..config.lines {
        let bytes = buffer.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        print!("{}", line);
        line.clear();
    }

    Ok(())
}

fn read_bytes(file: Box<dyn Read>, bytes: &usize) -> RetType<()> {
    let mut handle = file.take(*bytes as u64);
    let mut buffer = vec![0; *bytes];
    let n = handle.read(&mut buffer)?;
    print!("{}", String::from_utf8_lossy(&buffer[..n]));
    Ok(())
}

fn handle_file(filename: &str, filenum: usize, config: &Config) -> RetType<()> {
    match File::open(&filename) {
        Err(err) => eprintln!("{}: {}", filename, err),
        Ok(file) => {
            if config.files.len() > 1 {
                println!(
                    "{}==> {} <==",
                    if filenum > 0 { "\n" } else { "" },
                    filename
                )
            }

            if let Some(bytes) = config.bytes {
                read_bytes(Box::new(file), &bytes)?;
            }
            else {
                read_lines(Box::new(BufReader::new(file)), &config)?;
            };
        }
    }
    Ok(())
}

fn handle_stdin(config: &Config) -> RetType<()> {
    if let Some(bytes) = config.bytes {
        read_bytes(Box::new(stdin()), &bytes)?;
    }
    else {
        read_lines(Box::new(BufReader::new(stdin())), config)?;
    }
    Ok(())
}

pub fn run(config: Config) -> RetType<()> {
    for (filenum, filename) in config.files.iter().enumerate() {
        match filename.as_str() {
            "-" => handle_stdin(&config)?,
            _ => handle_file(filename, filenum, &config)?
        }
    }
    Ok(())
}
