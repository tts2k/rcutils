use clap::{App, Arg};
use regex::{Regex, RegexBuilder};
use std::cell::RefCell;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use std::{
    error::Error,
    fs::{read_dir, File},
};
use walkdir::WalkDir;

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
                .required(true),
        )
        .arg(
            Arg::new("file")
                .value_name("FILE")
                .takes_value(true)
                .help("Input file(s)")
                .default_value("-")
                .multiple_values(true),
        )
        .arg(
            Arg::new("count")
                .help("Count occurences")
                .short('c')
                .long("")
                .takes_value(false),
        )
        .arg(
            Arg::new("insensitive")
                .help("Case-insensitive")
                .short('i')
                .long("insensitive")
                .takes_value(false),
        )
        .arg(
            Arg::new("invert")
                .help("Invert match")
                .short('v')
                .long("invert-match")
                .takes_value(false),
        )
        .arg(
            Arg::new("recursive")
                .help("Recursive search")
                .short('r')
                .long("recursive")
                .takes_value(false),
        )
        .get_matches();

    let pattern = matches.remove_one::<String>("pattern").unwrap();
    let files = matches.remove_many::<String>("file").unwrap().collect();
    let insensitive = matches.contains_id("insensitive");
    let invert_match = matches.contains_id("invert");
    let recursive = matches.contains_id("recursive");
    let count = matches.contains_id("count");

    Ok(Config {
        pattern: match RegexBuilder::new(&pattern)
            .case_insensitive(insensitive)
            .build()
        {
            Ok(pattern) => pattern,
            _ => return Err(From::from(format!("Invalid pattern \"{}\"", pattern))),
        },
        files,
        count,
        invert_match,
        recursive,
    })
}

fn find_files(paths: &[String], recursive: bool) -> Vec<RetType<String>> {
    let result = RefCell::<Vec<RetType<String>>>::new(vec![]);

    let recursive_find = |path: &Path| match read_dir(path) {
        Err(e) => result.borrow_mut().push(Err(Box::new(e))),
        Ok(_) => {
            let entries = WalkDir::new(path);
            for entry in entries {
                match entry {
                    Err(e) => result.borrow_mut().push(Err(Box::new(e))),
                    Ok(e) => {
                        if e.path().is_file() {
                            result
                                .borrow_mut()
                                .push(Ok(String::from(e.path().to_string_lossy())))
                        }
                    }
                }
            }
        }
    };

    for path in paths {
        let path = Path::new(path);

        if path.is_file() {
            result
                .borrow_mut()
                .push(Ok(String::from(path.to_string_lossy())));
            continue;
        }

        if recursive {
            recursive_find(&path);
        } else {
            result.borrow_mut().push(Err(From::from(format!(
                "{} is a directory",
                path.to_string_lossy()
            ))))
        }
    }
    result.into_inner()
}

fn open(filename: &str) -> RetType<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)))),
    }
}

pub fn run(config: Config) -> RetType<()> {
    println!("pattern \"{}\"", config.pattern);

    let entries = find_files(&config.files, config.recursive);
    for entry in entries {
        match entry {
            Err(e) => eprintln!("{}", e),
            Ok(filename) => println!("file \"{}\"", filename),
        }
    }

    Ok(())
}
