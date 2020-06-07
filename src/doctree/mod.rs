/// This module defines the document tree and its nodes

use std::rc::{Rc, Weak};
use std::cell::RefCell;

mod node_types;
mod traits;
mod structural;
mod body;
mod inline;
use self::node_types::BranchNodeType;

use self::traits::{Node, BranchNode, InlineBranchNode, TextNode};



pub struct DocNode <T: Node> {
  parent: Parent<T>,
  children: Children<T>,
  types: Vec<BranchNodeType>,
  data: String,
}


/// ### Parent
/// A shorthand for an optional (parent might not exist)
/// weak reference to a parent node.
type Parent <T: Node> = Option< Weak<RefCell<T>>>;

/// ### Children
/// Shorthand for a vector of owned child nodes.
/// Empty vector indicates no children.
type Children <T: Node> = Vec<Rc<RefCell<T>>>;
