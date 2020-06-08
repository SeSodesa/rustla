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


/// ### Document
/// A node type representing to document root.
/// In addition to containing its direct children
/// (body elements),
/// `Document` is responsible for keeping track of
/// reference and citation nodes.
#[derive(Debug)]
pub struct Document <T: Node> {

  /// #### id
  /// A copy of the global node counter.
  /// Fixed when calling `Document::new`.
  id: usize,

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


impl <T: Node> Node for Document <T> {

  type ID = usize;
  type Parent = Parent<T>;

}


impl <T: Node> BranchNode <T> for Document <T> {

  type Children = Children<T>;

  /// ### new
  /// The `Document` constructor. Every value is either empty
  /// or 0 in the beginning.
  fn new(_node_id: &mut NodeId) -> Self {

    let mut id_counter = NodeId::new();

    Document {
      id: id_counter.assign(),
      id_counter: id_counter,
      parent: None,
      children: Vec::new(),
      src_line: 0,
      indirect_target_nodes: Vec::new(),
      substitution_defs: HashMap::new(),
      substitution_names: HashMap::new(),
      refs_to_nodes: HashMap::new(),
      ids_to_nodes: HashMap::new(),
      names_to_ids: HashMap::new(),
    }
  }


  fn walk(&self) {
    unimplemented!("fn walk not yet implemented!");
  }


  fn mut_walk(&mut self) {
      unimplemented!("fn mut_walk not implemented!");
  }


  fn add_child(&mut self, child: Rc<RefCell<T>>) {
      self.children.push(child);
  }

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
