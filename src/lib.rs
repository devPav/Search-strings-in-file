use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enought arguments");   
        };
        let case_sensitive = if args.len() == 3 {
            env::var("CASE_INSENSITIVE").is_err()    
        } else {
            args[3] == "1"
        };
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename, case_sensitive })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)    
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    };
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    };
    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn config_new_not_enought_parameters() -> Result<(), &'static str> {
        let args = vec![String::from("1")];
        Config::new(args)?;
        Ok(())
    }
    #[test]
    #[ignore]
    fn config_new_enought_parameters() -> Result<(), &'static str> {
        let args = vec![String::from("1"), String::from("2"), String::from("3")];
        Config::new(args)?;
        Ok(())
    }
    #[test]
    #[ignore]
    fn cannot_read_file_in_run_not_optimal() {
        let config = Config {
            query: String::from("1"),
            filename: String::from("2"),
            case_sensitive: true,
        };
        if let Err(e) = run(config) {
            panic!("{}", e);
        };
    }
    #[test]
    #[ignore]
    fn cannot_read_file_in_run_optimal() -> Result<(), Box<dyn Error>> {
        let config = Config {
            query: String::from("1"),
            filename: String::from("poem.txt"),
            case_sensitive: true,
        };
        run(config)?;
        Ok(())
    }
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
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));       
    }
}
