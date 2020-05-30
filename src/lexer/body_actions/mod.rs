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
  (TokenType::EqualsOverlinedHeading, r"^(?m)^={3,}\n[ \t]*(.+)\n={3,}\n", tokenize_section_title),
  (TokenType::DashOverlinedHeading, r"^(?m)^-{3,}\n[ \t]*(.+)\n-{3,}\n", tokenize_section_title),
  (TokenType::BacktickOverlinedHeading, r"^(?m)^`{3,}\n[ \t]*(.+)\n`{3,}\n", tokenize_section_title),
  (TokenType::ColonOverlinedHeading, r"^(?m)^:{3,}\n[ \t]*(.+)\n:{3,}\n", tokenize_section_title),
  (TokenType::SquoteOverlinedHeading, r"^(?m)^'{3,}\n[ \t]*(.+)\n'{3,}\n", tokenize_section_title),
  (TokenType::DquoteOverlinedHeading, r#"^(?m)^"{3,}\n[ \t]*(.+)\n"{3,}\n"#, tokenize_section_title),
  (TokenType::TildeOverlinedHeading, r"^(?m)^~{3,}\n[ \t]*(.+)\n~{3,}\n", tokenize_section_title),
  (TokenType::CaretOverlinedHeading, r"^(?m)^\^{3,}\n[ \t]*(.+)\n\^{3,}\n", tokenize_section_title),
  (TokenType::UnderscoreOverlinedHeading, r"^(?m)^_{3,}\n[ \t]*(.+)\n_{3,}\n", tokenize_section_title),
  (TokenType::AsteriskOverlinedHeading, r"^(?m)^\*{3,}\n[ \t]*(.+)\n\*{3,}\n", tokenize_section_title),
  (TokenType::PlusOverlinedHeading, r"^(?m)^\+{3,}\n[ \t]*(.+)\n\+{3,}\n", tokenize_section_title),
  (TokenType::HashOverlinedHeading, r"^(?m)^\#{3,}\n[ \t]*(.+)\n\#{3,}\n", tokenize_section_title),
  (TokenType::LessOverlinedHeading, r"^(?m)^<{3,}\n[ \t]*(.+)\n<{3,}\n", tokenize_section_title),
  (TokenType::MoreOverlinedHeading, r"^(?m)^>{3,}\n[ \t]*(.+)\n>{3,}\n", tokenize_section_title),

  // Normal headings
  // ---------------
  (TokenType::EqualsHeading, r"^(?m)^(.+)\n={3,}\n", tokenize_section_title),
  (TokenType::DashHeading, r"^(?m)^(.+)\n-{3,}\n", tokenize_section_title),
  (TokenType::BacktickHeading, r"^(?m)^(.+)\n`{3,}\n", tokenize_section_title),
  (TokenType::ColonHeading, r"^(?m)^(.+)\n:{3,}\n", tokenize_section_title),
  (TokenType::SquoteHeading, r"(?m)^(.+)\n'{3,}\n", tokenize_section_title),
  (TokenType::DquoteHeading, r#"^(?m)^(.+)\n"{3,}\n"#, tokenize_section_title),
  (TokenType::TildeHeading, r"^(?m)^(.+)\n~{3,}\n", tokenize_section_title),
  (TokenType::CaretHeading, r"^(?m)^(.+)\n\^{3,}\n", tokenize_section_title),
  (TokenType::UnderscoreHeading, r"^(?m)^(.+)\n_{3,}\n", tokenize_section_title),
  (TokenType::AsteriskHeading, r"^(?m)^(.+)\n\*{3,}\n", tokenize_section_title),
  (TokenType::PlusHeading, r"^(?m)^(.+)\n\+{3,}\n", tokenize_section_title),
  (TokenType::HashHeading, r"^(?m)^(.+)\n\#{3,}\n", tokenize_section_title),
  (TokenType::LessHeading, r"^(?m)^(.+)\n<{3,}\n", tokenize_section_title),
  (TokenType::MoreHeading, r"^(?m)^(.+)\n>{3,}\n", tokenize_section_title),

  // Lists
  // -----
  (TokenType::UnnumberedList, r"^(?m)^(\s*)([*\-+])( .+\n(?:^\s*  .+\n)*)", tokenize_unnumbered_list),
  (TokenType::NumberedDotList, r"^(?m)^\s*\(?[0-9#ivxlcmIVXLCM]+\. .+\n(?:\([0-9#ivxlcmIVXLCM]+\) .+\n)*", tokenize_numbered_list),
  (TokenType::NumberedLRparList, r"^(?m)^\s*\(?[0-9#ivxlcmIVXLCM]+\) .+\n(?:\([0-9#ivxlcmIVXLCM]+\) .+\n)*", tokenize_numbered_list),
  (TokenType::NumberedRparList, r"^(?m)^\s*[0-9#ivxlcmIVXLCM]+\) .+\n(?:[0-9#ivxlcmIVXLCM]+\) .+\n)*", tokenize_numbered_list),
  (TokenType::NoBolAlphaDotList, r"^(?m)^\s*[A-Z]+\. .+\n(?:[ \t]*[A-Z]+\. .+\n)+", tokenize_alpha_list),
  (TokenType::AlphaLRparList, r"^(?m)^\s*\(?[a-zA-Z]+\) .+\n(?:[ \t]*\([a-zA-Z]+\) .+\n)+", tokenize_alpha_list),
  (TokenType::AlphaRparList, r"^(?m)^\s*[a-zA-Z]+\) .+\n(?:[ \t]*[a-zA-Z]+\) .+\n)+", tokenize_alpha_list),
  (TokenType::DefinitionList, r"^(?m)^(?:(\s*).+\n(?:  .+\n)+\s)+", tokenize_definition_list),
  (TokenType::FieldList, r"^(?m)^\s*(?::.+: .+\n(?:[ \t]{2}.+\n)*)+", tokenize_field_list),

  // // Blocks
  // // ------
  (TokenType::LiteralBlock, r"^(?m)::\s*\n[ \t]+.*\n(?:(?:[ \t]+.*)?\n)+", tokenize_literal_block),
  (TokenType::PerLineLiteralBlock, r"^(?m)::\s*\n(>+ .+\n|>+[ \t]*\n)+\s*\n", tokenize_per_line_literal_block),
  (TokenType::LineBlock, r"^(?m)^\s*(?:\|.*\n|\|[ \t]*)+\s*", tokenize_line_block),
  (TokenType::Paragraph, r"^(?m)^\s*(?:^.+\n)+\s+", tokenize_paragraph),

  // // Directives
  // // ----------
  (TokenType::GeneralDirective, r"^(?m)^ *\.\.\s*[\w:-]+?::[ \t]*.*", tokenize_general_directive),

  // // Reference targets
  // // -----------------
  (TokenType::ReferenceTarget, r"^(?m)^[ \t]*\.\. _\w+:.*?$", tokenize_reference_target),
  (TokenType::FootnoteOrCitationTarget, r"^(?m)^ *\.\.\s*\[.+\].*?$", tokenize_footnote_or_citation_target),
  (TokenType::SubstitutionDefinition, r"^(?m)^ *\.\.\s*\|.+\|\s*[\w:-]+?::[ \t]*.*", tokenize_substitution_definition),

  // // Comments
  // // --------
  (TokenType::Comment, r"(?m)^ *\.\..*\n( +.*\n|\n)+", tokenize_comment),

];


/// ### tokenize_section_title
/// Creates the tokens related to overlined titles
fn tokenize_section_title (lex: &mut Lexer, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

  let title = cs.get(1).unwrap();
  lex.tokens.push(
    Token::new(
      tt,
      title.as_str().to_string(),
      // lex.row,
      // lex.col,
      title.start(),
      title.end(),
    )
  );
}


/// ### Tokenize_unnumbered_list
/// Tokenizes an unnumbered list
fn tokenize_unnumbered_list(lex: &mut Lexer, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

  let preceding_ws = cs.get(1).unwrap();

    // Whitespace replaced by a single blank line
  lex.tokens.push(
    Token::new(
      TokenType::BlankLine,
      String::from("\n\n"),
      // lex.row,
      // lex.col,
      preceding_ws.start(),
      preceding_ws.end()
    )
  );

  let bullet = cs.get(2).unwrap();
  lex.tokens.push(
    Token::new(
      TokenType::Bullet,
      bullet.as_str().to_string(),
      // lex.row,
      // lex.col,
      bullet.start(),
      bullet.end()
    )
  );

  let inline_src = cs.get(3).unwrap().as_str();

  let inline_toks = &mut Lexer::new_from_lexer(lex, inline_src, State::Inline).lex();

  println!("Inline tokens: {:?}", inline_toks);

  lex.tokens.append(inline_toks);


}

/// ### Tokenize_numbered_list
/// Tokenizes an unnumbered list
fn tokenize_numbered_list(lex: &mut Lexer, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### Tokenize_alpha_list
/// Tokenizes an unnumbered list
fn tokenize_alpha_list(lex: &mut Lexer, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}

/// ### Tokenize_definition_list
/// Tokenizes an unnumbered list
fn tokenize_definition_list(lex: &mut Lexer, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}

/// ### Tokenize_field_list
/// Tokenizes an unnumbered list
fn tokenize_field_list(lex: &mut Lexer, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_literal_block
/// Tokenizes a literal block
fn tokenize_literal_block(lex: &mut Lexer, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_per_line_literal_block
/// Tokenizes a per-line literal block
fn tokenize_per_line_literal_block(lex: &mut Lexer, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_line_block
/// Tokenizes a line block
fn tokenize_line_block(lex: &mut Lexer, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_paragraph
/// Tokenizes a paragraph
fn tokenize_paragraph(lex: &mut Lexer, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_general_directive
/// Tokenizes a paragraph
fn tokenize_general_directive(lex: &mut Lexer, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}



/// ### tokenize_reference_target
/// Tokenizes a reference target
fn tokenize_reference_target(lex: &mut Lexer, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_footnote_or_citation_target
/// Tokenizes both footnote and citation targets
fn tokenize_footnote_or_citation_target(lex: &mut Lexer, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_substitution_definition
/// Tokenizes a subsititution definition target
fn tokenize_substitution_definition(lex: &mut Lexer, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_comment
/// Tokenizes a comment target
fn tokenize_comment(lex: &mut Lexer, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}

