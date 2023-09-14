mod file_parser;

use std::error::Error;
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct ParserConfig {
    path: String,
    sheet_name: Option<String>
}

#[derive(Debug)]
pub struct UpdateConfig{
    file_path: String,
    key: String,
    translation: String
}

#[derive(Debug)]
pub enum Config {
    ParseConfig(ParserConfig),
    UpdateConfig(UpdateConfig)
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
        .arg(
            Arg::with_name("update")
                .value_name("UPDATE")
                .short('u')
                .takes_value(true)
                .help("Updates translation by key")
                .allow_invalid_utf8(true)
                .multiple(true)
        )
        .get_matches();

    let parse_val = matches.is_present("parse");
    if parse_val {
        let file_values = matches.values_of_lossy("parse").unwrap();
        let path = file_values.get(0).unwrap();
        let sheet_name = if let Some(name) = file_values.get(1) {
            Some(name.to_owned())
        } else {
            None
        };
        let config = Config::ParseConfig(ParserConfig { path: path.to_owned(), sheet_name });
        Ok(config)
    } else {
        let update_value = matches.values_of_lossy("update").unwrap();
        let file_path = update_value.get(0).expect("File path should be provided");
        let key = update_value.get(1).expect("Key should be provided");
        let translation = update_value.get(2).expect("Translation should be provided");
        let config = Config::UpdateConfig(UpdateConfig{ file_path: file_path.to_owned(), key: key.to_owned(), translation: translation.to_owned() });
        Ok(config)
    }
}

pub fn run(config: Config) -> MyResult<()> {
    match &config {
        Config::ParseConfig(parser_config) => {
            file_parser::parser::parse(parser_config.path.to_owned(), parser_config.sheet_name.to_owned());
        }
        Config::UpdateConfig(update_config) => {
            println!("Update config: {:?}", update_config);
            // Access fields of update_config if needed.
        }
    }
    Ok(())
}