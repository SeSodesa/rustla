/// This is the `lexer` module of ruSTLa

mod token;
mod state;
mod token_mappings;
mod error;

#[cfg(test)]
mod tests;

use std::fmt;
use regex;

use crate::lexer::token::{Token, TokenType};
use crate::lexer::state::{State};
use crate::lexer::error::{TokenizeError, LexError};

type Action = fn(&mut Lexer, TokenType, regex::Captures) -> ();

//#[derive(PartialEq)]
pub struct Lexer {
  source: &'static str,
  state: State,
  body_actions: Vec<(TokenType, regex::Regex, Action)>,
  inline_actions: Vec<(TokenType, regex::Regex, Action)>,
  tokens: Vec<Token>,
  buffer: String,
  lexeme_start: usize,
  lookahead: usize,
  row: usize,
  col: usize,
}

impl fmt::Debug for Lexer {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Lexer")
        .field("lexeme_start", &self.source)
        .field("buffer", &self.buffer)
        .finish()
  }
}

impl fmt::Display for Lexer {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Lexer location: row = {}, col = {}", self.row, self.col)
  }
}

/// Lexer type methods
impl Lexer {

  /// ### Lexer constructor
  pub fn new(source: &'static str) -> Lexer {

    let mut body_actions = Vec::new();
    let mut inline_actions = Vec::new();

    for (tt, re, fun) in token_mappings::body::BODY_TRANSITIONS.iter() {
      let r = regex::Regex::new(re).unwrap();
      body_actions.push((tt.clone(), r, *fun));
    }

    for (tt, re, fun) in token_mappings::inline::INLINE_TRANSITIONS.iter() {
      let r = regex::Regex::new(re).unwrap();
      inline_actions.push((tt.clone(), r, *fun));
    }

    Lexer {
      source: source,
      state: State::Body,
      body_actions: body_actions,
      inline_actions: inline_actions,
      tokens: Vec::new(),
      buffer: String::with_capacity(4096),
      lexeme_start: 0,
      lookahead: 0,
      row:0,
      col: 0,
    }
  }

  /// ### lex
  /// Loops over the source string
  /// until it has been consumed.
  /// Consumes the Lexer itself as well.
  fn lex(mut self) -> Vec<Token>{

    let s = self.source.clone();
    let mut chars = s.chars();

    while let Some(_) = chars.next() {
      let slice = chars.as_str();
      self.scan_token(slice);
    }

    self.tokens

  }

  /// ### tokenize_buffer
  /// Tries to match the contents of the buffer
  /// with the regexes found in the regex
  /// submodule. If a match is found,
  /// `Some<Token>` of matching type is returned.
  fn tokenize_buffer(&self) { // -> Some<Token> {

  }

  /// ### scan_token
  /// Reads the next lexeme and produces
  /// a token mathcing it. This is the
  /// core of the lexer itself.
  fn scan_token(&mut self, s: &str) {
    
    if self.state == State::Body {
      for (tt, re, a) in self.body_actions.clone().iter() {
        if let Some(cs) = re.captures(s) {
          self.lexeme_start = cs.get(0).unwrap().start();
          self.lookahead = cs.get(0).unwrap().end();
          a(self, tt.clone(), cs);
        }
      }
    } else if self.state == State::Inline {
      for (tt, re, a) in self.inline_actions.clone().iter() {
        if let Some(cs) = re.captures(s) {
          a(self, tt.clone(), cs);
        }
      }
    }
  }


  /// ### advance
  /// Reads the next character
  /// (unicode scalar, not grapheme cluster!)
  /// in the source.
  fn advance(&mut self) -> Option<char>{

    self.lookahead += 1;

    let c: char = self.source
    .chars()
    .nth(self.lookahead - 1)?;

    Some(c)
  }

  /// ### is_at_eof
  /// A function that checks whether all
  /// of the characters in the current file
  /// have been consumed.
  pub fn is_at_eof(&self) -> bool {
    self.lookahead >= self.source.chars().count()
  }

}
