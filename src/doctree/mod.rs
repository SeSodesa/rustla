/// This module defines the document tree and its nodes

use std::rc::{Rc, Weak};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

mod tests;

mod tree_zipper;
  use tree_zipper::TreeZipper;

mod node_types;
pub mod structural;
pub mod body;
pub mod inline;
use self::node_types::BranchNodeType;


/// ### DocTree
/// A container for the document tree.
/// In addition to holding ownership of the
/// tree (stored in a zipper), also contains
/// metadata about the tree.
pub struct DocTree {

  /// #### tree
  /// Holds the tree focused on a specific node.
  pub tree: TreeZipper,

  /// #### src_line
  /// The row currently under inspection by the parser.
  src_line: usize,

  /// #### indirect_target_nodes
  /// A vector of indirect target nodes.
  indirect_target_nodes: NodeRefVec,

  /// #### substitutiton_defs
  /// A map of substitution names to nodes containing substitution definitions.
  substitution_defs: HashMap<String, TreeNodeType>,

  /// #### substitution_names
  /// A mapping of case-normalized substitution names to the original names.
  substitution_names: HashMap<String, String>,

  /// #### refs_to_nodes
  /// A mapping of reference names to reference nodes.
  refs_to_nodes: HashMap<String, NodeRefVec>,

  /// #### ids_to_nodes
  /// A mapping of ids to vectors of reference nodes.
  ids_to_nodes: HashMap<usize, NodeRefVec>,

  /// #### names_to_ids
  /// A mapping of node names to their unique ids.
  names_to_ids: HashMap<String, usize>,

}


/// ### DocTree
/// Document tree container methods
impl DocTree {

  /// ### new
  /// A `DocTree` constructor.
  pub fn new(doc_name: String) -> Self {

    let root_data = TreeNodeType::Root(
      Root::new(doc_name)
    );

    let root_node = TreeNode::new(root_data);

    let zipper = TreeZipper::new(root_node);

    DocTree {
      tree: zipper,
      src_line: 0,
      indirect_target_nodes: Vec::new(),
      substitution_defs: HashMap::new(),
      substitution_names: HashMap::new(),
      refs_to_nodes: HashMap::new(),
      ids_to_nodes: HashMap::new(),
      names_to_ids: HashMap::new(),
    }

  }

}


/// ### TreeNode
/// A tree node that contains a struct of `TreeNodeType`
/// plus the information needed to traverse the tree.
#[derive(Debug)]
pub struct TreeNode {
  children: Children,
  data : TreeNodeType

}

impl TreeNode {

  /// ### new
  /// A `TreeNode` constructor.
  pub fn new(data: TreeNodeType) -> Self {
    
    TreeNode {
      children: Vec::new(),
      data: data
    }

  }

  /// ### push_child
  /// Pushes a given child node the the end of `self.children`.
  fn push_child (&mut self, node : TreeNode) {

    self.children.push(node);

  }

  /// ### traverse
  /// Traverses `TreeNode`s recursively.
  fn traverse(&mut self) {

    eprintln!("Entering {:?}", self.get_data_type());

    let children = &mut self.children;

    for child in children {
      child.traverse();
    }

  }


  /// ### get_data_type
  /// For retrieving an immutable reference to the data type of a node.
  /// Mainly for printing purposes.
  fn get_data_type (&self) -> &TreeNodeType {
    &self.data
  }

}


/// ### Root
/// The root node data container of the parse tree.
#[derive(Debug)]
pub struct Root {
  doc_name: String
}

impl Root {

  fn new(doc_name: String) -> Self {

    Root {
      doc_name: doc_name
    }

  }

}

/// ### TreeNodeType
/// An enumaration of the different possible document
/// node types.
#[derive(Debug)]
pub enum TreeNodeType {

  // DocTree root node
  Root(Root),

  // Structural elements
  Section(structural::Section),
  Topic(structural::Topic),
  Transition(structural::Transition),

