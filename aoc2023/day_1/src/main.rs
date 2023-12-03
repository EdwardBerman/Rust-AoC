use std::fs;

fn filter_digits(input: &str) -> String {
    input.chars().filter(|c| c.is_digit(10)).collect()
}

fn convert_substrings_to_numbers(input: &str) -> String {
    let number_words = vec![
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut converted_string = input.to_string();
    for (word, number) in number_words {
        let word_with_digit = insert_digit_in_middle(word, number);
        converted_string = converted_string.replace(word, &word_with_digit);
    }

    converted_string
}

fn insert_digit_in_middle(word: &str, digit: &str) -> String {
    let middle = word.len() / 2;
    let mut word_with_digit = String::new();
    word_with_digit.push_str(&word[..middle]);
    word_with_digit.push_str(digit);
    word_with_digit.push_str(&word[middle..]);
    word_with_digit
}

fn main() {
    println!("{}", convert_substrings_to_numbers("eightwo"));
    let contents = fs::read_to_string("/home/eddieberman/Rust-AoC/aoc2023/day_1/src/input.txt")
        .expect("Something went wrong reading the file");

    let lines: Vec<&str> = contents.split('\n').collect();
    let filtered_lines: Vec<String> = lines.iter()
        .map(|line| convert_substrings_to_numbers(line))
        .collect();

    let digits_only: Vec<String> = filtered_lines.iter()
        .map(|line| filter_digits(line))
        .collect();

    let mut total = 0;

    for line in &digits_only {
        if !line.is_empty() {
            let first_digit = line.chars().next().unwrap().to_digit(10).unwrap();
            let last_digit = line.chars().last().unwrap().to_digit(10).unwrap();
            let concatenated = format!("{}{}", first_digit, last_digit);
            total += concatenated.parse::<i32>().unwrap();
        }
    }

    println!("The total is: {}", total);
}

