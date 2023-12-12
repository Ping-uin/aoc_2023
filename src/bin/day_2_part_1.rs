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
    for line in read_to_string(filename).unwrap().lines() {
        content.push(line.to_string());
    }
    content
}

fn evaluate_games(content: Vec<String>) -> i32{
    // Regex pattern to find a number + string (=color)
    let re = Regex::new(r"(\d+)\s*(green|blue|red)").unwrap();
    let mut result = 0;
    let mut game_id;
    let mut valid;
    // Iterates over each game
    for game in content {
        valid = true; // Resets the validity bool for each game
        let parts: Vec<&str> = game.split(":").collect();
        game_id = (parts[0])[5..].parse::<i32>().unwrap(); // Selects the game id as i32
        // Takes the second part, the rounds of a game, and matches contents via regex
        // Iterates over each match group with the format (number + color)
        for cap in re.captures_iter(parts[1]) {
            // Only check if for a valid round, if a previous round hasnt been invalid already -> game invalid anyways
            if valid {valid = check_validity(cap[1].to_string(), cap[2].to_string());} else {continue;}
        }
        // Adds the game id to result if it's a valid game
        if valid {result += game_id;}
    }
    result
}

// Checks if a value is greater than the possible max value
// Returns false if so, passes true if each value is valid
fn check_validity(value:String, color: String) -> bool {
    let mut pass= true;
    // Matches which color value pair is checked
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