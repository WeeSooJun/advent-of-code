use std::collections::VecDeque;
use std::error::Error;
use std::fs;
use std::str::FromStr;

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let file_path = args[1].clone();
        Ok(Config { file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let first_line = contents.split_once("\n").unwrap().0;
    let num_stacks = calculate_number_of_stacks(first_line);
    let mut vec_of_stacks = init_stacks(num_stacks);
    for line in contents.lines(){
        if line.contains("[") && !line.contains("move") {
            process_line(line, &mut vec_of_stacks);
        }

        if line == "" {
            for i in 0..num_stacks {
                vec_of_stacks[i].reverse();
            }
        }

        if line.contains("move") {
            let move_action = MoveAction::from_str(line).unwrap();
            println!("{:?}", move_action);
            process_move_action(&move_action, &mut vec_of_stacks);
        }
    }
    println!("{:?}", vec_of_stacks.into_iter().map(|v| {
        v[v.len()-1]
    }).fold(String::new(), |a,b| a+&String::from(b)));
    Ok(())
}

fn process_line(line: &str, vector: &mut Vec<Vec<char>>) {
    let line_vec: Vec<char> = line.chars().collect();
    for i in (1..line_vec.len()).step_by(4) {
        if line_vec[i] != ' ' {
            vector[(i+3) / 4 - 1].push(line_vec[i])
        }
    }
} 

#[derive(Debug)]
struct InvalidMoveAction;

#[derive(Debug)]
struct MoveAction {
    num_moves: u32,
    source: usize, // 0-index
    destination: usize, // 0-index
}

fn process_move_action(move_action: &MoveAction, vector: &mut Vec<Vec<char>>) {
    let mut temp = VecDeque::new();
    for _ in 0..move_action.num_moves {
        let removed = vector[move_action.source].pop().unwrap();
        temp.push_front(removed);
    }
    vector[move_action.destination].append(&mut temp.into_iter().collect());
}

impl FromStr for MoveAction {
    type Err = InvalidMoveAction;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arr: Vec<&str> = s.split(" ").collect();
        Ok(MoveAction { num_moves: arr[1].parse().unwrap(), source: arr[3].parse::<usize>().unwrap() - 1, destination: arr[5].parse::<usize>().unwrap() - 1 })
    }
}

fn calculate_number_of_stacks(line: &str) -> usize {
    (line.len() + 1) / 4
}

fn init_stacks(size: usize) -> Vec<Vec<char>> {
    let mut result = Vec::with_capacity(size);
    for _ in 0..size {
        result.push(Vec::new());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
}
