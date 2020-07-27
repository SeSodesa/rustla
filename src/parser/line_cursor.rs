/// ## line_cursor
/// A submodule that contains the line cursor type and its associated function definitions.

use super::*;


/// ### LineCursor
/// A line cursor type of a parser that holds the abolsute and relative (in case of nested parsing sessions)
/// positions of the parser in the vector of source lines. The relative cursor is used to actually access
/// the source lines' contents, whereas the sum of relative and absolute cursors is used mainly for debug prints.
pub struct LineCursor <'parser> {

  /// #### relative_offset
  /// This is used to access the contents of the source lines vector held by the parser.
  relative_offset: usize,


  /// #### absolute_offset
  absolute_offset: &'parser mut usize,
}


impl <'parser> LineCursor <'parser> {

  /// ### new
  /// A `LineCursor` constructor.
  pub fn new (relative: usize, absolute: &'parser mut usize) -> Self {
    Self {
      relative_offset: relative,
      absolute_offset: absolute,
    }
  }


  /// ### increment
  /// Increments both the relative and absolute offsets by given `amount`.
  pub fn increment (&mut self, amount: usize) {
    self.relative_offset += amount;
    *self.absolute_offset += amount;
  }


  /// ### sum_total
  /// Returns the sum total of `self.relative_offset` and `*self.absolute_offset`.
  pub fn sum_total (&self) -> usize {
    self.relative_offset + *self.absolute_offset
  }

}