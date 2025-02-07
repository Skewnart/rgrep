
mod config;
mod searching;

use std::{env::{self, Args}, error::Error, fs, io};
use config::{Config, Input};

pub fn run() -> Result<(), Box<dyn Error>> {
    let args: Args = env::args();

    let config = Config::build(args)?;
    
    let content = match config.input {
        Input::Content(_content) => _content,
        Input::Filepath(_filepath) => readfile(_filepath)?
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
