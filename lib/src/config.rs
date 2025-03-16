use std::env;
use args_extractor::Prompt;

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

    pub fn build(prompt: Prompt) -> Result<Self, String> {

        let Some(_args) = prompt.arguments else {
            return Err("Query not provided".to_string());
        };

        let _query = _args[0].clone();

        let _input = 
        match prompt.content_piped {
            Some(content) => Input::Content(content),
            None => {
                match _args.get(1) {
                    Some(arg) => Input::Filepath(arg.clone()),
                    None => return Err("File not provided".to_string())
                }
            }
        };

        let mut _case_insensitive = env::var("CASE_INSENSITIVE").is_ok();

        if let Some(params) = prompt.parameters {
            for (key, _) in params {
                match key.as_str() {
                    "-i" => { _case_insensitive = true; },
                    _ => ()
                }
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
    use args_extractor::{PromptExtractor, StdinServiceMock};

    use super::*;

    fn extract_query_into_iter(input: &str) -> IntoIter<String> {
        input.split_whitespace().map(String::from).collect::<Vec<String>>().into_iter()
    }

    #[test]
    fn check_bad_format() {
        let args= extract_query_into_iter("program.exe");
        let prompt = PromptExtractor::new(StdinServiceMock { is_terminal: true })
            .extract(args)
            .expect("Prompt should be ok");
        let config = Config::build(prompt);
        
        assert!(config.is_err());

        let args= extract_query_into_iter("program.exe query");
        let prompt = PromptExtractor::new(StdinServiceMock { is_terminal: true })
            .extract(args)
            .expect("Prompt should be ok");

        let config = Config::build(prompt);
        
        assert!(config.is_err());
    }

    #[test]
    fn check_query_file() {
        let args= extract_query_into_iter("program.exe query file");
        let prompt = PromptExtractor::new(StdinServiceMock { is_terminal: true })
            .extract(args)
            .expect("Prompt should be ok");

        let config = Config::build(prompt);        
        
        assert!(config.is_ok());
        let config = config.expect("Result should be ok.");

        assert_eq!(config.query, "query");
        assert!(matches!(config.input, Input::Filepath(_)));
    }

    #[test]
    fn check_insensitive() {
        let args= extract_query_into_iter("program.exe query file -i");
        let prompt = PromptExtractor::new(StdinServiceMock { is_terminal: true })
            .extract(args)
            .expect("Prompt should be ok");

        let config = Config::build(prompt);   
        
        assert!(config.is_ok());
        let config = config.expect("Result should be ok.");

        assert!(config.case_insensitive);
        
        let args= extract_query_into_iter("program.exe query file");
        let prompt = PromptExtractor::new(StdinServiceMock { is_terminal: true })
            .extract(args)
            .expect("Prompt should be ok");

        let config = Config::build(prompt);   
        
        assert!(config.is_ok());
        let config = config.expect("Result should be ok.");

        assert!(!config.case_insensitive);    
    }
}