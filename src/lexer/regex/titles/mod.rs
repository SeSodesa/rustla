/// A regex submodule that contains title
/// related regular expressions,

mod test;

use regex::Regex;
use lazy_static::lazy_static;

// Patterns as raw strings
// =======================

/// Title over-|underline with equals sign
const TITLE_LINE_EQUALS: &'static str
  = r#"={3,}"#;

/// Title over-|underline with hyphen
const TITLE_LINE_DASH: &'static str
  = r#"-{3,}"#;

/// Title over-|underline with backtick
const TITLE_LINE_BACKTICK: &'static str
  = r#"`{3,}"#;

/// Title over-|underline with colon
const TITLE_LINE_COLON: &'static str
  = r#":{3,}"#;

/// Title over-|underline with double quote
const TITLE_LINE_SQUOTE: &'static str
  = r#"'{3,}"#;

/// Title over-|underline with double quote
const TITLE_LINE_DQUOTE: &'static str
  = r#""{3,}"#;

/// Title over-|underline with tilde
const TITLE_LINE_TILDE: &'static str
  = r#"\~{3,}"#;

/// Title over-|underline with caret
const TITLE_LINE_CARET: &'static str
= r#"\^{3,}"#;

/// Title over-|underline with underscore
const TITLE_LINE_UNDERSCORE: &'static str
= r#"_{3,}"#;

/// Title over-|underline with asterisk
const TITLE_LINE_ASTERISK: &'static str
= r#"\*{3,}"#;

/// Title over-|underline with plus sign
const TITLE_LINE_PLUS: &'static str
= r#"\+{3,}"#;

/// Title over-|underline with hash sign
const TITLE_LINE_HASH: &'static str
= r##"#{3,}"##;

/// Title over-|underline with < sign
const TITLE_LINE_LESS: &'static str
= r#"<{3,}"#;

/// Title over-|underline with > sign
const TITLE_LINE_MORE: &'static str
= r#">{3,}"#;


const ANY_TITLE_CHARS: &'static str
  = r#".+"#;


  
// Static Regexes 
// ==============

