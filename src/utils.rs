/// This submoddule contains useful functions that don't
/// belong to any specific entity in the program

use std::{str, path, fs, io};
use std::io::BufRead;

/// ### str_to_lines
/// Returns a `Vec<String>` from a given `&str`,
/// split at new lines `\n` or `\r\n`.
pub fn str_to_lines(string: &str) -> Vec<String> {

  let line_vec = string
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

  line_vec

}


/// ### read_path_lines
/// Read the lines of a given file into a buffer.
pub fn read_path_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where P: AsRef<path::Path> {
  let file:fs::File = fs::File::open(file_path)?;
  Ok(io::BufReader::new(file).lines())
}
