/// ## test_list_tables
/// 
/// A submodule for testing reStructuredText list tables.
/// 
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;

#[cfg(test)]

#[test]
fn list_table_01 () {

  let src = String::from("
.. list-table:: A title
   :widths: 2 2 2
   :width: 50 %
   :header-rows: 2
   :stub-columns: 1
   :align: center

   * - This is a cell in a table column
     - columns are represented by nested inner bullet list items

   * - This is the first column of another row
     - This is the second column.
   

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(PathBuf::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  if let TreeNodeType::ListTable { title, .. } = doctree.shared_child(1).shared_data() {
    assert_eq!(title.as_ref().unwrap().as_str(), "A title");
  } else {
    panic!()
  }
  if let TreeNodeType::BulletList { .. } = doctree.shared_child(1).shared_child(0).shared_data() {

  } else {
    panic!()
  }
  if let TreeNodeType::BulletListItem { .. } = doctree.shared_child(1).shared_child(0).shared_child(0).shared_data() {
    if let TreeNodeType::BulletList { .. } = doctree.shared_child(1).shared_child(0).shared_child(0).shared_child(0).shared_data() {
      if let TreeNodeType::BulletListItem { .. } = doctree.shared_child(1).shared_child(0).shared_child(0).shared_child(0).shared_child(0).shared_data() {
        // In table cell (1,1)
      } else {
        panic!()
      }
    } else {
      panic!()
    }
  } else {
    panic!()
  }


  if let TreeNodeType::BulletListItem { .. } = doctree.shared_child(1).shared_child(0).shared_child(1).shared_data() {
    // etc.
  } else {
    panic!()
  }
  todo!()
}