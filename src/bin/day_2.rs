use std::fs::read_to_string;
use regex::Regex;

const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

fn main() {
    let content = read_lines("src/inputs/day_2.txt");
    let result = evaluate_games(content);
    println!("{:?}",result);
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

fn evaluate_games(content: Vec<String>) -> i32{
    let re = Regex::new(r"(\d+)\s*(green|blue|red)").unwrap();
    let mut result = 0;
    let mut game_id;
    let mut valid;
    // Iterates over each game
    for game in content {
        valid = true;
        let parts: Vec<&str> = game.split(":").collect();
        game_id = (parts[0])[5..].parse::<i32>().unwrap();
        // println!("{:?}", game_id);
        // Takes the second part, the rounds of a game, and matches contents via regex
        for cap in re.captures_iter(parts[1]) {
            // println!("{:?}{:?}", cap[1].to_string(), cap[2].to_string());
            if valid {valid = check_validity(cap[1].to_string(), cap[2].to_string());} else {continue;}
            // println!("{:?}", valid);
        }
        // println!("NEXT GAME");
        if valid {result += game_id;}
    }
    result
}

// Checks if a value is greater than the possible max value
fn check_validity(value:String, color: String) -> bool {
    let mut pass= true;
    match color.as_str() {
        "blue" => {
            if value.parse::<i32>().unwrap()>BLUE{pass = false;}
        }
        "red" => {
            if value.parse::<i32>().unwrap()>RED{pass = false;}
        }
        "green" => {
            if value.parse::<i32>().unwrap()>GREEN{pass = false;}
        }
        _ => {println!("Validity check failed")}
    }
    pass
}