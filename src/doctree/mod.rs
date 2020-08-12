/// ## doctree
/// This module defines the document tree and its nodes
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use std::rc::{Rc, Weak};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

mod tests;

mod tree_zipper;
use tree_zipper::TreeZipper;
pub mod tree_node;
use tree_node::TreeNode;
pub mod tree_node_types;
use tree_node_types::TreeNodeType;
mod directives;
use directives::DirectiveNode;
mod hyperref_data;
use hyperref_data::{HyperrefData, ANON_REF_LABEL_PREFIX, ANON_REF_LABEL_SUFFIX};

use crate::common::{
  EnumDelims, EnumKind, NodeId,
  EnumAsInt, PatternName, FootnoteKind,
  HTMLAlignment, HorizontalAlignment, LenghtUnit,
  TableColWidths, MetricType, ToCBacklinks
};

/// ### DocTree
/// A container for the document tree.
/// In addition to holding ownership of the
/// tree (stored in a zipper), also contains
/// metadata about the tree.
pub struct DocTree {

  /// #### tree
  /// Holds the tree focused on a specific node.
  tree: TreeZipper,

  /// #### node_count
  /// Keeps track of how many nodes have been added to the tree thus far
  /// besides the root node, that gets an ID of `0`. Some nodes might differ
  /// in their behaviour depending on their insertion order into the tree.
  /// For example, a field list will be transformed into bibliographic data,
  /// if it is the first non-(whitespace|comment) node in the tree.
  pub node_count: NodeId,

  /// #### footnote_count
  /// The number of footnotes that have been entered into the document thus far.
  /// Main use for this counter is in auto-numbering footnotes with a '#'.
  hyperref_data: HyperrefData,

}


/// ### DocTree
/// Document tree container methods
impl DocTree {

  /// ### new
  /// A `DocTree` constructor.
  pub fn new(doc_name: String) -> Self {

    let root_id: NodeId = 0;
    let root_data = TreeNodeType::Root{doc_name};
    let root_node = TreeNode::new(root_data, root_id, None);

    DocTree {
      tree: TreeZipper::new(root_node, None, None),
      node_count: root_id + 1,
      hyperref_data: HyperrefData::new(),
    }
  }


  /// ### walk_to_root
  /// Walks to the root of the contained tree zipper.
  pub fn walk_to_root (mut self) -> Self {
    self.tree = self.tree.walk_to_root();
    self
  }


  /// ### print_tree
  /// Mainly for debugging purposes.
  /// 
  /// Prints the contaiend tree, focused on the current node.
  pub fn print_tree (&self) {
    eprintln!("The Document Tree\n=================");
    eprintln!("{:#?}", self.tree)
  }


  /// ### node_count
  /// Returns a copy of the current node count in the DocTree.
  pub fn node_count (&self) -> NodeId {
    self.node_count
  }


  /// ### print_internal_labels
  /// mainly for debugging purposes
  /// 
  /// Prints out the internal targe labels stored in `self.hyperref_data` currently being worked on.
  pub fn print_internal_labels (&self) {
    eprintln!("{:#?}", self.hyperref_data.shared_accumulated_internal_target_label());
  }
  
  
  /// ### focus_on_parent
  /// Focuses `self.tree` on its parent node if there is one.
  pub fn focus_on_parent (mut self) -> Self {

    self.tree = match self.tree.focus_on_parent() {
      Ok(tree) => tree,
      Err(tree) => {
        eprintln!("INFO: Tried focusing on node parent but no parent found.\n");
        tree
      }
    };

    self
  }


