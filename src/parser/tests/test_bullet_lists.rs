/// ## test_bullet_lists
/// A submodule for tests related to bullet lists.
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;
use std::path::PathBuf;

#[cfg(test)]


#[test]
fn bullet_list_01 () {

  let src = String::from("
- This is the first bullet list item.").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(PathBuf::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  match doctree.shared_child(1).shared_data() {
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
    
  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(PathBuf::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  match doctree.shared_child(1).shared_child(0).shared_child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!("First non-whitespace child of ListItem wasn't a paragraph!\n")
  }

  match doctree.shared_child(1).shared_child(0).shared_child(2).shared_data() {
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
    
  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(PathBuf::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  match doctree.shared_child(1).shared_child(0).shared_data() {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("First child of BulletList wasn't a ListItem!\n")
  }

  match doctree.shared_child(1).shared_child(1).shared_data() {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("Second child of BulletList wasn't a ListItem!\n")
  }

  match doctree.shared_child(1).shared_child(0).shared_child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!("First non-whitespace child of ListItem wasn't a paragraph!\n")
  }

  match doctree.shared_child(1).shared_child(0).shared_child(2).shared_data() {
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
    
  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(PathBuf::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.shared_child(1).shared_child(0).shared_data() {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("First child of BulletList wasn't a ListItem!\n")
  }

  match doctree.shared_child(1).shared_child(1).shared_data() {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("Second child of BulletList wasn't a ListItem!\n")
  }

  match doctree.shared_child(1).shared_child(0).shared_child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!("First non-whitespace child of ListItem wasn't a paragraph!\n")
  }

  match doctree.shared_child(1).shared_child(0).shared_child(2).shared_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!("Third non-whitespace child of ListItem wasn't a paragraph!\n")
  }

  match doctree.shared_child(2).shared_data() {
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
    
  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(PathBuf::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();

  doctree.print_tree();

  match doctree.shared_child(1).shared_child(0).shared_data() {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("First child of BulletList wasn't a ListItem!\n")
  }

  match doctree.shared_child(1).shared_child(0).shared_child(4).shared_data() {
    TreeNodeType::BulletList{..} => (),
    _ => panic!("Second child of BulletList wasn't a sublist!\n")
  }

  match doctree.shared_child(1).shared_child(1).shared_data() {
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
    
  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(PathBuf::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.shared_child(1).shared_child(0).shared_data() {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("First non-whitespace child of BulletList wasn't a ListItem!\n")
  }

  match doctree.shared_child(1).shared_child(0).shared_child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!("No Paragraph!\n")
  }

  match doctree.shared_child(1).shared_child(0).shared_child(4).shared_data() {
    TreeNodeType::BulletList { .. } => (),
    _ => panic!("No BulletList!\n")
  }

  match doctree.shared_child(1).shared_child(0).shared_child(4).shared_child(0).shared_data() {
    TreeNodeType::BulletListItem { .. } => (),
    _ => panic!("No BulletListItem!\n")
  }

  match doctree.shared_child(1).shared_child(0).shared_child(4).shared_child(1).shared_data() {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("No BulletListItem!\n")
  }

  match doctree.shared_child(1).shared_child(0).shared_child(4).shared_child(1).shared_child(2).shared_data() {
    TreeNodeType::BulletList{..} => (),
    _ => panic!("No BulletListItem!\n")
  }
  
  match doctree.shared_child(1).shared_child(1).shared_data() {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("Second non-whitespace child of BulletList wasn't a BulletList!\n")
  }

}
