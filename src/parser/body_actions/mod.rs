/// This module contains the body-related
/// state transitions of the parser.

use crate::parser::Parser;
use crate::parser::token::{Token, TokenType};
use crate::parser::state::State;
use crate::parser::Action;

use regex;

/// ### BODY_TRANSITIONS
/// This is a  list of tuples of the form
/// ```rust
/// (TokenType, regex, Action)
/// ```
/// Where the first element is a type of token found in
/// `crate::parser::token::TokenType`, the second element
/// is a `&'static str` that describes the regex invoved with
/// the `TokenType` and `Action` is a function pointer to
/// a function that handles that type of token.
pub const BODY_TRANSITIONS: &[(TokenType, &'static str, Action)] = &[

  // Overlined headings
  // ------------------
  (TokenType::EqualsOverlinedHeading, r"^(?m)^(\s*)(={3,}\n[ \t]*(.+)\n={3,})\n", tokenize_section_title),
  (TokenType::DashOverlinedHeading, r"^(?m)^(\s*)(-{3,}\n[ \t]*(.+)\n-{3,})\n", tokenize_section_title),
  (TokenType::BacktickOverlinedHeading, r"^(?m)^(\s*)(`{3,}\n[ \t]*(.+)\n`{3,})\n", tokenize_section_title),
  (TokenType::ColonOverlinedHeading, r"^(?m)^(\s*)(:{3,}\n[ \t]*(.+)\n:{3,})\n", tokenize_section_title),
  (TokenType::SquoteOverlinedHeading, r"^(?m)^(\s*)('{3,}\n[ \t]*(.+)\n'{3,})\n", tokenize_section_title),
  (TokenType::DquoteOverlinedHeading, r#"^(?m)^(\s*)("{3,}\n[ \t]*(.+)\n"{3,})\n"#, tokenize_section_title),
  (TokenType::TildeOverlinedHeading, r"^(?m)^(\s*)(~{3,}\n[ \t]*(.+)\n~{3,})\n", tokenize_section_title),
  (TokenType::CaretOverlinedHeading, r"^(?m)^(\s*)(\^{3,}\n[ \t]*(.+)\n\^{3,})\n", tokenize_section_title),
  (TokenType::UnderscoreOverlinedHeading, r"^(\s*)((?m)^_{3,}\n[ \t]*(.+)\n_{3,})\n", tokenize_section_title),
  (TokenType::AsteriskOverlinedHeading, r"^(?m)^(\s*)(\*{3,}\n[ \t]*(.+)\n\*{3,})\n", tokenize_section_title),
  (TokenType::PlusOverlinedHeading, r"^(?m)^(\s*)(\+{3,}\n[ \t]*(.+)\n\+{3,})\n", tokenize_section_title),
  (TokenType::HashOverlinedHeading, r"^(?m)^(\s*)(\#{3,}\n[ \t]*(.+)\n\#{3,})\n", tokenize_section_title),
  (TokenType::LessOverlinedHeading, r"^(?m)^(\s*)(<{3,}\n[ \t]*(.+)\n<{3,})\n", tokenize_section_title),
  (TokenType::MoreOverlinedHeading, r"^(?m)^(\s*)(>{3,}\n[ \t]*(.+)\n>{3,})\n", tokenize_section_title),

  // Normal headings
  // ---------------
  (TokenType::EqualsHeading, r"^(?m)^(\s*)((.+)\n={3,})\n", tokenize_section_title),
  (TokenType::DashHeading, r"^(?m)^(\s*)((.+)\n-{3,})\n", tokenize_section_title),
  (TokenType::BacktickHeading, r"^(?m)^(\s*)((.+)\n`{3,})\n", tokenize_section_title),
  (TokenType::ColonHeading, r"^(?m)^(\s*)((.+)\n:{3,})\n", tokenize_section_title),
  (TokenType::SquoteHeading, r"(?m)^(\s*)((.+)\n'{3,})\n", tokenize_section_title),
  (TokenType::DquoteHeading, r#"^(?m)^(\s*)((.+)\n"{3,})\n"#, tokenize_section_title),
  (TokenType::TildeHeading, r"^(?m)^(\s*)((.+)\n~{3,})\n", tokenize_section_title),
  (TokenType::CaretHeading, r"^(?m)^(\s*)((.+)\n\^{3,})\n", tokenize_section_title),
  (TokenType::UnderscoreHeading, r"^(\s*)(?m)^((.+)\n_{3,})\n", tokenize_section_title),
  (TokenType::AsteriskHeading, r"^(?m)^(\s*)((.+)\n\*{3,})\n", tokenize_section_title),
  (TokenType::PlusHeading, r"^(?m)^(\s*)((.+)\n\+{3,})\n", tokenize_section_title),
  (TokenType::HashHeading, r"^(?m)^(\s*)((.+)\n\#{3,})\n", tokenize_section_title),
  (TokenType::LessHeading, r"^(?m)^(\s*)((.+)\n<{3,})\n", tokenize_section_title),
  (TokenType::MoreHeading, r"^(?m)^(\s*)((.+)\n>{3,})\n", tokenize_section_title),

  // Blank Lines
  // -----------
  (TokenType::BlankLines, r"^(\s*\r?\n[ \t]*\r?\n)+", tokenize_blank_lines),

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
  (TokenType::Paragraph, r"^(?m)^(\s*)((?:\S.+\n)+)", tokenize_paragraph),

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


fn tokenize_blank_lines (lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}", tt);

  let m = cs.get(0).unwrap();

  lex.set_lexeme_limits(&m);

  lex.tokens.push(
    Token::new(
      tt,
      String::from("\n\n"),
      m.start() + lex.pos.pos,
      m.end() + lex.pos.pos,
    )
  );

  // lex.update_pos();

}


/// ### tokenize_section_title
/// Creates the tokens related to overlined titles
fn tokenize_section_title (lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

  let ws = cs.get(1).unwrap();

  lex.set_lexeme_limits(&ws);

  lex.tokens.push(
    Token::new(
      TokenType::BlankLines,
      String::from("\n\n"),
      ws.start() + lex.pos.pos,
      ws.end() + lex.pos.pos,
    )
  );

  //lex.update_pos();

  let text = cs.get(3).unwrap();
  let title = cs.get(2).unwrap();

  lex.set_lexeme_limits(&title);

  lex.tokens.push(
    Token::new(
      tt,
      String::from(text.as_str()),
      title.start() + lex.pos.pos,
      title.end() + lex.pos.pos,
    )
  );

  // lex.update_pos();

}


/// ### Tokenize_unnumbered_list
/// Tokenizes an unnumbered list
fn tokenize_unnumbered_list(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

  let list_item = cs.get(0).unwrap();

  println!("Tokenizing preceding whitespace...\n");

  let ws = cs.get(1).unwrap();

  lex.set_lexeme_limits(&ws);

  // lex.update_pos();

  lex.set_lexeme_limits(&list_item);

  lex.tokens.push(
    Token::new(
      tt,
      String::from(""),
      list_item.start() + lex.pos.pos,
      list_item.end() + lex.pos.pos
    )
  );

  let bullet = cs.get(2).unwrap();

  lex.set_lexeme_limits(&bullet);

  lex.tokens.push(
    Token::new(
      TokenType::Bullet,
      bullet.as_str().to_string(),
      bullet.start() + lex.pos.pos,
      bullet.end() + lex.pos.pos
    )
  );

  // lex.update_pos();

  lex.state = State::Inline;

}

/// ### Tokenize_numbered_list
/// Tokenizes an unnumbered list
fn tokenize_numbered_list(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### Tokenize_alpha_list
/// Tokenizes an unnumbered list
fn tokenize_alpha_list(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}

/// ### Tokenize_definition_list
/// Tokenizes an unnumbered list
fn tokenize_definition_list(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}

/// ### Tokenize_field_list
/// Tokenizes an unnumbered list
fn tokenize_field_list(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_literal_block
/// Tokenizes a literal block
fn tokenize_literal_block(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_per_line_literal_block
/// Tokenizes a per-line literal block
fn tokenize_per_line_literal_block(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_line_block
/// Tokenizes a line block
fn tokenize_line_block(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_paragraph
/// Tokenizes a paragraph
fn tokenize_paragraph(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

  let m = cs.get(0).unwrap();
  let ws = cs.get(1).unwrap();
  let par = cs.get(2).unwrap();

  lex.set_lexeme_limits(&ws);

  // lex.tokens.push(
  //   Token::new(
  //     TokenType::BlankLines,
  //     String::from("\n\n"),
  //     ws.start() + lex.pos.pos,
  //     ws.end() + lex.pos.pos,
  //   )
  // );

  // lex.update_pos();

  lex.set_lexeme_limits(&par);

  lex.tokens.push(
    Token::new(
      tt,
      String::from(""),
      par.start() + lex.pos.pos,
      par.end() + lex.pos.pos,
    )
  );

  lex.state = State::Inline;

}


/// ### tokenize_general_directive
/// Tokenizes a paragraph
fn tokenize_general_directive(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}



/// ### tokenize_reference_target
/// Tokenizes a reference target
fn tokenize_reference_target(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_footnote_or_citation_target
/// Tokenizes both footnote and citation targets
fn tokenize_footnote_or_citation_target(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_substitution_definition
/// Tokenizes a subsititution definition target
fn tokenize_substitution_definition(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_comment
/// Tokenizes a comment target
fn tokenize_comment(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}

