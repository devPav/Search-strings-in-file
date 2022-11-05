#[cfg(test)]
mod test {
    #[test]
    fn case_insensitive_success() {
        let query = "Duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["Duct tape."], minigrep::search(query, contents));
    }
}