/// This module contains the inline parts
/// of the lexer state transitions

use crate::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};
use crate::lexer::state::State;
use crate::lexer::Action;

use regex;

pub const INLINE_TRANSITIONS: &[(TokenType, &'static str, Action)] = &[
  (TokenType::Escape, r"\\(.)", tokenize_escape),
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
  (TokenType::Text, r"(.)+", tokenize_text),
];

fn tokenize_escape (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

  println!("\nTokenizing an escape sequence...");

  let s = cs.get(0).unwrap().start();
  let e = cs.get(0).unwrap().end();
  let c = cs.get(1).unwrap();

  lexer.tokens.push(
    Token{
      t_type: TokenType::Escape,
      lexeme: c.as_str().to_string(),
      // row: lexer.row,
      // col: lexer.col,
      begin: s,
      end: e,
    }
  );
}

fn tokenize_code (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

  println!("\nTokenizing code...");

}


fn tokenize_inline_reftarget (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

  println!("\nTokenizing inline ref target...");

}


fn tokenize_inline_ref (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

  println!("\nTokenizing inline reference...");

}

fn tokenize_role_content (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

  println!("\nTokenizing role (content first)...");

}

fn tokenize_content_role (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

  println!("\nTokenizing role (content last)...");

}


fn tokenize_strong_emphasis (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

  println!("\nTokenizing Strong emphasis...");

}


fn tokenize_emphasis (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

  println!("\nTokenizing emphasis...");

}

fn tokenize_footnote_or_citation (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

  println!("\nTokenizing footnote | citation...");

}


fn tokenize_hyperlink (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

  println!("\nTokenizing hyperlink...");

}

fn tokenize_text_no_ldelim (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

  println!("\nTokenizing text without starting delimiters...");

  let m = cs.get(0).unwrap();
  
  lexer.tokens.push(
    Token {
      t_type: TokenType::Text,
      lexeme: m.as_str().to_string(),
      // row: lexer.row,
      // col: lexer.col,
      begin: lexer.lexeme_start,
      end: lexer.lookahead,
    }
  );

}

fn tokenize_text (lexer: &mut Lexer, tt: TokenType, cs: regex::Captures) {

  println!("\nTokenizing plain text...");
  
  let m = cs.get(0).unwrap();
  
  lexer.tokens.push(
    Token {
      t_type: TokenType::Text,
      lexeme: m.as_str().to_string(),
      // row: lexer.row,
      // col: lexer.col,
      begin: m.start(),
      end: m.end(),
    }
  );
}