  /// ### push_data_and_focus
  /// Creates a new node from given data, pushes it to the
  /// children of currently focused on node and focuses on the new node.
  /// If this succeeds, also increments `self.node_count`.
  pub fn push_data_and_focus (mut self, node_data: TreeNodeType) -> Self {

    // Check if there is an incoming internal target label
    let acc_target_label = self.hyperref_data.mut_accumulated_internal_target_label();

    let target_label = if acc_target_label.is_empty() { None } else {

      match node_data {

        TreeNodeType::EmptyLine | TreeNodeType::WhiteSpace { .. } => { None },

        _ => {
          let label = Some(acc_target_label.join(HyperrefData::INTERNAL_TARGET_CONNECTOR));
          acc_target_label.clear();
          label
        }
      }
    };

    // Check for target or reference nodes...
    match &node_data {
      TreeNodeType::Footnote {target, label, .. } => {
        self.add_target(&node_data, label, self.node_count);
      }
      TreeNodeType::ExternalHyperlinkTarget { uri, target, .. } => {
        self.add_target(&node_data, target, self.node_count);
      }
      TreeNodeType::IndirectHyperlinkTarget {target, indirect_target, .. } => {
        self.add_target(&node_data, target, self.node_count);
        self.add_reference(&node_data, indirect_target, self.node_count);
      }
      _ => {}
    };

    self.tree = self.tree.push_data_and_focus(node_data, self.node_count, target_label).unwrap();
    self.node_count += 1;

    self
  }


  /// ### push_data
  /// Creates a new node from given data and pushes it to the
  /// children of currently focused on node.
  /// If this succeeds, also increments `self.node_count`.
  pub fn push_data (mut self, node_data: TreeNodeType) -> Self {

    // Check if there is an incoming internal target label
    let acc_target_label = self.hyperref_data.mut_accumulated_internal_target_label();
    let target_label = if acc_target_label.is_empty() { None } else {

      match node_data {

        TreeNodeType::EmptyLine | TreeNodeType::WhiteSpace { .. } => { None },

        _ => {
          let label = Some(acc_target_label.join(HyperrefData::INTERNAL_TARGET_CONNECTOR));
          acc_target_label.clear();
          label
        }
      }
    };

    // Check for target or reference nodes...
    match &node_data {
      TreeNodeType::Footnote {target, label, .. } => {
        self.add_target(&node_data, label, self.node_count);
      }
      TreeNodeType::ExternalHyperlinkTarget { uri, target, .. } => {
        self.add_target(&node_data, target, self.node_count);
      }
      TreeNodeType::IndirectHyperlinkTarget {target, indirect_target, .. } => {
        self.add_target(&node_data, target, self.node_count);
        self.add_reference(&node_data, indirect_target, self.node_count);
      }
      _ => {}
    };

    self.tree = self.tree.push_data(node_data, self.node_count, target_label).unwrap();
    self.node_count += 1;

    self
  }


  /// ### push_child
  /// Pushes a new node to the children of the node currently focused on.
  pub fn push_child (&mut self, mut node: TreeNode) {

    // Check if there is an incoming internal target label and if there is, add it to the node being processed.
    let acc_target_label = self.hyperref_data.mut_accumulated_internal_target_label();
    if !acc_target_label.is_empty() {

      match node.get_data() {

        TreeNodeType::EmptyLine | TreeNodeType::WhiteSpace { .. } => {}

        _ => {
          node.set_target_label(Some(acc_target_label.join(HyperrefData::INTERNAL_TARGET_CONNECTOR)));
          acc_target_label.clear();
        }
      }
    };


    // Check for target or reference nodes...
    match node.get_data() {
      TreeNodeType::Footnote {target, label, .. } => {
        self.add_target(node.get_data(), label, node.id);
      }
      TreeNodeType::ExternalHyperlinkTarget { uri, target, .. } => {
        self.add_target(node.get_data(), target, node.id);
      }
      TreeNodeType::IndirectHyperlinkTarget {target, indirect_target, .. } => {
        self.add_target(node.get_data(), target, node.id);
        self.add_reference(node.get_data(), indirect_target, node.id);
      }
      _ => {}
    };

    self.tree.push_child(node);
    self.node_count += 1;
  }


  /// ### shared_node_data
  /// Retrieves a shared reference to the data of the
  /// currently focused on node.
  pub fn shared_node_data (&self) -> &TreeNodeType {
    self.tree.node.get_data()
  }


