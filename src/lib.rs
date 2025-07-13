
use std::env::Args;
use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub ignore_case: bool
}

impl Config {
    // pub fn build (mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    pub fn build (mut args: Args) -> Result<Config, &'static str> {    
        // if args.len() < 3 {
        //     return Err("No enough arguments");
        // }

        args.next(); // consume the name of the program

        let ignore_case =  match env::var("IGNORE_CASE")
        .unwrap_or(String::from("0")).as_str() {
            "1" => true,
            _ => false,
        };

        let query = match args.next() {
            Some(s) => s,
            None => return Err("didn't get a query string") 
        };
        let file_name = match args.next() {
            Some(s) => s,
            None => return Err("didn't get a filepath")
        };

        Ok( Config { query, file_name, ignore_case } )
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file_name)?;
    
    let res = search(&config.query, &content, config.ignore_case); 

    res
    .iter()
    .for_each(|l| println!("{l}"));
    
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str, ignore_case: bool) -> Vec<&'a str> {
    // let mut matches = vec![];
    // for line in content.lines() {
    //     if (ignore_case && line.to_lowercase().contains(&query.to_lowercase()))
    //         || line.contains(query) {
    //             matches.push(line);
    //     }
    // }
    // matches
    content
    .lines()
    .filter(|l| 
        (ignore_case && l.to_lowercase().contains(&query.to_lowercase())) 
        || l.contains(&query))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_case_sensitive() {
        let q = "duct";
        let c = "Rust:
safe, fast, productive
Pick three.";

        assert_eq!(vec!["safe, fast, productive"], search(q, c, false));
    }

    #[test]
    fn search_case_insensitive() {
        let q = "rUsT";
        let c = "Rust:
safe, fast, productive
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:","Trust me."], search(q, c, true));
    }
}