  // Body level elements
  Paragraph(body::Paragraph),
  Compound(body::Compound),
  Container(body::Container),
  BulletList(body::BulletList),
  EnumeratedList(body::EnumeratedList),
  ListItem(body::ListItem),
  DefinitionList(body::DefinitionList),
  DefinitionListItem(body::DefinitionListItem),
  Term(body::Term),
  Classifier(body::Classifier),
  Definition(body::Definition),
  FieldList(body::FieldList),
  Field(body::Field),
  FieldName(body::FieldName),
  FieldBody(body::FieldBody),
  Option(body::Option),
  OptionArgument(body::OptionArgument),
  OptionGroup(body::OptionGroup),
  OptionList(body::OptionList),
  OptionListItem(body::OptionListItem),
  OptionString(body::OptionString),
  Description(body::Description),
  LiteralBlock(body::LiteralBlock),
  DoctestBlock(body::DoctestBlock),
  MathBlock(body::MathBlock),
  LineBlock(body::LineBlock),
  Line(body::Line),
  BlockQuote(body::BlockQuote),
  Attribution(body::Attribution),
  Attention(body::Attention),
  Caution(body::Caution),
  Danger(body::Danger),
  Error(body::Error),
  Important(body::Important),
  Note(body::Note),
  Tip(body::Tip),
  Hint(body::Hint),
  Warning(body::Warning),
  Admonition(body::Admonition),
  Comment(body::Comment),
  SubstitutionDefinition(body::SubstitutionDefinition),
  Target(body::Target),
  Footnote(body::Footnote),
  Citation(body::Citation),
  Label(body::Label),
  Figure(body::Figure),
  Caption(body::Caption),
  Legend(body::Legend),
  Table(body::Table),
  TableGroup(body::TableGroup),
  ColSpec(body::ColSpec),
  TableHead(body::TableHead),
  TableBody(body::TableBody),
  TableRow(body::TableRow),
  TableEntry(body::TableEntry),

  // Inline elements
  Text(inline::Text),
  Emphasis(inline::Emphasis),
  StrongEmphasis(inline::StrongEmphasis),
  Literal(inline::Literal),
  Reference(inline::Reference),
  FootnoteReference(inline::FootnoteReference),
  CitationReference(inline::CitationReference),
  SubstitutionReference(inline::SubstitutionReference),
  TitleReference(inline::TitleReference),
  Abbreviation(inline::Abbreviation),
  Acronym(inline::Acronym),
  SuperScript(inline::SuperScript),
  SubScript(inline::SubScript),
  Math(inline::Math),
  Image(inline::Image),
  Inline(inline::Inline),
  Problematic(inline::Problematic),
  Generated(inline::Generated)

}

/// ### NodeId
/// A global counter of document nodes
#[derive(Debug)]
pub struct NodeId {
  id: usize
}

impl NodeId {

  /// ### new
  /// A NodeId constructor. In the beginning,
  /// there are 0 Nodes.
  pub fn new() -> Self {
    NodeId {
      id: 0
    }
  }

  /// ### increment
  /// Increments the `NodeId` counter by 1.
  pub fn increment(&mut self) {
    self.id += 1;
  }

  /// ### get
  /// Return a copy of the NodeId counter.NodeId
  pub fn assign (&mut self) -> usize{
    let current = self.id;
    self.increment();
    current
  }

}

/// ### Parent
/// A shorthand for an optional (parent might not exist)
/// weak reference to a parent node.
type Parent = Option< Weak<RefCell<TreeNode>>>;

/// ### Children
/// Shorthand for a vector of owned child nodes.
/// Empty vector indicates no children.
type Children = Vec<TreeNode>;


/// ### NodeRefVec
/// A vector of weak pointers to internally mutable nodes.
type NodeRefVec = Vec<Weak<RefCell<TreeNode>>>;