  /// ### mut_node_data
  /// Retrieves a shared reference to the data of the
  /// currently focused on node.
  pub fn get_mut_node_data (&mut self) -> &mut TreeNodeType {
    self.tree.node.get_mut_data()
  }


  /// ### get_child_data
  /// Retrieves a shared reference to the data of the given child of the current node.
  pub fn get_child_data (&self, index: usize) -> &TreeNodeType {

    if let Some(children) = &self.tree.node.children {
      match children.get(index) {
        Some(node) => &node.data,
        None => {
          eprintln!("Focused on node does not have as many children as is implied.\nComputer says no...\n");
          panic!()
        }
      }
    } else {
      panic!("Cannot retrieve shared child data from a node that cannot have children. Computer says no...")
    }
  }


  pub fn n_of_children (&self) -> usize {
    self.tree.n_of_children ()
  }


  /// ### get_mut_child_data
  /// Retrieves a mutable reference to the data of the given child of the current node.
  pub fn get_mut_child_data (&mut self, index: usize) -> &mut TreeNodeType {

    if let Some(children) = &mut self.tree.node.children {
      match children.get_mut(index) {
        Some(node) => &mut node.data,
        None => {
          eprintln!("Focused on node does not have as many children as is implied.\nComputer says no...\n");
          panic!()
        }
      }
    } else {
      panic!("Cannot retrieve mutable child data from a node that cannot have children. Computer says no...")
    }
  }


  /// ### child
  /// Retrieves a shared reference to a given child.
  pub fn child (&self, index: usize) -> &TreeNode {

    if let Some(children) = &self.tree.node.children {
      match children.get(index) {
        Some(node) => node,
        None => {
          eprintln!("Focused on node does not have as many children as is implied.\nComputer says no...\n");
          panic!()
        }
      }
    } else { panic!("Cannot retrieve child from a node that cannot have children. Computer says no...") }
  }


  /// ### shared_sibling_data
  /// Retrieves the node data of a sibling of the currently focused-on node with the given index.
  pub fn shared_sibling_data (&self, sibling_index: usize) -> Option<&TreeNodeType> {
    if let Some(sibling_data) = self.tree.shared_sibling_data(sibling_index) {
      Some(sibling_data)
    } else {
      eprintln!("Warning: No sibling with index {}...\n", sibling_index);
      None
    }
  }


  /// ### index_in_parent
  /// Retrieves the index of the current node with respect to its parent.
  pub fn index_in_parent (&self) -> Option<usize> {
    self.tree.index_in_parent()
  }


  /// ### append_children
  /// Appends the nodes given in a given vector of nodes to the currently
  /// focused on node in `self.tree`.
  pub fn append_children (&mut self, nodes: &mut Children) {
    let children = nodes.len() as NodeId; // No overflow checks...
    self.tree.append_children(nodes);
    self.node_count += children;
  }


  /// ### has_footnote_label
  /// Checks whether the doctree already contains a hyperlink target with the given label.
  pub fn has_target_label (&self, label_to_be_inspected_for: &str) -> bool {
    self.hyperref_data.targets.contains_key(label_to_be_inspected_for)
  }


  /// ### current_node_id
  /// Retrieves a copy of the node id currently focused on.
  pub fn current_node_id (&self) -> NodeId {
    self.tree.node.id
  }


  /// ### add_target
  /// Adds a given label to the known hyperref targets or updates the actual targe node id
  /// if a label is already in the known labels.
  pub fn add_target (&mut self, node_data: &TreeNodeType, label: &String, id: NodeId) {

    match self.hyperref_data.targets.insert(label.clone(), id) {
      Some(node_id) => {
        eprintln!("Found an existing node with the target label \"{}\".\nReplacing duplicate node id value {} with {}...\n", label, node_id, id);
      }
      None => {}
    };

    if let TreeNodeType::Footnote { kind, .. } = node_data {

      eprintln!("kind: {:#?}", kind);

      if let &FootnoteKind::AutoSymbol = kind {
        self.increment_symbolic_footnotes();
      }
    }
  }


