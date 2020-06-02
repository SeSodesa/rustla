/// This submodule holds the definition of the Lexer position object.

#[derive(Debug, PartialEq)]
pub struct Pos {
  pos: usize,
  row: usize,
  col: usize,
}


impl Pos {

  /// ### new
  /// Constructor for a `Lexer` position  object `Pos`.
  pub fn new (pos: usize, row: usize, col: usize) -> Self{
    Pos {
      pos: pos,
      row: row,
      col:col,
    }
  }

}
