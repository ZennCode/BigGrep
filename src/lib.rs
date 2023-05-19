use atty;
use clap::Parser;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author="ZennDev", version, about="A small GREP tool", long_about = None)]
pub struct BigGrepArgs {
    /// Query string
    pub query: String,
    /// Path to file (Optional pipe in some text to filter)
    pub file_path: Option<PathBuf>,
    /// Option Case sensetive
    #[arg(long, default_value_t = false)]
    pub case_sensitive: bool,
}
impl BigGrepArgs {
    pub fn new() -> Self {
        BigGrepArgs::parse()
    }
}

fn get_content(config: &BigGrepArgs) -> Result<String, Box<dyn Error>> {
    let mut contents = String::new();
    if config.file_path != None {
        contents = fs::read_to_string(config.file_path.as_ref().unwrap().display().to_string())?;
    } else {
        if atty::is(atty::Stream::Stdin) {
            let mut buffer = String::new();
            println!("Path to file:");
            let stdin = io::stdin(); // We get `Stdin` here.
            stdin.read_line(&mut buffer)?;
            contents = fs::read_to_string(buffer.trim().to_string())?;
        } else {
            io::stdin().read_to_string(&mut contents)?;
        }
    }
    Ok(contents)
}

pub fn run(config: BigGrepArgs) -> Result<(), Box<dyn Error>> {
    let contents = get_content(&config)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    results.iter().for_each(|line| println!("{line}"));
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query.to_lowercase().as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "dukt";
        let contents = "\
Rust:
sicher, schnell, produktiv.
PRODUKTION.";
        assert_eq!(vec!["sicher, schnell, produktiv."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query: &str = "rUsT";
        let contents = "\
Rust:
sicher, schnell, produktiv.
Nimm drei.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
