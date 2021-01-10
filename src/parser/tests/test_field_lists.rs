/*!
A submodule for testing field lists.

Copyright © 2020 Santtu Söderholm
*/

use super::*;

#[cfg(test)]
#[test]
fn field_list_01() {
    let src =
"
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

"
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(src, doctree, 0, 0, State::Body, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    match doctree
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::FieldList { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::FieldListItem { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(1).unwrap().shared_data() {
        TreeNodeType::FieldListItem { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::BulletList { .. } => (),
        _ => panic!(),
    }
}

#[test]
fn field_list_02() {
    let src =
"
:field marker 1: Marker body
and a line with too little indentation


An ending paragraph...

"
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(src, doctree, 0, 0, State::Body, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    match doctree
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::FieldList { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(1).unwrap().shared_data() {
        TreeNodeType::Paragraph { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(2).unwrap().shared_data() {
        TreeNodeType::Paragraph { .. } => (),
        _ => panic!(),
    }
}

#[test]
fn field_list_03() {
    let src =
"
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

"
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(src, doctree, 0, 0, State::Body, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    match doctree
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::FieldList { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::FieldListItem { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(1).unwrap().shared_data() {
        TreeNodeType::FieldListItem { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(2).unwrap().shared_data() {
        TreeNodeType::FieldListItem { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(2).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::BulletList { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(3).unwrap().shared_data() {
        TreeNodeType::FieldListItem { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(3).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(4).unwrap().shared_data() {
        TreeNodeType::FieldListItem { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(4).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => (),
        _ => panic!(),
    }
}
