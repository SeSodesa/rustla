/// ## test_admonitions
/// A submodule for admonition unit tests

use super::*;


#[cfg(test)]


#[test]
fn admonition_01 () {
  let src = String::from("
  .. note:: This is a note admonition.
   This is the second line of the first paragraph.

  .. warning::
     This is another admonition.
     This is the second line of the first paragraph.

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  todo!()
}
