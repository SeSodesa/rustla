/// This module contains the inline parts
/// of the lexer state transitions

mod test;

use crate::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};
use crate::lexer::state::State;
use crate::lexer::Action;

use regex;

pub const INLINE_TRANSITIONS: &[(TokenType, &'static str, Action)] = &[
  (TokenType::Escape, r"^\\(.)", tokenize_escape),
  (TokenType::Code, r"^``([^`]+)``", tokenize_code),
  (TokenType::TargetReference, r"^`(.+?)<(.+?)>`(__?)", tokenize_inline_target_ref),
  (TokenType::InlineReference, r"^`(.+?)`__?", tokenize_inline_ref),
  (TokenType::RoleContent, r"^`.+?`:[a-zA-Z0-9:-]+?:", tokenize_role_content),
  (TokenType::ContentRole, r"^:[a-zA-Z0-9:-]+?:`.+?`", tokenize_content_role),
  (TokenType::StrongEmphasis, r"^\*\*.+?\*\*", tokenize_strong_emphasis),
  (TokenType::Emphasis, r"^\*.+?\*", tokenize_emphasis),
  (TokenType::FootnoteOrCitation, r"^\[.*?\]_", tokenize_footnote_or_citation),
  (TokenType::Hyperlink, r"^<.+?>", tokenize_hyperlink),
  //(TokenType::InlineWhitespace, r"[ \t]+", tokenize_inline_whitespace),
  (TokenType::Text, r"^[^\\\n\[*`:]+", tokenize_text_no_ldelim),
  (TokenType::Text, r"^(.)", tokenize_text),
];

fn tokenize_escape (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

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

fn tokenize_code (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  let code = cs.get(1).unwrap();

  lexer.tokens.push(
    Token::new(
      tt,
      code.as_str().to_string(),
      m.start(),
      m.end(),
    )
  );
}


fn tokenize_inline_target_ref (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let link_alias = cs.get(1).unwrap();
  let link = cs.get(2).unwrap();

  lexer.tokens.push(
    Token::new(
    TokenType::LinkAlias,
    link_alias.as_str().to_string(),
    link_alias.start(),
    link_alias.end()
    )
  );

  lexer.tokens.push(
    Token::new(
    TokenType::Hyperlink,
    link.as_str().to_string(),
    link.start(),
    link.end()
    )
  );

}


fn tokenize_inline_ref (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let target = cs.get(1).unwrap();

  lexer.tokens.push(
    Token::new(
    TokenType::TargetReference,
    target.as_str().to_string(),
    target.start(),
    target.end()
    )
  );

}

fn tokenize_role_content (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

}

fn tokenize_content_role (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

}


fn tokenize_strong_emphasis (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

}


fn tokenize_emphasis (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

}

fn tokenize_footnote_or_citation (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

}


fn tokenize_hyperlink (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

}

fn tokenize_inline_whitespace (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  
  lexer.tokens.push(
    Token {
      t_type: tt,
      lexeme: " ".to_string(),
      // row: lexer.row,
      // col: lexer.col,
      begin: lexer.pos.pos,
      end: lexer.pos.lookahead,
    }
  );

}


fn tokenize_text_no_ldelim (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  
  lexer.tokens.push(
    Token {
      t_type: tt,
      lexeme: m.as_str().to_string(),
      // row: lexer.row,
      // col: lexer.col,
      begin: lexer.pos.pos,
      end: lexer.pos.lookahead,
    }
  );

}

fn tokenize_text (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);
  
  let m = cs.get(0).unwrap();
  
  lexer.tokens.push(
    Token {
      t_type: tt,
      lexeme: m.as_str().to_string(),
      // row: lexer.row,
      // col: lexer.col,
      begin: m.start(),
      end: m.end(),
    }
  );
}
