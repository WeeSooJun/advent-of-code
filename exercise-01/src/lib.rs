use std::error::Error;
use std::fs;
// use std::env;

pub struct Config {
    pub file_path: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let mut first: u32 = 0;
    let mut second: u32 = 0;
    let mut third: u32 = 0;
    let mut sum = 0;

    for line in contents.lines() {
        if line.is_empty() {
            if sum > first {
                third = second;
                second = first;
                first = sum;
            } else if sum > second {
                third = second;
                second = sum;
            } else if sum > third {
                third = sum;
            }
            sum = 0;
        } else {
            sum += line.parse::<u32>().unwrap();
        }
    }

    println!("{}", first + second + third);

    Ok(())
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
}
