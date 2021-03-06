/*!
A submodule that contains the `HyperrefData` type and its methods.

Copyright © 2020 Santtu Söderholm
*/
use super::*;

/// This struct holds information related to footnotes and footnote references
/// intered into the document tree thus far. This information includes things
/// such as reserved foonote labels and mappings between
pub struct HyperrefData {

    /// A mapping of hyperref targets added to the doctree thus far,
    /// pointing to the node the label was entered into.
    targets: HashMap<String, NodeId>,

    /// A map of references to node identifiers that point to the key,
    /// entered into the doctree thus far.
    /// A string can point to multiple node ids,
    /// as multiple nodes can reference the same reference target.
    references: HashMap<String, Vec<NodeId>>,

    /// A counter of how many symbolic footnotes
    /// have been encountered and successfully
    /// inserted into the doctree. Used to access
    /// the next symbol to be used in
    /// `crate::common::FOONOTE_SYMBOLS` and the
    /// length of the label formed from said symbol
    /// with integer division and modulo operations.
    n_of_sym_footnotes: u32,

    /// Number of encountered symbolic footnote references
    n_of_sym_footnote_refs: u32,

    /// The number of anonymous targets entered into the document.
    n_of_anon_targets: u32,

    /// The number of anonymous targets entered into the document.
    n_of_anon_references: u32,

    /// This vector of labels is accumulated when an internal hyperlink target is encountered
    /// with the detected label of the target. If a node other than in internal target label
    /// is detected, the contents of this container are goven to the node.
    accumulated_internal_target_label: Vec<String>,
}

impl HyperrefData {

    /// A `HyperrefData` constructor.
    pub fn new() -> Self {
        HyperrefData {
            targets: HashMap::new(),
            references: HashMap::new(),
            n_of_sym_footnotes: 0,
            n_of_sym_footnote_refs: 0,
            n_of_anon_targets: 0,
            n_of_anon_references: 0,
            accumulated_internal_target_label: Vec::new(),
        }
    }

    /// Returns a shared reference to `self.targets`.
    pub fn shared_targets(&self) -> &HashMap<String, NodeId> {
        &self.targets
    }

    /// Returns a mutable reference to `self.targets`.
    pub fn mut_targets(&mut self) -> &mut HashMap<String, NodeId> {
        &mut self.targets
    }

    /// Returns a shared reference to `self.references`.
    pub fn shared_references(&self) -> &HashMap<String, Vec<NodeId>> {
        &self.references
    }

    /// Returns a mutable reference to `self.references`.
    pub fn mut_references(&mut self) -> &mut HashMap<String, Vec<NodeId>> {
        &mut self.references
    }

    /// Adds a label to the line of currently processed internal target labels.
    /// These will all reference the same target node, once a node differing from
    /// internal targets is encountered.
    pub fn add_internal_target_label(&mut self, label: String) {
        self.accumulated_internal_target_label.push(label);
    }

    /// Returns a mutable reference to the contained accumulated internal target label.
    pub fn mut_accumulated_internal_target_label(&mut self) -> &mut Vec<String> {
        &mut self.accumulated_internal_target_label
    }

    /// Returns a mutable reference to the contained accumulated internal target label.
    pub fn shared_accumulated_internal_target_label(&self) -> &Vec<String> {
        &self.accumulated_internal_target_label
    }

    /// Returns the accumulated internal target labels as a string,
    /// connected with the connector string "--".
    pub fn internal_labels_as_string(&self) -> String {
        self.accumulated_internal_target_label
            .join(Self::INTERNAL_TARGET_CONNECTOR)
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
    pub fn n_of_anon_targets(&self) -> u32 {
        self.n_of_anon_targets
    }

    /// Returns a copy of the contained anonymous reference target counter
    pub fn n_of_anon_target_refs(&self) -> u32 {
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

    /// A string for connecting internal target labels into a single String.
    pub const INTERNAL_TARGET_CONNECTOR: &'static str = "--";
}

/// The prefix of an anonymous reference target label.
/// This is inserted into the label of an anonymous reference target
/// to differentiate between automatically numbered footnotes and
/// anonymous or automatically labeled hyperlink targets.
/// The suffix will be the arabic ordinal of the anonymous target.
pub const ANON_REF_LABEL_PREFIX: &'static str = "[[-ANON-LABEL-";

/// The suffix of an anonymous reference target label.
/// This is inserted into the label of an anonymous reference target
/// to differentiate between automatically numbered footnotes and
/// anonymous or automatically labeled hyperlink targets.
/// The suffix will be the arabic ordinal of the anonymous target.
pub const ANON_REF_LABEL_SUFFIX: &'static str = "-]]";
