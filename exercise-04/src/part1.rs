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

#[derive(Debug)]
struct InvalidScheduleError;

#[derive(Debug)]
struct Schedule {
    lower: i32,
    upper: i32,
}

impl FromStr for Schedule {
    type Err = InvalidScheduleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once("-").and_then(|(l,r)| {
            if r.contains("-") {
                return None;
            }

            let lower = FromStr::from_str(l).unwrap();
            let upper = FromStr::from_str(r).unwrap();
            Some(Schedule { lower, upper })
        }).ok_or(InvalidScheduleError)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let mut sum = 0;
    for line in contents.lines() {
        let schedule_string_vector: Vec<&str> = line.split(",").collect();
        let left_schedule = Schedule::from_str(schedule_string_vector[0]).unwrap();
        let right_schedule = Schedule::from_str(schedule_string_vector[1]).unwrap();
        // println!("left_schedule is {:?}, right_schedule is {:?}", left_schedule, right_schedule);
        // println!("{}", left_schedule.lower < right_schedule.lower);
        // println!("{}", left_schedule.upper > right_schedule.upper);

        if (left_schedule.lower <= right_schedule.lower && left_schedule.upper >= right_schedule.upper) || (right_schedule.lower <= left_schedule.lower && right_schedule.upper >= left_schedule.upper) {
            sum += 1;
        }
    }
    println!("{}", sum);
    Ok(())
}




#[cfg(test)]
mod tests {
    use super::*;
}
