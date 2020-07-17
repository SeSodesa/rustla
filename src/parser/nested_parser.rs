/// A submodule that contains specifications related to a nested parser type.

use super::*;

/// ### NestedParser
/// Because of type constraints, it is not possible to simply use the parser
/// specified in the supermodule in a nested manner, especially if one would
/// like to have the nested parser manipulate the line cursor of the parent
/// parser via a mutable reference.
pub struct NestedParser <'parser> {

  /// #### src_lines
  /// A vector of string constructed from a given text block.
  src_lines: Vec<String>,

  /// #### current_line_ref
  /// A mutable reference to the line cursor of the parent parser.
  /// As it is a reference, it needs to have the lifetime of the parent parser, `'parser`.
  current_line: &'parser mut usize,

  /// #### doctree
  /// The DocTree originally owned by the parent parser, which should be
  /// returned to the parent once any nested parsing sessions are done.
  doctree: Option<DocTree>,

  /// #### machine_stack
  /// A stack of states similar to the one owned by the parent `Parser`.
  machine_stack: Vec<Option<StateMachine>>,
}


impl <'parser> NestedParser <'parser> {

  /// ### new
  /// A `Parser` constructor used in constructing nested `Parser`s.
  /// This is very useful when trying to parse lists, as a list might contain sublists starting on the same line as the superlist,
  /// such as
  /// ```rst
  /// 1. a. List item 1a.
  ///    b. List item 1b.
  /// ```
  /// It then makes sense to read in the next block of text with the indentation of the sublist and parse those with a nested parser
  /// in an `EnumeratedList` state. Unlike with `Parser::new`, the initial state is not optional, when a nested parser is constructed
  /// with this method.
  fn new(src: String, doctree: DocTree, current_line_ref: &'parser mut usize, initial_state: StateMachine) -> Self {

    Self {
      src_lines: src.lines().map(|s| s.to_string()).collect::<Vec<String>>(),
      current_line: current_line_ref,
      doctree: Some(doctree),
      machine_stack: vec!(Some(initial_state)),
    }

  }


  /// ### parse
  /// The parsing method of the nested parser works almost identically to the one of the unnested one,
  /// except it has to work with a mutable reference to the line cursor of the parent parser.
  pub fn parse (&'parser mut self) {



  }


}