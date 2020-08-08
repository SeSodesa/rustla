/// ## test_bullet_lists
/// A submodule for tests related to bullet lists.

use super::*;

#[cfg(test)]


#[test]
fn bullet_list_01 () {

  let src = String::from("
- This is the first bullet list item.");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  match doctree.child(1).get_data() {
    TreeNodeType::BulletList{..}=> (),
    _ => panic!("No bullet list node where one was expected!\n")
  }

}


#[test]
fn bullet_list_02 () {

  let src = String::from("
  - List item 1

    Second paragraph of the list item.
    
    Third paragraph of this list item...
    
  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  match doctree.child(1).child(0).child(0).get_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!("First non-whitespace child of ListItem wasn't a paragraph!\n")
  }

  match doctree.child(1).child(0).child(2).get_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!("Third non-whitespace child of ListItem wasn't a paragraph!\n")
  }
}


#[test]
fn bullet_list_03 () {

  let src = String::from("
  - List item 1

    Second paragraph of the list item.

  - List item 2
    
  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  match doctree.child(1).child(0).get_data() {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("First child of BulletList wasn't a ListItem!\n")
  }

  match doctree.child(1).child(1).get_data() {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("Second child of BulletList wasn't a ListItem!\n")
  }

  match doctree.child(1).child(0).child(0).get_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!("First non-whitespace child of ListItem wasn't a paragraph!\n")
  }

  match doctree.child(1).child(0).child(2).get_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!("Third non-whitespace child of ListItem wasn't a paragraph!\n")
  }

}



#[test]
fn bullet_list_04 () {

  let src = String::from("
  - List item 1

    Second paragraph of the list item.

  - List item 2

  asfasdfdsfasfasdfasfd
    
  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.child(1).child(0).get_data() {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("First child of BulletList wasn't a ListItem!\n")
  }

  match doctree.child(1).child(1).get_data() {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("Second child of BulletList wasn't a ListItem!\n")
  }

  match doctree.child(1).child(0).child(0).get_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!("First non-whitespace child of ListItem wasn't a paragraph!\n")
  }

  match doctree.child(1).child(0).child(2).get_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!("Third non-whitespace child of ListItem wasn't a paragraph!\n")
  }

  match doctree.child(2).get_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!("No empty line after bullet list!\n")
  }

}



#[test]
fn bullet_list_05 () {

  let src = String::from("
  - List item 1

    Second paragraph of the list item.

    - Sublist item 1

    - Sublist item 2

  - List item 2

  asfasdfdsfasfasdfasfd
    
  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();

  doctree.print_tree();

  match doctree.child(1).child(0).get_data() {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("First child of BulletList wasn't a ListItem!\n")
  }

  match doctree.child(1).child(0).child(4).get_data() {
    TreeNodeType::BulletList{..} => (),
    _ => panic!("Second child of BulletList wasn't a sublist!\n")
  }

  match doctree.child(1).child(1).get_data() {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("Third child of BulletList wasn't a ListItem!\n")
  }

}



#[test]
fn bullet_list_06 () {

  let src = String::from("
  + List item 1

    Second paragraph of the list item.

    - Sublist item 1

    - Sublist item 2

      * Subsublist item 1

      * Subsublist item 2

    - Sublist item 3

  + List item 2

  asfasdfdsfasfasdfasfd
    
  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.child(1).child(0).get_data() {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("First non-whitespace child of BulletList wasn't a ListItem!\n")
  }

  match doctree.child(1).child(0).child(0).get_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!("No Paragraph!\n")
  }

  match doctree.child(1).child(0).child(4).get_data() {
    TreeNodeType::BulletList { .. } => (),
    _ => panic!("No BulletList!\n")
  }

  match doctree.child(1).child(0).child(4).child(0).get_data() {
    TreeNodeType::BulletListItem { .. } => (),
    _ => panic!("No BulletListItem!\n")
  }

  match doctree.child(1).child(0).child(4).child(1).get_data() {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("No BulletListItem!\n")
  }

  match doctree.child(1).child(0).child(4).child(1).child(2).get_data() {
    TreeNodeType::BulletList{..} => (),
    _ => panic!("No BulletListItem!\n")
  }
  
  match doctree.child(1).child(1).get_data() {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("Second non-whitespace child of BulletList wasn't a BulletList!\n")
  }

}
