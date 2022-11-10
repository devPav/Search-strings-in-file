use std::env::Args;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did'n find quary"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did'n find filename"),
        };
        let case_sensitive = match args.next() {
            Some(arg) => arg == "1",
            None => std::env::var("CASE_INSENSITIVE").is_err(),
        };
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn config_new_not_enought_parameters() -> Result<(), &'static str> {
        let args = std::env::args();
        Config::new(args)?;
        Ok(())
    }
    #[test]
    #[ignore]
    fn config_new_enought_parameters() -> Result<(), &'static str> {
        let args = std::env::args();
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
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
