use std::fs::read_to_string;

// Main method of the module
pub fn run() {
    let result = read_lines("src/inputs/day_1.txt");
    println!("{}", result);
}

// Iterates over the textfile line by line
fn read_lines(filename: &str) -> u32 {
    let mut result = 0;
    for line in read_to_string(filename).unwrap().lines() {
        result += pick_numbers(line);
    }
    result
}

// Iterates over each char in a line to find the first and last
fn pick_numbers(line: &str) -> u32 {
    let mut numbers_in_line: Vec<char> = Vec::new();
    for char in line.chars() {
        if char.is_digit(10) {
            numbers_in_line.push(char);
        } 
    }
    let mut number= numbers_in_line.first().unwrap().to_string();
    number.push_str(numbers_in_line.last().unwrap().to_string().as_str());
    let number:u32 = number.parse::<u32>().unwrap();
    number
}
