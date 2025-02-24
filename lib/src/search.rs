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

    pub fn search<'a>(self, query : &str, content : &'a str) -> Vec<&'a str> {
        if self.case_insensitive {
            content
                .lines()
                .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
                .collect()
        }
        else {   
            content
                .lines()
                .filter(|line| line.contains(query))
                .collect()
        }
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sensitive_no_result() {
        let content =
"\
The first line.
the second line.";
        
        let query = "test";
        let search = Search { case_insensitive: false };

        assert_eq!(Vec::<&str>::new(), search.search(query, content));
    }

    #[test]
    fn sensitive_upper_one_result() {
        let content =
"\
The first line.
the second line.";
        
        let query = "The";
        let search = Search { case_insensitive: false };

        assert_eq!(vec!["The first line."], search.search(query, content));
    }

    #[test]
    fn sensitive_lower_one_result() {
        let content =
"\
The first line.
the second line.";
        
        let query = "the";
        let search = Search { case_insensitive: false };

        assert_eq!(vec!["the second line."], search.search(query, content));
    }

    #[test]
    fn sensitive_two_results() {
        let content =
"\
The first line.
the second line.";
        
        let query = "line";
        let search = Search { case_insensitive: false };

        assert_eq!(vec!["The first line.", "the second line."], search.search(query, content));
    }

    #[test]
    fn insensitive_one_result() {
        let content =
"\
The first line.
the second line.";
        
        let query = "first";
        let search = Search { case_insensitive: true };

        assert_eq!(vec!["The first line."], search.search(query, content));
    }

    #[test]
    fn insensitive_two_results() {
        let content =
"\
The first line.
the second line.";
        
        let query = "the";
        let search = Search { case_insensitive: true };

        assert_eq!(vec!["The first line.", "the second line."], search.search(query, content));
    }
}