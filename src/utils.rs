/// This submoddule contains useful functions and other constructs that don't
/// sensibly belong to any specific entity in the program.

use std::{str, path, fs, io};
use std::io::BufRead;

// =======================
// Text handling utilities
// =======================

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


/// ### strip_indent
/// A whitespace-aware function for stripping indentation
/// from `String`s. Returns `Ok(String)` if successful.
/// If non-whitespace characters are encountered before
/// the notified `amount` has been stripped, an `Err(message)`
/// is returned instead.
pub fn strip_indent(line: String, amount: usize) -> Result<String, &'static str> {

  if line.is_empty() {
    return Ok(line)
  }

  let mut chars = line.chars();

  for i in 0..amount {

    let c = chars.next().unwrap();

    if !c.is_whitespace() && i < amount {
      return Err("\nNon-whitespace character encountered before supposed indentation level reached.\n");
    }

  }

  Ok(chars.as_str().to_string())

}

// ==========================
// Enumerators and converters
// ==========================

pub enum EnumeratorFormat {
  Arabic(u64),            // 1, 2, 3, 4, 5, ...
  LowerAlpha(LowerAlpha), // a, b, c, d, e, ...
  UpperAlpha(UpperAlpha), // A, B, C, D, E, ...
  LowerRoman(LowerRoman), // i, ii, iii, iv, v, ...
  UpperRoman(UpperRoman), // I, II, III, IV, V, ...
  Automatic,              // symbol #
}



/// ### UpperAlpha
/// Upper-case alphabetic numerals.
pub enum UpperAlpha {
  A, B, C, D, E, F, G,
  H, I, J, K, L, M, N,
  O, P, Q, R, S, T, U,
  V, W, X, Y, Z,
}

impl From<UpperAlpha> for u32 {
  fn from (numeral: UpperAlpha) -> Self {
    match numeral {
      UpperAlpha::A => 1,
      UpperAlpha::B => 2,
      UpperAlpha::C => 3,
      UpperAlpha::D => 4,
      UpperAlpha::E => 5,
      UpperAlpha::F => 6,
      UpperAlpha::G => 7,
      UpperAlpha::H => 8,
      UpperAlpha::I => 9,
      UpperAlpha::J => 10,
      UpperAlpha::K => 11,
      UpperAlpha::L => 12,
      UpperAlpha::M => 13,
      UpperAlpha::N => 14,
      UpperAlpha::O => 15,
      UpperAlpha::P => 16,
      UpperAlpha::Q => 17,
      UpperAlpha::R => 18,
      UpperAlpha::S => 19,
      UpperAlpha::T => 20,
      UpperAlpha::U => 21,
      UpperAlpha::V => 22,
      UpperAlpha::W => 23,
      UpperAlpha::X => 24,
      UpperAlpha::Y => 25,
      UpperAlpha::Z => 26,
    }
  }
}


/// ### LowerAlpha
/// Lower-case alphabetic numerals.
pub enum LowerAlpha {
  A, B, C, D, E, F, G,
  H, I, J, K, L, M, N,
  O, P, Q, R, S, T, U,
  V, W, X, Y, Z,
}

impl From<LowerAlpha> for u32 {
  fn from (numeral: LowerAlpha) -> Self {
    match numeral {
      LowerAlpha::A => 1,
      LowerAlpha::B => 2,
      LowerAlpha::C => 3,
      LowerAlpha::D => 4,
      LowerAlpha::E => 5,
      LowerAlpha::F => 6,
      LowerAlpha::G => 7,
      LowerAlpha::H => 8,
      LowerAlpha::I => 9,
      LowerAlpha::J => 10,
      LowerAlpha::K => 11,
      LowerAlpha::L => 12,
      LowerAlpha::M => 13,
      LowerAlpha::N => 14,
      LowerAlpha::O => 15,
      LowerAlpha::P => 16,
      LowerAlpha::Q => 17,
      LowerAlpha::R => 18,
      LowerAlpha::S => 19,
      LowerAlpha::T => 20,
      LowerAlpha::U => 21,
      LowerAlpha::V => 22,
      LowerAlpha::W => 23,
      LowerAlpha::X => 24,
      LowerAlpha::Y => 25,
      LowerAlpha::Z => 26,
    }
  }
}


/// ### UpperRoman
/// Upper-case Roman numerals.
pub enum UpperRoman {
  M, CM, D, CD, C, XC,
  L, XL, X, IX, V, IV,
  I,
}

impl From<UpperRoman> for u32 {
  fn from (numeral: UpperRoman) -> Self {
    match numeral {
      UpperRoman::M   => 1000,
      UpperRoman::CM  => 900,
      UpperRoman::D   => 500,
      UpperRoman::CD  => 400,
      UpperRoman::C   => 100,
      UpperRoman::XC  => 90,
      UpperRoman::L   => 50,
      UpperRoman::XL  => 40,
      UpperRoman::X   => 10,
      UpperRoman::IX  => 9,
      UpperRoman::V   => 5,
      UpperRoman::IV  => 4,
      UpperRoman::I   => 1,
    }
  }
}


/// ### LowerRoman
/// Lower-case Roman numerals.
pub enum LowerRoman {
  M, CM, D, CD, C, XC,
  L, XL, X, IX, V, IV,
  I,
}

impl From<LowerRoman> for u32 {
  fn from (numeral: LowerRoman) -> Self {
    match numeral {
      LowerRoman::M   => 1000,
      LowerRoman::CM  => 900,
      LowerRoman::D   => 500,
      LowerRoman::CD  => 400,
      LowerRoman::C   => 100,
      LowerRoman::XC  => 90,
      LowerRoman::L   => 50,
      LowerRoman::XL  => 40,
      LowerRoman::X   => 10,
      LowerRoman::IX  => 9,
      LowerRoman::V   => 5,
      LowerRoman::IV  => 4,
      LowerRoman::I   => 1,
    }
  }
}
