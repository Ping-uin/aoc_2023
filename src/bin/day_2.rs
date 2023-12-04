use std::fs::read_to_string;

fn main() {
    let content = read_lines("src/inputs/day_2.txt");
    println!("{:?}",content)
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut content: Vec<String> = Vec::new();
    // let mut changed_lines:Vec<String> = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        content.push(line.to_string());
    }
    content
}