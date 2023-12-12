use std::error::Error;
use std::fs;

pub struct Config {
    pub file_path: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let mut total: u32 = 0;

    for line in contents.lines() {
      let vector: Vec<&str> = line.split_whitespace().collect();
      let opponent = vector[0];
      let result = vector[1];
      
      total += get_shape_score(opponent, result) + get_outcome(result);
    }

    println!("{}", total);

    Ok(())
}

fn get_shape_score(opponent: &str, result: &str) -> u32 {
    match opponent {
        "A" => match result { 
            "X" => 3,
            "Y" => 1,
            _ => 2,
        },
        "B" => match result { 
            "Y" => 2,
            "Z" => 3,
            _ => 1,
        },
        "C" => match result { 
            "Z" => 1,
            "X" => 2,
            _ => 3,
        },
        _ => 0,
    }
}

fn get_outcome(shape: &str) -> u32 {
    match shape {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => 0 // put as zero first might do some error handling next time
    }
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
