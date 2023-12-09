use std::fs::read_to_string;

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
    let mut id = 0;
    let possible = false;
    for game in content {
        let mut current_game = Vec::new();
        current_game = game.split(";").collect();
        // split each part at " " and add numbers
        
    }
}