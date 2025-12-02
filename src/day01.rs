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

            let distance = number.abs();
            let pos_mod = dial.rem_euclid(100);

            let mut first = match first_char {
                'L' => pos_mod,
                'R' => (100 - pos_mod) % 100,
                _ => panic!("Invalid direction"),
            };
            if first == 0 {
                first = 100;
            }

            if first <= distance {
                let extra = (distance - first) / 100;
                zero_count += 1 + extra;
            }

            dial = match first_char {
                'L' => (dial - number).rem_euclid(100),
                'R' => (dial + number).rem_euclid(100),
                _ => panic!("Invalid direction"),
            };
        }
    }

    println!("Number of times dial points at 0: {}", zero_count);
}

fn main() {
    part_one();
    part_two();
}
