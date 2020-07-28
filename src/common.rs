/// This submoddule contains useful functions and other constructs that don't
/// sensibly belong to any specific entity in the program.

use std::{str, path, fs, io, convert::TryFrom};
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

// ============
// Type aliases
// ============

/// ### NodeId
/// A type alias for an integer used as a node identifier.
pub type NodeId = u32;

/// ### EnumInt
/// A type alias for different kinds of enumerators such as list or foonote ordinals in integer format.
pub type EnumAsInt = u32;


// ==========================
// Enumerators and converters
// ==========================

/// ### EnumFormat
/// Enumerated list item labels can either end with a period `.` or a right parenthesis `)`.
/// A third option is to enclose them in matching parentheses `(` and `)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnumDelims {
  Period, Parens, RParen,
}

/// ### EnumKind
/// List enumerator labels can be Arabic numerals, lower- or upper-case alphet `a--z` or `A--Z`,
/// or lower- or upper-case Roman numerals between `1--4999`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnumKind {
  Arabic, LowerAlpha, UpperAlpha, LowerRoman, UpperRoman, Automatic,
}


/// ### FootnoteKind
/// There are 4 different kinds of footnote markers:
/// 1. Manually numbered: .. [1] , .. [2], ...
/// 2. automatically numbered: .. [#] 
/// 3. automatically nubered with a simple reference name: .. [#simple_ref-name]
/// 4. Automatically generated symbolic markers: .. [*]
#[derive(Debug, Clone, Copy)]
pub enum FootnoteKind {
  Manual,
  AutoNumbered,
  SimpleRefName,
  AutoSymbol,
}


/// ### IterpretedTextKind
/// There are 3 types of interpreted inline text, such as math:
/// 1. where the given role precedes the interpreted content and
/// 2. where the interpreted content precedes the given role.
/// 3. where  the type is not specified and the default role is used.
#[derive(Debug, Clone, Copy)]
pub enum InterpretedTextKind {
  Default,
  RoleThenContent,
  ContentThenRole
}


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

impl TryFrom<u32> for UpperAlpha {

  type Error = &'static str;

  /// ### try_from
  /// Converts a `u32` to a corresponding `UpperAlpha` numeral variant.
  fn try_from (numeral: u32) -> Result<Self, Self::Error> {
    match numeral {
      1   => Ok(Self::A),
      2   => Ok(Self::B),
      3   => Ok(Self::C),
      4   => Ok(Self::D),
      5   => Ok(Self::E),
      6   => Ok(Self::F),
      7   => Ok(Self::G),
      8   => Ok(Self::H),
      9   => Ok(Self::I),
      10  => Ok(Self::J),
      11  => Ok(Self::K),
      12  => Ok(Self::L),
      13  => Ok(Self::M),
      14  => Ok(Self::N),
      15  => Ok(Self::O),
      16  => Ok(Self::P),
      17  => Ok(Self::Q),
      18  => Ok(Self::R),
      19  => Ok(Self::S),
      20  => Ok(Self::T),
      21  => Ok(Self::U),
      22  => Ok(Self::V),
      23  => Ok(Self::W),
      24  => Ok(Self::X),
      25  => Ok(Self::Y),
      26  => Ok(Self::Z),
      _   => Err("No matching upper-case alphanumeral for a given integer\n")
    }
  }
}

impl TryFrom<&str> for UpperAlpha {

  type Error = &'static str;

