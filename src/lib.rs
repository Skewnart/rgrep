
mod config;
mod searching;

use std::{env, error::Error, fs, io};
use config::Config;

pub fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args)?;
    let content = readfile(&config)?;

    let result = if config.case_insensitive {
        searching::search_case_insensitive(&config.query, &content)
    }
    else {
        searching::search(&config.query, &content)
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

fn readfile(config: &Config) -> Result<String, io::Error> {
    fs::read_to_string(config.filepath.clone())
}
