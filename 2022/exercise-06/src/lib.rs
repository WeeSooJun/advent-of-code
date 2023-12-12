use std::collections::{HashSet, VecDeque};
use std::error::Error;
use std::fs;

pub struct Config {
    pub file_path: String,
    pub marker_length: usize,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let file_path = args[1].clone();
        let marker_length = args[2].clone().parse().unwrap();
        Ok(Config {
            file_path,
            marker_length,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let contents_vec = contents.chars().collect::<Vec<char>>();
    let temp = &contents_vec[..config.marker_length - 1];
    let mut queue: VecDeque<char> = VecDeque::from_iter(Vec::from(temp));
    for i in (config.marker_length - 1)..contents_vec.len() {
        queue.push_back(contents_vec[i]);
        if is_marker(&queue) {
            println!("{}", i + 1);
            break;
        }
        queue.pop_front();
    }
    Ok(())
}

fn is_marker(vector: &VecDeque<char>) -> bool {
    // println!("{:?}", vector);
    let mut uniq = HashSet::new();
    vector.into_iter().all(move |x| uniq.insert(x))
}

#[cfg(test)]
mod tests {
    use super::*;
}
