use std::{env::{self}, io::{self, BufRead, IsTerminal}};

pub struct Config {
    pub query: String,
    pub input: Input,
    pub case_insensitive: bool
}

impl Config {

    pub fn build<'a>(mut args : impl Iterator<Item = String>) -> Result<Self, &'a str> {
        args.next();

        let Some(_query) = args.next() else {
            return Err("Query not provided");
        };

        let _from_pipe = !io::stdin().is_terminal();

        let _input = if _from_pipe {
            Input::Content(io::stdin().lock().lines().fold(String::from(""), |acc, line| acc + &line.unwrap() + "\n"))
        }
        else if let Some(_filepath) = args.next() {
            Input::Filepath(_filepath)
        } else { 
            return Err("File not provided");
        };

        let mut _case_insensitive = env::var("CASE_INSENSITIVE").is_ok();

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-i" => { _case_insensitive = true; },
                _ => ()
            }
        }
    
        Ok(Self { 
            query : _query,
            input: _input,
            case_insensitive : _case_insensitive
        })
    }
}

pub enum Input {
    Filepath(String),
    Content(String)
}