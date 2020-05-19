/// This is the `lexer` module of ruSTLa

pub mod token;
mod tests;
pub mod error;

use std::fmt;
use crate::lexer::token::{Token, TokenType};
use crate::lexer::error::{TokenizeError, LexError};

#[derive(PartialEq)]
pub struct Lexer {
  source: String,
  tokens: Vec<Token>,
  lexeme_start: usize,
  lookahead: usize,
  row: usize,
  col: usize,
}

impl fmt::Debug for Lexer {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Lexer")
        .field("source", &self.source)
        .field("id", &self.tokens)
        .finish()
  }
}

/// Lexer type methods
impl Lexer {

  /// ### Lexer constructor
  pub fn new(source: String) -> Lexer {
    Lexer {
      source: source,
      tokens: Vec::new(),
      lexeme_start: 0,
      lookahead: 0,
      row:0,
      col: 0,
    }
  }

  /// ### lex
  /// Pushes the tokens generated by
  /// `scan_token` to `Lexer.tokens`
  fn lex(mut self) -> Result<Vec<Token>, LexError> {
    while ! self.is_at_eof() {
      self.lexeme_start = self.lookahead;
      let tok = match Lexer::scan_token(&mut self) {
        Ok(tok) => tok,
        Err(e) => return Err(LexError::new(&e.row, &e.col))
      };
      self.tokens.push(tok);
    }
    self.tokens.push(
      Token:: new(
        TokenType::EOF,
        String::from(""),
        self.row,
      )
    );
    Ok(self.tokens)
  }

  /// ### scan_token
  /// Reads the next lexeme and produces
  /// a token mathcing it. This is the
  /// core of the lexer itself.
  fn scan_token(&mut self) -> Result<Token, TokenizeError>{
    
    let c: char = match self.advance() {
      Some(c) => c,
      None => return Err(TokenizeError::new(&self.row, &self.col))
    };

    match c {
      _ => Err(TokenizeError::new(&self.row, &self.col))
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


  /// ### add_token
  /// Pushes a token from the lexeme between
  /// `lexeme_start` and `lookahead`
  fn add_token () {

  }

  /// ### is_at_eof
  /// A function that checks whether all
  /// of the characters in the current file
  /// have been consumed.
  pub fn is_at_eof(&self) -> bool {
    self.lookahead >= self.source.chars().count()
  }

}
