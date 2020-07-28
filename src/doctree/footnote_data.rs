/// ## footnote_data
/// A submodule that contains the FoonoteData type and its methods.

use std::collections::HashSet;

use super::*;

/// ### FootnoteData
/// This struct holds information related to footnotes and footnote references
/// intered into the document tree thus far. This information includes things
/// such as reserved foonote labels and mappings between
pub struct FootnoteData {

  /// #### numbered_foonotes
  /// A mapping of foonote labels added to the doctree to node identifiers.
  footnotes: HashMap<String, NodeId>,

  /// #### n_of_sym_footnotes
  /// A counter of how many symbolic footnotes
  /// have been encountered and successfully
  /// inserted into the doctree. Used to access
  /// the next symbol to be used in
  /// `crate::common::FOONOTE_SYMBOLS` and the
  /// length of the label formed from said symbol
  /// with integer division and modulo operations:
  /// ```rust
  /// let label_length = n_of_sym_footnotes / FOOTNOTE_SYMBOLS.len();
  /// let symbol_index = n_of_sym_footnotes % FOOTNOTE_SYMBOLS.len();
  /// ```
  n_of_sym_footnotes: u32,
}


impl FootnoteData {

  /// ### new
  /// A FoonoteData constructor.
  pub fn new() -> Self {

    Self {
      footnotes: HashMap::new(),
      n_of_sym_footnotes: 0,
    }
  }
}

