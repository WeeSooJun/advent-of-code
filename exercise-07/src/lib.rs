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
    directories: Vec<String>,
    files: Vec<File>,
    size: usize,
    full_path: String,
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
        directories: Vec::new(),
        files: Vec::new(),
        size: 0,
        full_path: "/".to_owned(),
    };
    let mut current_dir_name = "/".to_string(); // initialize root
    let mut current_path = "/".to_string();
    hashmap.insert(current_dir_name.clone(), root);
    for line in contents.lines() {
        if line.contains("/") {
            continue;
        }

        match line {
            line if line.starts_with("$ cd") => {
                if (&line[5..] == "..") {
                    let (new, _) = current_path.rsplit_once("/").unwrap();
                    let (new, _) = new.rsplit_once("/").unwrap();
                    if new == "" {
                        current_path = "/".to_string()
                    } else {
                        current_path = new.to_string() + "/";
                    }
                } else {
                    current_dir_name = line[5..].to_string();
                    current_path.push_str(&(line[5..].to_owned() + "/"));
                }
            }
            line if line.starts_with("$ ls") => process_curr_dir(),
            line if line.starts_with("dir") => {
                let dir_name = &line[4..];
                let mut key = current_path.clone();
                key.push_str(&dir_name);
                hashmap.insert(
                    key,
                    Directory {
                        directories: Vec::new(),
                        files: Vec::new(),
                        size: 0,
                        full_path: current_path.clone() + dir_name,
                    },
                );
                let (test, _) = current_path.rsplit_once("/").unwrap();
                let mut get_key = test.to_string();
                if current_dir_name == "/" {
                    get_key = "/".to_string()
                }
                hashmap
                    .get_mut(&get_key)
                    .unwrap()
                    .directories
                    .push(current_path.clone() + dir_name);
            }
            _ => {
                let (size, name) = line.split_once(" ").unwrap();
                let size: usize = size.parse().unwrap();
                let name = name.to_string();
                let (another, _) = current_path.rsplit_once("/").unwrap();
                let mut get_key = another.to_string();
                if current_path == "/" {
                    get_key = "/".to_string()
                }
                hashmap
                    .get_mut(&get_key)
                    .unwrap()
                    .files
                    .push(File { size, name })
            } // gna leave this like that for now, by right should match digits
        };
    }

    let mut children: Vec<(String, usize)> = Vec::new();

    // Sum up the sizes of the files in each directory first
    hashmap.values_mut().for_each(|directory| {
        directory.size = directory
            .files
            .clone()
            .into_iter()
            .fold(0, |acc, file| acc + file.size)
    });

    // Get all directories that are "leaves"
    hashmap
        .values()
        .filter(|directory| directory.directories.len() == 0)
        .for_each(|directory| {
            children.push((directory.full_path.clone(), directory.size));
        });

    // Initialise visited children count
    let mut another = HashMap::new();
    hashmap.values().for_each(|directory| {
        another.insert(directory.full_path.clone(), 0);
    });

    while children.len() != 0 {
        let mut new_children = Vec::new();

        for child in children {
            hashmap
                .values_mut()
                .filter(|directory| directory.directories.contains(&child.0))
                .for_each(|directory| {
                    directory.size += child.1;
                    *another.get_mut(&directory.full_path).unwrap() += 1;
                    if *another.get(&directory.full_path).unwrap() == directory.directories.len() {
                        // add "new" children in the sense that once the visited count is equals to the number of child directories then it essentially becomes a "leave" and you can consolidate file sizes by then
                        new_children.push((directory.full_path.clone(), directory.size));
                    }
                });
        }

        children = new_children;
    }

    let result = hashmap
        .values()
        .filter(|directory| {
            directory.size >= 30_000_000 - (70_000_000 - hashmap.get("/").unwrap().size)
        })
        .map(|directory| directory.size)
        .min()
        .unwrap();

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
