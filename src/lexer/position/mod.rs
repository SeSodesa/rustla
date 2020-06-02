/// This submodule holds the definition of the Lexer position object.

#[derive(Debug, PartialEq)]
/// ### Pos
/// A struct to hold `Lexer` position data.
pub struct Pos {
  pub pos: usize,
  pub lookahead: usize,
  pub row: usize,
  pub col: usize,
}


impl Pos {

  /// ### new
  /// Constructor for a `Lexer` position  object `Pos`.
  pub fn new (pos: usize, lookahead: usize, row: usize, col: usize) -> Self{
    Pos {
      pos: pos,
      lookahead: lookahead,
      row: row,
      col:col,
    }
  }

}
