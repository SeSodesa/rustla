/// This module contains the inline parts
/// of the lexer state transitions

use crate::lexer::Lexer;
use crate::lexer::token::TokenType;
use crate::lexer::state::State;
use crate::lexer::token_mappings::Action;

use regex;

pub const INLINE_TRANSITIONS: &[(TokenType, &'static str, Action)] = &[
  (TokenType::Escape, r"\\.", tokenize_escape),
  (TokenType::Code, r"``", tokenize_code),
  (TokenType::TargetReference, r"`.+?<.+?>`__?", tokenize_inline_reftarget),
  (TokenType::InlineReference, r"`.+?`__?", tokenize_inline_ref),
  (TokenType::RoleContent, r"`.+?`:[a-zA-Z0-9:-]+?:", tokenize_role_content),
  (TokenType::ContentRole, r":[a-zA-Z0-9:-]+?:`.+?`", tokenize_content_role),
  (TokenType::StrongEmphasis, r"\*\*.+?\*\*", tokenize_strong_emphasis),
  (TokenType::Emphasis, r"\*.+?\*", tokenize_emphasis),
  (TokenType::FootnoteOrCitation, r"\[.*?\]_", tokenize_footnote_or_citation),
  (TokenType::Hyperlink, r"<.+?>", tokenize_hyperlink),
  (TokenType::Text, r"[^\\\n\[*`:]+", tokenize_text_no_ldelim),
  (TokenType::Text, r".", tokenize_text),
];

fn tokenize_escape (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

}

fn tokenize_code (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

}


fn tokenize_inline_reftarget (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

}


fn tokenize_inline_ref (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

}

fn tokenize_role_content (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

}

fn tokenize_content_role (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

}


fn tokenize_strong_emphasis (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

}


fn tokenize_emphasis (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

}

fn tokenize_footnote_or_citation (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

}


fn tokenize_hyperlink (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

}

fn tokenize_text_no_ldelim (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

}

fn tokenize_text (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

}
