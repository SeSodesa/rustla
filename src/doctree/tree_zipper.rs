/// A zipper module for accessing `TreeNode`s.
/// Inspired by https://stackoverflow.com/a/36168919/6449910

use super::*;

/// ### TreeZipper
/// A [zipper](https://en.wikipedia.org/wiki/Zipper_%28data_structure%29)
/// of `TreeNode`s. Makes it possible to traverse the tree and
/// access a specific child/parent in constant time.
#[derive(Debug)]
pub struct TreeZipper {
  pub node: TreeNode,
  pub parent: Option<Box<TreeZipper>>,
  pub index_in_parent: Option<usize>,
}


impl TreeZipper {

  /// ### new
  /// A `TreeZipper` constructor. A new `TreeZipper`
  /// consists of nothing but the root node.
  pub fn new(node: TreeNode, parent: Option<Box<TreeZipper>>, index_in_parent: Option<usize>) -> Self {

    Self {
      node: node,
      parent: parent,
      index_in_parent: index_in_parent,
    }

  }

  /// ### push child
  /// Adds a child node to the contained node.
  pub fn push_child (&mut self, tree_node: TreeNode) {
    self.node.push_child(tree_node)
  }


  /// ### append_children
  /// Adds a sequence of children to `self.node.children`.
  pub fn append_children (&mut self, children: &mut Vec<TreeNode>) {
    self.node.append_children(children);
  }


  /// ### focus_on_child
  /// Moves focus to a specific child of a node.
  /// Returns `Ok(TreeZipper)` focused
  /// on the child, if successful. Otherwise
  /// returns with `Err(message: &str)`
  pub fn focus_on_child (mut self, index: usize) -> Result<Self, Self> {

    let child: TreeNode;

    if !self.node.children.is_empty() && !index >= self.node.children.len() {

      child = self.node.children.swap_remove(index);

    } else {
      eprintln!("Child with given index does not exist.\nReturning parent...\n");
      return Err(self);

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
  pub fn focus_on_parent(self) -> Result<Self, Self> {

    // Destructuring the provided TreeZipper
    let Self { node, parent, index_in_parent } = self;

    // Destructuring the parent provided by the above destructure
    let Self {
      node: mut parent_node,
      parent: parent_parent,
      index_in_parent: parent_index_in_parent,
    } = match parent {
      Some(parent) => *parent,
      None => {
        eprintln!("No parent, returning unmodified zipper...\n");
        return Err(Self{node: node, parent: parent, index_in_parent: index_in_parent})
      }
    };

    let index = match index_in_parent {
      Some(index) => index,
      None => {
        eprintln!("Parent found but something funky going on with index in parent...\n");
        return Err(Self{node: node, parent: parent_parent, index_in_parent: index_in_parent})
      }
    };

    // Perform the opposite of Vec::swap_remove
    parent_node.children.push(node);
    let len = parent_node.children.len();
    parent_node.children.swap(index, len - 1);

    Ok(
      Self {
        node: parent_node,
        parent: parent_parent,
        index_in_parent: parent_index_in_parent,
      }
    )

  }


  /// ### walk_to_root
  /// A function that walks up the tree (zipper) until no more parents are encountered.
  pub fn walk_to_root (mut self) -> Self {

    loop {
      self = match self.focus_on_parent() {
        Ok(parent) => parent,
        Err(self_unchanged) => {
          self = self_unchanged;
          break
        }
      };
    };

    self
  }

  /// ### focus_on_last_child
  /// Moves the focus to the last child of the current focus.
  pub fn focus_on_last_child (self) -> Result<Self, Self> {

    let children_len = self.node.children.len();

    let with_focus_on_latest_child = match self.focus_on_child(children_len - 1) {
      Ok(tree_zipper) => tree_zipper,
      Err(parent) => {
        
        return Err(parent)
      }
    };

    Ok(with_focus_on_latest_child)

  }


  /// ### focus_on_sibling
  /// Moves focus to the given nth sibling.
  pub fn focus_on_sibling (self, sibling_index: usize) -> Result<Self, Self> {

    let parent = if let Some(parent) = &self.parent {
      match self.focus_on_parent() {
        Ok(parent) => parent,
        Err(unmodified_self) => {
          return Err(unmodified_self)
        }
      }
    } else {
      return Err(self)
    };

    let sibling = match parent.focus_on_child(sibling_index) {
      Ok(child) => child,
      Err(parent_itself) => {
        eprintln!("No such sibling.\nReturning parent...\n");
        return Err(parent_itself)
      }
    };

    Ok(sibling)

  }


  /// ### push_and_focus
  /// Given a variant `TreeNodeType`, constructs a TreeNode from the data,
  /// pushes it to current node's children and focuses on it.
  pub fn push_and_focus(mut self, node_data: TreeNodeType) -> Result<Self, Self> {

    let list_node = TreeNode::new(node_data);

    self.node.push_child(list_node);

    let node_result = match self.focus_on_last_child() {
      Ok(child_zipper) => Ok(child_zipper),
      Err(node_itself) => {
        eprintln!("Warning: Couldn't focus on lates child node.\nReturning node itself.\n");
        Err(node_itself)
      }
    };

    node_result
  }

}
