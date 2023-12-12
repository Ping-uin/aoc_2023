use std::fs::read_to_string;
// use regex::Regex;

fn main() {
    let content = read_lines("src/inputs/test_input.txt");
    // let content = read_lines("src/inputs/day_3.txt");
    let plan = create_field_vec(content);
    // Iterating over each row of characters
    for row in plan {
        // Iterating over each character in the row
        for ch in row {
            println!("{}", ch);
        }
    }
    // println!("{:?}", plan);
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        content.push(line.to_string());
    }
    content
}

fn create_field_vec(content: Vec<String>) -> Vec<Vec<char>> {
    let mut plan: Vec<Vec<char>> = Vec::new();
    for line in content {
        let mut row: Vec<char> = Vec::new();
        for char in line.chars() {
            row.push(char);
        }
        // println!("{:?}", row);
        plan.push(row);
    }
    plan
}

fn find_value() {

}

fn find_symbol(char: char) -> bool{
    let found_char = false;

    found_char
}
