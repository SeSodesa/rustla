/// ## test_unknown_directives
///
/// A submodule for unit testing unknown directives
///
/// author: Santtu Söderholm
/// email: santtu.soderholm@tuni.fi

use super::*;

#[cfg(test)]


#[test]
fn unknown_directive_01 () {

  let src = String::from("
.. some-directive-without-options:: some argument here...
  :option1: something
  :option2: something else

A paragraph.


  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(PathBuf::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  match doctree.shared_child(0).shared_data() {
    TreeNodeType::LiteralBlock { text } => {}
    _ => panic!()
  }

  match doctree.shared_child(1).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
    _ => panic!()
  }
}


#[test]
fn unknown_directive_02 () {

  let src = String::from("
Below is an unknown directive. It will be parsed as a literal block.

  .. unknown:: argument
    :option1: a
    :option2: bunch
    :option3: of options
    :option3: here
    :option5: a

    Paragraph inside unknown directive

    - And a bullet list with just one item

  This is no longer a part of the above literal block inside a block quote.

This paragraph ends the block quote.

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(PathBuf::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  match doctree.shared_child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(1).shared_data() {
    TreeNodeType::BlockQuote { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(0).shared_data() {
    TreeNodeType::LiteralBlock { text } => {}
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(1).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(2).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
    _ => panic!()
  }

}
