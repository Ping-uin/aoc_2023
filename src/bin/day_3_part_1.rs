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
    let mut values: Vec<i32> = Vec::new();
    for row in &plan {
        i += 1;
        j = 0;
        // Iterating over each character in the row
        for char in row {
            j += 1;
            if find_symbol(*char) {
                // Define all possible directions (relative coordinates) to check around a point
                let directions = [
                    (-1, -1), (-1, 0), (-1, 1),
                    (0, -1),           (0, 1),
                    (1, -1), (1, 0),  (1, 1),
                ];
            
                for &(dx, dy) in &directions {
                    let new_i = i as i32 + dx;
                    let new_j = j as i32 + dy;
            
                    // Check if the new position is within the grid boundaries
                    if new_i >= 0 && new_i < plan.len() as i32 && new_j >= 0 && new_j < plan[0].len() as i32 {
                        let new_i = new_i as usize;
                        let new_j = new_j as usize;
            
                        // Check if the character at the new position is a number
                        if let Some(number) = plan[new_i][new_j].to_digit(10) {
                            find_full_number(row, number as i32);
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", values);
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

fn find_full_number(row: &Vec<char>,number: i32) -> i32 {
    
    1
}

// fn find_value(plan: &Vec<Vec<char>>, x: i32, y: i32, i: i32, j: i32) -> Vec<i32> {
  
//     values
// }

fn find_symbol(char: char) -> bool {
    let mut found_sym = false;
    if char != '.' && !char.is_ascii_digit() {
        // println!("{}", char);
        found_sym = true;
    }
    found_sym
}
