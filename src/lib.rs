mod file_parser;

use std::error::Error;
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    path: String,
    sheet_name: Option<String>
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("translator")
        .version("0.1.0")
        .author("Oleksandr Zoria <zoriaalexandr@gmail.com>")
        .about("Translaion xml parser")
        .arg(
            Arg::with_name("parse")
                .value_name("PARSE")
                .short('p')
                .takes_value(true)
                .help("Parse file, accepts at least one argument, first path to the xlsx file, second sheet name")
                .allow_invalid_utf8(true)
                .multiple(true)
        )
        .get_matches();

    let file_values = matches.values_of_lossy("parse").unwrap();
    let path = file_values.get(0).unwrap();
    let sheet_name = if let Some(name) = file_values.get(1) {
        Some(name.to_owned())
    } else {
        None
    };
    Ok(Config { path: path.to_owned(), sheet_name })
}

pub fn run(config: Config) -> MyResult<()> {
    file_parser::parser::parse(config.path, config.sheet_name);
    Ok(())
}