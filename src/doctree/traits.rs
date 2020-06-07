/// This submodule contains the trait definitions
/// needed by the document tree.


/// ### trait Node
/// A trait defining functionality for general document tree nodes.
/// A `Node` instance has to know how to clone itself via the `Clone` trait.
pub trait Node: Clone {

  type Parent;

  /// ### new
  /// A `Node` constructor.
  fn new() -> Self;
 
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
  fn add_child(&mut self);

}

/// ### trait BranchNode
/// A subtrait of the general `Node` trait.
/// Document `Element`s are nodes that contain children.
pub trait BranchNode: Node {

  type Children;

  /// ### new
  /// A `BranchNode` constructor.
  /// Branch nodes have children, so at the very least an empty
  /// vector of children has to be intitialized.
  fn new() -> Self;

}

/// ### trait InlineBranchNode
/// Functionality needed by nodes that contain only
/// nodes representing inline text elements.
pub trait InlineBranchNode: BranchNode {

}

/// ### trait TextNode
/// Functionality for inline elements.
/// These don't have children and therefore
/// implement `Node` and not `BranchNode`.
pub trait TextNode: Node {

}