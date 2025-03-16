
mod config;
mod search;

use std::{env, error::Error, fs, io};
use args_extractor::{PromptExtractor, StdinService};
use config::{Config, Input};
use search::Search;

pub fn run() -> Result<(), Box<dyn Error>> {    
    let args = env::args();
    let prompt = PromptExtractor::new(StdinService::new())
        .extract(args)?;

    let config = Config::build(prompt)?;
    
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
