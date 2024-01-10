use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let content = read_lines("src/inputs/test_input.txt"); // Test Input result should be 30
    let mut curr_card = 0;
    let mut result = 0;
    let mut value = 0;
    let mut card_values: Vec<i32> = Vec::new();
    let mut card_counters: HashMap<i32, i32> = HashMap::new();
    // fill the HashMap with all key/value pairs
    for line in content {
        curr_card += 1;
        value = evaluate_card(line);
        // println!("------------------------");
        // println!("current card: {:?}", curr_card);
        // println!("current values: {:?}", value);
        card_values.push(value);
        card_counters.insert(curr_card, 1);
    }
    // Iterate over the HashMap to add all additional cards
    for elem in 1..=card_counters.len() {
        for i in 0..*card_values.get(elem-1).unwrap() {
            // println!("ELEMENT: {}", elem);
            *card_counters.get_mut(&(elem as i32)).unwrap() += *card_values.get(elem-1).unwrap();
        }
        result += card_counters.get(&(elem as i32)).unwrap();
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

// Evaluates a single card to determine how many correct numbers are on it
fn evaluate_card(card: String) -> i32 {
    let card_content: Vec<&str> = card.split("|").collect();
    let winning_numbers: Vec<&str> = card_content[0].split(" ").collect();
    let my_numbers: Vec<&str> = card_content[1].split(" ").collect();
    let mut counter = 0;
    for number in my_numbers {
        if winning_numbers.contains(&number) && number.parse::<i32>().is_ok() {
            counter += 1;
        }
    }
    counter
}
