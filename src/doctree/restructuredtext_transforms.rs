/// ## reStructuredText transforms
/// 
/// A submodule that defines the transforms performed for each node type,
/// before the tree is printed. These include things like transforming the
/// second child of a `Figure` node to a `Caption`, if it is a `Paragraph`.
/// 
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;

impl DocTree {

  /// Modifies `self.tree` with the known reStrucuturedText transforms.
  pub fn perform_restructuredtext_transforms (&mut self) {

    todo!()
  }
}

impl TreeZipper {

  /// Recursively modifies the data of `self.node` and its children,
  /// based on the node type `self.node.data`.
  pub fn perform_restructuredtext_transforms (self) -> Result<Self, Self> {

    todo!()
  }
}

impl TreeNode {

  /// Transforms `self.data` into a different type based on its current value.
  pub fn perform_restructuredtext_transforms (&mut self) {

    todo!()
  }
}
