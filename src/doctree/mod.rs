/// This module defines the document tree and its nodes

use std::rc::{Rc, Weak};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

mod tests;

mod tree_zipper;
  use tree_zipper::TreeZipper;

mod node_types;
pub mod structural_nodes;
pub mod body_nodes;
pub mod inline_nodes;
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

    let zipper = TreeZipper::new(root_node, None, None);

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
  pub children: Children,
  pub data : TreeNodeType

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
  pub fn push_child (&mut self, node : TreeNode) {

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
#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
pub enum TreeNodeType {

  // DocTree root node
  Root(Root),

  // Structural elements
  Section(structural_nodes::Section),
  Topic(structural_nodes::Topic),
  Transition(structural_nodes::Transition),

  // Body level elements
  Paragraph(body_nodes::Paragraph),
  Compound(body_nodes::Compound),
  Container(body_nodes::Container),
  BulletList(body_nodes::BulletList),
  EnumeratedList(body_nodes::EnumeratedList),
  ListItem(body_nodes::ListItem),
  DefinitionList(body_nodes::DefinitionList),
  DefinitionListItem(body_nodes::DefinitionListItem),
  Term(body_nodes::Term),
  Classifier(body_nodes::Classifier),
  Definition(body_nodes::Definition),
  FieldList(body_nodes::FieldList),
  Field(body_nodes::Field),
  FieldName(body_nodes::FieldName),
  FieldBody(body_nodes::FieldBody),
  Option(body_nodes::Option),
  OptionArgument(body_nodes::OptionArgument),
  OptionGroup(body_nodes::OptionGroup),
  OptionList(body_nodes::OptionList),
  OptionListItem(body_nodes::OptionListItem),
  OptionString(body_nodes::OptionString),
  Description(body_nodes::Description),
  LiteralBlock(body_nodes::LiteralBlock),
  DoctestBlock(body_nodes::DoctestBlock),
  MathBlock(body_nodes::MathBlock),
  LineBlock(body_nodes::LineBlock),
  Line(body_nodes::Line),
  BlockQuote(body_nodes::BlockQuote),
  Attribution(body_nodes::Attribution),
  Attention(body_nodes::Attention),
  Caution(body_nodes::Caution),
  Danger(body_nodes::Danger),
  Error(body_nodes::Error),
  Important(body_nodes::Important),
  Note(body_nodes::Note),
  Tip(body_nodes::Tip),
  Hint(body_nodes::Hint),
  Warning(body_nodes::Warning),
  Admonition(body_nodes::Admonition),
  Comment(body_nodes::Comment),
  SubstitutionDefinition(body_nodes::SubstitutionDefinition),
  Target(body_nodes::Target),
  Footnote(body_nodes::Footnote),
  Citation(body_nodes::Citation),
  Label(body_nodes::Label),
  Figure(body_nodes::Figure),
  Caption(body_nodes::Caption),
  Legend(body_nodes::Legend),
  Table(body_nodes::Table),
  TableGroup(body_nodes::TableGroup),
  ColSpec(body_nodes::ColSpec),
  TableHead(body_nodes::TableHead),
  TableBody(body_nodes::TableBody),
  TableRow(body_nodes::TableRow),
  TableEntry(body_nodes::TableEntry),

  // Inline elements
  Text(inline_nodes::Text),
  Emphasis(inline_nodes::Emphasis),
  StrongEmphasis(inline_nodes::StrongEmphasis),
  Literal(inline_nodes::Literal),
  InlineTarget(inline_nodes::InlineTarget),
  Reference(inline_nodes::Reference),
  FootnoteReference(inline_nodes::FootnoteReference),
  CitationReference(inline_nodes::CitationReference),
  SubstitutionReference(inline_nodes::SubstitutionReference),
  TitleReference(inline_nodes::TitleReference),
  AbsoluteURI(inline_nodes::AbsoluteURI),
  StandaloneEmail(inline_nodes::StandaloneEmail),
  Abbreviation(inline_nodes::Abbreviation),
  Acronym(inline_nodes::Acronym),
  SuperScript(inline_nodes::SuperScript),
  SubScript(inline_nodes::SubScript),
  Math(inline_nodes::Math),
  Image(inline_nodes::Image),
  Inline(inline_nodes::Inline),
  Problematic(inline_nodes::Problematic),
  Generated(inline_nodes::Generated),
  WhiteSpace(inline_nodes::WhiteSpace),

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
