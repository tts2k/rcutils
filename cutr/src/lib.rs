use clap::{App, Arg};
use std::{
    error::Error,
    io::{self, BufReader, BufRead},
    str
};
use std::fs::File;
use regex::Regex;
use Extract::*;
use csv::{StringRecord, WriterBuilder, ReaderBuilder};

type RetType<T> = Result<T, Box<dyn Error>>;
type PositionList = Vec<usize>;

#[derive(Debug)]
pub enum Extract {
    Fields(PositionList),
    Bytes(PositionList),
    Chars(PositionList),
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    delimiter: u8,
    extract: Extract,
}

fn parse_pos(range: &str) -> RetType<PositionList> {
    let mut fields: Vec<usize> = vec![];
    let range_re = Regex::new(r"(\d+)?-(\d+)?").unwrap();
    for val in range.split(',') {
        if let Some(cap) = range_re.captures(val) {
            let n1: &usize = &cap[1].parse()?;
            let n2: &usize = &cap[2].parse()?;

            if n1 < n2 {
                for n in *n1..=*n2 {
                    fields.push(n);
                }
            } else {
                return Err(From::from(format!(
                    "First number in range ({}) \
                    must be lower than the second number({})",
                    n1, n2
                )));
            }
        } else {
            match val.parse() {
                Ok(n) if n > 0 => fields.push(n),
                _ => {
                    return Err(From::from(format!(
                        "illegal list value: \"{}\"",
                        val
                    )))
                }
            }
        }
    };

    Ok(fields.into_iter().map(|i| i - 1).collect())
}

pub fn get_args() -> RetType<Config> {
    let mut matches = App::new("cutr")
        .version("0.1.0")
        .author("Thai Son Tran")
        .about("Rust cut")
        .arg(
            Arg::new("files")
                .help("Cut file")
                .value_name("FILE(s)")
                .default_value("-")
                .min_values(1)
        )
        .arg(
            Arg::new("delimiter")
                .help("use DELIM instead of TAB for field delimiter")
                .short('d')
                .long("delimiter")
                .value_name("DELIM")
                .default_value("\t")
        )
        .arg(
            Arg::new("bytes")
                .help("select only these bytes")
                .short('b')
                .long("bytes")
                .value_name("BYTES")
                .conflicts_with_all(&["fields", "chars"])
        )
        .arg(
            Arg::new("chars")
                .help("select only these characters")
                .short('c')
                .long("characters")
                .value_name("CHARS")
                .conflicts_with_all(&["fields", "bytes"])
        )
        .arg(
            Arg::new("fields")
                .help("select only these fields")
                .short('f')
                .long("fields")
                .value_name("FIELDS")
                .conflicts_with_all(&["chars", "bytes"])
        )
        .get_matches();

    let delimiter = matches.remove_one::<String>("delimiter").unwrap();
    let delim_bytes = delimiter.as_bytes();
    if delimiter.is_empty() || delim_bytes.len() > 1 {
        return Err(From::from(format!(
            "--delim \"{}\" must be a single byte",
            delimiter
        )));
    }

    let fields = matches.value_of("fields").map(parse_pos).transpose()?;
    let bytes = matches.value_of("bytes").map(parse_pos).transpose()?;
    let chars = matches.value_of("chars").map(parse_pos).transpose()?;
    let extract = if let Some(fields_pos) = fields {
        Fields(fields_pos)
    } else if let Some(byte_pos) = bytes {
        Bytes(byte_pos)
    } else if let Some(char_pos) = chars {
        Chars(char_pos)
    } else {
        return Err(From::from("Must have --fields, --bytes, or --chars"));
    };

    Ok(Config {
        files: matches.remove_many::<String>("files").unwrap().collect(),
        delimiter: delim_bytes[0],
        extract
    })
}

fn open(filename: &str) -> RetType<Box< dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

fn extract_chars(line: &str, char_pos: &[usize]) -> String {
    if line.is_empty() {
        return String::from("")
    }

    let line_vec = line.chars().collect::<Vec<_>>();
    char_pos.iter()
        .filter_map(|pos| line_vec.get(*pos))
        .collect::<String>()
}

fn extract_bytes(line: &str, byte_pos: &[usize]) -> String {
    if line.is_empty() {
        return String::from("")
    }

    let bytes = line.as_bytes();
    let selected: Vec<u8> = byte_pos.iter()
        .filter_map(|i| bytes.get(*i)) 
        .cloned()
        .collect();
    String::from_utf8_lossy(&selected).into_owned()
}

fn extract_fields<'a>(record: &'a StringRecord, field_pos: &'a PositionList) -> Vec<&'a str> {
    field_pos
        .iter()
        .filter_map(|i| record.get(*i))
        .collect()
}

pub fn run(config: Config) -> RetType<()> {
    for filename in &config.files    {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                match &config.extract {
                    Bytes(list) => {
                        for line in file.lines() {
                            let bytes = extract_bytes(line?.as_str(), &list);
                            println!("{}", bytes);
                        }
                    }
                    Chars(list) => {
                        for line in file.lines() {
                            let chars = extract_chars(line?.as_str(), &list);
                            println!("{}", chars);
                        }
                    }
                    Fields(list) => {
                        let mut reader = ReaderBuilder::new()
                            .delimiter(config.delimiter)
                            .has_headers(false)
                            .from_reader(file);

                        let mut wtr = WriterBuilder::new()
                            .delimiter(config.delimiter)
                            .from_writer(io::stdout());

                        for record in reader.records() {
                            let record = record?;
                            wtr.write_record(extract_fields(&record, &list))?;
                        }
                    }
                };
            }
        }
    }

    Ok(())
}
