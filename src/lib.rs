use std::env;
use std::error::Error;
use std::fs;


pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    /// Creates a new Config object, by parsing the provided command line arguments
    /// and by reading the environment variables.
    /// 
    /// 
    /// # Errors
    /// 
    /// * Returns `Didn't get a query string` if the command line arguments iterator
    /// only have one or less elements.
    /// * Returns `Didn't get a file path` if the command line arguments iterator
    /// only have two or less elements.
    /// 
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Ignore name of the program
        args.next();
        
        // Parse arguments
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file_path string")
        };
        
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
        Ok(Config { query, file_path, ignore_case })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    
    for line in results {
        println!("{line}");
    }
    
    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}


pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";
        
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
