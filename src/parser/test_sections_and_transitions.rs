/// ## test_sections_and_transitions
/// A submodule for testing document transitions and section titles.
/// 
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;

#[cfg(test)]


#[test]
fn transition_01 () {

  let src = String::from("
  
Below is a transition.

=======

The line is at least 4 symbols long.

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.child(4).shared_data() {
    TreeNodeType::Transition => (),
    _ => panic!()
  }
}


#[test]
fn over_under_section_01 () {

  let src = String::from("
  
Below is an over- and underlined section title.

=======================================
 This is Spart.. I mean a section title
========================================

The line is at least 4 symbols long.

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  todo!()
}
