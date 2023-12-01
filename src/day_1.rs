use std::fs::read_to_string;

pub fn run() {
    let result = read_lines("src/inputs/day_1.txt");
    println!("{}", result);
}

fn read_lines(filename: &str) -> u32 {
    let mut result = 0;
    for line in read_to_string(filename).unwrap().lines() {
        let mut numbers_in_line: Vec<char> = Vec::new();
        for char in line.chars() {
            if char.is_digit(10) {
                numbers_in_line.push(char);
            } 
        }
        let mut number= numbers_in_line.first().unwrap().to_string();
        number.push_str(numbers_in_line.last().unwrap().to_string().as_str());
        let number_val:u32 = number.parse::<u32>().unwrap();
        result += number_val;
        // result = all_numbers.first().unwrap() + all_numbers.last().unwrap();
    }
    result
}
