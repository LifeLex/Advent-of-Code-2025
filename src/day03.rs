mod utils;

use utils::read_file_to_string;

fn main() {
    let input = read_file_to_string("input.txt").unwrap();
    println!("Input: {}", input);
}
