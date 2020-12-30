/*!
A submodule for testing the DocTree constructor function.

Copyright © 2020 Santtu Söderholm
*/
use super::*;

#[cfg(test)]
#[test]
fn new_doctree() {
    let doc_name = PathBuf::from("abc");

    let dt = DocTree::new(doc_name);

    let root_is_root: bool = match dt.shared_node().shared_data() {
        TreeNodeType::Document { .. } => true,
        _ => false,
    };

    assert!(root_is_root);
}
