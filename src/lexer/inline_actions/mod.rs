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
  (TokenType::Text, r"^[^\\\n\[*`:]+", tokenize_text_no_ldelim),
  (TokenType::Text, r"^(.)", tokenize_text),
  (TokenType::InlineWhitespace, r"[ \t]+", tokenize_inline_whitespace),
];

fn tokenize_escape (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let s = cs.get(0).unwrap().start();
  let e = cs.get(0).unwrap().end();
  let c = cs.get(1).unwrap();

  lexer.tokens.push(
    Token::new(
      tt,
      String::from(c.as_str()),
      s + lexer.pos.pos,
      e + lexer.pos.pos,
    )
  );
}

fn tokenize_code (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  let code = cs.get(1).unwrap();

  lexer.set_lexeme_limits(&m);

  lexer.tokens.push(
    Token::new(
      tt,
      String::from(code.as_str()),
      m.start() + lexer.pos.pos,
      m.end() + lexer.pos.pos,
    )
  );

  lexer.update_pos();

}


fn tokenize_inline_target_ref (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  let link_alias = cs.get(1).unwrap();
  let link = cs.get(2).unwrap();

  lexer.set_lexeme_limits(&m);

  lexer.tokens.push(
    Token::new(
      tt,
      String::from(""),
      m.start() + lexer.pos.pos,
      m.end() + lexer.pos.pos
    )
  );

  lexer.set_lexeme_limits(&link_alias);

  lexer.tokens.push(
    Token::new(
      TokenType::LinkAlias,
      String::from(link_alias.as_str()),
      link_alias.start() + lexer.pos.pos,
      link_alias.end() + lexer.pos.pos
    )
  );

  lexer.update_pos();

  lexer.set_lexeme_limits(&link);

  lexer.tokens.push(
    Token::new(
      TokenType::Hyperlink,
      String::from(link.as_str()),
      link.start() + lexer.pos.pos,
      link.end() + lexer.pos.pos
    )
  );

  lexer.set_lexeme_limits(&m);

  lexer.update_pos();

}


fn tokenize_inline_ref (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  let target = cs.get(1).unwrap();

  lexer.set_lexeme_limits(&m);

  lexer.tokens.push(
    Token::new(
      TokenType::TargetReference,
      String::from(target.as_str()),
      m.start() + lexer.pos.pos,
      m.end() + lexer.pos.pos
    )
  );

  lexer.update_pos();

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

  lexer.set_lexeme_limits(&m);
  
  lexer.tokens.push(
    Token::new(
      tt,
      String::from(" "),
      m.start() + lexer.pos.pos,
      m.end() + lexer.pos.pos,
    )
  );

  lexer.update_pos();

}


fn tokenize_text_no_ldelim (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();

  lexer.set_lexeme_limits(&m);
  
  lexer.tokens.push(
    Token::new(
      tt,
      String::from(m.as_str()),
      m.start() + lexer.pos.pos,
      m.end() + lexer.pos.pos,
    )
  );

  lexer.update_pos();

}

fn tokenize_text (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);
  
  let m = cs.get(0).unwrap();
  
  lexer.set_lexeme_limits(&m);

  lexer.tokens.push(
    Token::new(
      tt,
      String::from(m.as_str()),
      m.start() + lexer.pos.pos,
      m.end() + lexer.pos.pos,
    )
  );

  lexer.update_pos();

}
