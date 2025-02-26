
mod config;
mod search;
mod stdin;

use std::{env, error::Error, fs, io};
use config::{Config, Input};
use search::Search;
use stdin::StdinService;

pub fn run() -> Result<(), Box<dyn Error>> {    
    let args = env::args();

    let config = Config::build(StdinService::new(), args)?;
    
    let content: &String = match &config.input {
        Input::Content(_content) => _content,
        Input::Filepath(_filepath) => &readfile(_filepath)?
    };

    let search = Search::build(&config);
    let lines = search.search(&config.query, content);

    for line in lines {
        println!("{line}");
    }

    Ok(())
}

fn readfile(_filepath: &String) -> Result<String, io::Error> {
    fs::read_to_string(_filepath)
}
