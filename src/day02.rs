use std::fs;

fn part_one() {
    let input_path = "inputs/day02.txt";
    let input = fs::read_to_string(input_path).expect("Failed to read input file");
    let input_data = input.split(',');
    let mut result: u64 = 0;
    let mut count = 0;
    for number in input_data {
        let number_range = number
            .split("-")
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        for num in number_range[0]..=number_range[1] {
            let num_str = num.to_string();
            let len = num_str.len();
            if len % 2 == 0 {
                let (first_half, second_half) = num_str.split_at(len / 2);
                if first_half == second_half {
                    count += 1;
                    result += num;
                    println!(
                        "{} has equal halves: {} == {}",
                        num, first_half, second_half
                    );

                    println!("Result at iteration: {}", result);
                }
            }
        }
    }
    println!("Total count: {}", count);
    println!("Result {}", result);
}

fn part_two() {
    let input_path = "inputs/day02.txt";
    let input = fs::read_to_string(input_path).expect("Failed to read input file");
    let input_data = input.split(',');
    let mut result: u64 = 0;
    let mut count = 0;
    for number in input_data {
        let number_range = number
            .split("-")
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        for num in number_range[0]..=number_range[1] {
            let num_str = num.to_string();
            let len = num_str.len();
            let mut found = false;

            for part_len in 1..=(len / 2) {
                if len % part_len == 0 {
                    let num_parts = len / part_len;
                    let first_part = &num_str[0..part_len];
                    let mut all_equal = true;

                    for i in 1..num_parts {
                        let start = i * part_len;
                        let end = start + part_len;
                        if &num_str[start..end] != first_part {
                            all_equal = false;
                            break;
                        }
                    }

                    if all_equal {
                        if !found {
                            count += 1;
                            result += num;
                            found = true;
                        }
                        println!(
                            "{} can be divided into {} parts of '{}' ({} parts)",
                            num, num_parts, first_part, num_parts
                        );
                    }
                }
            }

            if found {
                println!("Result at iteration: {}", result);
            }
        }
    }
    println!("Total count: {}", count);
    println!("Result {}", result);
}

fn main() {
    // part_one();
    part_two();
}
