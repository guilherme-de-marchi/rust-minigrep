use std::{fs, env};
use std::error::Error;

pub struct Config<'a> {
    query: &'a String,
    path: &'a String,
    ignore_case: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &'a Vec<String>) -> Result<Config<'a>, &'static str> {
        let query = match args.get(1) {
            Some(x) => x,
            None => return Err("could not get args[1]"),
        };
    
        let path = match args.get(2) {
            Some(x) => x,
            None => return Err("could not get args[2]"),
        };
    
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query: query, path: path, ignore_case: ignore_case})
    }
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&conf.path)?;
    
    let result = if conf.ignore_case {
        search_case_insensitive(&conf.query, &content)
    } else {
        search(&conf.query, &content)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &'a str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

# [cfg(test)]
mod tests {
    # [test]
    fn search_one_result() {
        let query = "abc";
        let content = "\
123456
abcdef
456789";

        assert_eq!(vec!["abcdef"], super::search(query, content));
    }

    # [test]
    fn search_case_sensitive() {
        let query = "abc";
        let content = "\
123456
abcdef
ABCDEF
456789";

        assert_eq!(vec!["abcdef"], super::search(query, content));
    }

    # [test]
    fn search_case_insensitive() {
        let query = "AbC";
        let content = "\
123456
abcdef
ABCDEF
456789";

        assert_eq!(vec!["abcdef", "ABCDEF"], super::search_case_insensitive(query, content));
    }
}