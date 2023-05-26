use std::error::Error;
use std::fs;


pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Check if user has specified enough command line arguments
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // Parse arguments
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = search(&config.query, &contents);
    for line in results {
        println!("{line}");
    }
    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&'a str> = Vec::new();
    
    // Find all the lines that contain the query string
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    return results;
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
