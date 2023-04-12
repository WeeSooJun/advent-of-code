use std::collections::HashSet;
use std::error::Error;
use std::fs;

pub struct Config {
    pub file_path: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let mut sum: u32 = 0;
    for line in contents.lines() {
        let mut sack = HashSet::new();
        let characters: Vec<char> = line.chars().collect();
        for i in 0..(line.len() / 2) {
            let value = convert_to_value(characters[i]);
            sack.insert(value);
        }
        for j in (line.len() / 2)..line.len() {
            let value = convert_to_value(characters[j]);
            if sack.contains(&value) {
                sum += value;
                break;
            }
        }
    }

    println!("{}", sum);

    Ok(())
}

fn convert_to_value(character: char) -> u32 {
    if character.is_ascii_uppercase() {
        return character as u32 - 38;
    }

    return character as u32 - 96;
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
