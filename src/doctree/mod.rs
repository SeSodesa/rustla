/// This module defines the document tree and its nodes

use std::rc::{Rc, Weak};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

mod tests;

mod tree_zipper;
use tree_zipper::TreeZipper;
mod directives;
use directives::DirectiveType;
mod hyperref_data;
use hyperref_data::{HyperrefData, ANON_REF_LABEL_PREFIX, ANON_REF_LABEL_SUFFIX};

use crate::common::{EnumDelims, EnumKind, NodeId, EnumAsInt, PatternName, FootnoteKind};

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


  /// ### get_node_data
  /// Retrieves a shared reference to the data of the
  /// currently focused on node.
  pub fn get_node_data (&self) -> &TreeNodeType {
    self.tree.node.get_data()
  }


  /// ### get_node_data
  /// Retrieves a shared reference to the data of the
  /// currently focused on node.
  pub fn get_mut_node_data (&mut self) -> &mut TreeNodeType {
    self.tree.node.get_mut_data()
  }


  /// ### get_child_data
  /// Retrieves a shared reference to the data of the given child of the current node.
  pub fn get_child_data (&self, index: usize) -> &TreeNodeType {
    match self.tree.node.children.get(index) {
      Some(node) => &node.data,
      None => {
        eprintln!("Focused on node does not have as many children as is implied.\nComputer says no...\n");
        panic!()
      }
    }
  }


  /// ### get_mut_child_data
  /// Retrieves a mutable reference to the data of the given child of the current node.
  pub fn get_mut_child_data (&mut self, index: usize) -> &mut TreeNodeType {
    match self.tree.node.children.get_mut(index) {
      Some(node) => &mut node.data,
      None => {
        eprintln!("Focused on node does not have as many children as is implied.\nComputer says no...\n");
        panic!()
      }
    }
  }


  /// ### child
  /// Retrieves a shared reference to a given child.
  pub fn child (&self, index: usize) -> &TreeNode {
    match self.tree.node.children.get(index) {
      Some(node) => node,
      None => {
        eprintln!("Focused on node does not have as many children as is implied.\nComputer says no...\n");
        panic!()
      }
    }
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


/// ### TreeNode
/// A tree node that contains a struct of `TreeNodeType`
/// plus the information needed to traverse the tree.
#[derive(Debug)]
pub struct TreeNode {
  pub id: NodeId,
  target_label: Option<String>,
  pub data : TreeNodeType,
  pub children: Children,

}

impl TreeNode {

  /// ### new
  /// A `TreeNode` constructor.
  pub fn new(data: TreeNodeType, id: NodeId, target_label: Option<String>) -> Self {
    
    TreeNode {
      id: id,
      target_label: target_label,
      children: Vec::new(),
      data: data
    }
  }


  /// ### new_from_id_ref
  /// Works similarly to `TreeNode::new`, except also increments the id
  /// behind the given address in addition to assignning the previous value
  /// to the node being constructred.
  pub fn new_from_id_ref (data: TreeNodeType, id_ref: &mut NodeId, target_label: Option<String>) -> Self {

    let node = Self {
      id: *id_ref, // assign current id value to node
      target_label: target_label,
      children: Vec::new(),
      data: data,
    };

    *id_ref += 1; // increment the id before returning with new node
    node
  }


  /// ### shared_target_label
  /// Returns a shared (immutable) reference to the optional target label.
  /// If the label is `None`, as is hasn't been set, returns an
  /// empty string slice instead.
  pub fn shared_target_label (&self) -> &str {
    if let Some(label) = self.target_label.as_ref() { label } else { "" }
  }
  
  
  /// ### push_child
  /// Pushes a given child node the the end of `self.children`.
  pub fn push_child (&mut self, node : TreeNode) {

    if self.child_is_allowed(&node.data) {
      self.children.push(node);
    } else {
      eprintln!("Child of type {:#?} not allowed inside a {:#?}.\nComputer says no...\n", node.data, self.data);
      panic!();
    }
  }


  /// ### append_children
  /// Appends multiple children to `self.children`.
  pub fn append_children(&mut self, children: &mut Vec<TreeNode>) {

    // Check whether all children are valid
    for child in children.iter() {
      if self.child_is_allowed(&child.data) {
        continue
      } else {
        eprintln!("Found incompatible child {:#?} when appending children to {:#?}.\nComputer says no...\n", child.data, self.data);
        panic!();
      }
    }

    self.children.append(children);
  }


  /// ### set_target_label
  /// Sets the target label of the node to given `Option<String>`.
  pub fn set_target_label (&mut self, label: Option<String>) {
    self.target_label = label;
  }


  /// ### child_is_allowed
  /// Checks whether a node is allowed to be inserted into another node.
  pub fn child_is_allowed (&self, node_data: &TreeNodeType) -> bool {

    match self.data {

      // These elements are allowed to contain body level nodes
      TreeNodeType::Root { .. }           | TreeNodeType::BulletListItem { .. } | TreeNodeType::EnumeratedListItem { .. }
      | TreeNodeType::DefinitionListItem  | TreeNodeType::FieldListItem { .. }  | TreeNodeType::OptionListItem
      | TreeNodeType::BlockQuote          | TreeNodeType::Footnote { .. }       | TreeNodeType::Citation { .. }  => {
        match node_data {
          TreeNodeType::Paragraph                         | TreeNodeType::BulletList { .. }               | TreeNodeType::EnumeratedList { .. }
          | TreeNodeType::DefinitionList                  | TreeNodeType::FieldList { .. }                | TreeNodeType::OptionList
          | TreeNodeType::LiteralBlock { .. }             | TreeNodeType::LineBlock                       | TreeNodeType::BlockQuote
          | TreeNodeType::DoctestBlock                    | TreeNodeType::Footnote  { .. }                | TreeNodeType::Citation { .. }
          | TreeNodeType::ExternalHyperlinkTarget { .. }  | TreeNodeType::IndirectHyperlinkTarget { .. }  | TreeNodeType::Directive { .. }  | TreeNodeType::SubstitutionDefinition
          | TreeNodeType::Comment                         | TreeNodeType::EmptyLine
            => true,
          _ => false
        }
      },

      // Bullet lists may only contain empty lines or bullet list items
      TreeNodeType::BulletList { .. } => {
        match node_data {
          TreeNodeType::EmptyLine | TreeNodeType::BulletListItem { .. } => true,
          _ => false
        }
      }

      // Enumerated lists may only contain empty lines or enumerated list items
      TreeNodeType::EnumeratedList { .. } => {
        match node_data {
          TreeNodeType::EmptyLine | TreeNodeType::EnumeratedListItem { .. } => true,
          _ => false
        }
      }

      // Field lists may only contain empty lines or field list items
      TreeNodeType::FieldList { .. } => {
        match node_data {
          TreeNodeType::EmptyLine | TreeNodeType::FieldListItem { .. } => true,
          _ => false
        }
      }

      // Option lists may only contain empty lines or option list items
      TreeNodeType::OptionList { .. } => {
        match node_data {
          TreeNodeType::EmptyLine | TreeNodeType::OptionListItem { .. } => true,
          _ => false
        }
      }

      // Only paragraphs may contain inline nodes
      TreeNodeType::Paragraph => {
        match node_data {
          TreeNodeType::Emphasis { .. }             | TreeNodeType::StrongEmphasis { .. }         | TreeNodeType::InterpretedText
          | TreeNodeType::Literal { .. }            | TreeNodeType::InlineTarget { .. }           | TreeNodeType::FootnoteReference { .. }
          | TreeNodeType::CitationReference { .. }  | TreeNodeType::SubstitutionReference { .. }  | TreeNodeType::AbsoluteURI { .. }
          | TreeNodeType::StandaloneEmail { .. }    | TreeNodeType::Text { .. }                   | TreeNodeType::WhiteSpace { .. }
            => true,
          _ => false
        }
      },
      _ => false
    }
  }


  /// ### child
  /// Returns a reference to a child node of a given index.
  /// Panics, if the child does not exist.
  pub fn child (&self, index: usize) -> &Self {
    match self.children.get(index) {
      Some(node) => node,
      None => panic!("No child at index {}.\nComputer says no...\n", index)
    }
  }

  /// ### get_data_type
  /// For retrieving an immutable reference to the data type of a node.
  /// Mainly for printing purposes.
  pub fn get_data (&self) -> &TreeNodeType {
    &self.data
  }


  /// ### get_data_type
  /// For retrieving an immutable reference to the data type of a node.
  /// Mainly for printing purposes.
  pub fn get_mut_data (&mut self) -> &mut TreeNodeType {
    &mut self.data
  }
}


/// ### TreeNodeType
/// An enumaration of the different possible document
/// node types.
#[derive(Debug)]
pub enum TreeNodeType {

  /// #### Root
  /// The root node of an reStructuredText document tree.
  /// Contains the (name|absolute path) of the document
  /// as its only field.
  Root{
    doc_name: String
  },

  /// #### EmptyLine
  /// A simple empty line, that contains no actual data.
  /// These can be contained in pretty much any container
  /// node, such as lists or list items, in addition to
  /// existing between body level elements.
  EmptyLine,

  /// #### Section
  /// A section title node, that contains the title text,
  /// in addition to its marker type and (sub)section level.
  Section {

  },

  /// #### Transition
  /// A node corresponding to LaTeX's `\hrulefill` command.
  Transition {

  },

  // Body level elements

  /// #### Paragraph
  /// A node constructed of a left-aligned block of text
  /// with no special starter markers.
  Paragraph,

  /// #### BulletList
  /// An unnumbered list node. These may only contain `BulletListItem` nodes
  /// or `EmptyLine`s as their direct children.
  BulletList {
    bullet: char,
    bullet_indent: usize,
    text_indent: usize
  },

  /// #### BulletListItem
  /// An unnumbered list item. Cna contain any `Body` level elements
  /// as its direct children.
  BulletListItem{
    bullet: char,
    bullet_indent: usize,
    text_indent: usize
  },

  /// #### EnumeratedList
  /// An enumerated list node. Cna only contain `EnumeratedListItem` and `EmptyLine`
  /// nodes as its direct children.
  EnumeratedList {
    delims: EnumDelims,
    kind: EnumKind,
    start_index: usize,
    n_of_items: usize,
    enumerator_indent: usize,
  },

  /// #### EnumeratedListItem
  /// Child node type of `EnumeratedList`. Can contain any `Body`elements
  /// as its children.
  EnumeratedListItem {
    delims: EnumDelims,
    kind: EnumKind,
    index_in_list: usize,
    enumerator_indent: usize,
    text_indent: usize
  },

  /// #### DefinitionList
  /// A list of definitions. Contains `DefinitionListItems` or `EmptyLine` nodes
  /// as its direct children.
  DefinitionList,

  /// #### DefinitionListItem
  /// A child node type of `DefinitionList`.
  /// Contains a map of `DefinitionTerm`s and the corresponding
  /// `TermDefinitions`, in addition to optional term classifiers.
  DefinitionListItem,

  /// #### DefinitionTerm
  /// The term that is to be defined in a `DefinitionList`.
  DefinitionTerm,

  /// #### Classifier
  /// A classifier for a `DefinitionTerm` in a `DefinitionList`.
  /// Could be the type of a varible in a function decraration, or something similar.
  Classifier,

  /// #### TermDefinition
  /// A definition of a term in a `DefinitionList`.
  TermDefinition,

  /// #### FieldList
  /// A list of fields, that are used as a part of the
  /// reStructuredText extension syntax, such as directives.
  /// Bibliographies are a special case of these types of lists.
  FieldList {
    marker_indent: usize,
  },

  /// #### FieldListItem
  /// A field item of a `FieldList`. Consists of a marker with a field name and a
  /// field body consisting of arbitrary body elements.
  /// ```text
  /// +--------------------+----------------------+
  /// | ":" field name ":" | field body           |
  /// +-------+------------+                      |
  ///         | (body elements)+                  |
  ///         +-----------------------------------+
  /// ```
  FieldListItem {
    raw_marker_name: String,
    marker_name_as_inline_nodes: Vec<TreeNodeType>,
    marker_indent: usize,
    body_indent: usize,
  },

  /// #### FieldBody
  /// The parameter that `FieldName` refers to. May contain arbitrary body elements,
  /// just like bulleted and enumerated list items. The first line after the marker specifies
  /// the indentation used as a reference for parsing the rest of the block.
  FieldBody {
    indentation: usize
  },
  
  /// #### OptionList
  /// A two-column list of command line options, such as the ones typically seen on unix `man` pages.
  /// Four types of options are supported:
  ///
  /// 1. short POSIX options with one '-' and an opion letter,
  /// 2. Long POSIX options with "--", followed by an option word.
  ///    Some systems might use a single dash.
  /// 3. Old GNU-style options starting with a '+', followed by an option letter (!!!deprecated!!!)
  /// 4. DOS/VMS options starting with a '/', followed by an option letter or a word.
  /// 
  /// The recognized syntax is based on Python's `getopt.py` module.
  OptionList,

  /// #### OptionListItem
  /// A single option in an `OptionList`. Consists of an option,
  /// folllowed by and optional argument and a description.
  /// May contain arbitrary indented body elements after these:
  /// ```text
  /// +----------------------------+-------------+
  /// | option [" " argument] "  " | description |
  /// +-------+--------------------+             |
  ///         | (body elements)+                 |
  ///         +----------------------------------+
  /// ```
  OptionListItem,

  /// #### LiteralBlock
  /// Paragraph (possibly empty) ending in a "::" signifies the start of a literal block of text.
  /// Text contained in a literal block is not interpreted in any way,
  /// but simply stored in this node as is.
  LiteralBlock,

  /// #### DoctestBlock
  /// These are interactive Python sessions contained in Python docstrings.
  /// Based on the Python standard library [doctest](http://www.python.org/doc/current/lib/module-doctest.html) module.
  /// 
  /// Doctest blocks begin with ">>>", the python REPL main prompt and end with a blank line.
  /// They are a special case of the literal block and if both are present,
  /// the literal block takes precedence.
  DoctestBlock,
  
  /// #### MathBlock
  /// A node for display-style mathematics (LaTeX).
  MathBlock,

  /// #### LineBlock
  /// A block of text where each new line begins with an unindented '|',
  /// followed be text with specific left-alignment, used as a reference
  /// for the rest of the block.
  /// Allows writing blocks of text, where the struture of the lines
  /// is meaningful, such as poetry.
  /// 
  /// The symbols '|' may be omitted, as they signify the start of a new
  /// line in the rendered output.
  /// ```txt
  /// +------+-----------------------+
  /// | "| " | line                  |
  /// +------| continuation line     |
  ///        +-----------------------+
  /// ```
  LineBlock,

  /// #### Line
  /// A general line node. Might signify the start of a transtition or a section title.
  Line,
  
  /// #### BlockQuote
  /// Text indented relative to previous text,
  /// without markup indicating the start of
  /// a list or other container nodes.
  /// A block quote may end with an `Attribution`,
  /// which allows placing multiple block quotes
  /// in a sequence.
  BlockQuote,

  /// #### Attribution
  /// An optional attribution of a `BlockQuote`.
  /// If a `BlockQuote` contains an attribution,
  /// the following node may be a `BlockQuote as well,
  /// but not otherwise.
  Attribution,

  /// #### SubstitutionDefinition
  /// Explicit markup node, as in begins with ".. " followed by a vertical bar '|',
  /// substitution text and another '|'. The text may not begin or end with whitespace.
  /// Substitution definition blocks may contain a nested, *inline compatible* directive
  /// *without* the leading ".. ", such as `image` or `replace`.
  /// ```text
  /// +-------+-----------------------------------------------------+
  /// | ".. " | "|" substitution text "| " directive type "::" data |
  /// +-------+ directive block                                     |
  ///         |                                                     |
  ///         +-----------------------------------------------------+
  /// ```
  SubstitutionDefinition,

  /// #### Footnote
  /// A foonote citation target. Contains a label and the foornote text itself.
  Footnote {
    body_indent: usize,
    kind: FootnoteKind,
    label: String, // Displayed label
    target: String // Reference target
  },

  /// #### Citation
  /// A generic citation target.
  Citation {
    body_indent: usize,
    label: String,
  },

  /// #### ExternalHyperlinkTarget
  /// A target for an external hyperlink.
  /// Contains a URI pointing  to an external resource
  ExternalHyperlinkTarget {
    marker_indent: usize,
    target: String,
    uri: String,
  },

  /// #### IndirectHyperlinkTarget
  /// An indirect hyperlink target. Contains a hyperlink reference pointing
  /// to an internal or and external hyperlink.
  IndirectHyperlinkTarget {
    marker_indent: usize,
    target: String,
    indirect_target: String,
  },

  /// #### Directive
  /// One of many differents kinds of directives.
  Directive {
    dir_type: DirectiveType,
  },

  /// #### Comment
  /// An rST comment, that might get removed by the writer of the object code.
  Comment,


  // Inline elements
  // ---------------

  /// #### Text
  /// A plain text node, that contains no special markup.
  Text {
    text:String
  },

  /// #### Emphasis
  /// Emphasised or italicized text.
  Emphasis {
    text: String
  },

  /// #### StrongEmphasis
  /// Strongly emphasised text, usually rendered in bold.
  StrongEmphasis {
    text:String
  },

  /// #### InterpretedText
  /// Text, whose meaning depends entirely on the given `role`:
  /// (:role:`content`|`content`:role:). There are predefined roles
  /// such as `math` or `emphasis`, but others may be defined by applications.
  InterpretedText,

  /// #### Literal
  /// Literal text, usually reserved for code.
  Literal {
    text: String
  },

  /// #### InlineTarget
  /// An inline reference target.
  InlineTarget {
    target_label: String
  },

  /// #### Reference
  /// A general reference to a reference target.
  Reference {
    displayed_text: String,
    target_label: String
  },

  /// #### FootnoteReference
  /// A reference to a foot note.
  FootnoteReference {
    displayed_text: String,
    target_label: String
  },

  /// #### CitationReference
  /// A reference to a bibliographic citation.
  CitationReference {
    displayed_text: String,
    target_label: String
  },

  /// #### SubstitutionReference
  /// A reference that is to be substituted with the reference target directive output.
  SubstitutionReference {
    displayed_text: String,
    target_label: String
  },

  /// #### TitleReference
  /// A reference to a title.
  TitleReference {
    displayed_text: String,
    target_label: String
  },

  /// ### AbsoluteURI
  /// A reference to a web address.
  AbsoluteURI{
    text: String
  },

  /// #### StandaloneEmail
  /// A reference to an email address.
  StandaloneEmail{
    text: String
  },

  /// #### Math
  /// An inline math node.
  Math {
    text: String
  },

  /// #### Whitespace
  /// Generic whitespace that covers everything from spaces to newlines.
  WhiteSpace{
    text: String
  },

}


/// ### Children
/// Shorthand for a vector of owned child nodes.
/// Empty vector indicates no children.
type Children = Vec<TreeNode>;
