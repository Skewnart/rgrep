use std::{env, fs, io, process};

mod config;
mod searching;

use config::Config;


pub fn run() {
    let args: Vec<String> = env::args().collect();

    // dbg!(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1);
    });

    let content = readfile(&config).unwrap_or_else(|err| {
        println!("Application error : {}", err);
        process::exit(1);
    });

    let result = if config.case_insensitive {
        searching::search_case_insensitive(&config.query, &content)
    }
    else {
        searching::search(&config.query, &content)
    };

    for line in result {
        println!("{line}");
    }
}

fn readfile(config: &Config) -> Result<String, io::Error> {
    fs::read_to_string(config.filepath.clone())
}
