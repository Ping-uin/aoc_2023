use std::fs::read_to_string;
use regex::{ Regex, Captures };

const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

fn main() {
    let content = read_lines("src/inputs/day_2.txt");
    // println!("{:?}",content);
    evaluate_games(content);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    // let mut changed_lines:Vec<String> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        // println!("{}",line);
        content.push(line.to_string());
    }
    content
}

fn evaluate_games(content: Vec<String>) {
    let re = Regex::new(r"Game (\d+):").unwrap();
    for game in content {
        if let Some(captures) = re.captures(&game) {
            if let Some(id) = captures.get(1) {
                let id = id.as_str().parse::<u32>().unwrap();
                println!("ID: {}", id);
            }
        }
    }
}