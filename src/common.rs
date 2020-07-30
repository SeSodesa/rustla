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

#[derive(Copy, Clone, Debug)]
/// ### PatternName
/// An enum of transition regex pattern names, both for body and inline level elements.
pub enum PatternName {

  // Body elements, possibly nested
  EmptyLine,
  Bullet,
  Citation,
  Enumerator {delims: EnumDelims, kind: EnumKind},

  FieldMarker,
  Footnote { kind: FootnoteKind },
  OptionMarker,
  DocTest,
  LineBlock,
  ExplicitMarkup,
  AnonymousTarget,
  Line,
  Paragraph,
  Text,

  // Inline Elements for parsing Strings
  Escape,
  StrongEmphasis, // **strongly emphasised text**
  Emphasis, // *emphasized text*
  Interpreted { kind: InterpretedTextKind }, // Plain interpreted text with the default role set by transpiler.
  PhraseRef, // A reference in the form `text with spaces`__?
  SimpleRef, // A reference that doesn't need backticks: reference__?
  Literal, // Code
  FootNoteRef,
  InlineTarget, // Reference target in inline text: _`target label`
  SubstitutionRef, // Reference to substitution definition. Is replaced by the definition
  ImplicitURL,
  StandaloneHyperlink,
  WhiteSpace,
}


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

// ===========
//  Constants
// ===========

/// #### FOOTNOTE_SYMBOLS
/// The standard Docutils system uses these symbols as footnote marks
/// when a FootnoteKind::AutoSymbol is detected.
/// They are, from first to last:
/// 
/// 1.  asterisk/star (`*`)
/// 2.  dagger (`†`|`U+02020`)
/// 3.  double dagger (`‡`|`U+02021`)
/// 4.  section mark (`§`|`U+000A7`)
/// 5.  pilcrow or paragraph mark (`¶`|`U+000B6`)
/// 6.  number sign (`#`)
/// 7.  spade suit (`♠`|`U+02660`)
/// 8.  heart suit (`♥`|`U+02665`)
/// 9.  diamond suit (`♦`|`U+02666`)
/// 10. club suit (`♣`|`U+02663`)
/// 
/// As the next autosymbol is detected the next unused item
/// from this list will be used as the footnote label character.
/// If `n` is the number of times this list has been iteratred over
/// and `s` the current autosymbol, then the actual label
/// of the footnote is `s^(n+1)`. For example, if a document has
/// `12` automatically symboled footnotes and a new one is constructed,
/// then its label will be `‡‡ = ‡² = ‡¹⁺¹`.
pub const FOOTNOTE_SYMBOLS: [char; 10] = [
  '*', '†','‡','§','¶','#','♠','♥','♦','♣'
];
