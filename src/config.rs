use std::{env::{self}, io::{self, BufRead, IsTerminal}};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filepath: String,
    pub content: String,
    pub case_insensitive: bool
}

impl Config {

    pub fn build<'a>(mut args : impl Iterator<Item = String>) -> Result<Self, &'a str> {
        args.next();

        let Some(_query) = args.next() else {
            return Err("Query not provided");
        };

        let _from_pipe = !io::stdin().is_terminal();

        let (_filepath, _content) = if _from_pipe {
            (
                String::from(""), 
                io::stdin().lock().lines().fold(String::from(""), |acc, line| acc + &line.unwrap() + "\n")
            )
        }
        else{            
            (
                if let Some(_filepath) = args.next() {
                    _filepath
                } else { 
                    return Err("File not provided");
                },
                String::from("")
            )
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
            filepath : _filepath,
            content : _content,
            case_insensitive : _case_insensitive
        })
    }
}