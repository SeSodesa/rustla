/// This submodule contains the trait definitions
/// needed by the document tree.

use super::*;

/// ### trait Node
/// A trait defining functionality for general document tree nodes,
/// that every node must implement at the very least.
pub trait Node {

  type ID;

}

/// ### trait BranchNode
/// A subtrait of the general `Node` trait.
/// Document `Element`s are nodes that contain children.
pub trait BranchNode<T: Node>: Node {

  type Children;
  type Parent;

  /// ### new
  /// A `BranchNode` constructor.
  /// Branch nodes have children, so at the very least an empty
  /// vector of children has to be intitialized.
  fn new(id: &mut NodeId) -> Self;

  /// ### walk
  /// Immutably visits each node in a tree.
  /// Mainly for reading or printing purposes.
  fn walk(&self);

  /// ### mut_walk
  /// Same as `walk`, but for making modifications
  /// to nodes in a tree.
  fn mut_walk(&mut self);

  /// ### add_child
  /// Pushes a new child node to a the child node vector of a node.
  fn add_child(&mut self, child: Rc<RefCell<T>>);

}

pub trait LeafNode: Node {

  type Parent;

  /// ### new
  /// A leaf node constructor.
  fn new (ID: &mut NodeId) -> Self;

}

/// ### trait InlineBranchNode
/// Functionality needed by nodes that contain only
/// nodes representing inline text elements.
pub trait InlineBranchNode<T: Node>: BranchNode <T> {

}

/// ### trait TextNode
/// Functionality for inline elements.
/// These don't have children and therefore
/// implement `Node` and not `BranchNode`.
pub trait TextNode: Node {

}