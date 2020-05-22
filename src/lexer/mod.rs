/// This is the `lexer` module of ruSTLa

pub mod token;
mod token_mappings;
mod tests;
pub mod error;

use std::fmt;
use crate::lexer::token::{Token, TokenType};
use crate::lexer::error::{TokenizeError, LexError};

#[derive(PartialEq)]
pub struct Lexer {
  source: String,
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
  pub fn new(source: String) -> Lexer {
    Lexer {
      source: source,
      tokens: Vec::new(),
      buffer: String::with_capacity(4096),
      lexeme_start: 0,
      lookahead: 0,
      row:0,
      col: 0,
    }
  }

  /// ### lex
  /// Pushes the tokens generated by
  /// `scan_token` to `Lexer.tokens`
  /// Consumes the Lexer.
  fn lex(mut self) { // -> Result<Vec<Token>, LexError>
    for (i, c) in self.source.chars().enumerate() {
      self.buffer.push(c);
      self.tokenize_buffer();
    }
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
  /// `lexeme_start` and `lookahead` into
  /// Lexer.tokens
  fn add_token (&mut self, token_type: TokenType) {
    let s = self.source.to_owned();
    let slice = &s[(self.lexeme_start)..self.lookahead];
    self.tokens.push(
      Token{
        t_type: token_type,
        lexeme: String::from(slice),
        row: self.row,
      }
    );
  }

  /// ### is_at_eof
  /// A function that checks whether all
  /// of the characters in the current file
  /// have been consumed.
  pub fn is_at_eof(&self) -> bool {
    self.lookahead >= self.source.chars().count()
  }

}
