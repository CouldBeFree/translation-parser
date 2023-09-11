use std::{error::Error, vec};
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    path: String
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("translator")
        .version("0.1.0")
        .author("Oleksandr Zoria <zoriaalexandr@gmail.com>")
        .about("Translaion xml parser")
        .arg(
            Arg::with_name("test")
                .value_name("TEST")
                .short('t')
                .takes_value(true)
                .help("Test")
                .allow_invalid_utf8(true)
                .multiple(true)
        )
        .get_matches();

    let test = matches.values_of_lossy("test").unwrap();

    for i in test {
        println!("F, {}", i);
    }
    // let value = test.get(0).unwrap();
    // println!("Value, {}", value);
    // Ok(Config { path: value.to_owned() })
    Ok(Config { path: "Mlope".to_string() })
}

pub fn run(config: Config) -> MyResult<()> {
    println!("Path, {}", config.path);
    Ok(())
}