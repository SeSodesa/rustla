/// This is the `lexer` module of ruSTLa

mod token;
mod state;
mod body_actions;
mod inline_actions;
mod error;

#[cfg(test)]
mod tests;

use std::fmt;
use std::str;
use regex;

use crate::lexer::token::{Token, TokenType};
use crate::lexer::state::{State};
use crate::lexer::error::{TokenizeError, LexError};


/// ### Action
/// A function pointer type alias for a Lexer action
type Action = fn(&mut Lexer, TokenType, regex::Captures) -> ();

//#[derive(PartialEq)]
pub struct Lexer <'t> {
  source: &'t str,
  state: State,
  body_actions: Vec<(TokenType, regex::Regex, Action)>,
  inline_actions: Vec<(TokenType, regex::Regex, Action)>,
  tokens: Vec<Token>,
  lexeme_start: usize,
  lookahead: usize,
  pos: usize,
  row: usize,
  col: usize,
}

impl fmt::Debug for Lexer <'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Lexer")
        .field("lexeme_start", &self.lexeme_start)
        .field("lookahead", &self.lookahead)
        .finish()
  }
}

impl fmt::Display for Lexer <'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Lexer location: row = {}, col = {}", self.row, self.col)
  }
}

/// Lexer type methods
impl <'t> Lexer <'t> {

  /// ### new
  /// A Lexer constructor
  pub fn new(source: &'static str, state: state::State) -> Self {

    let mut body_actions = Vec::new();
    let mut inline_actions = Vec::new();

    for (tt, re, fun) in body_actions::BODY_TRANSITIONS.iter() {
      let r = regex::Regex::new(re).unwrap();
      body_actions.push((tt.clone(), r, *fun));
    }

    for (tt, re, fun) in inline_actions::INLINE_TRANSITIONS.iter() {
      let r = regex::Regex::new(re).unwrap();
      inline_actions.push((tt.clone(), r, *fun));
    }

    Lexer {
      source: source,
      state: state,
      body_actions: body_actions,
      inline_actions: inline_actions,
      tokens: Vec::new(),
      lexeme_start: 0,
      lookahead: 0,
      pos: 0,
      row:0,
      col: 0,
    }
  }

  /// # new_from_lexer
  /// Allows constructing a Lexer from another lexer.
  /// Mainly useful for generating sub lexers
  /// for inline lexing.
  pub fn new_from_lexer (lexer: &Lexer, src: &'t str, state: state::State) -> Self {

    let pos = lexer.pos;
    let row = lexer.row;
    let col = lexer.col;

    Lexer {
      source: src,
      state: state,
      body_actions: lexer.body_actions.clone(),
      inline_actions: lexer.inline_actions.clone(),
      tokens: Vec::new(),
      lexeme_start: lexer.lexeme_start,
      lookahead: lexer.lookahead,
      pos: pos,
      row: row,
      col: col,
    }
  }

  /// ### lex
  /// Loops over the source string
  /// until it has been consumed,
  /// calling `scan_token` to try and match
  /// lexemes at the current position.
  /// Consumes the Lexer itself as well.
  fn lex(mut self) -> Vec<Token> {

    let s = self.source;//.clone();
    let mut chars = s.chars();

    while let Some(_c) = chars.next() {

      self.scan_token(&mut chars);

    }

    self.tokens

  }

  /// ### scan_token
  /// Reads the next lexeme and produces
  /// a token mathcing it. This is the
  /// core of the lexer itself.
  fn scan_token(&mut self, chars: &mut str::Chars) {
    
    let s = chars.as_str();

    if self.state == State::Body {
      for (tt, re, a) in self.body_actions.clone().iter() {

        if let Some(cs) = re.captures(s) {

          self.perform_action(a, tt, chars, cs);

          break

        } else {
          continue
        }
      }

    } else if self.state == State::Inline {
      
      for (tt, re, a) in self.inline_actions.clone().iter() {

        if let Some(cs) = re.captures(s) {

          self.perform_action(a, tt, chars, cs);

          break

        } else {
          continue
        }
      }
    }

  }

/// ### perform_action
/// Calls the callback function `a` corresponding to
/// the detected lexeme.
fn perform_action(&mut self, a: &Action, tt: &TokenType, chars: &mut str::Chars, cs: regex::Captures) {

  self.lexeme_start = cs.get(0).unwrap().start() + self.pos;
  self.lookahead = cs.get(0).unwrap().end() + self.pos;

  a(self, tt.clone(), cs);

  self.lexeme_start = self.lookahead;

  self.update_pos(chars);

}


/// ### update_pos
/// Pushes `pos` to the end
/// position of the
/// latest detected lexeme.
/// If this doesn't succeed, simply
/// makes sure `self.pos` doesn't
/// lag behind `self.lexeme_start`.
fn update_pos(&mut self, chars: &mut str::Chars) {
  
  println!("Updating pos...\n");

  while self.pos < self.lexeme_start {

    if let Some(c) = chars.next() {

      println!("Consuming {:?}...", c);

      self.pos += 1;
      self.col += 1;

      println!("Updated (pos, begin, end) -> ({}, {}, {})\n", self.pos, self.lexeme_start, self.lookahead);

      if c == '\n' {
        self.row += 1;
        self.col = 0;
      }

    } else {
      break
    }

  }

  println!("Updated (pos, begin, end) -> ({}, {}, {})\n", self.pos, self.lexeme_start, self.lookahead);

}


}

/// ### val_from_key
/// Goes through a given list of tuples
/// ```
/// (TokenType, str_pattern, Action)
/// ```
/// and looks for a matching tokentype.
/// If it finds one, returns and `Option<&'static str>`,
/// otherwise returns `None`.
fn val_from_key(search_key: &TokenType, map: &[(TokenType, &'static str, Action)]) -> Option<&'static str> {
  for (_, val, _) in map.iter().filter(|&(map_key, _, _)| map_key == search_key) { 
    return Some(val);
  }
  None
}

