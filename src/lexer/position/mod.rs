/// This submodule holds the definition of the Lexer position object.

#[derive(Debug, PartialEq)]
/// ### Pos
/// A struct to hold `Lexer` position data.
pub struct Pos {
  pub pos: usize,
  pub lexeme_start: usize,
  pub lookahead: usize,
  pub row: usize,
  pub col: usize,
}


impl Pos {

  /// ### new
  /// Constructor for a `Lexer` position  object `Pos`.
  pub fn new () -> Self{
    Pos {
      pos: 0,
      lexeme_start: 0,
      lookahead: 0,
      row: 0,
      col:0,
    }
  }

}
