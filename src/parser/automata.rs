/// ## Automata
/// 
/// IUn this submodule the patterns in `crate::parser::regex_patterns` are compiled into finite automata
/// using the standard Rust regex crate.
/// 
/// (c) 2020 Santtu Söderholm <santtu.soderholm@tuni.fi>

use lazy_static::lazy_static;
use regex::Regex;
use crate::parser::regex_patterns;

lazy_static! {

  /// A DFA for recognising block quote attributions.
  static ref ATTRIBUTION_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::ATTRIBUTION_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize ATTRIBUTION automaton. Computer says no...")
  };

  /// A DFA for recognising blank lines.
  static ref BLANK_LINE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::BLANK_LINE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize BLANK_LINE automaton. Computer says no...")
  };

  /// A DFA for recognising blank lines.
  static ref BULLET_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::BULLET_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize BULLET automaton. Computer says no...")
  };

  /// A DFA for recognising enumerators.
  static ref ENUMERATOR_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::ATTRIBUTION_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize ENUMERATOR automaton. Computer says no...")
  };

  /// A DFA for recognising arabic numerals.
  static ref ARABIC_PARENS_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::ARABIC_PARENS_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize ARABIC_PARENS automaton. Computer says no...")
  };

  /// A DFA for recognising arabic numerals.
  static ref ARABIC_RPAREN_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::ARABIC_RPAREN_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize ARABIC_RPAREN automaton. Computer says no...")
  };

  /// A DFA for recognising arabic numerals.
  static ref ARABIC_PERIOD_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::ARABIC_PERIOD_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize ARABIC_PERIOD automaton. Computer says no...")
  };

  /// A DFA for recognising lower alphabetic numerals.
  static ref LOWER_ALPHA_PARENS_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::LOWER_ALPHA_PARENS_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize LOWER_ALPHA_PARENS automaton. Computer says no...")
  };

  /// A DFA for recognising lower alphabetic numerals.
  static ref LOWER_ALPHA_RPAREN_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::LOWER_ALPHA_RPAREN_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize LOWER_ALPHA_RPAREN automaton. Computer says no...")
  };

  /// A DFA for recognising lower alphabetic numerals.
  static ref LOWER_ALPHA_PERIOD_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::LOWER_ALPHA_PERIOD_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize LOWER_ALPHA_PERIOD automaton. Computer says no...")
  };

  /// A DFA for recognising upper alphabetic numerals.
  static ref UPPER_ALPHA_PARENS_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::UPPER_ALPHA_PARENS_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize UPPER_ALPHA_PARENS automaton. Computer says no...")
  };

  /// A DFA for recognising upper alphabetic numerals.
  static ref UPPER_ALPHA_RPAREN_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::UPPER_ALPHA_RPAREN_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize UPPER_ALPHA_RPAREN automaton. Computer says no...")
  };

  /// A DFA for recognising upper alphabetic numerals.
  static ref UPPER_ALPHA_PERIOD_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::UPPER_ALPHA_PERIOD_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize UPPER_ALPHA_PERIOD automaton. Computer says no...")
  };

  /// A DFA for recognising lower roman numerals.
  static ref LOWER_ROMAN_PARENS_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::LOWER_ROMAN_PARENS_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize LOWER_ROMAN_PARENS automaton. Computer says no...")
  };

  /// A DFA for recognising lower roman numerals.
  static ref LOWER_ROMAN_RPAREN_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::LOWER_ROMAN_RPAREN_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize LOWER_ROMAN_RPAREN automaton. Computer says no...")
  };

  /// A DFA for recognising lower roman numerals.
  static ref LOWER_ROMAN_PERIOD_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::LOWER_ROMAN_PERIOD_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize LOWER_ROMAN_PERIOD automaton. Computer says no...")
  };

  /// A DFA for recognising upper roman numerals.
  static ref UPPER_ROMAN_PARENS_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::UPPER_ROMAN_PARENS_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize UPPER_ROMAN_PARENS automaton. Computer says no...")
  };

  /// A DFA for recognising upper roman numerals.
  static ref UPPER_ROMAN_RPAREN_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::UPPER_ROMAN_RPAREN_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize UPPER_ROMAN_RPAREN automaton. Computer says no...")
  };

  /// A DFA for recognising upper Roman numerals.
  static ref UPPER_ROMAN_PERIOD_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::UPPER_ROMAN_PERIOD_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize UPPER_ROMAN_PERIOD automaton. Computer says no...")
  };

  /// A DFA for recognising field markers.
  static ref FIELD_MARKER_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::FIELD_MARKER_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize FIELD_MARKER automaton. Computer says no...")
  };

  /// A DFA for recognising indented literal blocks.
  static ref INDENTED_LITERAL_BLOCK_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::INDENTED_LITERAL_BLOCK_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize INDENTED_LITERAL_BLOCK automaton. Computer says no...")
  };

  /// A DFA for recognising quoted literal blocks.
  static ref QUOTED_LITERAL_BLOCK_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::QUOTED_LITERAL_BLOCK_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize QUOTED_LITERAL_BLOCK automaton. Computer says no...")
  };

  /// A DFA for recognising manually numbered footnotes.
  static ref MANUAL_FOOTNOTE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::MANUAL_FOOTNOTE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize MANUAL_FOOTNOTE automaton. Computer says no...")
  };

  /// A DFA for recognising automatically numbered footnotes.
  static ref AUTO_NUM_FOOTNOTE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::AUTO_NUM_FOOTNOTE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize AUTO_NUM_FOOTNOTE automaton. Computer says no...")
  };

  /// A DFA for recognising simple refname footnotes.
  static ref SIMPLE_NAME_FOOTNOTE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::SIMPLE_NAME_FOOTNOTE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize SIMPLE_NAME_FOOTNOTE automaton. Computer says no...")
  };

  /// A DFA for recognising automatically assigned footnote symbols.
  static ref AUTO_SYM_FOOTNOTE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::AUTO_SYM_FOOTNOTE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize AUTO_SYM_FOOTNOTE automaton. Computer says no...")
  };

  /// A DFA for recognising citations.
  static ref CITATION_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::CITATION_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize CITATION automaton. Computer says no...")
  };

  /// A DFA for recognising hyperlink targets.
  static ref HYPERLINK_TARGET_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::HYPERLINK_TARGET_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize HYPERLINK_TARGET automaton. Computer says no...")
  };

  /// A DFA for recognising substitution definitions.
  static ref SUBSTITUTION_DEF_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::SUBSTITUTION_DEF_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize SUBSTITUTION_DEF automaton. Computer says no...")
  };

  /// A DFA for recognising directives.
  static ref DIRECTIVE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::DIRECTIVE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize DIRECTIVE automaton. Computer says no...")
  };

  /// A DFA for recognising comments.
  static ref COMMENT_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::COMMENT_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize COMMENT automaton. Computer says no...")
  };

  /// A DFA for recognising transition and section title lines.
  static ref LINE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::LINE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize LINE automaton. Computer says no...")
  };

  /// A DFA for recognising blocks of text after all other options have been exhausted.
  static ref TEXT_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::TEXT_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize TEXT automaton. Computer says no...")
  };

  /// A DFA for recognising inline stong emphasis.
  static ref STRONG_EMPH_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::STRONG_EMPH_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize STRONG_EMPH automaton. Computer says no...")
  };

  /// A DFA for recognising inline emphasis.
  static ref EMPH_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::EMPH_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize EMPH automaton. Computer says no...")
  };

  /// A DFA for recognising inline literals.
  static ref LITERAL_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::LITERAL_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize LITERAL automaton. Computer says no...")
  };

  /// A DFA for recognising inline reference targets.
  static ref INLINE_TARGET_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::INLINE_TARGET_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize INLINE_TARGET automaton. Computer says no...")
  };

  /// A DFA for recognising inline interpreted text.
  static ref INTERPRETED_TEXT_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::INTERPRETED_TEXT_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize INTERPRETED_TEXT automaton. Computer says no...")
  };

  /// A DFA for recognising inline phrase references.
  static ref PHRASE_REF_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::PHRASE_REF_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize PHRASE_REF automaton. Computer says no...")
  };

  /// A DFA for recognising inline simple references.
  static ref SIMPLE_REF_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::SIMPLE_REF_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize SIMPLE_REF automaton. Computer says no...")
  };

  /// A DFA for recognising inline footnote references.
  static ref FOOTNOTE_REF_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::FOOTNOTE_REF_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize FOOTNOTE_REF automaton. Computer says no...")
  };

  /// A DFA for recognising inline citation references.
  static ref CITATION_REF_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::CITATION_REF_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize CITATION_REF automaton. Computer says no...")
  };

  /// A DFA for recognising inline substotution references (macro expansions).
  static ref SUBSTITUTION_REF_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::SUBSTITUTION_REF_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize SUBSTITUTION_REF automaton. Computer says no...")
  };

  /// A DFA for recognising inline URIs.
  static ref URI_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::URI_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize URI automaton. Computer says no...")
  };

  // A+ specific automata

  /// A DFA for recognising A+ Point of interest column breaks.
  static ref APLUS_COL_BREAK_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::APLUS_COL_BREAK_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize APLUS_COL_BREAK automaton. Computer says no...")
  };

  /// A DFA for recognising A+ questionnaire directives.
  static ref APLUS_QUESTIONNAIRE_DIRECTIVE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::APLUS_QUESTIONNAIRE_DIRECTIVE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize APLUS_QUESTIONNAIRE_DIRECTIVE automaton. Computer says no...")
  };

  /// A DFA for recognising A+ pick-any choices.
  static ref APLUS_PICK_ONE_CHOICE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::APLUS_PICK_ONE_CHOICE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize APLUS_PICK_ONE_CHOICE automaton. Computer says no...")
  };

  /// A DFA for recognising A+ pick-any choices.
  static ref APLUS_PICK_ANY_CHOICE_AUTOMATON: regex::Regex = if let Ok(automaton) = Regex::new(regex_patterns::APLUS_PICK_ANY_CHOICE_PATTERN) {
    automaton
  } else {
    panic!("Could not initialize APLUS_PICK_ANY_CHOICE automaton. Computer says no...")
  };

}
