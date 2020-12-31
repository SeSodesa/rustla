/*!
In this submodule the patterns in `crate::parser::regex_patterns` are lazily compiled into finite automata
using the standard Rust `regex` crate.

Copyright © 2020 Santtu Söderholm
*/
use crate::parser::regex_patterns;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {

  /// A DFA for recognising block quote attributions.
  pub static ref ATTRIBUTION_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::ATTRIBUTION_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize ATTRIBUTION automaton. Computer says no...")
  };

  /// A DFA for recognising blank lines.
  pub static ref BLANK_LINE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::BLANK_LINE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize BLANK_LINE automaton. Computer says no...")
  };

  /// A DFA for recognising blank lines.
  pub static ref BULLET_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::BULLET_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize BULLET automaton. Computer says no...")
  };

  /// A DFA for recognising enumerators.
  pub static ref ENUMERATOR_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::ENUMERATOR_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize ENUMERATOR automaton. Computer says no...")
  };

  /// A DFA for recognising field markers.
  pub static ref FIELD_MARKER_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::FIELD_MARKER_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize FIELD_MARKER automaton. Computer says no...")
  };

  /// A DFA for recognising indented literal blocks.
  pub static ref INDENTED_LITERAL_BLOCK_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::INDENTED_LITERAL_BLOCK_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize INDENTED_LITERAL_BLOCK automaton. Computer says no...")
  };

  /// A DFA for recognising quoted literal blocks.
  pub static ref QUOTED_LITERAL_BLOCK_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::QUOTED_LITERAL_BLOCK_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize QUOTED_LITERAL_BLOCK automaton. Computer says no...")
  };

  /// A DFA for recognising the tops and bottoms of grid tables.
  pub static ref GRID_TABLE_TOP_AND_BOT_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::GRID_TABLE_TOP_AND_BOT_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize GRID_TABLE_TOP_AND_BOT_AUTOMATON automaton. Computer says no...")
  };

  /// A DFA for recognising the tops of simple tables.
  pub static ref SIMPLE_TABLE_TOP_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::SIMPLE_TABLE_TOP_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize SIMPLE_TABLE_TOP_AUTOMATON automaton. Computer says no...")
  };

  /// A DFA for recognising the bottoms of simple tables.
  pub static ref SIMPLE_TABLE_BOTTOM_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::SIMPLE_TABLE_BOTTOM_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize SIMPLE_TABLE_BOTTOM_AUTOMATON automaton. Computer says no...")
  };

  /// A DFA for recognizing footnotes.
  pub static ref FOOTNOTE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::FOOTNOTE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize FOOTNOTE_AUTOMATON. Computer says no...");
  };

  /// A DFA for recognising manually numbered footnotes.
  pub static ref MANUAL_FOOTNOTE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::MANUAL_FOOTNOTE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize MANUAL_FOOTNOTE automaton. Computer says no...")
  };

  /// A DFA for recognising automatically numbered footnotes.
  pub static ref AUTO_NUM_FOOTNOTE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::AUTO_NUM_FOOTNOTE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize AUTO_NUM_FOOTNOTE automaton. Computer says no...")
  };

  /// A DFA for recognising simple refname footnotes.
  pub static ref SIMPLE_NAME_FOOTNOTE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::SIMPLE_NAME_FOOTNOTE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize SIMPLE_NAME_FOOTNOTE automaton. Computer says no...")
  };

  /// A DFA for recognising automatically assigned footnote symbols.
  pub static ref AUTO_SYM_FOOTNOTE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::AUTO_SYM_FOOTNOTE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize AUTO_SYM_FOOTNOTE automaton. Computer says no...")
  };

  /// A DFA for recognising citations.
  pub static ref CITATION_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::CITATION_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize CITATION automaton. Computer says no...")
  };

  /// A DFA for recognising hyperlink targets.
  pub static ref HYPERLINK_TARGET_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::HYPERLINK_TARGET_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize HYPERLINK_TARGET automaton. Computer says no...")
  };

  /// A DFA for recognising substitution definitions.
  pub static ref SUBSTITUTION_DEF_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::SUBSTITUTION_DEF_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize SUBSTITUTION_DEF automaton. Computer says no...")
  };

  /// A DFA for recognising directives.
  pub static ref DIRECTIVE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::DIRECTIVE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize DIRECTIVE automaton. Computer says no...")
  };

  /// A DFA for recognising comments.
  pub static ref COMMENT_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::COMMENT_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize COMMENT automaton. Computer says no...")
  };

  /// A DFA for recognising transition and section title lines.
  pub static ref LINE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::LINE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize LINE automaton. Computer says no...")
  };

  /// A DFA for recognising blocks of text after all other options have been exhausted.
  pub static ref TEXT_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::TEXT_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize TEXT automaton. Computer says no...")
  };

  /// A DFA for recognising inline stong emphasis.
  pub static ref STRONG_EMPH_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::STRONG_EMPH_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize STRONG_EMPH automaton. Computer says no...")
  };

  /// A DFA for recognising inline emphasis.
  pub static ref EMPH_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::EMPH_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize EMPH automaton. Computer says no...")
  };

  /// A DFA for recognising inline literals.
  pub static ref LITERAL_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::LITERAL_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize LITERAL automaton. Computer says no...")
  };

  /// A DFA for recognising inline reference targets.
  pub static ref INLINE_TARGET_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::INLINE_TARGET_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize INLINE_TARGET automaton. Computer says no...")
  };

  /// A DFA for recognising inline interpreted text.
  pub static ref INTERPRETED_TEXT_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::INTERPRETED_TEXT_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize INTERPRETED_TEXT automaton. Computer says no...")
  };

  /// A DFA for recognising inline phrase references.
  pub static ref PHRASE_REF_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::PHRASE_REF_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize PHRASE_REF automaton. Computer says no...")
  };

  /// A DFA for recognising inline simple references.
  pub static ref SIMPLE_REF_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::SIMPLE_REF_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize SIMPLE_REF automaton. Computer says no...")
  };

  /// A DFA for recognising inline footnote references.
  pub static ref FOOTNOTE_REF_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::FOOTNOTE_REF_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize FOOTNOTE_REF automaton. Computer says no...")
  };

  /// A DFA for recognising inline citation references.
  pub static ref CITATION_REF_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::CITATION_REF_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize CITATION_REF automaton. Computer says no...")
  };

  /// A DFA for recognising inline substotution references (macro expansions).
  pub static ref SUBSTITUTION_REF_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::SUBSTITUTION_REF_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize SUBSTITUTION_REF automaton. Computer says no...")
  };

  /// A DFA for recognising inline URIs.
  pub static ref URI_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::URI_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize URI automaton. Computer says no...")
  };

  // A+ specific automata

  /// A DFA for recognising A+ Point of interest column breaks.
  pub static ref APLUS_COL_BREAK_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::APLUS_COL_BREAK_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize APLUS_COL_BREAK automaton. Computer says no...")
  };

  /// A DFA for recognising A+ questionnaire directives.
  pub static ref APLUS_QUESTIONNAIRE_DIRECTIVE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::APLUS_QUESTIONNAIRE_DIRECTIVE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize APLUS_QUESTIONNAIRE_DIRECTIVE automaton. Computer says no...")
  };

  /// A DFA for recognising A+ pick-any choices.
  pub static ref APLUS_PICK_ONE_CHOICE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::APLUS_PICK_ONE_CHOICE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize APLUS_PICK_ONE_CHOICE automaton. Computer says no...")
  };

  /// A DFA for recognising A+ pick-any choices.
  pub static ref APLUS_PICK_ANY_CHOICE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::APLUS_PICK_ANY_CHOICE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize APLUS_PICK_ANY_CHOICE automaton. Computer says no...")
  };

}
