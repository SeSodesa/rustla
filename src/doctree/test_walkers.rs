/// ## test_walkers
/// 
/// A unit test module for testing document tree walker methods.
/// 
/// author: Santtu Söderholm
/// email:  santtu.soderolm@tuni.fi

use super::*;

#[cfg(test)]


#[test]
fn walk_to_id_01 () {

  use crate::parser::Parser;
  use crate::common::ParsingResult;

  let src = String::from("
  .. admonition::
    This is a generic admonition, the argument of which starts on
    the line following the directive marker.
    :class: options start here
    :name: here is a reference name
    :unrecognized: This option is discarded by the parsing function.

    The admonition contents start here,
    with a single paragraph.

    - followed by
    - a bullet list

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree.print_tree();

  doctree = doctree.walk(TraversalType::ID(3));
  doctree.print_tree();

  todo!()
}