/// This is the lexer module of ruSTLa

pub mod token;

use std::fmt;
use crate::lexer::token::Token;


  static LC: usize = 0;
  pub struct Lexer {
    id: usize,
    source: String,
    tokens: Vec<Token>
  }

  impl fmt::Debug for Lexer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lexer")
         .finish()
    }
  }