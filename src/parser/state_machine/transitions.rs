/// Module contains a list of transition tuples

pub mod structural;
pub mod body;
pub mod inline;

use regex;
use lazy_static::lazy_static;

use super::*;

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
  (TokenType::EqualsOverlinedHeading, r"^(?m)^(\s*)(={3,}\n[ \t]*(.+)\n={3,})\n", parse_section_title),
  (TokenType::DashOverlinedHeading, r"^(?m)^(\s*)(-{3,}\n[ \t]*(.+)\n-{3,})\n", parse_section_title),
  (TokenType::BacktickOverlinedHeading, r"^(?m)^(\s*)(`{3,}\n[ \t]*(.+)\n`{3,})\n", parse_section_title),
  (TokenType::ColonOverlinedHeading, r"^(?m)^(\s*)(:{3,}\n[ \t]*(.+)\n:{3,})\n", parse_section_title),
  (TokenType::SquoteOverlinedHeading, r"^(?m)^(\s*)('{3,}\n[ \t]*(.+)\n'{3,})\n", parse_section_title),
  (TokenType::DquoteOverlinedHeading, r#"^(?m)^(\s*)("{3,}\n[ \t]*(.+)\n"{3,})\n"#, parse_section_title),
  (TokenType::TildeOverlinedHeading, r"^(?m)^(\s*)(~{3,}\n[ \t]*(.+)\n~{3,})\n", parse_section_title),
  (TokenType::CaretOverlinedHeading, r"^(?m)^(\s*)(\^{3,}\n[ \t]*(.+)\n\^{3,})\n", parse_section_title),
  (TokenType::UnderscoreOverlinedHeading, r"^(\s*)((?m)^_{3,}\n[ \t]*(.+)\n_{3,})\n", parse_section_title),
  (TokenType::AsteriskOverlinedHeading, r"^(?m)^(\s*)(\*{3,}\n[ \t]*(.+)\n\*{3,})\n", parse_section_title),
  (TokenType::PlusOverlinedHeading, r"^(?m)^(\s*)(\+{3,}\n[ \t]*(.+)\n\+{3,})\n", parse_section_title),
  (TokenType::HashOverlinedHeading, r"^(?m)^(\s*)(\#{3,}\n[ \t]*(.+)\n\#{3,})\n", parse_section_title),
  (TokenType::LessOverlinedHeading, r"^(?m)^(\s*)(<{3,}\n[ \t]*(.+)\n<{3,})\n", parse_section_title),
  (TokenType::MoreOverlinedHeading, r"^(?m)^(\s*)(>{3,}\n[ \t]*(.+)\n>{3,})\n", parse_section_title),

  // Normal headings
  // ---------------
  (TokenType::EqualsHeading, r"^(?m)^(\s*)((.+)\n={3,})\n", parse_section_title),
  (TokenType::DashHeading, r"^(?m)^(\s*)((.+)\n-{3,})\n", parse_section_title),
  (TokenType::BacktickHeading, r"^(?m)^(\s*)((.+)\n`{3,})\n", parse_section_title),
  (TokenType::ColonHeading, r"^(?m)^(\s*)((.+)\n:{3,})\n", parse_section_title),
  (TokenType::SquoteHeading, r"(?m)^(\s*)((.+)\n'{3,})\n", parse_section_title),
  (TokenType::DquoteHeading, r#"^(?m)^(\s*)((.+)\n"{3,})\n"#, parse_section_title),
  (TokenType::TildeHeading, r"^(?m)^(\s*)((.+)\n~{3,})\n", parse_section_title),
  (TokenType::CaretHeading, r"^(?m)^(\s*)((.+)\n\^{3,})\n", parse_section_title),
  (TokenType::UnderscoreHeading, r"^(\s*)(?m)^((.+)\n_{3,})\n", parse_section_title),
  (TokenType::AsteriskHeading, r"^(?m)^(\s*)((.+)\n\*{3,})\n", parse_section_title),
  (TokenType::PlusHeading, r"^(?m)^(\s*)((.+)\n\+{3,})\n", parse_section_title),
  (TokenType::HashHeading, r"^(?m)^(\s*)((.+)\n\#{3,})\n", parse_section_title),
  (TokenType::LessHeading, r"^(?m)^(\s*)((.+)\n<{3,})\n", parse_section_title),
  (TokenType::MoreHeading, r"^(?m)^(\s*)((.+)\n>{3,})\n", parse_section_title),

  // Blank Lines
  // -----------
  (TokenType::BlankLines, r"^(\s*\r?\n[ \t]*\r?\n)+", parse_blank_lines),

  // Lists
  // -----
  (TokenType::UnnumberedList, r"^(?m)^(\s*)([*\-+])( .+\n(?:^\s*  .+\n)*)", parse_unnumbered_list),
  (TokenType::NumberedDotList, r"^(?m)^\s*\(?[0-9#ivxlcmIVXLCM]+\. .+\n(?:\([0-9#ivxlcmIVXLCM]+\) .+\n)*", parse_numbered_list),
  (TokenType::NumberedLRparList, r"^(?m)^\s*\(?[0-9#ivxlcmIVXLCM]+\) .+\n(?:\([0-9#ivxlcmIVXLCM]+\) .+\n)*", parse_numbered_list),
  (TokenType::NumberedRparList, r"^(?m)^\s*[0-9#ivxlcmIVXLCM]+\) .+\n(?:[0-9#ivxlcmIVXLCM]+\) .+\n)*", parse_numbered_list),
  (TokenType::NoBolAlphaDotList, r"^(?m)^\s*[A-Z]+\. .+\n(?:[ \t]*[A-Z]+\. .+\n)+", parse_alpha_list),
  (TokenType::AlphaLRparList, r"^(?m)^\s*\(?[a-zA-Z]+\) .+\n(?:[ \t]*\([a-zA-Z]+\) .+\n)+", parse_alpha_list),
  (TokenType::AlphaRparList, r"^(?m)^\s*[a-zA-Z]+\) .+\n(?:[ \t]*[a-zA-Z]+\) .+\n)+", parse_alpha_list),
  (TokenType::DefinitionList, r"^(?m)^(?:(\s*).+\n(?:  .+\n)+\s)+", parse_definition_list),
  (TokenType::FieldList, r"^(?m)^\s*(?::.+: .+\n(?:[ \t]{2}.+\n)*)+", parse_field_list),

  // // Blocks
  // // ------
  (TokenType::LiteralBlock, r"^(?m)::\s*\n[ \t]+.*\n(?:(?:[ \t]+.*)?\n)+", parse_literal_block),
  (TokenType::PerLineLiteralBlock, r"^(?m)::\s*\n(>+ .+\n|>+[ \t]*\n)+\s*\n", parse_per_line_literal_block),
  (TokenType::LineBlock, r"^(?m)^\s*(?:\|.*\n|\|[ \t]*)+\s*", parse_line_block),
  (TokenType::Paragraph, r"^(?m)^(\s*)((?:\S.+\n)+)", parse_paragraph),

  // // Directives
  // // ----------
  (TokenType::GeneralDirective, r"^(?m)^ *\.\.\s*[\w:-]+?::[ \t]*.*", parse_general_directive),

  // // Reference targets
  // // -----------------
  (TokenType::ReferenceTarget, r"^(?m)^[ \t]*\.\. _\w+:.*?$", parse_reference_target),
  (TokenType::FootnoteOrCitationTarget, r"^(?m)^ *\.\.\s*\[.+\].*?$", parse_footnote_or_citation_target),
  (TokenType::SubstitutionDefinition, r"^(?m)^ *\.\.\s*\|.+\|\s*[\w:-]+?::[ \t]*.*", parse_substitution_definition),

  // // Comments
  // // --------
  (TokenType::Comment, r"(?m)^ *\.\..*\n( +.*\n|\n)+", parse_comment),

];


pub const INLINE_TRANSITIONS: &[(TokenType, &'static str, Action)] = &[
  (TokenType::Escape, r"^\\(.)", parse_escape),
  (TokenType::Code, r"^``([^`]+)``", parse_code),
  (TokenType::TargetReference, r"^`(.+?) <(.+?)>`(__?)", parse_inline_target_ref),
  (TokenType::InlineReference, r"^`(.+?)`(__?)", parse_inline_ref),
  (TokenType::RoleContent, r"^:([a-zA-Z0-9:-]+?):`(.+?)`", parse_role_content),
  (TokenType::ContentRole, r"^`(.+?)`:([a-zA-Z0-9:-]+?):", parse_content_role),
  (TokenType::StrongEmphasis, r"^\*\*(.+?)\*\*", parse_strong_emphasis),
  (TokenType::Emphasis, r"^\*(.+?)\*", parse_emphasis),
  (TokenType::FootnoteOrCitation, r"^\[(.+?)\]_", parse_footnote_or_citation),
  (TokenType::URI, r"^<(([^:/?#]+):)?(//([^/?#]*))?([^?#]*)(\?([^#]*))?(#(.*))?>", parse_uri), // Regex taken from https://tools.ietf.org/html/rfc3986#appendix-B
  (TokenType::BlankLines, r"^(\r?\n[ \t]*\r?\n)+", parse_blankline),
  (TokenType::Newline, r"^\n", parse_newline),
  (TokenType::Text, r"^[^\\\n\[*`:<>]+", parse_text_no_ldelim),
  (TokenType::Text, r"^(.)", parse_text),
  (TokenType::InlineWhitespace, r"[ \t]+", parse_inline_whitespace),
];


lazy_static! {

  /// ### ACTION_MAP
  /// A static map of actions specified for the `Lexer` type.
  /// This allows for the easy creation of sublexers,
  /// as with both the parent and child, the type of actions
  /// can simply be a reference to this map.
  /// 
  /// Plus, with this regexes are only compiled into automata once.
  static ref ACTION_MAP: ActionMap = {
    let mut action_map = collections::HashMap::new();

    let mut body_actions = Vec::with_capacity(BODY_TRANSITIONS.len());
    let mut inline_actions = Vec::with_capacity(INLINE_TRANSITIONS.len());

    for (tt, re, fun) in BODY_TRANSITIONS.iter() {
      let r = regex::Regex::new(re).unwrap();
      body_actions.push((tt.clone(), r, *fun));
    }

    action_map.insert(State::Body, body_actions);

    for (tt, re, fun) in INLINE_TRANSITIONS.iter() {
      let r = regex::Regex::new(re).unwrap();
      inline_actions.push((tt.clone(), r, *fun));
    }

    action_map.insert(State::Inline, inline_actions); 
    
    action_map

  };
}