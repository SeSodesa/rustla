/// ## test_block_quotes
/// 
/// A submodule for unit testing block quotes.
/// 
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;

#[cfg(test)]


#[test]
fn block_quote_01 () {

  let src = String::from("
  This is a paragraph inside a block quote.
  Indentation determines quotation level.

    This second paragraph is inside a nested block quote,
    as this has more indentation the the above paragraph.

  This paragraph is again at the first level of quotation.

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  match doctree.child(1).shared_data() {
    TreeNodeType::BlockQuote { .. } => {}
    _ => panic!()
  }

  match doctree.child(1).child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
    _ => panic!()
  }

  match doctree.child(1).child(1).shared_data() {
    TreeNodeType::EmptyLine { .. } => {}
    _ => panic!()
  }

  match doctree.child(1).child(2).shared_data() {
    TreeNodeType::BlockQuote { .. } => {}
    _ => panic!()
  }

  match doctree.child(1).child(2).child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
    _ => panic!()
  }

  match doctree.child(1).child(3).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
    _ => panic!()
  }
}
