/// ## footnote_data
/// A submodule that contains the FoonoteData type and its methods.
///
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;

/// ### FootnoteData
/// This struct holds information related to footnotes and footnote references
/// intered into the document tree thus far. This information includes things
/// such as reserved foonote labels and mappings between
pub struct HyperrefData {

  /// #### targets
  /// A mapping of foonote labels added to the doctree to node identifiers.
  targets: HashMap<String, NodeId>,

  /// #### references
  /// A map of references to node identifiers entered into the doctree thus far.
  references: HashMap<String, NodeId>,

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

  /// Number of encountered symbolic footnote references
  n_of_sym_footnote_refs: u32,

  /// #### n_of_anon_targets
  /// The number of anonymous targets entered into the document.
  n_of_anon_targets: u32,

  /// #### n_of_anon_refs
  /// The number of anonymous targets entered into the document.
  n_of_anon_references: u32,

  /// #### accumulated_internal_target_label
  /// This label is accumulated when an internal hyperlink target is encountered
  /// with the detected label of the target. If a non-internal target is encountered
  /// and this is not empty, the elements are joined with the string `Self::INTERNAL_TARGET_CONNECTOR`,
  /// and the resulting String is given to the detected node as a target label.
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
      n_of_sym_footnote_refs: 0,
      n_of_anon_targets: 0,
      n_of_anon_references: 0,
      accumulated_internal_target_label: Vec::new()
    }
  }


  /// ### shared_targets
  /// Returns a shared reference to `self.targets`.
  pub fn shared_targets (&self) -> &HashMap<String, NodeId> {
    &self.targets
  }

  /// ### mut_targets
  /// Returns a mutable reference to `self.targets`.
  pub fn mut_targets (&mut self) -> &mut HashMap<String, NodeId> {
    &mut self.targets
  }


  /// ### shared_references
  /// Returns a shared reference to `self.references`.
  pub fn shared_references (&self) -> &HashMap<String, NodeId> {
    &self.references
  }


  /// ### mut_references
  /// Returns a mutable reference to `self.references`.
  pub fn mut_references (&mut self) -> &mut HashMap<String, NodeId> {
    &mut self.references
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


  /// Returns a copy of the contained symbolic footnote counter
  pub fn n_of_symbolic_footnotes(&self) -> u32 {
    self.n_of_sym_footnotes
  }


  /// Returns a copy of the contained symbolic footnote reference counter
  pub fn n_of_symbolic_footnote_refs(&self) -> u32 {
    self.n_of_sym_footnote_refs
  }


  /// Returns a copy of the contained anonymous reference target counter
  pub fn n_of_anon_targets (&self) -> u32 {
    self.n_of_anon_targets
  }


  /// Returns a copy of the contained anonymous reference target counter
  pub fn n_of_anon_target_refs (&self) -> u32 {
    self.n_of_anon_references
  }


  /// Increments the number of symbolic footnotes by a given `amount`.
  /// Performs an overflow check
  pub fn increment_symbolic_footnote_counter_by(&mut self, amount: u32) {
    if let Some(val) = self.n_of_sym_footnotes.checked_add(amount) {
      self.n_of_sym_footnotes = val;
    } else {
      panic!("Tried incrementing symbolic footnote counter {} by {} but overflew. Computer says no...", self.n_of_sym_footnotes, amount)
    }
  }


  /// Increments the number of symbolic footnote references by a given `amount`.
  /// Performs an overflow check
  pub fn increment_symbolic_footnote_ref_counter_by(&mut self, amount: u32) {
    if let Some(val) = self.n_of_sym_footnotes.checked_add(amount) {
      self.n_of_sym_footnotes = val;
    } else {
      panic!("Tried incrementing symbolic footnote referemce counter {} by {} but overflew. Computer says no...", self.n_of_sym_footnote_refs, amount)
    }
  }


  /// Increments the number of symbolic footnotes by a given `amount`.
  /// Performs an overflow check
  pub fn increment_anonymous_target_counter_by(&mut self, amount: u32) {
    if let Some(val) = self.n_of_anon_targets.checked_add(amount) {
      self.n_of_anon_targets = val;
    } else {
      panic!("Tried incrementing the number of anonymous reference target counter {} by {} but overflew. Computer says no...", self.n_of_anon_targets, amount)
    }
  }


  /// Increments the number of symbolic footnote references by a given `amount`.
  /// Performs an overflow check
  pub fn increment_anonymous_target_ref_counter_by(&mut self, amount: u32) {
    if let Some(val) = self.n_of_anon_references.checked_add(amount) {
      self.n_of_anon_references = val;
    } else {
      panic!("Tried incrementing anonymous target reference counter {} by {} but overflew. Computer says no...", self.n_of_anon_references, amount)
    }
  }


  /// ### INTERNAL_TARGET_CONNECTOR
  /// A string for connecting internal target labels into a single String.
  pub const INTERNAL_TARGET_CONNECTOR: &'static str = "--";
}


  /// ### ANON_REF_LABEL_PREFIX
  /// The prefix of an anonymous reference target label.
  /// This is inserted into the label of an anonymous reference target
  /// to differentiate between automatically numbered footnotes and
  /// anonymous or automatically labeled hyperlink targets.
  /// The suffix will be the arabic ordinal of the anonymous target.
  pub const ANON_REF_LABEL_PREFIX: &'static str = "[[-ANON-LABEL-";


  /// ### ANON_REF_LABEL_SUFFIX
  /// The suffix of an anonymous reference target label.
  /// This is inserted into the label of an anonymous reference target
  /// to differentiate between automatically numbered footnotes and
  /// anonymous or automatically labeled hyperlink targets.
  /// The suffix will be the arabic ordinal of the anonymous target.
  pub const ANON_REF_LABEL_SUFFIX: &'static str = "-]]";
