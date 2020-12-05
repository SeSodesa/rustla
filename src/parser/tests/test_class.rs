/// ## test_class
///
/// A submodule for thesting the parsing of the `class` directive.
///
/// Copyright (c) 2020, Santtu Söderholm <santtu.soderholm@tuni.fi>
use super::*;

#[cfg(test)]
#[test]
fn class_01() {
    let src = String::from(
        "
.. class:: class1 class2 class3

  This paragraph will receive the above class
  attributes during the transformtion stage of parsing.

  * So
  * Will
  * This
  * bullet list

This paragraph is no longer of classes 1, 2 and 3.

  ",
    )
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(src, doctree, None, 0, None, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    if let TreeNodeType::Class { classes, .. } = doctree.shared_child(0).shared_data() {
        assert_eq!(classes.join(" ").as_str(), "class1 class2 class3");

        if let TreeNodeType::Paragraph { .. } =
            doctree.shared_child(0).shared_child(0).shared_data()
        {
        } else {
            panic!()
        }
        if let TreeNodeType::BulletList { .. } =
            doctree.shared_child(0).shared_child(1).shared_data()
        {
        } else {
            panic!()
        }
    } else {
        panic!()
    }
    if let TreeNodeType::Paragraph { .. } = doctree.shared_child(1).shared_data() {
    } else {
        panic!()
    }
}
