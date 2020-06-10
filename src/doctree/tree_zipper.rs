/// A zipper module for accessing `TreeNode`s.

use super::*;

/// ### TreeZipper
/// A [zipper](https://en.wikipedia.org/wiki/Zipper_%28data_structure%29)
/// of `TreeNode`s. Makes it possible to traverse the tree and
/// locate a specific child/parent in constant time.
pub struct TreeZipper {
  node: TreeNode,
  parent: Option<Box<TreeZipper>>,
  index_in_parent: Option<usize>,
}


impl TreeZipper {

  /// ### new
  /// A `TreeZipper` constructor.
  fn new(node: TreeNode, parent: Option<Box<TreeZipper>>, index_in_parent: usize) -> Self {

    Self {
      node: node,
      parent: parent,
      index_in_parent: Some(index_in_parent),
    }

  }

  /// ### get_child
  /// Provides access to a child with a specific index in the
  /// parent node `children` vector. Returns and *owned* instance of
  /// the child that later needs to be inserted back into the parent
  /// node via `assign_parent`. This combination maintains the order
  /// of the children.
  fn get_child (mut self, index: usize) -> Result<Self, &'static str> {

    let child: TreeNode;

    if !self.node.children.is_empty() {
      child = self.node.children.swap_remove(index);
    } else {
      return Err("This node has no children!");
    }

    

    Ok(
      Self {
        node: child,
        parent: Some(Box::new(self)),
        index_in_parent: Some(index),
      }
    )

  }

  /// ### assign_parent
  /// The opposite operation to `get_child`.
  /// Inserts the child returned by it back into the
  /// parent `TreeZipper` node `children`,
  /// maintaining order (assuming `Vec::swap_remove` was used).
  fn assign_parent(self) -> Result<Self, &'static str> {

    // Destructuring the provided TreeZipper
    let Self { node, parent, index_in_parent } = self;

    // Destructuring the parent provided by the above destructure
    let Self {
      node: mut parent_node,
      parent: parent_parent,
      index_in_parent: parent_index_in_parent,
    } = match parent {
      Some(parent) => *parent,
      None => return Err("This node has no parent!")
    };

    let index = match index_in_parent {
      Some(index) => index,
      None => return Err("No index in parent!")
    };

    // Perform the opposite of Vec::swap_remove
    parent_node.children.push(node);
    let len = parent_node.children.len();
    parent_node.children.swap(index, len - 1);

    Ok(
      Self {
        node:parent_node,
        parent: parent_parent,
        index_in_parent: parent_index_in_parent,
      }
    )

  }

}


