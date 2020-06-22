/// A zipper module for accessing `TreeNode`s.
/// Inspired by https://stackoverflow.com/a/36168919/6449910

use super::*;

/// ### TreeZipper
/// A [zipper](https://en.wikipedia.org/wiki/Zipper_%28data_structure%29)
/// of `TreeNode`s. Makes it possible to traverse the tree and
/// access a specific child/parent in constant time.
pub struct TreeZipper {
  pub node: TreeNode,
  pub parent: Option<Box<TreeZipper>>,
  pub index_in_parent: Option<usize>,
}


impl TreeZipper {

  /// ### new
  /// A `TreeZipper` constructor. A new `TreeZipper`
  /// consists of nothing but the root node.
  pub fn new(node: TreeNode) -> Self {

    Self {
      node: node,
      parent: None,
      index_in_parent: None,
    }

  }

  /// ### focus_on_child
  /// Moves focus to a specific child of a node.
  /// Returns `Ok(TreeZipper)` focused
  /// on the child, if successful. Otherwise
  /// returns with `Err(message: &str)`
  fn focus_on_child (mut self, index: usize) -> Result<Self, &'static str> {

    let child: TreeNode;

    if !self.node.children.is_empty() && !index >= self.node.children.len() {

      child = self.node.children.swap_remove(index);

    } else {

      return Err("This node has less children than the given index implies!\n");

    }

    Ok(
      Self {
        node: child,
        parent: Some(Box::new(self)),
        index_in_parent: Some(index),
      }
    )

  }


  /// ### focus_on_parent
  /// Moves focus to the parent of the current node,
  /// or at least tries to. Returns with `Ok(TreeZipper)`
  /// if successful and `Err(message: &str)` if not.
  fn focus_on_parent(self) -> Result<Self, &'static str> {

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


  /// ### focus_on_last_child
  /// Moves the focus to the last child of the current focus.
  pub fn focus_on_last_child (self) -> Result<Self, &'static str> {

    let children_len = self.node.children.len();

    let with_focus_on_latest_child = match self.focus_on_child(children_len - 1) {
      Ok(tree_zipper) => tree_zipper,
      Err(e) => {
        eprintln!("{}", e);
        return Err("Couldn't access last child.")
      }
    };

    Ok(with_focus_on_latest_child)

  }


  /// ### focus_on_sibling
  /// Moves focus to the given nth sibling.
  pub fn focus_on_sibling (self, sibling_index: usize) -> Result<Self, &'static str> {

    let parent = if let Some(parent) = &self.parent {
      match self.focus_on_parent() {
        Ok(parent) => parent,
        Err(e) => {
          eprintln!("{}", e);
          return Err("Could not focus on sibling because of missing parent.")
        }
      }
    } else {
      return Err("Parent missing...\n")
    };

    let sibling = match parent.focus_on_child(sibling_index) {
      Ok(child) => child,
      Err(e) => {
        eprintln!("{}", e);
        return Err("Could not access child\n")
      }
    };

    Ok(sibling)

  }

}
