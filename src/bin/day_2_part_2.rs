use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let content = read_lines("src/inputs/day_2.txt");
    let result = evaluate_games(content);
    println!("{:?}", result);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        content.push(line.to_string());
    }
    content
}

fn evaluate_games(content: Vec<String>) -> i32 {
    // Regex pattern to find a number + string (=color)
    let re = Regex::new(r"(\d+)\s*(green|blue|red)").unwrap();
    let mut result = 0;
    // Iterates over each game
    for game in content {
        let mut rgb = vec![0, 0, 0];
        let parts: Vec<&str> = game.split(":").collect();
        // Takes the second part, the rounds of a game, and matches contents via regex
        // Iterates over each match group with the format (number + color)
        for cap in re.captures_iter(parts[1]) {
            rgb = get_min_value(cap[1].to_string(), cap[2].to_string(), rgb);
        }
        result += rgb[0]*rgb[1]*rgb[2];
    }
    result
}

fn get_min_value(value: String, color: String, mut rgb: Vec<i32>) -> Vec<i32> {
    match color.as_str() {
        "red" => {
            if value.parse::<i32>().unwrap() > rgb[0] {
                rgb[0] = value.parse::<i32>().unwrap();
            }
        }
        "green" => {
            if value.parse::<i32>().unwrap() > rgb[1] {
                rgb[1] = value.parse::<i32>().unwrap();
            }
        }
        "blue" => {
            if value.parse::<i32>().unwrap() > rgb[2] {
                rgb[2] = value.parse::<i32>().unwrap();
            }
        }
        _ => {
            println!("Validity check failed");
        }
    }
    rgb
}