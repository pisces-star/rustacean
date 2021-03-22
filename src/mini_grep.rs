use std::{env, fs, process};
use std::result::Result::Err;
use std::error::Error;

pub fn min_grep() {
    let args: Vec<String> = env::args().collect();
    println!("{:#?}", args);

    let config = Config::new(&args).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments:{}", error);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error:{}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
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

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}


struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
          return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::mini_grep::search_case_insensitive;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let content = "
        Rust:
        fast,safe,productive.
        Pick three.
        Duct tape.";
        assert_eq!(vec!["fast,safe,productive"], serach(query, content));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "
        Rust:
        fast,safe,productive.
        Pick three.
        Trust me.";
        assert_eq!(Vec!["Rust:","Trust me."], search_case_insensitive(query, content))
    }
}