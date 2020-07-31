/// ## footnote_data
/// A submodule that contains the FoonoteData type and its methods.

use std::collections::HashSet;

use super::*;

/// ### FootnoteData
/// This struct holds information related to footnotes and footnote references
/// intered into the document tree thus far. This information includes things
/// such as reserved foonote labels and mappings between
pub struct HyperrefData {

  /// #### targets
  /// A mapping of foonote labels added to the doctree to node identifiers.
  pub targets: HashMap<String, NodeId>,

  /// #### references
  /// A map of references to node identifiers entered into the doctree thus far.
  pub references: HashMap<String, NodeId>,

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
  pub n_of_sym_footnotes: u32,

  /// #### n_of_anon_targets
  /// The number of anonymous targets entered into the document.
  pub n_of_anon_targets: u32,

  /// #### n_of_anon_refs
  /// The number of anonymous targets entered into the document.
  pub n_of_anon_refs: u32
}


impl HyperrefData {

  /// ### new
  /// A FoonoteData constructor.
  pub fn new() -> Self {

    HyperrefData {
      targets: HashMap::new(),
      references: HashMap::new(),
      n_of_sym_footnotes: 0,
      n_of_anon_targets: 0,
      n_of_anon_refs: 0
    }
  }
}

