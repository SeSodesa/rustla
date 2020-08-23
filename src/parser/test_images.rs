/// ## test_images
/// A submodule for unit tests related to images.
/// 
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;

#[cfg(test)]


#[test]
fn image_01 () {
  let src = String::from("
  .. image::
    This is a generic admonition, the argument of which starts on
    the line following the directive marker.
    :class: options start here
    :name: here is a reference name
    :unrecognized: This option is discarded by the parsing function.

    The admonition contents start here,
    with a single paragraph.

    - followed by
    - a bullet list

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();
  todo!()
}