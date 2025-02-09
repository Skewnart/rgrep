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
        let mut results = Vec::new();
    
        if self.case_insensitive {
            let search = search.to_lowercase();
            for line in content.lines() {
                if line.to_lowercase().contains(&search) {
                    results.push(line);
                }
            }
        }
        else {
            for line in content.lines() {
                if line.contains(search) {
                    results.push(line);
                }
            }
        }
    
        results
        // content
        //     .lines()
        //     .map(|line| line.to_lowercase())
        //     .filter(|line| line.contains(&search.to_lowercase()))
        //     .collect()
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