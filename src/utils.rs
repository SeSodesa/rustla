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

/// ### EnumeratorFormat
/// An enumeration of the different ways of representing
/// natural numbers in *reStructuredText* enumerated lists.
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
  V, W, X, Y, Z, None,
}

impl From<u32> for UpperAlpha {

  /// ### from
  /// Converts a `u32` to a corresponding `UpperAlpha` numeral variant.
  fn from (numeral: u32) -> Self {
    match numeral {
      1   => Self::A,
      2   => Self::B,
      3   => Self::C,
      4   => Self::D,
      5   => Self::E,
      6   => Self::F,
      7   => Self::G,
      8   => Self::H,
      9   => Self::I,
      10  => Self::J,
      11  => Self::K,
      12  => Self::L,
      13  => Self::M,
      14  => Self::N,
      15  => Self::O,
      16  => Self::P,
      17  => Self::Q,
      18  => Self::R,
      19  => Self::S,
      20  => Self::T,
      21  => Self::U,
      22  => Self::V,
      23  => Self::W,
      24  => Self::X,
      25  => Self::Y,
      26  => Self::Z,
      _   => Self::None
    }
  }
}

impl From<&str> for UpperAlpha {

  /// ### from
  /// Converts a `&str` to a corresponding `UpperAlpha` numeral variant.
  fn from (alpha_str: &str) -> Self {
    match alpha_str {
      "A" => Self::A, "B" => Self::B,
      "C" => Self::C, "D" => Self::D,
      "E" => Self::E, "F" => Self::F,
      "G" => Self::G, "H" => Self::H,
      "I" => Self::I, "J" => Self::J,
      "K" => Self::K, "L" => Self::L,
      "M" => Self::M, "N" => Self::N,
      "O" => Self::O, "P" => Self::P,
      "Q" => Self::Q, "R" => Self::R,
      "S" => Self::S, "T" => Self::T,
      "U" => Self::U, "V" => Self::V,
      "W" => Self::W, "X" => Self::X,
      "Y" => Self::Y, "Z" => Self::Z,
      _ => Self::None
    }
  }
}


/// ### LowerAlpha
/// Lower-case alphabetic numerals.
pub enum LowerAlpha {
  A, B, C, D, E, F, G,
  H, I, J, K, L, M, N,
  O, P, Q, R, S, T, U,
  V, W, X, Y, Z, None
}


impl From<u32> for LowerAlpha {
  /// ### from
  /// Converts a `u32` to a corresponding `LowerAlpha` numeral variant.
  fn from (numeral: u32) -> Self {
    match numeral {
      1   => Self::A,
      2   => Self::B,
      3   => Self::C,
      4   => Self::D,
      5   => Self::E,
      6   => Self::F,
      7   => Self::G,
      8   => Self::H,
      9   => Self::I,
      10  => Self::J,
      11  => Self::K,
      12  => Self::L,
      13  => Self::M,
      14  => Self::N,
      15  => Self::O,
      16  => Self::P,
      17  => Self::Q,
      18  => Self::R,
      19  => Self::S,
      20  => Self::T,
      21  => Self::U,
      22  => Self::V,
      23  => Self::W,
      24  => Self::X,
      25  => Self::Y,
      26  => Self::Z,
      _   => Self::None
    }
  }
}

impl From<&str> for LowerAlpha {

  /// ### from
  /// Converts a `&str` to a corresponding `LowerAlpha` numeral variant.
  fn from (alpha_str: &str) -> Self {
    match alpha_str {
      "a" => Self::A, "b" => Self::B,
      "c" => Self::C, "d" => Self::D,
      "e" => Self::E, "f" => Self::F,
      "g" => Self::G, "h" => Self::H,
      "i" => Self::I, "j" => Self::J,
      "k" => Self::K, "l" => Self::L,
      "m" => Self::M, "n" => Self::N,
      "o" => Self::O, "p" => Self::P,
      "q" => Self::Q, "r" => Self::R,
      "s" => Self::S, "t" => Self::T,
      "u" => Self::U, "v" => Self::V,
      "w" => Self::W, "x" => Self::X,
      "y" => Self::Y, "z" => Self::Z,
      _ => Self::None
    }
  }
}


/// ### UpperRoman
/// Upper-case Roman numerals.
pub enum UpperRoman {
  M, CM, D, CD, C, XC,
  L, XL, X, IX, V, IV,
  I, None
}

impl From<u32> for UpperRoman {

  /// ### from
  /// Converts a `u32` to an `UpperRoman` numeral variant.
  fn from (numeral: u32) -> Self {
    match numeral {
      1000  => Self::M,
      900   => Self::CM,
      500   => Self::D,
      400   => Self::CD,
      100   => Self::C,
      90    => Self::XC,
      50    => Self::L,
      40    => Self::XL,
      10    => Self::X,
      9     => Self::IX,
      5     => Self::V,
      4     => Self::IV,
      1     => Self::I,
      _     => Self::None
    }
  }
}

impl From<&str> for UpperRoman {
  fn from (roman_str: &str) -> Self {
    match roman_str {
      "M" => Self::M,
      "CM" => Self::CM,
      "D" => Self::D,
      "CD" => Self::CD,
      "C" => Self::C,
      "XC" => Self::XC,
      "L" => Self::L,
      "XL" => Self::XL,
      "X" => Self::X,
      "IX" => Self::IX,
      "V" => Self::V,
      "IV" => Self::IV,
      "I" => Self::I,
      _ => Self::None,
    }
  }
}


/// ### LowerRoman
/// Lower-case Roman numerals.
pub enum LowerRoman {
  M, CM, D, CD, C, XC,
  L, XL, X, IX, V, IV,
  I, None
}

impl From<u32> for LowerRoman {

  /// ### from
  /// Converts a `u32` to an `LowerRoman` numeral variant.
  fn from (numeral: u32) -> Self {
    match numeral {
      1000  => Self::M,
      900   => Self::CM,
      500   => Self::D,
      400   => Self::CD,
      100   => Self::C,
      90    => Self::XC,
      50    => Self::L,
      40    => Self::XL,
      10    => Self::X,
      9     => Self::IX,
      5     => Self::V,
      4     => Self::IV,
      1     => Self::I,
      _     => Self::None
    }
  }
}

impl From<&str> for LowerRoman {

  /// ### from
  /// Converts a `&str` to a corresponding `LowerRoman` numeral variant.
  fn from (roman_str: &str) -> Self {
    match roman_str {
      "m" => Self::M,
      "cm" => Self::CM,
      "d" => Self::D,
      "cd" => Self::CD,
      "c" => Self::C,
      "xc" => Self::XC,
      "l" => Self::L,
      "xl" => Self::XL,
      "x" => Self::X,
      "ix" => Self::IX,
      "v" => Self::V,
      "iv" => Self::IV,
      "i" => Self::I,
      _ => Self::None,
    }
  }
}
