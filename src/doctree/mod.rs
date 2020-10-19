/// ## doctree
/// This module defines the document tree and its nodes
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use std::rc::{Rc, Weak};

use std::collections::HashMap;

mod larst_writer;
mod restructuredtext_transforms;
mod node_categories;
mod tree_zipper;
use tree_zipper::TreeZipper;
pub mod tree_node;
use tree_node::TreeNode;
pub mod tree_node_types;
use tree_node_types::TreeNodeType;
pub mod directives;
use directives::{DirectiveNode, AdmonitionDirective, ImageDirective, BodyElementDirective, TableDirective, DocumentPartDirective, ReferenceDirective, HTMLSpecificDirective, SubstitutionDefDirective, MiscellaneousDirective, AplusDirective};
mod hyperref_data;
use hyperref_data::{HyperrefData, ANON_REF_LABEL_PREFIX, ANON_REF_LABEL_SUFFIX};
mod class_data;
use class_data::ClassData;
mod section_data;
use section_data::SectionData;
mod walkers;

use crate::common::{
  SectionLineStyle,
  EnumDelims, EnumKind, NodeId,
  EnumAsInt, PatternName, FootnoteKind,
  HTMLAlignment, HorizontalAlignment, Length,
  TableColWidths, MetricType, ToCBacklinks
};

// --------------
//  Test modules
// --------------
mod tests;
mod test_walkers;

/// ### DocTree
/// A container for the document tree.
/// In addition to holding ownership of the
/// tree (stored in a zipper), also contains
/// metadata about the tree.
pub struct DocTree {

  /// #### filename_stem
  /// The canonicalized file path without the file suffix.
  filename_stem: String,

  /// #### file_folder
  /// The path to the folder the source file is stored in.
  /// The object file will be stored in the same folder with a different suffix.
  file_folder: String,


  /// #### tree
  /// Holds the tree focused on a specific node.
  tree: TreeZipper,

  /// #### node_count
  /// Keeps track of how many nodes have been added to the tree thus far
  /// besides the root node, that gets an ID of `0`. Some nodes might differ
  /// in their behaviour depending on their insertion order into the tree.
  /// For example, a field list will be transformed into bibliographic data,
  /// if it is the first non-(whitespace|comment) node in the tree.
  node_count: NodeId,

  /// #### hyperref_data
  /// The container for hyperref data related to the doctree.
  hyperref_data: HyperrefData,

  /// #### class_data
  /// A container that holds on to the possibly generated HTML classes.
  class_data: ClassData,

  /// #### section_data
  /// A container that keeps track of known section styles and section levels corresponding to them.
  section_data: SectionData

}

use std::path::PathBuf;

/// ### DocTree
/// Document tree container methods
impl DocTree {

  /// ### new
  /// A `DocTree` constructor.
  pub fn new(doc_name: PathBuf) -> Self {

    let root_id: NodeId = 0;
    let root_data = TreeNodeType::Document;
    let root_node = TreeNode::new(root_data, root_id, None, None);

    let file_stem: String = if let Some(path_os_str) = doc_name.file_stem() {
      if let Some(path_str) = path_os_str.to_str() {
        path_str.to_string()
      } else {
        panic!("Invalid unicode in file path. Computer says no...")
      }
    } else {
      panic!("No recognizable source file name to be found. Computer says no...")
    };

    let file_folder = if let Some(parent) = doc_name.parent() {
      if let Some(path_str) = parent.to_str() {
        path_str.to_string()
      } else {
        panic!("Source folder path could not be converted to a string. Computer says no...")
      }
    } else {
      panic!("Source is not in any folder (even root). Computer says no...")
    };

    DocTree {
      filename_stem: file_stem,
      file_folder: file_folder,
      tree: TreeZipper::new(root_node, None, None),
      node_count: root_id + 1,
      hyperref_data: HyperrefData::new(),
      class_data: ClassData::new(),
      section_data: SectionData::new()
    }
  }


  /// ### n_of_nodes
  /// 
  /// Returns the value of the contnained node counter.
  pub fn n_of_nodes (&self) -> NodeId {
    self.node_count
  }


  /// ### print_tree
  /// Mainly for debugging purposes.
  /// 
  /// Prints the contaiend tree, focused on the current node.
  pub fn print_tree (&self) {
    eprintln!("The Document Tree\n=================");
    eprintln!("{:#?}", self.tree)
  }


  /// ### print_node
  /// 
  /// Prints the currently focused on node.
  fn print_node (&self) {
    eprintln!("{:#?}", self.tree.node)
  }


