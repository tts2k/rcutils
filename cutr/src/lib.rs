use clap::{App, Arg};
use std::{error::Error, env::args};
use Extract::*;

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

pub fn get_args() -> RetType<Config> {
    let matches = App::new("cutr")
        .version("0.1.0")
        .author("Thai Son Tran")
        .about("Rust cut")
        .arg(
            Arg::new("files")
                .help("Cut file")
                .name("FILE")
                .default_value("-")
                .allow_invalid_utf8(true)
        )
        .arg(
            Arg::new("delimiter")
                .help("use DELIM instead of TAB for field delimiter")
                .short('d')
                .long("delimiter")
                .name("DELIM")
        )
        .arg(
            Arg::new("bytes")
                .help("select only these bytes")
                .short('b')
                .long("bytes")
                .name("LIST")
                .conflicts_with("characters")
                .conflicts_with("fields")
        )
        .arg(
            Arg::new("characters")
                .help("select only these characters")
                .short('c')
                .long("characters")
                .name("LIST")
                .conflicts_with("bytes")
                .conflicts_with("characters")
        )
        .arg(
            Arg::new("fields")
                .help("select only these fields")
                .short('f')
                .long("fields")
                .name("LIST")
                .conflicts_with("fields")
                .conflicts_with("bytes")
        )
        .get_matches();

    let extract = || {
        let values = matches.get_many::<usize>("files").unwrap().cloned().collect();

    };

    Ok(Config {
        files: matches.get_many::<String>("files").unwrap().cloned().collect(),
        delimiter: *matches.get_one("delimiter").unwrap()
    })
}
