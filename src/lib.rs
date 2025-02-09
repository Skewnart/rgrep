
mod config;
mod search;

use std::{env::{self, Args}, error::Error, fs, io};
use config::{Config, Input};
use search::Search;

pub fn run() -> Result<(), Box<dyn Error>> {
    let args: Args = env::args();

    let config = Config::build(args)?;
    
    let content: String = match &config.input {
        Input::Content(_content) => _content.clone(),
        Input::Filepath(_filepath) => readfile(_filepath.clone())?
    };

    let search = Search::build(&config);
    let lines = search.search(&config.query, &content);

    for line in lines {
        println!("{line}");
    }

    Ok(())
}

fn readfile(_filepath: String) -> Result<String, io::Error> {
    fs::read_to_string(_filepath)
}
