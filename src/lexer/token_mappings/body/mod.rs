/// This module contains the body-related
/// state transitions of the lexer.

mod title_tests;
mod list_tests;
mod block_tests;

use crate::lexer::token::TokenType;
use crate::lexer::state::State;

/// ### BODY_TRANSITIONS
/// This is a  list of all possible
/// Body element tokens and their matching
/// regexes. The elements are scanned in order,
/// the ones at the top, as in the most significant
/// ones being matched agains first.
const BODY_TRANSITIONS: &[(TokenType, &'static str, State)] = &[
  // Overlined headings
  // ------------------
  (TokenType::EqualsOverlinedHeading, r"(?m)^{3,}\n[ \t]*.+\n={3,}\n", State::Body),
  (TokenType::DashOverlinedHeading, r"(?m)^-{3,}\n[ \t]*.+\n-{3,}\n", State::Body),
  (TokenType::BacktickOverlinedHeading, r"(?m)^`{3,}\n[ \t]*.+\n`{3,}\n", State::Body),
  (TokenType::ColonOverlinedHeading, r"(?m)^:{3,}\n[ \t]*.+\n:{3,}\n", State::Body),
  (TokenType::SquoteOverlinedHeading, r"(?m)^'{3,}\n[ \t]*.+\n'{3,}\n", State::Body),
  (TokenType::DquoteOverlinedHeading, r#"(?m)^"{3,}\n[ \t]*.+\n"{3,}\n"#, State::Body),
  (TokenType::TildeOverlinedHeading, r"(?m)^~{3,}\n[ \t]*.+\n~{3,}\n", State::Body),
  (TokenType::CaretOverlinedHeading, r"(?m)^\^{3,}\n[ \t]*.+\n\^{3,}\n", State::Body),
  (TokenType::UnderscoreOverlinedHeading, r"(?m)^_{3,}\n[ \t]*.+\n_{3,}\n", State::Body),
  (TokenType::AsteriskOverlinedHeading, r"(?m)^\*{3,}\n[ \t]*.+\n\*{3,}\n", State::Body),
  (TokenType::PlusOverlinedHeading, r"(?m)^\+{3,}\n[ \t]*.+\n\+{3,}\n", State::Body),
  (TokenType::HashOverlinedHeading, r"(?m)^\#{3,}\n[ \t]*.+\n\#{3,}\n", State::Body),
  (TokenType::LessOverlinedHeading, r"(?m)^<{3,}\n[ \t]*.+\n<{3,}\n", State::Body),
  (TokenType::MoreOverlinedHeading, r"(?m)^>{3,}\n[ \t]*.+\n>{3,}\n", State::Body),

  // Normal headings
  // ---------------
  (TokenType::EqualsHeading, r"(?m)^.+\n={3,}\n", State::Body),
  (TokenType::DashHeading, r"(?m)^.+\n-{3,}\n", State::Body),
  (TokenType::BacktickHeading, r"(?m)^.+\n`{3,}\n", State::Body),
  (TokenType::ColonHeading, r"(?m)^.+\n:{3,}\n", State::Body),
  (TokenType::SquoteHeading, r"(?m)^.+\n'{3,}\n", State::Body),
  (TokenType::DquoteHeading, r#"(?m)^.+\n"{3,}\n"#, State::Body),
  (TokenType::TildeHeading, r"(?m)^.+\n~{3,}\n", State::Body),
  (TokenType::CaretHeading, r"(?m)^.+\n\^{3,}\n", State::Body),
  (TokenType::UnderscoreHeading, r"(?m)^.+\n_{3,}\n", State::Body),
  (TokenType::AsteriskHeading, r"(?m)^.+\n\*{3,}\n", State::Body),
  (TokenType::PlusHeading, r"(?m)^.+\n\+{3,}\n", State::Body),
  (TokenType::HashHeading, r"(?m)^.+\n\#{3,}\n", State::Body),
  (TokenType::LessHeading, r"(?m)^.+\n<{3,}\n", State::Body),
  (TokenType::MoreHeading, r"(?m)^.+\n>{3,}\n", State::Body),

  // Lists
  // -----
  (TokenType::UnnumberedList, r"(?m)^\s*[*\-+] .+\n(?:[*\-+] .+\n)+", State::Body),
  (TokenType::NumberedDotList, r"(?m)^\s*\(?[0-9#ivxlcmIVXLCM]+\. .+\n(?:\([0-9#ivxlcmIVXLCM]+\) .+\n)*", State::Inline),
  (TokenType::NumberedLRparList, r"(?m)^\s*\(?[0-9#ivxlcmIVXLCM]+\) .+\n(?:\([0-9#ivxlcmIVXLCM]+\) .+\n)*", State::Inline),
  (TokenType::NumberedRparList, r"(?m)^\s*[0-9#ivxlcmIVXLCM]+\) .+\n(?:[0-9#ivxlcmIVXLCM]+\) .+\n)*", State::Inline),
  (TokenType::NoBolAlphaDotList, r"(?m)^\s*[A-Z]+\. .+\n(?:[ \t]*[A-Z]+\. .+\n)+", State::Inline),
  (TokenType::AlphaLRparList, r"(?m)^\s*\(?[a-zA-Z]+\) .+\n(?:[ \t]*\([a-zA-Z]+\) .+\n)+", State::Inline),
  (TokenType::AlphaRparList, r"(?m)^\s*[a-zA-Z]+\) .+\n(?:[ \t]*[a-zA-Z]+\) .+\n)+", State::Inline),
  (TokenType::DefinitionList, r"(?m)^(?:(\s*).+\n(?:  .+\n)+\s)+", State::Inline),
  (TokenType::FieldList, r"(?m)^\s*(?::.+: .+\n(?:[ \t]{2}.+\n)*)+", State::Inline),

  // Blocks
  // ------
  (TokenType::LiteralBlock, r"(?m)::\s*\n[ \t]+.*\n(?:(?:[ \t]+.*)?\n)+", State::Body),
  (TokenType::PerLineLiteralBlock, r"(?m)::\s*\n(>+ .+\n|>+[ \t]*\n)+\s*\n", State::Body),
  (TokenType::LineBlock, r"(?m)^\s*(?:\|.*\n|\|[ \t]*)+\s*", State::Inline),
  (TokenType::Paragraph, r"(?m)^\s*(?:^.+\n)+\s+", State::Inline),

  // Directives
  // ----------
  (TokenType::GeneralDirective, r"(?m)^ *\.\.\s*[\w:-]+?::[ \t]*.*", State::Inline),

  // Reference targets
  // -----------------
  (TokenType::ReferenceTarget, r"(?m)^[ \t]*\.\. _\w+:.*?$", State::Inline),
  (TokenType::FootnoteOrCitationTarget, r"(?m)^ *\.\.\s*\[.+\].*?$", State::Inline),
  (TokenType::SubstitutionDefinition, r"(?m)^ *\.\.\s*\|.+\|\s*[\w:-]+?::[ \t]*.*", State::Inline),

  // Comments
  // --------
  (TokenType::Comment, r"(?m)^ *\.\..*\n( +.*\n|\n)+", State::Body),

];
