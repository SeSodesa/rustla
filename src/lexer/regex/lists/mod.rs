/// This file contains regexes for reStructuredText
/// lists. It is a submodule of lexer::regex.

mod test;

use regex::Regex;
use lazy_static::lazy_static;

// Constant raw string expressions
// ===============================

/// Unnumbered list item
const UNNUMBERED_LIST: &'static str
  = r"\s*[*-+] .+\n(?:[*.+]  .+)*";

/// Numbered list item with a dot
const NUMBERED_LIST_DOT: &'static str
  = r"\s*[0-9#ivxlcmIVXLCM]+\. .+\n(?:[0-9#ivxlcmIVXLCM]+\. .+)*";

/// Numbered list with parentheses around labels
const NUMBERED_LIST_LRPAREN: &'static str
  = r"\s*\(?[0-9#ivxlcmIVXLCM]+\) .+\n(?:\([0-9#ivxlcmIVXLCM]+\) .+)*";

/// Numbered list with parentheses on
/// the right side of labels
const NUMBERED_LIST_RPAREN: &'static str
  = r"\s*[0-9#ivxlcmIVXLCM]+\) .+\n(?:[0-9#ivxlcmIVXLCM]+\) .+)*";

/// Numbered list with parentheses on
/// the right side of labels
const ALPHA_LIST_DOT: &'static str
  = r"\s*[A-Z]+\. .+\n(?:[A-Z]+\. .+\n)+";


/// Numbered list with parentheses on
/// the right side of labels
const ALPHA_LIST_LRPAREN: &'static str
  = r"\s*\(?[a-zA-Z]+\) .+\n(?:\([a-zA-Z]+\) .+\n)+";


/// Numbered list with parentheses on
/// the right side of labels
const ALPHA_LIST_RPAREN: &'static str
  = r"\s*[a-zA-Z]+\) .+\n(?:[a-zA-Z]+\) .+\n)+";




// Regexes describing lists
// ========================

lazy_static! {

  /// Regex representing an unnumbered list
  static ref UNNUMBERED_LIST_RE: Regex
    = Regex::new(
      format!(
        r"(?m)^{}", UNNUMBERED_LIST
      ).as_str()
    ).unwrap();

  /// Regex representing a numbered list
  /// with dots on the right sides of labels
  static ref NUMBERED_LIST_DOT_RE: Regex
    = Regex::new(
      format!(
        r"(?m)^{}", NUMBERED_LIST_DOT
      ).as_str()
    ).unwrap();

/// Regex representing a numbered list
  /// with dots on the right sides of labels
  static ref NUMBERED_LIST_LRPAREN_RE: Regex
    = Regex::new(
      format!(
        r"(?m)^{}", NUMBERED_LIST_LRPAREN
      ).as_str()
    ).unwrap();

/// Regex representing a numbered list
  /// with dots on the right sides of labels
  static ref NUMBERED_LIST_RPAREN_RE: Regex
    = Regex::new(
      format!(
        r"(?m)^{}", NUMBERED_LIST_RPAREN
      ).as_str()
    ).unwrap();

  /// Regex representing a numbered list
  /// with dots on the right sides of labels
  static ref ALPHA_LIST_DOT_RE: Regex
    = Regex::new(
      format!(
        r"(?m)^{}", ALPHA_LIST_DOT
      ).as_str()
    ).unwrap();

/// Regex representing a numbered list
  /// with dots on the right sides of labels
  static ref ALPHA_LIST_LRPAREN_RE: Regex
    = Regex::new(
      format!(
        r"(?m)^{}", ALPHA_LIST_LRPAREN
      ).as_str()
    ).unwrap();

/// Regex representing a numbered list
  /// with dots on the right sides of labels
  static ref ALPHA_LIST_RPAREN_RE: Regex
    = Regex::new(
      format!(
        r"(?m)^{}", ALPHA_LIST_RPAREN
      ).as_str()
    ).unwrap();

}