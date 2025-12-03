mod utils;

use utils::read_file_to_string;

fn part_one() {
    let input = read_file_to_string("inputs/day03.txt").unwrap();
    let input_lines = input.lines();
    let mut joltage = 0;
    for line in input_lines {
        let numbers: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        println!("Numbers: {:?}", numbers.iter().collect::<Vec<&u32>>());

        // Find the maximum value
        let max_val = *numbers.iter().max().unwrap();

        // Find first occurrence of max
        let first_max_idx = numbers.iter().position(|&n| n == max_val).unwrap();
        println!("Max value {} and index {}", max_val, first_max_idx);
        if first_max_idx == numbers.len() - 1 {
            let previous_max = numbers.iter().rev().skip(1).max().unwrap();
            println!("Max value when max is at the end: {}", previous_max);
            let result = format!("{}{}", previous_max.to_string(), max_val.to_string());
            println!("Result from max at end: {}", result);
            joltage += result.parse::<u32>().unwrap();
        } else {
            // Max is not at the end, find second largest distinct digit
            println!("Max value when max is not at the end: {}", max_val);
            println!(
                "Numbers from max to end: {:?}",
                &numbers[first_max_idx + 1..]
            );
            let second_max = numbers[first_max_idx + 1..].iter().max().unwrap();
            println!(
                "Second max value when max is not at the end: {}",
                second_max
            );
            // Calculate the result based on the second largest distinct digit
            let result = format!("{}{}", max_val, second_max);
            println!("Result from max not at end: {}", result);
            joltage += result.parse::<u32>().unwrap();
        }
    }
    println!("Result: {}", joltage);
}

fn part_two() {}

fn main() {
    part_one();
}
