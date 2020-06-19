/// This is the test module for the DocTree struct.

use super::*;

#[cfg(test)]


#[test]
fn new_doctree() {

  let dt = DocTree::new();

  let root_is_root:bool = match dt.tree.node.data {
    TreeNodeType::Root => true,
    _ => false
  };

  assert!(
    root_is_root
  );

}