use std::{ fs::read_to_string, collections::HashMap };

pub fn main() {
    let content = read_lines("src/inputs/day_1.txt");
    // println!("{:?}", content);
    let result = pick_numbers(content);
    println!("{:?}", result);
    
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    // let mut changed_lines:Vec<String> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        content.push(rewrite_lines(line));
    }
    content
}

fn rewrite_lines(line: &str) -> String {
    let mut filtered_line = line.to_string();
    let numbers: HashMap<String, &str> = HashMap::from([
        ("zero".to_string(), "0"),
        ("one".to_string(), "1"),
        ("two".to_string(), "2"),
        ("three".to_string(), "3"),
        ("four".to_string(), "4"),
        ("five".to_string(), "5"),
        ("six".to_string(), "6"),
        ("seven".to_string(), "7"),
        ("eight".to_string(), "8"),
        ("nine".to_string(), "9"),
    ]);
    let no_attachment: bool = false;
    for element in numbers.keys() {
        if line.contains(element) && no_attachment {
            filtered_line = filtered_line.replace(element, numbers.get(element).unwrap());
        }
    }
    filtered_line
}

fn pick_numbers(content: Vec<String>) -> u32 {
    let mut number: String;
    let mut result:u32 = 0;
    for line in content {
        let mut numbers_in_line: Vec<char> = Vec::new();
        for char in line.chars() {
            if char.is_numeric() {
                numbers_in_line.push(char)
            }
        }
        number = numbers_in_line.first().unwrap().to_string();
        number.push(numbers_in_line.last().unwrap().to_owned());
        // println!("{:?}", number);
        result += number.parse::<u32>().unwrap();
    }
    result
}