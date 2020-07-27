/// ## footnote_data
/// A submodule that contains the FoonoteData type and its methods.

use std::collections::HashSet;

use super::*;

/// ### FootnoteData
/// This struct holds information related to footnotes and footnote references
/// intered into the document tree thus far. This information includes things
/// such as reserved foonote labels and mappings between
pub struct FootnoteData {

  /// #### max_id
  /// The largest footnote label converted to a `u32` entered into the doctree thus far.
  /// This is useful when trying to figure out what ID should be given
  /// to an auto-numbered footnote.
  max_id: NodeId,

  /// #### entered_labels
  /// A mapping of foonote labels added to the doctree to node identifiers.
  footnotes: HashMap<String, NodeId>,

  /// #### references
  /// A mapping of footnote references to node IDs.
  /// Allows us to look up which nodes reference which footnotes.
  references: HashMap<String, NodeId>

}


impl FootnoteData {

  /// ### new
  /// A FoonoteData constructor.
  pub fn new() -> Self {

    Self {
      max_id: 0,
      footnotes: HashMap::new(),
      references: HashMap::new()
    }
  }
}

