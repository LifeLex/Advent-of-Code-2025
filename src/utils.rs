use std::fs;
use std::io;
/// Reads the contents of a file at the given path and returns it as a String.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the file
///
/// # Returns
///
/// * `Result<String, io::Error>` - The file contents as a String, or an error
///
/// # Examples
///
/// ```
/// let contents = read_file_to_string("input.txt")?;
/// println!("{}", contents);
/// ```
pub fn read_file_to_string(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}