  /// ### add_target
  /// Adds a given label to the known hyperref targets or updates the actual targe node id
  /// if a label is already in the known labels.
  pub fn add_reference (&mut self, node_data: &TreeNodeType, label: &String, id: NodeId) {

    match self.hyperref_data.references.insert(label.clone(), id) {
      Some(node_id) => {
        eprintln!("Found an existing node with the reference label \"{}\".\nReplacing duplicate node id value {} with {}...\n", label, node_id, id);
      }
      None => {}
    };
  }


  /// ### push_to_internal_target_stack
  /// Pushes a given label to the chain of detected internal target labels.
  /// Once a non-internal target is encountered, this array of labels will be
  /// made to point to the newly detected node and cleared.
  pub fn push_to_internal_target_stack (&mut self, label: String) {
    self.hyperref_data.add_internal_target_label(label);
  }


  /// ### n_of_symbolic_footnotes
  /// Returns the number of symbolic footnotes that have been entered into the doctree.
  pub fn n_of_symbolic_footnotes (&self) -> u32 {
    self.hyperref_data.n_of_sym_footnotes
  }


  /// ### increment_symbolic_footnotes
  /// Increments symbolic footnote counter of the doctree by 1.
  pub fn increment_symbolic_footnotes (&mut self) {
    self.hyperref_data.n_of_sym_footnotes += 1;
  }


  /// ### increment_anon_targets
  /// Increases the counter for anonymous targets entered into the doctree thus far by one.
  pub fn increment_anon_targets (&mut self) {
    self.hyperref_data.n_of_anon_targets += 1;
  }


  /// ### increment_anon_references
  /// Increases the counter for anonymous targets entered into the doctree thus far by one.
  pub fn increment_anon_references (&mut self) {
    self.hyperref_data.n_of_anon_references += 1;
  }

  /// ### next_anon_target_n
  /// Increments the anon target counter and returns a copy of the result.
  pub fn next_anon_target_n (&mut self) -> u32 {
    self.increment_anon_targets();
    self.hyperref_data.n_of_anon_targets
  }


  /// ### next_anon_reference_n
  /// Increments the anon reference counter and returns a copy of the result.
  pub fn next_anon_reference_n (&mut self) -> u32 {
    self.increment_anon_references();
    self.hyperref_data.n_of_anon_references
  }

  /// ### next_anon_target_label
  /// Returns an allocated String representation of the next anonymous target label.
  pub fn next_anon_target_label (&mut self) -> String {
    format!("{}{}{}", ANON_REF_LABEL_PREFIX, self.next_anon_target_n(), ANON_REF_LABEL_SUFFIX)
  }


  /// ### next_anon_reference_label
  /// Returns an allocated String representation of the next anonymous reference label.
  pub fn next_anon_reference_label (&mut self) -> String {
    format!("{}{}{}", ANON_REF_LABEL_PREFIX, self.next_anon_reference_n(), ANON_REF_LABEL_SUFFIX)
  }


    /// ### shared_targets
  /// Returns a shared reference to `self.targets`.
  pub fn shared_targets (&self) -> &HashMap<String, NodeId> {
    self.hyperref_data.shared_targets()
  }

  /// ### mut_targets
  /// Returns a mutable reference to `self.targets`.
  pub fn mut_targets (&mut self) -> &mut HashMap<String, NodeId> {
    self.hyperref_data.mut_targets()
  }


  /// ### shared_references
  /// Returns a shared reference to `self.references`.
  pub fn shared_references (&self) -> &HashMap<String, NodeId> {
    self.hyperref_data.shared_references()
  }


  /// ### mut_references
  /// Returns a mutable reference to `self.references`.
  pub fn mut_references (&mut self) -> &mut HashMap<String, NodeId> {
    self.hyperref_data.mut_references()
  }

}


/// ### Children
/// Shorthand for a vector of owned child nodes.
/// Empty vector indicates no children.
type Children = Vec<TreeNode>;
