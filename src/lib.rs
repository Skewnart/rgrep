
mod config;
mod searching;

use std::{env::{self, Args}, error::Error, fs, io};
use config::Config;

pub fn run() -> Result<(), Box<dyn Error>> {
    let args: Args = env::args();

    let config = Config::build(args)?;
    
    let content = if config.content.is_some() {
        config.content.unwrap() //soon removed
    } else {
        readfile(config.filepath.unwrap_or("".to_string()))?
    };

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

fn readfile(_filepath: String) -> Result<String, io::Error> {
    fs::read_to_string(_filepath)
}
