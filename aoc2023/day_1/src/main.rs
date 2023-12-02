use std::fs;

fn main() {
    // Adjust the file path as necessary for your setup
    let contents = fs::read_to_string("/home/eddieberman/aoc2023/day_1/src/input.txt")
        .expect("Something went wrong reading the file");

    // Split the file contents into lines and collect them into a Vec<String>
    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

    let total = calculate_total(&lines);
    println!("The total is: {}", total);
}

fn calculate_total(lines: &[String]) -> i32 {
    let nums = vec![
        "zero", "0", "one", "1", "two", "2", "three", "3", "four", "4", 
        "five", "5", "six", "6", "seven", "7", "eight", "8", "nine", "9",
    ];
    let mut total = 0;

    for line in lines {
        let mut first = None;
        'out: for i in 0..line.len() {
            for (index, num) in nums.iter().enumerate() {
                if i + num.len() > line.len() {
                    continue;
                }
                if &line[i..i + num.len()] == *num {
                    first = Some(1 + index / 2);
                    break 'out;
                }
            }
        }
        let first = first.expect("invalid input");

        let mut last = None;
        'out: for i in (0..line.len()).rev() {
            for (index, num) in nums.iter().enumerate() {
                if i + num.len() > line.len() {
                    continue;
                }
                if &line[i..i + num.len()] == *num {
                    last = Some(1 + index / 2);
                    break 'out;
                }
            }
        }
        let last = last.expect("invalid input");

        total += 10 * first as i32 + last as i32;
    }

    total
}

