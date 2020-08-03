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
  pub n_of_anon_refs: u32,

  /// #### accumulated_internal_target_label
  /// This label is accumulated when an internal hyperlink target is encountered
  /// with the detected label of the target. If a non-internal target is encountered
  /// and this is not empty, the elements are joined with a hyphen, and the resulting
  /// String is given to the detected node as a target label.
  accumulated_internal_target_label: Vec<String>,
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
      n_of_anon_refs: 0,
      accumulated_internal_target_label: Vec::new()
    }
  }


  /// ### add_internal_target_label
  /// Adds a label to the line of currently processed internal target labels.
  /// These will all reference the same target node, once a node differing from
  /// internal targets is encountered.
  pub fn add_internal_target_label (&mut self, label: String) {
    self.accumulated_internal_target_label.push(label);
  }


  /// ### mut_accumulated_internal_target_label
  /// Returns a mutable reference to the contained accumulated internal target label.
  pub fn mut_accumulated_internal_target_label (&mut self) -> &mut Vec<String> {
    &mut self.accumulated_internal_target_label
  }


  /// ### shard_accumulated_internal_target_label
  /// Returns a mutable reference to the contained accumulated internal target label.
  pub fn shared_accumulated_internal_target_label (&self) -> &Vec<String> {
    &self.accumulated_internal_target_label
  }


  /// ### internal_labels_as_string
  /// Returns the accumulated internal target labels as a string,
  /// connected with the connector string "--".
  pub fn internal_labels_as_string (&self) -> String {
    self.accumulated_internal_target_label.join(Self::INTERNAL_TARGET_CONNECTOR)
  }


  /// ### INTERNAL_TARGET_CONNECTOR
  /// A string for connecting internal target labels into a single String.
  pub const INTERNAL_TARGET_CONNECTOR: &'static str = "--"; 
}

