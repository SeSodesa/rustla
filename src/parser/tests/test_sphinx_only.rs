/*!
A submodule for testing the Sphinx `only` directive.

Copyright © 2020 Santtu Söderholm
*/

use super::*;

#[cfg(test)]
#[test]
fn sphinx_only_01() {
    let src =
        r#"
.. only:: html and (latex or draft)

  This is a paragraph that is included in the output
  only if the directive argument is "true".

  * As is this bullet list...
  * ... with a second item

"#
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(src, doctree, 0, 0, State::Body, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    if let TreeNodeType::SphinxOnly { expression, .. } = doctree
        .shared_child(0).unwrap().shared_data() {
        assert_eq!(expression, r#"html and (latex or draft)"#);
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
}
