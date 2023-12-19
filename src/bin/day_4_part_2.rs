use std::fs::read_to_string;
use std::collections::HashMap;

// IDEAS
// HashMap with ID and Counter
// Recursive function per line going through all iterations
// backwards solving? simply add known counters 

fn main() {
    let content = read_lines("src/inputs/test_input.txt"); // Test Input result should be 30
    let mut curr_card = 0;
    let mut result = 0;
    let mut scratch_cards: HashMap<i32, Vec<i32>> = HashMap::new();
    for line in content {
        let mut additional_cards: Vec<i32> = Vec::new();
        curr_card += 1;
        let range = evaluate_card(line);
        for i in 1..=range {
            additional_cards.push(curr_card + i);
        }
        scratch_cards.insert(curr_card, additional_cards);
        result += count_cards(scratch_cards.clone(), curr_card, result);
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
    // println!("Card: {}", counter);
    counter
}

fn count_cards(mut cards: HashMap<i32, Vec<i32>>, curr_card: i32, mut result: i32) -> i32 {
    result += 1;
    println!("Current Card:{:?}", curr_card);
    println!("Contains {:?}", cards[&curr_card]);
    for card in cards[&curr_card.clone()] {
        result+= count_cards(cards.clone(), curr_card, result);
    }
    result
}
