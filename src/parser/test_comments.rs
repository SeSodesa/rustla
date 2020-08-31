/// ## test_comments
/// 
/// A unit test module for reST comments.
/// 
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;

#[cfg(test)]

#[test]
fn comment_01 () {

  let src = String::from("
.. This is a comment on a single line
  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.child(1).shared_data() {
    TreeNodeType::Comment { text } => {
      if text.as_ref().unwrap().as_str() != "This is a comment on a single line" {
        eprintln!("Erraneous text: {:#?}\n", text); panic!()
      }
    }
    _ => panic!()
  }
}