  /// ### print_node_id
  /// 
  /// Prints the id of the currently focused on node.
  fn print_node_id (&self) {
    eprintln!("{:#?}", self.tree.node.id)
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
  /// Creates a new node from given data, pushes it to the children of currently focused on node and focuses on the new node.
  /// If this succeeds, also increments `self.node_count`.
  /// Returns `Result::{Ok(self), Err(self)}`, depending on the success of this operation.
  pub fn push_data_and_focus (mut self, node_data: TreeNodeType) -> Result<Self, Self> {

    let target_labels = self.hyperref_actions(&node_data);
    let classes = self.classes();
    match self.tree.push_data_and_focus(node_data, self.node_count, target_labels, classes) {
      Ok(tree) => {
        self.node_count += 1;
        self.tree = tree;
        Ok(self)
      }
      Err(tree) => {
        self.tree = tree;
        Err(self)
      }
    }
  }


  /// ### push_data
  /// Creates a new node from given data and pushes it to the children of currently focused on node.
  /// If this succeeds, also increments `self.node_count`.
  /// Returns self in either `Ok` or an `Err`.
  pub fn push_data (mut self, node_data: TreeNodeType) -> Result<Self, Self> {

    let target_labels = self.hyperref_actions(&node_data);
    let classes = self.classes();
    match self.tree.push_data(node_data, self.node_count, target_labels, classes) {
      Ok(tree) => {
        self.tree = tree;
        self.node_count += 1;
        Ok(self)
      },
      Err(tree) => {
        self.tree = tree;
        Err(self)
      }
    }
  }


  /// ### push_child
  /// Pushes a new node to the children of the node currently focused on.
  /// If the addition was successful, returns `Ok(())`, else returns the given node wrapped in an `Err`.
  pub fn push_child (&mut self, mut node: TreeNode) -> Result<(), TreeNode> {

    // Check if there is an incoming internal target label and if there is, add it to the node being processed.
    let acc_target_label = self.hyperref_data.mut_accumulated_internal_target_label();
    if !acc_target_label.is_empty() {

      match node.shared_data() {

        TreeNodeType::EmptyLine | TreeNodeType::WhiteSpace { .. } => {}
        _ => {
          node.set_target_label(Some(acc_target_label.drain(..).collect()));
          acc_target_label.clear();
        }
      }
    };

    self.hyperref_actions(node.shared_data());
    match self.tree.push_child(node) {
      Ok(()) => {
        self.node_count += 1;
        Ok(())
      },
      Err(node) => Err(node)
    }
  }


  /// ### node_specific_actions
  /// Performs any node specific actions to the doctree based on given node data.
  /// Returns an optional internal target label
  fn hyperref_actions (&mut self, shared_node_data: &TreeNodeType) -> Option<Vec<String>> {

    use crate::common::normalize_refname;

    // Check if there is an incoming internal target label
    let accumulated_target_label = self.hyperref_data.mut_accumulated_internal_target_label();
    let mut target_label: Option<Vec<String>> = if accumulated_target_label.is_empty() { None } else {

      match shared_node_data {

        TreeNodeType::EmptyLine | TreeNodeType::WhiteSpace { .. } => { None },

        _ => {
          let label = Some(accumulated_target_label.drain(..).collect());
          accumulated_target_label.clear();
          label
        }
      }
    };

    // Check for targetable or referential nodes. If one is encountered, add it to the known targets or references.
    let normalized_refname = match &shared_node_data {
      TreeNodeType::Footnote {target, label, .. } => {

        let normalized_refname = normalize_refname(label);
        self.add_target(&shared_node_data, &normalize_refname(normalized_refname.as_str()), self.node_count);
        Some(normalized_refname)
      }
      TreeNodeType::ExternalHyperlinkTarget { uri, target, .. } => {

        let normalized_refname = normalize_refname(target);
        self.add_target(&shared_node_data, &normalize_refname(normalized_refname.as_str()), self.node_count);
        Some(normalized_refname)
      }
      TreeNodeType::IndirectHyperlinkTarget {target, indirect_target, .. } => {

        let normalized_target_refname = normalize_refname(target);
        let normalized_indirect_refname = normalize_refname(target);

        self.add_target(&shared_node_data, &normalize_refname(normalized_target_refname.as_str()), self.node_count);
        self.add_reference(&shared_node_data, &normalize_refname(normalized_indirect_refname.as_str()), self.node_count);
        Some(normalized_target_refname)
      }
      TreeNodeType::Section {title_text, level, line_style } => {

        let normalized_refname = normalize_refname(title_text);

        self.add_target(&shared_node_data, &normalize_refname(normalized_refname.as_str()), self.node_count);
        self.section_data.add_section_level(*line_style);
        if *level > self.section_data.highest_encountered_section_level() {
          self.section_data.increment_encountered_section_number();
        }
        Some(normalized_refname)
      }
      _ => None
    };

    if let Some(refname) = normalized_refname {
      if let Some(label) = &mut target_label {
        label.push(refname)
      } else {
        target_label = Some(vec![refname])
      }
    }

    target_label
  }

  /// ### class_actions
  ///
  /// Returns the stack of incoming classes, if there are any.
  fn classes (&mut self) -> Option<Vec<String>>{
    let classes = self.class_data.mut_classes();
    if classes.is_empty() { None } else {
      Some(classes.drain(..).collect())
    }
  }


  /// ### shared_node
  /// Returns a shared reference to the current node .
  pub fn shared_node (&self) -> &TreeNode {
    &self.tree.node
  }


  /// ### mut_node
  /// Returns a shared reference to the current node .
  pub fn mut_node (&mut self) -> &mut TreeNode {
    &mut self.tree.node
  }


  /// ### shared_children
  /// Returns an optional shared reference to the current node's children, if the exist.
  pub fn shared_children (&self) -> Option<&Vec<TreeNode>> {
    if let Some(children) = self.tree.shared_children() {
      Some(children)
    } else {
      None
    }
  }


  /// ### mut_children
  /// Returns an optional mutable reference to the current node's children, if the exist.
  pub fn mut_children (&mut self) -> Option<&mut Vec<TreeNode>> {
    if let Some(children) = self.tree.mut_children() {
      Some(children)
    } else {
      None
    }
  }


  /// ### shared_node_data
  /// Retrieves a shared reference to the data of the
  /// currently focused on node.
  pub fn shared_node_data (&self) -> &TreeNodeType {
    self.tree.node.shared_data()
  }


  /// ### mut_node_data
  /// Retrieves a shared reference to the data of the
  /// currently focused on node.
  pub fn mut_node_data (&mut self) -> &mut TreeNodeType {
    self.tree.node.mut_data()
  }


  /// ### get_child_data
  /// Retrieves a shared reference to the data of the given child of the current node.
  pub fn get_child_data (&self, index: usize) -> &TreeNodeType {

    if let Some(children) = self.tree.node.shared_children() {
      match children.get(index) {
        Some(node) => node.shared_data(),
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

    if let Some(children) = self.tree.node.mut_children() {
      match children.get_mut(index) {
        Some(node) => node.mut_data(),
        None => {
          eprintln!("Focused on node does not have as many children as is implied.\nComputer says no...\n");
          panic!()
        }
      }
    } else {
      panic!("Cannot retrieve mutable child data from a node that cannot have children. Computer says no...")
    }
  }


  /// ### shared_child
  /// Retrieves a shared reference to a given child.
  pub fn shared_child (&self, index: usize) -> &TreeNode {

    if let Some(children) = self.tree.node.shared_children() {
      match children.get(index) {
        Some(node) => node,
        None => {
          panic!("Focused on node does not have as many children as is implied. Computer says no...")
        }
      }
    } else { panic!("Cannot retrieve child from a node that cannot have children. Computer says no...") }
  }


  /// ### mut_child
  /// Retrieves a shared reference to a given child.
  pub fn mut_child (&mut self, index: usize) -> &mut TreeNode {

    if let Some(children) = self.tree.node.mut_children() {
      match children.get_mut(index) {
        Some(node) => node,
        None => {
          panic!("Focused on node does not have as many children as is implied. Computer says no...")
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

  /// ### new_section_data
  /// Generates a new section node data container by comparing the given `section_style` to known styles
  /// and corresponding levels via `self.section_levels`. If a section of such style already exists, the level of the section
  /// is simply set to the level matching it. If not, the maximum known level is plus 1
  /// is assigned to the section data.
  /// 
  /// Note that this function does not yet modify known section data or hyperref targets.
  /// This is donw only if pushing the node data to the tree succeeds, and is handled
  /// by the related methods.
  pub fn new_section_data (&self, title_text: &str, section_style: SectionLineStyle) -> TreeNodeType {

    let section_level = self.section_data.line_style_section_level(&section_style);
    TreeNodeType::Section {
      level: section_level,
      title_text: title_text.to_string(),
      line_style: section_style
    }
  }


  /// ### add_section
  /// Adds a new section to the doctree, also taking care of adding the section title
  /// to the hyperref data of the tree, updating the section counter and mapping
  /// the section type to the appropriate section level.
  pub fn add_section (mut self, title_text: &str, section_style: SectionLineStyle) -> Self {

    let section_data = self.new_section_data(title_text, section_style);

    self = match self.push_data(section_data) {
      Ok(tree) | Err(tree) => tree
    };
    self
  }


  /// ### walk_to_parent_section_level
  /// Walks up the tree to a given section level.
  pub fn walk_to_parent_section_level (mut self, level: usize) -> Self {

    self.tree = self.tree.walk_to_parent_section_level(level);
    self
  }


  /// ### shared_parent_ref
  /// Returns an `Option`al shared reference to the parent node.
  pub fn shared_parent_ref (&self) -> Option<&TreeZipper> {
    self.tree.shared_parent_ref()
  }


  /// ### shared_data
  /// Returns a shared reference to the data of the current node.
  pub fn shared_data(&self) -> &TreeNodeType {
    self.tree.shared_data()
  }


  /// ### shared_parent_data
  /// Returns an `Option`al shared reference to parent node data.
  pub fn shared_parent_data (&self) -> Option<&TreeNodeType> {

    if let Some(parent_ref) = self.shared_parent_ref() {
      Some(parent_ref.shared_data())
    } else {
      None
    }
  }

}


/// ### Children
/// Shorthand for a vector of owned child nodes.
/// Empty vector indicates no children.
type Children = Vec<TreeNode>;
