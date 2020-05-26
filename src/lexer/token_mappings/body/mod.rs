/// This module contains the body-related
/// state transitions of the lexer.

mod title_tests;
mod list_tests;
mod block_tests;
mod directive_tests;
mod ref_target_tests;
mod comment_tests;

use crate::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};
use crate::lexer::state::State;
use crate::lexer::token_mappings::Action;

use regex;

/// ### BODY_TRANSITIONS
/// This is a  list of all possible
/// Body element tokens and their matching
/// regexes. The elements are scanned in order,
/// the ones at the top, as in the most significant
/// ones being matched against first.
///
/// Once a token is scanned, a state transition
/// occurs. If a lexeme might contain inline
/// elements such as ``code``, the transition will be to
/// State::Inline, otherwise a transition
/// to the state itself occurs.
pub const BODY_TRANSITIONS: &[(TokenType, &'static str, Action)] = &[

  // Overlined headings
  // ------------------
  (TokenType::EqualsOverlinedHeading, r"(?m)^{3,}\n[ \t]*(.+)\n={3,}\n", tokenize_section_title),
  (TokenType::DashOverlinedHeading, r"(?m)^-{3,}\n[ \t]*(.+)\n-{3,}\n", tokenize_section_title),
  (TokenType::BacktickOverlinedHeading, r"(?m)^`{3,}\n[ \t]*(.+)\n`{3,}\n", tokenize_section_title),
  (TokenType::ColonOverlinedHeading, r"(?m)^:{3,}\n[ \t]*(.+)\n:{3,}\n", tokenize_section_title),
  (TokenType::SquoteOverlinedHeading, r"(?m)^'{3,}\n[ \t]*(.+)\n'{3,}\n", tokenize_section_title),
  (TokenType::DquoteOverlinedHeading, r#"(?m)^"{3,}\n[ \t]*(.+)\n"{3,}\n"#, tokenize_section_title),
  (TokenType::TildeOverlinedHeading, r"(?m)^~{3,}\n[ \t]*(.+)\n~{3,}\n", tokenize_section_title),
  (TokenType::CaretOverlinedHeading, r"(?m)^\^{3,}\n[ \t]*(.+)\n\^{3,}\n", tokenize_section_title),
  (TokenType::UnderscoreOverlinedHeading, r"(?m)^_{3,}\n[ \t]*(.+)\n_{3,}\n", tokenize_section_title),
  (TokenType::AsteriskOverlinedHeading, r"(?m)^\*{3,}\n[ \t]*(.+)\n\*{3,}\n", tokenize_section_title),
  (TokenType::PlusOverlinedHeading, r"(?m)^\+{3,}\n[ \t]*(.+)\n\+{3,}\n", tokenize_section_title),
  (TokenType::HashOverlinedHeading, r"(?m)^\#{3,}\n[ \t]*(.+)\n\#{3,}\n", tokenize_section_title),
  (TokenType::LessOverlinedHeading, r"(?m)^<{3,}\n[ \t]*(.+)\n<{3,}\n", tokenize_section_title),
  (TokenType::MoreOverlinedHeading, r"(?m)^>{3,}\n[ \t]*(.+)\n>{3,}\n", tokenize_section_title),

  // Normal headings
  // ---------------
  (TokenType::EqualsHeading, r"(?m)^(.+)\n={3,}\n", tokenize_section_title),
  (TokenType::DashHeading, r"(?m)^(.+)\n-{3,}\n", tokenize_section_title),
  (TokenType::BacktickHeading, r"(?m)^(.+)\n`{3,}\n", tokenize_section_title),
  (TokenType::ColonHeading, r"(?m)^(.+)\n:{3,}\n", tokenize_section_title),
  (TokenType::SquoteHeading, r"(?m)^(.+)\n'{3,}\n", tokenize_section_title),
  (TokenType::DquoteHeading, r#"(?m)^(.+)\n"{3,}\n"#, tokenize_section_title),
  (TokenType::TildeHeading, r"(?m)^(.+)\n~{3,}\n", tokenize_section_title),
  (TokenType::CaretHeading, r"(?m)^(.+)\n\^{3,}\n", tokenize_section_title),
  (TokenType::UnderscoreHeading, r"(?m)^(.+)\n_{3,}\n", tokenize_section_title),
  (TokenType::AsteriskHeading, r"(?m)^(.+)\n\*{3,}\n", tokenize_section_title),
  (TokenType::PlusHeading, r"(?m)^(.+)\n\+{3,}\n", tokenize_section_title),
  (TokenType::HashHeading, r"(?m)^(.+)\n\#{3,}\n", tokenize_section_title),
  (TokenType::LessHeading, r"(?m)^(.+)\n<{3,}\n", tokenize_section_title),
  (TokenType::MoreHeading, r"(?m)^(.+)\n>{3,}\n", tokenize_section_title),

  // // Lists
  // // -----
  // (TokenType::UnnumberedList, r"(?m)^\s*[*\-+] .+\n(?:[*\-+] .+\n)+", State::Body),
  // (TokenType::NumberedDotList, r"(?m)^\s*\(?[0-9#ivxlcmIVXLCM]+\. .+\n(?:\([0-9#ivxlcmIVXLCM]+\) .+\n)*", State::Inline),
  // (TokenType::NumberedLRparList, r"(?m)^\s*\(?[0-9#ivxlcmIVXLCM]+\) .+\n(?:\([0-9#ivxlcmIVXLCM]+\) .+\n)*", State::Inline),
  // (TokenType::NumberedRparList, r"(?m)^\s*[0-9#ivxlcmIVXLCM]+\) .+\n(?:[0-9#ivxlcmIVXLCM]+\) .+\n)*", State::Inline),
  // (TokenType::NoBolAlphaDotList, r"(?m)^\s*[A-Z]+\. .+\n(?:[ \t]*[A-Z]+\. .+\n)+", State::Inline),
  // (TokenType::AlphaLRparList, r"(?m)^\s*\(?[a-zA-Z]+\) .+\n(?:[ \t]*\([a-zA-Z]+\) .+\n)+", State::Inline),
  // (TokenType::AlphaRparList, r"(?m)^\s*[a-zA-Z]+\) .+\n(?:[ \t]*[a-zA-Z]+\) .+\n)+", State::Inline),
  // (TokenType::DefinitionList, r"(?m)^(?:(\s*).+\n(?:  .+\n)+\s)+", State::Inline),
  // (TokenType::FieldList, r"(?m)^\s*(?::.+: .+\n(?:[ \t]{2}.+\n)*)+", State::Inline),

  // // Blocks
  // // ------
  // (TokenType::LiteralBlock, r"(?m)::\s*\n[ \t]+.*\n(?:(?:[ \t]+.*)?\n)+", State::Body),
  // (TokenType::PerLineLiteralBlock, r"(?m)::\s*\n(>+ .+\n|>+[ \t]*\n)+\s*\n", State::Body),
  // (TokenType::LineBlock, r"(?m)^\s*(?:\|.*\n|\|[ \t]*)+\s*", State::Inline),
  // (TokenType::Paragraph, r"(?m)^\s*(?:^.+\n)+\s+", State::Inline),

  // // Directives
  // // ----------
  // (TokenType::GeneralDirective, r"(?m)^ *\.\.\s*[\w:-]+?::[ \t]*.*", State::Inline),

  // // Reference targets
  // // -----------------
  // (TokenType::ReferenceTarget, r"(?m)^[ \t]*\.\. _\w+:.*?$", State::Inline),
  // (TokenType::FootnoteOrCitationTarget, r"(?m)^ *\.\.\s*\[.+\].*?$", State::Inline),
  // (TokenType::SubstitutionDefinition, r"(?m)^ *\.\.\s*\|.+\|\s*[\w:-]+?::[ \t]*.*", State::Inline),

  // // Comments
  // // --------
  // (TokenType::Comment, r"(?m)^ *\.\..*\n( +.*\n|\n)+", State::Body),

];

/// ### overlined_title_handler
/// Creates the tokens related to overlined titles
fn tokenize_section_title (lexer: &mut Lexer, tt:TokenType, cs: regex::Captures) {
  let title = cs.get(1).map_or("", |c| c.as_str());
  lexer.tokens.push(
    Token::new(
      tt,
      title.to_string(), 
      lexer.lexeme_start, 
      lexer.lookahead
    )
  );
}
