pub struct Config {
    pub query: String,
    pub filepath: String
}

impl Config {

    pub fn build(configs : &[String]) -> Result<Self, &str> {

        if configs.len() != 3 { 
            return Err("configs not 3 arguments");
        }
        
        let query = configs[1].clone();
        let filepath  = configs[2].clone();
    
        Ok(Self { query, filepath })
    }
}