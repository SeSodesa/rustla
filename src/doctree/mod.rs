/// This module defines the document tree and its nodes

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;

mod node_types;
mod traits;
mod structural;
mod body;
mod inline;
use self::node_types::BranchNodeType;

use self::traits::{Node, BranchNode, InlineBranchNode, TextNode};


/// ### DocTree
/// A container for the document tree.
/// In addition to holding ownership of the
/// root of the tree, holds metadata related to
/// the state of the tree.
pub struct DocTree <T: Node> {

  /// #### tree_root
  /// Holds on to the tree root node,
  /// providing access to the rest of the tree.
  tree_root: DocNode,

  /// ####  id_counter
  /// Keeps track of node ids.
  /// Knows how to yield a acopy of the value within,
  /// incrementing it by one. This should be
  /// called when a new node is created.
  id_counter: NodeId,

  /// #### parent
  /// The document has no parent node.
  parent: Parent<T>,

  /// #### children
  children: Children<T>,

  /// #### src_line
  /// The row currently under inspection by the parser.
  src_line: usize,

  /// #### indirect_target_nodes
  /// A vector of indirect target nodes.
  indirect_target_nodes: NodeRefVec<T>,

  /// #### substitutiton_defs
  /// A map of substitution names to nodes containing substitution definitions.
  substitution_defs: HashMap<String, T>,

  /// #### substitution_names
  /// A mapping of case-normalized substitution names to the original names.
  substitution_names: HashMap<String, String>,

  /// #### refs_to_nodes
  /// A mapping of reference names to reference nodes.
  refs_to_nodes: HashMap<String, NodeRefVec<T>>,

  /// #### ids_to_nodes
  /// A mapping of ids to vectors of reference nodes.
  ids_to_nodes: HashMap<usize, NodeRefVec<T>>,

  /// #### names_to_ids
  /// A mapping of node names to their unique ids.
  names_to_ids: HashMap<String, usize>,

}

/// ### DocNode
/// An enumaration of the different possible document
/// node types.
pub enum DocNode {

  // DocTree root node
  Root,

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
  Emphasis,                StrongEmphasis,
  Literal,                 Reference,
  FootnoteReference,       CitationReference,
  SubstitutionReference,   TitleReference,
  Abbreviation,            Acronym,
  SuperScript,             SubScript,
  Math,                    Image,
  Inline,                  Problematic,
  Generated

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
  /// Returna copy of the NodeId counter.NodeId
  pub fn assign (&mut self) -> usize{
    let current = self.id;
    self.increment();
    current
  }

}

/// ### Parent
/// A shorthand for an optional (parent might not exist)
/// weak reference to a parent node.
type Parent <T> = Option< Weak<RefCell<T>>>;

/// ### Children
/// Shorthand for a vector of owned child nodes.
/// Empty vector indicates no children.
type Children <T> = Vec<Rc<RefCell<T>>>;


/// ### NodeRefVec
/// A vector of weak pointers to internally mutable nodes.
type NodeRefVec <T> = Vec<Weak<RefCell<T>>>;
