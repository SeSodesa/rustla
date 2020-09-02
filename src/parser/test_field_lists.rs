/// ## test_field_lists
/// A submodule for tests related to field lists.
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;

#[cfg(test)]


#[test]
fn field_list_01 () {

  let src = String::from("
:field marker 1: Marker body
  with a line indented relative to it

:field marker 2: Body with
    some more indentation
    and a third line as well

    * and
    * why
    * not
    * a list
    * as well

An ending paragraph...

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.shared_child(1).shared_data() {
    TreeNodeType::FieldList { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(0).shared_data() {
    TreeNodeType::FieldListItem { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(0).shared_child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(1).shared_data() {
    TreeNodeType::FieldListItem { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(1).shared_child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(1).shared_child(2).shared_data() {
    TreeNodeType::BulletList { .. } => (),
    _ => panic!()
  }

}


#[test]
fn field_list_02 () {

  let src = String::from("
:field marker 1: Marker body
and a line with too little indentation


An ending paragraph...

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.shared_child(1).shared_data() {
    TreeNodeType::FieldList { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(2).shared_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(3).shared_data() {
    TreeNodeType::EmptyLine => (),
    _ => panic!()
  }

  match doctree.shared_child(4).shared_data() {
    TreeNodeType::EmptyLine => (),
    _ => panic!()
  }

  match doctree.shared_child(5).shared_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!()
  }
}


#[test]
fn field_list_03 () {

  let src = String::from("
:Date: 2001-08-16
:Version: 1
:Authors: - Me
          - Myself
          - I
:Indentation: Since the field marker may be quite long, the second
    and subsequent lines of the field body do not have to line up
    with the first line, but they must be indented relative to the
    field name marker, and they must line up with each other.
:Parameter i: integer

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();


  match doctree.shared_child(1).shared_data() {
    TreeNodeType::FieldList { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(0).shared_data() {
    TreeNodeType::FieldListItem { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(0).shared_child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(1).shared_data() {
    TreeNodeType::FieldListItem { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(1).shared_child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(2).shared_data() {
    TreeNodeType::FieldListItem { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(2).shared_child(0).shared_data() {
    TreeNodeType::BulletList { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(3).shared_data() {
    TreeNodeType::FieldListItem { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(3).shared_child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(4).shared_data() {
    TreeNodeType::FieldListItem { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(4).shared_child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!()
  }
}
