
pub fn search<'a>(search : &str, content : &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in content.lines() {
        if line.contains(search) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(search : &str, content : &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    let search = search.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&search) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, content));
    }
}