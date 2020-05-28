/// This module contains the body-related
/// state transitions of the lexer.

#[cfg(test)]
mod title_tests;
#[cfg(test)]
mod list_tests;
#[cfg(test)]
mod block_tests;
#[cfg(test)]
mod directive_tests;
#[cfg(test)]
mod ref_target_tests;
#[cfg(test)]
mod comment_tests;

use crate::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};
use crate::lexer::state::State;
use crate::lexer::Action;

use regex;

/// ### BODY_TRANSITIONS
/// This is a  list of tuples of the form
/// ```rust
/// (TokenType, regex, Action)
/// ```
/// Where the first element is a type of token found in
/// `crate::lexer::token::TokenType`, the second element
/// is a `&'static str` that describes the regex invoved with
/// the `TokenType` and `Action` is a function pointer to
/// a function that handles that type of token.
pub const BODY_TRANSITIONS: &[(TokenType, &'static str, Action)] = &[

  // Overlined headings
  // ------------------
  (TokenType::EqualsOverlinedHeading, r"(?m)^={3,}\n[ \t]*(.+)\n={3,}\n", Lexer::tokenize_section_title),
  (TokenType::DashOverlinedHeading, r"(?m)^-{3,}\n[ \t]*(.+)\n-{3,}\n", Lexer::tokenize_section_title),
  (TokenType::BacktickOverlinedHeading, r"(?m)^`{3,}\n[ \t]*(.+)\n`{3,}\n", Lexer::tokenize_section_title),
  (TokenType::ColonOverlinedHeading, r"(?m)^:{3,}\n[ \t]*(.+)\n:{3,}\n", Lexer::tokenize_section_title),
  (TokenType::SquoteOverlinedHeading, r"(?m)^'{3,}\n[ \t]*(.+)\n'{3,}\n", Lexer::tokenize_section_title),
  (TokenType::DquoteOverlinedHeading, r#"(?m)^"{3,}\n[ \t]*(.+)\n"{3,}\n"#, Lexer::tokenize_section_title),
  (TokenType::TildeOverlinedHeading, r"(?m)^~{3,}\n[ \t]*(.+)\n~{3,}\n", Lexer::tokenize_section_title),
  (TokenType::CaretOverlinedHeading, r"(?m)^\^{3,}\n[ \t]*(.+)\n\^{3,}\n", Lexer::tokenize_section_title),
  (TokenType::UnderscoreOverlinedHeading, r"(?m)^_{3,}\n[ \t]*(.+)\n_{3,}\n", Lexer::tokenize_section_title),
  (TokenType::AsteriskOverlinedHeading, r"(?m)^\*{3,}\n[ \t]*(.+)\n\*{3,}\n", Lexer::tokenize_section_title),
  (TokenType::PlusOverlinedHeading, r"(?m)^\+{3,}\n[ \t]*(.+)\n\+{3,}\n", Lexer::tokenize_section_title),
  (TokenType::HashOverlinedHeading, r"(?m)^\#{3,}\n[ \t]*(.+)\n\#{3,}\n", Lexer::tokenize_section_title),
  (TokenType::LessOverlinedHeading, r"(?m)^<{3,}\n[ \t]*(.+)\n<{3,}\n", Lexer::tokenize_section_title),
  (TokenType::MoreOverlinedHeading, r"(?m)^>{3,}\n[ \t]*(.+)\n>{3,}\n", Lexer::tokenize_section_title),

  // Normal headings
  // ---------------
  (TokenType::EqualsHeading, r"(?m)^(.+)\n={3,}\n", Lexer::tokenize_section_title),
  (TokenType::DashHeading, r"(?m)^(.+)\n-{3,}\n", Lexer::tokenize_section_title),
  (TokenType::BacktickHeading, r"(?m)^(.+)\n`{3,}\n", Lexer::tokenize_section_title),
  (TokenType::ColonHeading, r"(?m)^(.+)\n:{3,}\n", Lexer::tokenize_section_title),
  (TokenType::SquoteHeading, r"(?m)^(.+)\n'{3,}\n", Lexer::tokenize_section_title),
  (TokenType::DquoteHeading, r#"(?m)^(.+)\n"{3,}\n"#, Lexer::tokenize_section_title),
  (TokenType::TildeHeading, r"(?m)^(.+)\n~{3,}\n", Lexer::tokenize_section_title),
  (TokenType::CaretHeading, r"(?m)^(.+)\n\^{3,}\n", Lexer::tokenize_section_title),
  (TokenType::UnderscoreHeading, r"(?m)^(.+)\n_{3,}\n", Lexer::tokenize_section_title),
  (TokenType::AsteriskHeading, r"(?m)^(.+)\n\*{3,}\n", Lexer::tokenize_section_title),
  (TokenType::PlusHeading, r"(?m)^(.+)\n\+{3,}\n", Lexer::tokenize_section_title),
  (TokenType::HashHeading, r"(?m)^(.+)\n\#{3,}\n", Lexer::tokenize_section_title),
  (TokenType::LessHeading, r"(?m)^(.+)\n<{3,}\n", Lexer::tokenize_section_title),
  (TokenType::MoreHeading, r"(?m)^(.+)\n>{3,}\n", Lexer::tokenize_section_title),

  // Lists
  // -----
  (TokenType::UnnumberedList, r"(?m)^\s*[*\-+] .+\n(?:[*\-+] .+\n)+", Lexer::tokenize_unnumbered_list),
  (TokenType::NumberedDotList, r"(?m)^\s*\(?[0-9#ivxlcmIVXLCM]+\. .+\n(?:\([0-9#ivxlcmIVXLCM]+\) .+\n)*", Lexer::tokenize_numbered_list),
  (TokenType::NumberedLRparList, r"(?m)^\s*\(?[0-9#ivxlcmIVXLCM]+\) .+\n(?:\([0-9#ivxlcmIVXLCM]+\) .+\n)*", Lexer::tokenize_numbered_list),
  (TokenType::NumberedRparList, r"(?m)^\s*[0-9#ivxlcmIVXLCM]+\) .+\n(?:[0-9#ivxlcmIVXLCM]+\) .+\n)*", Lexer::tokenize_numbered_list),
  (TokenType::NoBolAlphaDotList, r"(?m)^\s*[A-Z]+\. .+\n(?:[ \t]*[A-Z]+\. .+\n)+", Lexer::tokenize_alpha_list),
  (TokenType::AlphaLRparList, r"(?m)^\s*\(?[a-zA-Z]+\) .+\n(?:[ \t]*\([a-zA-Z]+\) .+\n)+", Lexer::tokenize_alpha_list),
  (TokenType::AlphaRparList, r"(?m)^\s*[a-zA-Z]+\) .+\n(?:[ \t]*[a-zA-Z]+\) .+\n)+", Lexer::tokenize_alpha_list),
  (TokenType::DefinitionList, r"(?m)^(?:(\s*).+\n(?:  .+\n)+\s)+", Lexer::tokenize_definition_list),
  (TokenType::FieldList, r"(?m)^\s*(?::.+: .+\n(?:[ \t]{2}.+\n)*)+", Lexer::tokenize_field_list),

  // // Blocks
  // // ------
  (TokenType::LiteralBlock, r"(?m)::\s*\n[ \t]+.*\n(?:(?:[ \t]+.*)?\n)+", Lexer::tokenize_literal_block),
  (TokenType::PerLineLiteralBlock, r"(?m)::\s*\n(>+ .+\n|>+[ \t]*\n)+\s*\n", Lexer::tokenize_per_line_literal_block),
  (TokenType::LineBlock, r"(?m)^\s*(?:\|.*\n|\|[ \t]*)+\s*", Lexer::tokenize_line_block),
  (TokenType::Paragraph, r"(?m)^\s*(?:^.+\n)+\s+", Lexer::tokenize_paragraph),

  // // Directives
  // // ----------
  (TokenType::GeneralDirective, r"(?m)^ *\.\.\s*[\w:-]+?::[ \t]*.*", Lexer::tokenize_general_directive),

  // // Reference targets
  // // -----------------
  (TokenType::ReferenceTarget, r"(?m)^[ \t]*\.\. _\w+:.*?$", Lexer::tokenize_reference_target),
  (TokenType::FootnoteOrCitationTarget, r"(?m)^ *\.\.\s*\[.+\].*?$", Lexer::tokenize_footnote_or_citation_target),
  (TokenType::SubstitutionDefinition, r"(?m)^ *\.\.\s*\|.+\|\s*[\w:-]+?::[ \t]*.*", Lexer::tokenize_substitution_definition),

  // // Comments
  // // --------
  (TokenType::Comment, r"(?m)^ *\.\..*\n( +.*\n|\n)+", Lexer::tokenize_comment),

];



impl Lexer {

  /// ### tokenize_section_title
  /// Creates the tokens related to overlined titles
  fn tokenize_section_title (&mut self, tt:TokenType, cs: regex::Captures) {

    println!("Found {:?} at row {}, col {}", tt, self.row, self.col);

    let title = cs.get(1).map_or("", |c| c.as_str());
    self.tokens.push(
      Token::new(
        tt,
        title.to_string(),
        self.row,
        self.col
      )
    );
  }


  /// ### Tokenize_unnumbered_list
  /// Tokenizes an unnumbered list
  fn tokenize_unnumbered_list(&mut self, tt:TokenType, cs: regex::Captures) {

  }

  /// ### Tokenize_numbered_list
  /// Tokenizes an unnumbered list
  fn tokenize_numbered_list(&mut self, tt:TokenType, cs: regex::Captures) {

  }


  /// ### Tokenize_alpha_list
  /// Tokenizes an unnumbered list
  fn tokenize_alpha_list(&mut self, tt:TokenType, cs: regex::Captures) {

  }

  /// ### Tokenize_definition_list
  /// Tokenizes an unnumbered list
  fn tokenize_definition_list(&mut self, tt:TokenType, cs: regex::Captures) {

  }

  /// ### Tokenize_field_list
  /// Tokenizes an unnumbered list
  fn tokenize_field_list(&mut self, tt:TokenType, cs: regex::Captures) {

  }


  /// ### tokenize_literal_block
  /// Tokenizes a literal block
  fn tokenize_literal_block(&mut self, tt:TokenType, cs: regex::Captures) {

  }


  /// ### tokenize_per_line_literal_block
  /// Tokenizes a per-line literal block
  fn tokenize_per_line_literal_block(&mut self, tt:TokenType, cs: regex::Captures) {

  }


  /// ### tokenize_line_block
  /// Tokenizes a line block
  fn tokenize_line_block(&mut self, tt:TokenType, cs: regex::Captures) {

  }


  /// ### tokenize_paragraph
  /// Tokenizes a paragraph
  fn tokenize_paragraph(&mut self, tt:TokenType, cs: regex::Captures) {

  }


  /// ### tokenize_general_directive
  /// Tokenizes a paragraph
  fn tokenize_general_directive(&mut self, tt:TokenType, cs: regex::Captures) {

  }



  /// ### tokenize_reference_target
  /// Tokenizes a reference target
  fn tokenize_reference_target(&mut self, tt:TokenType, cs: regex::Captures) {

  }


  /// ### tokenize_footnote_or_citation_target
  /// Tokenizes both footnote and citation targets
  fn tokenize_footnote_or_citation_target(&mut self, tt:TokenType, cs: regex::Captures) {

  }


  /// ### tokenize_substitution_definition
  /// Tokenizes a subsittution definition target
  fn tokenize_substitution_definition(&mut self, tt:TokenType, cs: regex::Captures) {

  }


  /// ### tokenize_comment
  /// Tokenizes a subsittution definition target
  fn tokenize_comment(&mut self, tt:TokenType, cs: regex::Captures) {

  }

}



