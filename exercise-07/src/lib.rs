use std::error::Error;
use std::fs;

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let file_path = args[1].clone();
        Ok(Config { file_path })
    }
}

struct Directory {
    directories: Vec<Directory>,
    files: Vec<File>,
}

struct File {
    size: usize,
    name: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let mut current_directory = Directory {
        directories: Vec::new(),
        files: Vec::new(),
    };
    for line in contents.lines() {
        if line.contains("/") {
            continue;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
}
