use std::collections::HashMap;

use regex::Regex;
fn main() {
    let input = include_str!("./text.txt");
    let list_of_numbers = create_list_of_numbers(input);
    let sum = sum_of_number_list(list_of_numbers);
    println!("The sum of numbers is {:?}", sum);
}

fn create_list_of_numbers(input: &str) -> Vec<u32> {
    let list_of_numbers = input
    .lines()
    .map(|line| {
        let digits = find_spelled_out_digits(line);
        let first_digit = digits.first().unwrap_or(&0);
        let last_digit = digits.last().unwrap_or(&0);
    (first_digit * 10) + last_digit
    })
    .collect();
    return list_of_numbers;
}

fn sum_of_number_list(list: Vec<u32>) -> u32 {
    list.iter().sum()
}

fn find_spelled_out_digits(line: &str) -> Vec<u32> {
    let digit_map = create_digit_map();
    let regex = Regex::new(r"(?i)(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    regex
        .find_iter(line)
        .filter_map(|mat| {
            let digit_str = mat.as_str().to_lowercase();
            digit_map.get(&digit_str).copied().or_else(|| digit_str.parse().ok())
        })
        .collect()
}

fn create_digit_map() -> HashMap<String, u32> {
    let mut map = HashMap::new();
    map.insert("one".to_string(), 1);
    map.insert("two".to_string(), 2);
    map.insert("three".to_string(), 3);
    map.insert("four".to_string(), 4);
    map.insert("five".to_string(), 5);
    map.insert("six".to_string(), 6);
    map.insert("seven".to_string(), 7);
    map.insert("eight".to_string(), 8);
    map.insert("nine".to_string(), 9);
    (0..=9).for_each(|n| { map.insert(n.to_string(), n); });
    map
}

//How do I use regex in Rust?