  /// ### from
  /// Converts a `&str` to a corresponding `UpperAlpha` numeral variant.
  fn try_from (alpha_str: &str) -> Result<Self, Self::Error> {
    match alpha_str {
      "A" => Ok(Self::A), "B" => Ok(Self::B),
      "C" => Ok(Self::C), "D" => Ok(Self::D),
      "E" => Ok(Self::E), "F" => Ok(Self::F),
      "G" => Ok(Self::G), "H" => Ok(Self::H),
      "I" => Ok(Self::I), "J" => Ok(Self::J),
      "K" => Ok(Self::K), "L" => Ok(Self::L),
      "M" => Ok(Self::M), "N" => Ok(Self::N),
      "O" => Ok(Self::O), "P" => Ok(Self::P),
      "Q" => Ok(Self::Q), "R" => Ok(Self::R),
      "S" => Ok(Self::S), "T" => Ok(Self::T),
      "U" => Ok(Self::U), "V" => Ok(Self::V),
      "W" => Ok(Self::W), "X" => Ok(Self::X),
      "Y" => Ok(Self::Y), "Z" => Ok(Self::Z),
      _ => Err("No matching upper-case alphanumeral for given &str\n")
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


impl TryFrom<u32> for LowerAlpha {

  type Error = &'static str;

  /// ### try_from
  /// Converts a `u32` to a corresponding `LowerAlpha` numeral variant,.
  fn try_from (numeral: u32) -> Result<Self, Self::Error> {
    match numeral {
      1   => Ok(Self::A),
      2   => Ok(Self::B),
      3   => Ok(Self::C),
      4   => Ok(Self::D),
      5   => Ok(Self::E),
      6   => Ok(Self::F),
      7   => Ok(Self::G),
      8   => Ok(Self::H),
      9   => Ok(Self::I),
      10  => Ok(Self::J),
      11  => Ok(Self::K),
      12  => Ok(Self::L),
      13  => Ok(Self::M),
      14  => Ok(Self::N),
      15  => Ok(Self::O),
      16  => Ok(Self::P),
      17  => Ok(Self::Q),
      18  => Ok(Self::R),
      19  => Ok(Self::S),
      20  => Ok(Self::T),
      21  => Ok(Self::U),
      22  => Ok(Self::V),
      23  => Ok(Self::W),
      24  => Ok(Self::X),
      25  => Ok(Self::Y),
      26  => Ok(Self::Z),
      _   => Err("No matching lower-case alphanumeral for a given integer\n")
    }
  }
}

impl TryFrom<&str> for LowerAlpha {

  type Error = &'static str;

  /// ### try_from
  /// Converts a `&str` to a corresponding `UpperAlpha` numeral variant.
  fn try_from (alpha_str: &str) -> Result<Self, Self::Error> {
    match alpha_str {
      "a" => Ok(Self::A), "b" => Ok(Self::B),
      "c" => Ok(Self::C), "d" => Ok(Self::D),
      "e" => Ok(Self::E), "f" => Ok(Self::F),
      "g" => Ok(Self::G), "h" => Ok(Self::H),
      "i" => Ok(Self::I), "j" => Ok(Self::J),
      "k" => Ok(Self::K), "l" => Ok(Self::L),
      "m" => Ok(Self::M), "n" => Ok(Self::N),
      "o" => Ok(Self::O), "p" => Ok(Self::P),
      "q" => Ok(Self::Q), "r" => Ok(Self::R),
      "s" => Ok(Self::S), "t" => Ok(Self::T),
      "u" => Ok(Self::U), "v" => Ok(Self::V),
      "w" => Ok(Self::W), "x" => Ok(Self::X),
      "y" => Ok(Self::Y), "z" => Ok(Self::Z),
      _ => Err("No matching lower-case alphanumeral for given &str\n")
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

impl TryFrom<u32> for UpperRoman {

  type Error = &'static str;

  /// ### try_from
  /// Converts a `u32` to an `UpperRoman` numeral variant.
  fn try_from (numeral: u32) -> Result<Self, Self::Error> {
    match numeral {
      1000  => Ok(Self::M),
      900   => Ok(Self::CM),
      500   => Ok(Self::D),
      400   => Ok(Self::CD),
      100   => Ok(Self::C),
      90    => Ok(Self::XC),
      50    => Ok(Self::L),
      40    => Ok(Self::XL),
      10    => Ok(Self::X),
      9     => Ok(Self::IX),
      5     => Ok(Self::V),
      4     => Ok(Self::IV),
      1     => Ok(Self::I),
      _     => Err("No matching upper-case Roman numeral for a given integer\n")
    }
  }
}

impl TryFrom<&str> for UpperRoman {

  type Error = &'static str;

  /// ### try_from
  /// Tries to convert a `&str` to an `UpperRoman` variant.
  fn try_from (roman_str: &str) -> Result<Self, Self::Error> {
    match roman_str {
      "M"   => Ok(Self::M),
      "CM"  => Ok(Self::CM),
      "D"   => Ok(Self::D),
      "CD"  => Ok(Self::CD),
      "C"   => Ok(Self::C),
      "XC"  => Ok(Self::XC),
      "L"   => Ok(Self::L),
      "XL"  => Ok(Self::XL),
      "X"   => Ok(Self::X),
      "IX"  => Ok(Self::IX),
      "V"   => Ok(Self::V),
      "IV"  => Ok(Self::IV),
      "I"   => Ok(Self::I),
      _     => Err("No matching upper-case roman numeral for given &str\n")
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

impl TryFrom<u32> for LowerRoman {

  type Error = &'static str;

  /// ### try_from
  /// Converts a `u32` to an `UpperRoman` numeral variant.
  fn try_from (numeral: u32) -> Result<Self, Self::Error> {
    match numeral {
      1000  => Ok(Self::M),
      900   => Ok(Self::CM),
      500   => Ok(Self::D),
      400   => Ok(Self::CD),
      100   => Ok(Self::C),
      90    => Ok(Self::XC),
      50    => Ok(Self::L),
      40    => Ok(Self::XL),
      10    => Ok(Self::X),
      9     => Ok(Self::IX),
      5     => Ok(Self::V),
      4     => Ok(Self::IV),
      1     => Ok(Self::I),
      _     => Err("No matching lower-case Roman numeral for a given integer\n")
    }
  }
}

impl TryFrom<&str> for LowerRoman {

  type Error = &'static str;

  /// ### try_from
  /// Tries to convert a `&str` to a `LowerRoman` variant.
  fn try_from (roman_str: &str) -> Result<Self, Self::Error> {
    match roman_str {
      "m"   => Ok(Self::M),
      "cm"  => Ok(Self::CM),
      "d"   => Ok(Self::D),
      "cd"  => Ok(Self::CD),
      "c"   => Ok(Self::C),
      "xc"  => Ok(Self::XC),
      "l"   => Ok(Self::L),
      "xl"  => Ok(Self::XL),
      "x"   => Ok(Self::X),
      "ix"  => Ok(Self::IX),
      "v"   => Ok(Self::V),
      "iv"  => Ok(Self::IV),
      "i"   => Ok(Self::I),
      _     => Err("No matching lower-case roman numeral for given &str\n")
    }
  }
}
