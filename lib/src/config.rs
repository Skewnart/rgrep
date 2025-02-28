use std::{env::{self}, fs::OpenOptions, io::{self, BufRead}};
use crate::stdin::Terminal;

pub enum Input {
    Filepath(String),
    Content(String)
}

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

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub arguments: Vec<String>
}

#[derive(Debug)]
pub struct CmdLineExtractor {
    pub program_name: String,
    pub content_piped: Option<String>,
    pub arguments: Option<Vec<String>>,
    pub parameters: Option<Vec<Parameter>>,
}

impl CmdLineExtractor {
    pub fn build(_terminal_service : impl Terminal, mut args : impl Iterator<Item = String>) -> Result<Self, String> {

        let Some(_program_name) = args.next() else {
            return Err("Program name was not automatically provided.".to_string());
        };

        let _from_pipe = !_terminal_service.is_terminal();

        let _content_piped = if _from_pipe {
            Some(io::stdin().lock().lines().fold(String::from(""), |acc, line| acc + &line.unwrap() + "\n"))
        } else {
            None
        };

        let mut _arguments: Vec<String> = vec![];
        let mut _parameters: Vec<Parameter> = vec![];

        let mut current_parameter: Option<Parameter> = None;

        for arg in args {
            if arg.starts_with("-") {
                if let Some(parameter) = current_parameter {
                    _parameters.push(parameter);
                }

                current_parameter = Some(Parameter {
                    name: arg,
                    arguments: Vec::<String>::new()
                });
            }
            else {
                match current_parameter {
                    None => { _arguments.push(arg); },
                    Some(ref mut parameter) => {parameter.arguments.push(arg);}
                }
            }
        }

        if let Some(parameter) = current_parameter {
            _parameters.push(parameter);
        }

        Ok(Self{
            program_name: _program_name,
            content_piped : _content_piped,
            arguments : if !_arguments.is_empty() { Some(_arguments) } else { None },
            parameters : if !_parameters.is_empty() { Some(_parameters) } else { None }
        })
    }
}

#[cfg(test)]
mod tests {

    use std::vec::IntoIter;
    use crate::stdin::StdinServiceMock;

    use super::*;

    fn extract_query_into_iter(input: &str) -> IntoIter<String> {
        input.split_whitespace().map(String::from).collect::<Vec<String>>().into_iter()
    }

    #[test]
    fn check_bad_format() {
        let args= extract_query_into_iter("program.exe");
        let config = Config::build(StdinServiceMock { is_terminal: true }, args);
        
        assert!(config.is_err());

        let args= extract_query_into_iter("program.exe query");
        let config = Config::build(StdinServiceMock { is_terminal: true }, args);
        
        assert!(config.is_err());
    }

    #[test]
    fn check_query_file() {
        let args= extract_query_into_iter("program.exe query file");
        let config = Config::build(StdinServiceMock { is_terminal: true }, args);        
        
        assert!(config.is_ok());
        let config = config.expect("Result should be ok.");

        assert_eq!(config.query, "query");
        assert!(matches!(config.input, Input::Filepath(_)));
    }

    #[test]
    fn check_insensitive() {
        let args= extract_query_into_iter("program.exe query file -i");
        let config = Config::build(StdinServiceMock { is_terminal: true }, args);
        
        assert!(config.is_ok());
        let config = config.expect("Result should be ok.");

        assert!(config.case_insensitive);
        
        let args= extract_query_into_iter("program.exe query file");
        let config = Config::build(StdinServiceMock { is_terminal: true }, args);
        
        assert!(config.is_ok());
        let config = config.expect("Result should be ok.");

        assert!(!config.case_insensitive);    
    }

    #[test]
    fn test() {
        let args= extract_query_into_iter("program.exe query file -i");
        let config = CmdLineExtractor::build(StdinServiceMock { is_terminal: true }, args);

        eprintln!("{:?}", config);
    }
}