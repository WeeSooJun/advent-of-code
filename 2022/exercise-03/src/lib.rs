use std::collections::{ HashSet };
use std::error::Error;
use std::fs;

pub struct Config {
    pub file_path: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let vec_line: Vec<&str> = contents.lines().collect();
    let mut sum: u32 = 0;
    for i in (0..vec_line.len()).step_by(3) {
        let vector = vec![vec_line[i],vec_line[i+1], vec_line[i+2]];
        match process_group(vector) {
            Ok(val) => sum += val,
            Err(_) => { println!("More than one badge in input") },
        };
    };
    println!("{}", sum);

    Ok(())
}

#[derive(Debug, Clone)]
struct MoreThanOneBadgeError {}

fn process_group(group_string: Vec<&str>) -> Result<u32, MoreThanOneBadgeError> {
    let mut hashset:HashSet<u32, _> = HashSet::from_iter(1..=52);
    for i in 0..3 {
        hashset = convert_to_hashset(group_string[i]).intersection(&hashset).cloned().collect();
    }

    let mut count = 0;
    let mut result = 0;
    for val in hashset.iter() {
        count += 1;
        result = *val;
    }

    if count >= 2 {
        return Err(MoreThanOneBadgeError {});
    }

    Ok(result)

}

fn convert_to_hashset(line: &str) -> HashSet<u32> {
    let mut hashset = HashSet::new();
    for character in line.chars() {
        hashset.insert(convert_to_value(character));
    }
    hashset
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
