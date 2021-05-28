/*!
A submodule for testing the class directive.

Copyright © 2020 Santtu Söderholm
*/

use super::*;

#[cfg(test)]
#[test]
fn class_01() {
    let src =
"
.. class:: class1 class2 class3

  This paragraph will receive the above class
  attributes during the transformtion stage of parsing.

  * So
  * Will
  * This
  * bullet list

This paragraph is no longer of classes 1, 2 and 3.

"
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(&src, doctree, 0, 0, State::Body, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    if let TreeNodeType::Class { classes, .. } = doctree
        .shared_child(0).unwrap().shared_data() {
        assert_eq!(classes.join(" ").as_str(), "class1 class2 class3");

        if let TreeNodeType::Paragraph { .. } =
            doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap().shared_data()
        {
        } else {
            panic!()
        }
        if let TreeNodeType::BulletList { .. } =
            doctree
        .shared_child(0).unwrap()
        .shared_child(1).unwrap().shared_data()
        {
        } else {
            panic!()
        }
    } else {
        panic!()
    }
    if let TreeNodeType::Paragraph { .. } = doctree
        .shared_child(1).unwrap().shared_data() {
    } else {
        panic!()
    }
}
