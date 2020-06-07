/// This module defines the document tree and its nodes


mod node_types;
use self::node_types::NodeType;

use std::rc::{Rc, Weak};
use std::cell::RefCell;



pub struct DocNode <T: Node> {
  parent: Parent<T>,
  children: Children<T>,
  types: Vec<NodeType>,
  data: String,
}

/// ### Node
/// A trait defining functionality for document tree nodes
pub trait Node {
  fn walk(&self);
}

/// ### Element
/// A subtrait of the general `Node` trait.
/// Document `Element`s are nodes.
pub trait Element: Node {

  fn add_child(&mut self);

}


/// ### Parent
/// A shorthand for an optional (parent might not exist)
/// weak reference to a parent node.
type Parent <T> = Option< Weak<RefCell<DocNode<T>>>>;

/// ### Children
/// Shorthand for a vector of owned child nodes.
/// Empty vector indicates no children.
type Children <T> = Vec<Rc<RefCell<DocNode<T>>>>;

