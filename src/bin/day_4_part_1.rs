use std::fs::read_to_string;

fn main() {
    let content = read_lines("src/inputs/day_4.txt");
    let mut result = 0;
    for line in content {
        result += evaluate_card(line);
    }
    println!("{}", result);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        content.push(line.to_string());
    }
    content
}

fn evaluate_card(card: String) -> i32 {
    let mut factor = 0;
    let card_content: Vec<&str> = card.split("|").collect(); 
    let winning_numbers:Vec<&str> = card_content[0].split(" ").collect();
    let my_numbers:Vec<&str> = card_content[1].split(" ").collect();
    for number in my_numbers {
        if winning_numbers.contains(&number) && number.parse::<i32>().is_ok() {
            // println!("{}", number);
            factor = factor + 1;
        }
    }
    let points = calculate_score(factor);
    points
}

fn calculate_score(factor: i32) -> i32 {
    let mut points = 0;
    if factor > 0 {
        points = 1;
        for i in 1..=(factor-1) {
            points = points*2;
        }
    }
    points
}