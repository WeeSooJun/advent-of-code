use std::collections::HashMap;
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

#[derive(Debug, Clone)]
struct Directory {
    parent_name: String,
    name: String,
    directories: Vec<String>,
    files: Vec<File>,
    size: usize,
}

#[derive(Debug, Clone)]
struct File {
    size: usize,
    name: String,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let mut hashmap: HashMap<String, Directory> = HashMap::new();
    let mut root = Directory {
        parent_name: "".to_owned(),
        name: "/".to_owned(),
        directories: Vec::new(),
        files: Vec::new(),
        size: 0,
    };
    let mut current_dir_name = "/".to_string(); // initialize root
    hashmap.insert(current_dir_name.clone(), root);
    for line in contents.lines() {
        if line.contains("/") {
            continue;
        }

        match line {
            line if line.starts_with("$ cd") => current_dir_name = line[5..].to_string(),
            line if line.starts_with("$ ls") => process_curr_dir(),
            line if line.starts_with("dir") => {
                let dir_name = &line[4..];
                hashmap.insert(
                    dir_name.to_string(),
                    Directory {
                        parent_name: current_dir_name.clone(),
                        name: dir_name.to_owned(),
                        directories: Vec::new(),
                        files: Vec::new(),
                        size: 0,
                    },
                );
                hashmap
                    .get_mut(&current_dir_name)
                    .unwrap()
                    .directories
                    .push(dir_name.to_string());
            }
            _ => {
                let (size, name) = line.split_once(" ").unwrap();
                let size: usize = size.parse().unwrap();
                let name = name.to_string();
                hashmap
                    .get_mut(&current_dir_name)
                    .unwrap()
                    .files
                    .push(File { size, name })
            } // gna leave this like that for now, by right should match digits
        };
    }

    println!("Initial hashmap: {:?}", hashmap);

    let mut children: Vec<(String, usize)> = Vec::new();

    hashmap.values_mut().for_each(|directory| {
        directory.size = directory
            .files
            .clone()
            .into_iter()
            .fold(0, |acc, file| acc + file.size)
    });

    hashmap
        .values_mut()
        .filter(|directory| directory.directories.len() == 0)
        .for_each(|directory| {
            children.push((directory.name.clone(), directory.size));
        });

    while children.len() != 0 {
        println!("children: {:?}", children);
        let mut new_children = Vec::new();
        hashmap
            .values_mut()
            .filter(|directory| {
                directory.directories.clone().into_iter().any(|dir_name| {
                    children
                        .clone()
                        .into_iter()
                        .map(|item| item.0)
                        .collect::<Vec<String>>()
                        .contains(&dir_name)
                })
            })
            .for_each(|directory| {
                new_children.push((directory.name.clone(), directory.size));
                directory.size += children
                    .clone()
                    .into_iter()
                    .map(|item| {
                        // println!("{:?}", item);
                        item.1
                    })
                    .sum::<usize>();
            });
        children = new_children;
    }

    println!("Final hashmap: {:?}", hashmap);
    let result: usize = hashmap
        .values()
        .filter(|directory| directory.size <= 100_000)
        .map(|directory| {
            // println!("{:?}", directory);
            directory.size
        })
        .sum();
    println!("{}", result);

    Ok(())
}

fn process_cd() {}

fn process_curr_dir() {}

fn process_file() {}

fn process_dir() {}

#[cfg(test)]
mod tests {
    use super::*;
}
