use std::error::Error;
use std::fs;

pub struct Config {
    pub file_path: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in contents.lines() {
        println!("{}", line);
    }

    // println!("{}", );

    Ok(())
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let file_path = args[1].clone();

        Ok(Config { file_path })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
