use std::fs;

const DIAL_START: i32 = 50;

fn part_one() {
    let input_path = "inputs/day01.txt";
    let input = fs::read_to_string(input_path).expect("Failed to read input file");
    let input_results_vec = input.lines();

    let mut dial = DIAL_START;
    let mut number_of_zeros = 0;

    for result in input_results_vec {
        if let Some(first_char) = result.chars().next() {
            let number: i32 = result[1..].parse().unwrap();
            match first_char {
                'L' => dial -= number,
                'R' => dial += number,
                _ => panic!("Invalid direction"),
            }
            dial = dial.rem_euclid(100);
            if dial == 0 {
                number_of_zeros += 1;
            }
        }
    }

    println!("Number of Zeros {}", number_of_zeros);
}

fn part_two() {
    let input_path = "inputs/day01.txt";
    let input = fs::read_to_string(input_path).expect("Failed to read input file");
    let input_results_vec = input.lines();
    let mut dial = DIAL_START;
    let mut zero_count = 0;

    for result in input_results_vec {
        if let Some(first_char) = result.chars().next() {
            let number: i32 = result[1..].parse().unwrap();

            let start = dial;

            // How many complete 100s do we traverse?
            let full_cycles = number / 100;
            let remainder = number % 100;

            // Does the remainder cross 0?
            let crosses_zero = if first_char == 'R' {
                (start + remainder) >= 100
            } else {
                start < remainder
            };

            zero_count += full_cycles + if crosses_zero { 1 } else { 0 };

            match first_char {
                'L' => dial -= number,
                'R' => dial += number,
                _ => panic!("Invalid direction"),
            }

            dial = dial.rem_euclid(100);
        }
    }

    println!("Number of times dial points at 0: {}", zero_count);
}

fn main() {
    part_one();
    part_two();
}
