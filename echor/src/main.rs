use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Thai Son Tran <tranthaison2000@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1)
                .allow_invalid_utf8(true)
        )
        .arg(
            Arg::with_name("omit_newline")
                .help("Do not print new line")
                .takes_value(false)
                .short('n')
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let ending = if matches.is_present("omit_newline") { ' ' } else { '\n' };

    print!("{}{}", text.join(" "), ending);
}
