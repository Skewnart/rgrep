use std::env;

pub struct Config {
    pub query: String,
    pub filepath: String,
    pub case_insensitive: bool
}

impl Config {

    pub fn build(configs : &[String]) -> Result<Self, &str> {

        if configs.len() <= 2 || configs.len() > 4 { 
            return Err("needs 2 or 3 arguments");
        }

        let mut case_insensitive = env::var("CASE_INSENSITIVE").is_ok();
        if configs.len() == 4 && configs[3] == "-i" {
            case_insensitive = true;
        }
        
        let query = configs[1].clone();
        let filepath  = configs[2].clone();
    
        Ok(Self { query, filepath, case_insensitive })
    }
}