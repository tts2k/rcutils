use std::{error::Error, fs, path::Path};
use clap::{App, Arg};
use regex::Regex;
use walkdir::{WalkDir, DirEntry};

type RetType<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link
}

#[derive(Debug)]
pub struct Config {
    dirs: Vec<String>,
    names: Option<Vec<Regex>>,
    entry_types: Option<Vec<EntryType>>,
}

pub fn get_args() -> RetType<Config> {
    let matches = App::new("findr")
        .version("0.1.0")
        .author("Thai Son Tran")
        .about("Rust find")
        .arg(
            Arg::new("dirs")
                .takes_value(true)
                .default_value(".")
                .value_name("DIR")
                .allow_invalid_utf8(true)
                .help("Search directory")
                .min_values(1)
        )
        .arg(
            Arg::new("names")
                .short('n')
                .long("name")
                .value_name("NAME")
                .allow_invalid_utf8(true)
                .takes_value(true)
                .multiple(true)
        )
        .arg(
            Arg::new("types")
                .short('t')
                .long("type")
                .value_name("TYPE")
                .takes_value(true)
                .allow_invalid_utf8(true)
                .possible_values(&["f", "d", "l"])
                .multiple(true)
        )
        .get_matches();

    let dirs = matches.values_of_lossy("dirs").unwrap();
    let mut names = vec![];
    if let Some(vals) = matches.values_of_lossy("names") {
        for name in vals {
            match Regex::new(&name) {
                Ok(re) => names.push(re),
                _ => {
                    return Err(From::from(format!("Invalid --name \"{}\"", name)))
                }
            }
        }
    }

    let entry_types = matches.values_of_lossy("types").map(|vals| {
        vals.iter()
            .filter_map(|val| match val.as_str() {
                "d" => Some(EntryType::Dir),
                "f" => Some(EntryType::File),
                "l" => Some(EntryType::Link),
                _ => None,
            })
            .collect()
    });

    Ok(Config {
        dirs,
        names: if names.is_empty() { None } else { Some(names) },
        entry_types
    })
}

pub fn run(config: Config) -> RetType<()> {
    let type_filter = |entry: &DirEntry| {
        match &config.entry_types {
            Some(types) => types.iter().any(|t| {
                    match t {
                        EntryType::Link => entry.path_is_symlink(),
                        EntryType::Dir => entry.file_type().is_dir(),
                        EntryType::File => entry.file_type().is_file(),
                    }
                }),
            None => true,
        }
    };

    let name_filter = |entry: &DirEntry| match &config.names {
        Some(names) => names
            .iter()
            .any(|re| re.is_match(&entry.file_name().to_string_lossy())),
        _ => true
    };

    for dirname in &config.dirs {
        let path = Path::new(&dirname);

        match fs::read_dir(&path) {
            Err(e) => {
                if path.is_file() {
                    let print_path = match &config.entry_types {
                        Some(types) => {
                            types.iter().any(|t| {
                                t == &EntryType::File
                            })
                        },
                        None => true
                    };

                    if print_path {
                        println!("{}", dirname);
                    }
                    continue;
                }
                eprintln!("{}: {}", dirname, e);
            }
            _ => {
                let entries = WalkDir::new(dirname)
                    .into_iter()
                    .filter_map(|e| match e {
                        Err(err) => {
                            eprintln!("{}", err);
                            None
                        },
                        Ok(e) => Some(e)
                    })
                    .filter(type_filter)
                    .filter(name_filter)
                    .map(|entry| entry.path().display().to_string())
                    .collect::<Vec<String>>();
                println!("{}", entries.join("\n"));
            }
        }
    }
    Ok(())
}
