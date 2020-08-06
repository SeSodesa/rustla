/// ## test_mixed_structures
/// A submodule for tests related to bullet lists.

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

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.child(1).get_data() {
    TreeNodeType::EnumeratedList { .. } => (),
    _ => panic!()
  }

  match doctree.child(1).child(0).get_data() {
    TreeNodeType::EnumeratedListItem { .. } => (),
    _ => panic!()
  }

  match doctree.child(1).child(0).child(0).get_data() {
    TreeNodeType::BulletList { .. } => (),
    _ => panic!()
  }

  match doctree.child(1).child(0).child(0).child(0).get_data() {
    TreeNodeType::BulletListItem { .. } => (),
    _ => panic!()
  }

  match doctree.child(1).child(0).child(0).child(1).get_data() {
    TreeNodeType::BulletListItem { .. } => (),
    _ => panic!()
  }

  match doctree.child(1).child(0).child(0).child(1).child(2).get_data() {
    TreeNodeType::EnumeratedList { .. } => (),
    _ => panic!()
  }

  match doctree.child(1).child(0).child(1).get_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!()
  }

  match doctree.child(1).child(1).get_data() {
    TreeNodeType::EnumeratedListItem { .. } => (),
    _ => panic!()
  }
 
}
