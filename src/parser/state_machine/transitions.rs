/// Module contains a list of transition tuples

pub mod structural;
pub mod body;
pub mod inline;

use regex;

use crate::parser::Parser;
use crate::parser::state_machine::Action;
use crate::parser::token::{Token, TokenType};
use crate::parser::state::State;
use super::transitions::body::*;
use super::transitions::inline::*;

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


pub const INLINE_TRANSITIONS: &[(TokenType, &'static str, Action)] = &[
  (TokenType::Escape, r"^\\(.)", tokenize_escape),
  (TokenType::Code, r"^``([^`]+)``", tokenize_code),
  (TokenType::TargetReference, r"^`(.+?) <(.+?)>`(__?)", tokenize_inline_target_ref),
  (TokenType::InlineReference, r"^`(.+?)`(__?)", tokenize_inline_ref),
  (TokenType::RoleContent, r"^:([a-zA-Z0-9:-]+?):`(.+?)`", tokenize_role_content),
  (TokenType::ContentRole, r"^`(.+?)`:([a-zA-Z0-9:-]+?):", tokenize_content_role),
  (TokenType::StrongEmphasis, r"^\*\*(.+?)\*\*", tokenize_strong_emphasis),
  (TokenType::Emphasis, r"^\*(.+?)\*", tokenize_emphasis),
  (TokenType::FootnoteOrCitation, r"^\[(.+?)\]_", tokenize_footnote_or_citation),
  (TokenType::URI, r"^<(([^:/?#]+):)?(//([^/?#]*))?([^?#]*)(\?([^#]*))?(#(.*))?>", tokenize_uri), // Regex taken from https://tools.ietf.org/html/rfc3986#appendix-B
  (TokenType::BlankLines, r"^(\r?\n[ \t]*\r?\n)+", tokenize_blankline),
  (TokenType::Newline, r"^\n", tokenize_newline),
  (TokenType::Text, r"^[^\\\n\[*`:<>]+", tokenize_text_no_ldelim),
  (TokenType::Text, r"^(.)", tokenize_text),
  (TokenType::InlineWhitespace, r"[ \t]+", tokenize_inline_whitespace),
];