/// ## test_mixed_structures
///
/// A submodule for tests related to mixed document structures.
///
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;

#[cfg(test)]


#[test]
fn mixed_nested_lists_01 () {

  let src = String::from("
(i) * List item 1
      of a nested bullet list within
      an enumerated list...

    * Nested list item 2

      b) Nested enuemrated list in a nested bullet list

    Second paragraph of list item i.

(ii) List item 2 of the parent list.

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(PathBuf::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  match doctree.shared_child(0).shared_data() {
    TreeNodeType::EnumeratedList { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(0).shared_child(0).shared_data() {
    TreeNodeType::EnumeratedListItem { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(0).shared_child(0).shared_child(0).shared_data() {
    TreeNodeType::BulletList { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(0).shared_child(0).shared_child(0).shared_child(0).shared_data() {
    TreeNodeType::BulletListItem { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(0).shared_child(0).shared_child(0).shared_child(1).shared_data() {
    TreeNodeType::BulletListItem { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(0).shared_child(0).shared_child(0).shared_child(1).shared_child(1).shared_data() {
    TreeNodeType::EnumeratedList { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(0).shared_child(0).shared_child(1).shared_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(0).shared_child(1).shared_data() {
    TreeNodeType::EnumeratedListItem { .. } => (),
    _ => panic!()
  }
}
