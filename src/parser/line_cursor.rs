/// ## line_cursor
/// A submodule that contains the line cursor type and its associated function definitions.
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;


/// ### LineCursor
/// A line cursor type of a parser that holds the start line and offset from it.
/// The relative offset cursor is used to actually access
/// the source lines' contents, whereas the sum of relative and absolute cursors is
/// used mainly for debug prints and|or error messages.
#[derive(Debug)]
pub struct LineCursor {

  /// #### offset
  /// This is used to access the contents of the source lines vector held by the parser.
  /// It should generally be initialized to `0`.
  offset: Line,


  /// #### baseline
  /// The line of text that a parser started working on.
  baseline: Line,
}


impl LineCursor {

  /// ### new
  /// A `LineCursor` constructor.
  pub fn new (relative: Line, absolute: Line) -> Self {
    Self {
      offset: relative,
      baseline: absolute,
    }
  }


  /// ### relative_offset
  /// Retrieves the line relative to the baseline that a (nested) parser is on.
  pub fn relative_offset (&self) -> Line {
    self.offset
  }


  /// ### relative_offset_mut_ref
  /// Retrieves a mutable reference to the line that the (nested) parser is on.
  pub fn relative_offset_mut_ref (&mut self) -> &mut Line {
    &mut self.offset
  }


  /// ### increment
  /// Increments relative offset by given `amount`.
  pub fn increment_by (&mut self, amount: Line) {
    self.offset = match self.offset.checked_add(amount) {
      Some(line) => line,
      None => {
        eprintln!("Tried incrementing relative line offset by {} on line {} but overflowed.\nComputer says no...\n", amount, self.sum_total());
        panic!()
      }
    };
  }


  /// ### sum_total
  /// Returns the sum total of `self.relative_offset` and `*self.baseline`.
  pub fn sum_total (&self) -> Line {
    self.offset + self.baseline
  }

}


/// ### Line
/// A type alias for a line vector index.
pub type Line = usize;

