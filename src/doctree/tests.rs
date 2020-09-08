/// ## tests
/// This is the test module for the DocTree struct.
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;
use std::path::PathBuf;

#[cfg(test)]


#[test]
fn new_doctree() {

  let doc_name = PathBuf::from("abc");

  let dt = DocTree::new(doc_name);

  let root_is_root:bool = match dt.tree.node.data {
    TreeNodeType::Document{..} => true,
    _ => false
  };

  assert!(
    root_is_root
  );

}
