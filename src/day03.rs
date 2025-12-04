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

fn part_two() {
    let input = read_file_to_string("inputs/day03.txt").unwrap();
    let input_lines = input.lines();
    let mut joltage = 0;
    for line in input_lines {
        let numbers: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
        println!("Numbers: {:?}", numbers);

        let target_length = 12;
        let n = numbers.len();

        // Greedy algorithm: build a 12-digit number by selecting the largest digit
        // at each position that still allows us to complete 12 digits
        let mut result = String::new();
        let mut start_idx = 0;

        for position in 0..target_length {
            let remaining_needed = target_length - position - 1;
            let search_end = n - remaining_needed;

            // Find the maximum digit in the valid range
            let mut max_digit = 0;
            let mut max_idx = start_idx;

            for i in start_idx..search_end {
                if numbers[i] > max_digit {
                    max_digit = numbers[i];
                    max_idx = i;
                }
            }

            result.push_str(&max_digit.to_string());
            start_idx = max_idx + 1;
        }

        println!("Result: {}", result);
        joltage += result.parse::<u64>().unwrap();
    }
    println!("Joltage: {}", joltage);
}

fn main() {
    part_one();
    part_two();
}
