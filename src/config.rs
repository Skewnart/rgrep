use std::{env, io::{self, BufRead, IsTerminal}};

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub filepath: String,
    pub content: String,
    pub case_insensitive: bool
}

impl Config {

    pub fn build(args : &[String]) -> Result<Self, &str> {

        if args.len() < 2 {
            return Err("Query not provided");
        }

        let _query = args[1].clone();

        let _from_pipe = !io::stdin().is_terminal();

        let (_filepath, _content) = if _from_pipe {
            (
                String::from(""), 
                io::stdin().lock().lines().fold(String::from(""), |acc, line| acc + &line.unwrap() + "\n")
            )
        }
        else{
            if args.len() < 3 {
                return Err("File not provided");
            }
            
            (
                args[2].clone(),
                String::from("")
            )
        };

        let mut _case_insensitive = env::var("CASE_INSENSITIVE").is_ok();
        if args.iter().any(|config| config == "-i") {
            _case_insensitive = true;
        }
    
        Ok(Self { 
            query : _query,
            filepath : _filepath,
            content : _content,
            case_insensitive : _case_insensitive
        })
    }
}