lazy_static! {

  // Overlined titles
  // ----------------

  /// Regex representing overlined title with an equals sign
  static ref OVERLINED_TITLE_EQUALS: Regex
    = Regex::new(
      format!(
        r"(?m)^{}\n{}[ \t]*\n{}\n",
        TITLE_LINE_EQUALS, ANY_TITLE_CHARS, TITLE_LINE_EQUALS
      ).as_str()
    ).unwrap();
 
  /// Regex representing overlined title with a dash
  static ref OVERLINED_TITLE_DASH: Regex
  = Regex::new(
    format!(
      r"(?m)^{}\n{}[ \t]*\n{}\n",
      TITLE_LINE_DASH, ANY_TITLE_CHARS, TITLE_LINE_DASH
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a backtick
  static ref OVERLINED_TITLE_BACKTICK: Regex
  = Regex::new(
    format!(
      r"(?m)^{}\n{}[ \t]*\n{}\n",
      TITLE_LINE_BACKTICK, ANY_TITLE_CHARS, TITLE_LINE_BACKTICK
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a colon
  static ref OVERLINED_TITLE_COLON: Regex
  = Regex::new(
    format!(
      r"(?m)^{}\n{}[ \t]*\n{}\n",
      TITLE_LINE_COLON, ANY_TITLE_CHARS, TITLE_LINE_COLON
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a single quote
  static ref OVERLINED_TITLE_SQUOTE: Regex
  = Regex::new(
    format!(
      r"(?m)^{}\n{}[ \t]*\n{}\n",
      TITLE_LINE_SQUOTE, ANY_TITLE_CHARS, TITLE_LINE_SQUOTE
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a double quote
  static ref OVERLINED_TITLE_DQUOTE: Regex
  = Regex::new(
    format!(
      r"(?m)^{}\n{}[ \t]*\n{}\n",
      TITLE_LINE_DQUOTE, ANY_TITLE_CHARS, TITLE_LINE_DQUOTE
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a double quote
  static ref OVERLINED_TITLE_TILDE: Regex
  = Regex::new(
    format!(
      r"(?m)^{}\n{}[ \t]*\n{}\n",
      TITLE_LINE_TILDE, ANY_TITLE_CHARS, TITLE_LINE_TILDE
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a double quote
  static ref OVERLINED_TITLE_CARET: Regex
  = Regex::new(
    format!(
      r"(?m)^{}\n{}[ \t]*\n{}\n",
      TITLE_LINE_CARET, ANY_TITLE_CHARS, TITLE_LINE_CARET
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a double quote
  static ref OVERLINED_TITLE_UNDERSCORE: Regex
  = Regex::new(
    format!(
      r"(?m)^{}\n{}[ \t]*\n{}\n",
      TITLE_LINE_UNDERSCORE, ANY_TITLE_CHARS, TITLE_LINE_UNDERSCORE
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a double quote
  static ref OVERLINED_TITLE_ASTERISK: Regex
  = Regex::new(
    format!(
      r"(?m)^{}\n{}[ \t]*\n{}\n",
      TITLE_LINE_ASTERISK, ANY_TITLE_CHARS, TITLE_LINE_ASTERISK
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a double quote
  static ref OVERLINED_TITLE_PLUS: Regex
  = Regex::new(
    format!(
      r"(?m)^{}\n{}[ \t]*\n{}\n",
      TITLE_LINE_PLUS, ANY_TITLE_CHARS, TITLE_LINE_PLUS
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a double quote
  static ref OVERLINED_TITLE_HASH: Regex
  = Regex::new(
    format!(
      r"(?m)^{}\n{}[ \t]*\n{}\n",
      TITLE_LINE_HASH, ANY_TITLE_CHARS, TITLE_LINE_HASH
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a double quote
  static ref OVERLINED_TITLE_LESS: Regex
  = Regex::new(
    format!(
      r"(?m)^{}\n{}[ \t]*\n{}\n",
      TITLE_LINE_LESS, ANY_TITLE_CHARS, TITLE_LINE_LESS
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a double quote
  static ref OVERLINED_TITLE_MORE: Regex
  = Regex::new(
    format!(
      r"(?m)^{}\n{}[ \t]*\n{}\n",
      TITLE_LINE_MORE, ANY_TITLE_CHARS, TITLE_LINE_MORE
    ).as_str()
  ).unwrap();


  // Underlined titles
  // ----------------

    /// Regex representing overlined title with an equals sign
    static ref TITLE_EQUALS: Regex
    = Regex::new(
      format!(
        r"(?m)^{}[ \t]*\n{}\n",
        ANY_TITLE_CHARS, TITLE_LINE_EQUALS
      ).as_str()
    ).unwrap();
 
  /// Regex representing overlined title with a dash
  static ref TITLE_DASH: Regex
  = Regex::new(
    format!(
      r"(?m)^{}[ \t]*\n{}\n",
      ANY_TITLE_CHARS, TITLE_LINE_DASH
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a backtick
  static ref TITLE_BACKTICK: Regex
  = Regex::new(
    format!(
      r"(?m)^{}[ \t]*\n{}\n",
      ANY_TITLE_CHARS, TITLE_LINE_BACKTICK
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a colon
  static ref TITLE_COLON: Regex
  = Regex::new(
    format!(
      r"(?m)^{}[ \t]*\n{}\n",
      ANY_TITLE_CHARS, TITLE_LINE_COLON
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a single quote
  static ref TITLE_SQUOTE: Regex
  = Regex::new(
    format!(
      r"(?m)^{}[ \t]*\n{}\n",
      ANY_TITLE_CHARS, TITLE_LINE_SQUOTE
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a double quote
  static ref TITLE_DQUOTE: Regex
  = Regex::new(
    format!(
      r"(?m)^{}[ \t]*\n{}\n",
      ANY_TITLE_CHARS, TITLE_LINE_DQUOTE
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a double quote
  static ref TITLE_TILDE: Regex
  = Regex::new(
    format!(
      r"(?m)^{}[ \t]*\n{}\n",
      ANY_TITLE_CHARS, TITLE_LINE_TILDE
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a double quote
  static ref TITLE_CARET: Regex
  = Regex::new(
    format!(
      r"(?m)^{}[ \t]*\n{}\n",
      ANY_TITLE_CHARS, TITLE_LINE_CARET
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a double quote
  static ref TITLE_UNDERSCORE: Regex
  = Regex::new(
    format!(
      r"(?m)^{}[ \t]*\n{}\n",
      ANY_TITLE_CHARS, TITLE_LINE_UNDERSCORE
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a double quote
  static ref TITLE_ASTERISK: Regex
  = Regex::new(
    format!(
      r"(?m)^{}[ \t]*\n{}\n",
      ANY_TITLE_CHARS, TITLE_LINE_ASTERISK
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a double quote
  static ref TITLE_PLUS: Regex
  = Regex::new(
    format!(
      r"(?m)^{}[ \t]*\n{}\n",
      ANY_TITLE_CHARS, TITLE_LINE_PLUS
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a double quote
  static ref TITLE_HASH: Regex
  = Regex::new(
    format!(
      r"(?m)^{}[ \t]*\n{}\n",
      ANY_TITLE_CHARS, TITLE_LINE_HASH
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a double quote
  static ref TITLE_LESS: Regex
  = Regex::new(
    format!(
      r"(?m)^{}[ \t]*\n{}\n",
      ANY_TITLE_CHARS, TITLE_LINE_LESS
    ).as_str()
  ).unwrap();

  /// Regex representing overlined title with a double quote
  static ref TITLE_MORE: Regex
  = Regex::new(
    format!(
      r"(?m)^{}[ \t]*\n{}\n",
      ANY_TITLE_CHARS, TITLE_LINE_MORE
    ).as_str()
  ).unwrap();

}