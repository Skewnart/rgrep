use std::{env::{self}, io::{self, BufRead}};
use crate::stdin::Terminal;

#[derive(Debug)]
pub enum Input {
    Filepath(String),
    Content(String)
}

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub input: Input,
    pub case_insensitive: bool
}

impl Config {

    pub fn build(_terminal_service : impl Terminal, mut args : impl Iterator<Item = String>) -> Result<Self, String> {
        args.next();

        let Some(_query) = args.next() else {
            return Err("Query not provided".to_string());
        };

        let _from_pipe = !_terminal_service.is_terminal();

        let _input = if _from_pipe {
            Input::Content(io::stdin().lock().lines().fold(String::from(""), |acc, line| acc + &line.unwrap() + "\n"))
        }
        else if let Some(_filepath) = args.next() {
            Input::Filepath(_filepath)
        } else { 
            return Err("File not provided".to_string());
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


#[cfg(test)]
mod tests {

    use std::vec::IntoIter;

    use super::*;

    pub struct StdinServiceMock;
    impl StdinServiceMock {
        pub fn new() -> Self {
            Self {}
        }
    }
    impl Terminal for StdinServiceMock {
        fn is_terminal(&self) -> bool {
            true
        }
    }

    fn extract_query_into_iter(input: &str) -> IntoIter<String> {
        input.split_whitespace().map(String::from).collect::<Vec<String>>().into_iter()
    }

    #[test]
    fn check_bad_format() {
        let args= extract_query_into_iter("program.exe");
        let config = Config::build(StdinServiceMock::new(), args);
        
        assert!(config.is_err());

        let args= extract_query_into_iter("program.exe query");
        let config = Config::build(StdinServiceMock::new(), args);

        println!("{:?}", config);
        
        assert!(config.is_err());
    }

    #[test]
    fn check_query_file() {
        let args= extract_query_into_iter("program.exe query file");
        let config = Config::build(StdinServiceMock::new(), args);        

        println!("{:?}", config);
        
        assert!(config.is_ok());
        let config = config.expect("Result should be ok.");

        assert_eq!(config.query, "query");
        assert!(matches!(config.input, Input::Filepath(_)));
    }

    #[test]
    fn check_insensitive() {
        let args= extract_query_into_iter("program.exe query file -i");
        let config = Config::build(StdinServiceMock::new(), args);
        
        assert!(config.is_ok());
        let config = config.expect("Result should be ok.");

        assert!(config.case_insensitive);
        
        let args= extract_query_into_iter("program.exe query file");
        let config = Config::build(StdinServiceMock::new(), args);
        
        assert!(config.is_ok());
        let config = config.expect("Result should be ok.");

        assert!(!config.case_insensitive);    
    }
}