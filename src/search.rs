use crate::config::Config;

pub struct Search {
    pub case_insensitive: bool
}

impl Search {
    pub fn build(config: &Config) -> Self {
        Self {
            case_insensitive: config.case_insensitive
        }
    }

    pub fn search<'a>(self, search : &str, content : &'a str) -> Vec<&'a str> {
        if self.case_insensitive {
            content
                .lines()
                .filter(|line| line.to_lowercase().contains(&search.to_lowercase()))
                .collect()
        }
        else {   
            content
                .lines()
                .filter(|line| line.contains(search))
                .collect()
        }
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

//     #[test]
//     fn one_result() {
//         let query = "duct";
//         let content = "\
// Rust:
// safe, fast, productive.
// Pick three.";

//         assert_eq!(vec!["safe, fast, productive."], search(query, content));
//     }
}