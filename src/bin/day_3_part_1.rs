use std::fs::read_to_string;
// use regex::Regex;

fn main() {
    // let content = read_lines("src/inputs/test_input.txt");
    let content = read_lines("src/inputs/day_3.txt");
    let x = content.len();
    let y = content.get(0).unwrap().len();
    let plan = create_field_vec(content);
    let mut result = 0;
    let mut i = 0;
    let mut j = 0;
    println!("{}", x);
    println!("{}", y);
    for row in plan {
        i += 1;
        j = 0;
        // Iterating over each character in the row
        for char in row {
            j += 1;
            if find_symbol(char) {
                find_value(x as i32, y as i32, i, j);
            }
        }
    }
    println!("{:?}", result);
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

fn find_number() -> i32 {
1
}

fn find_value(x: i32, y: i32, i: i32, j: i32) -> Vec<i32> {
    let values: Vec<i32> = Vec::new();
    values
}

fn find_symbol(char: char) -> bool {
    let mut found_sym = false;
    if char != '.' && !char.is_ascii_digit() {
        // println!("{}", char);
        found_sym = true;
    }
    found_sym
}
