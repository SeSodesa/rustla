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


#[test]
fn block_quote_02 () {

  let src = String::from("
  This is a paragraph inside a block quote.
  Indentation determines quotation level.
  The following attribution ends this block quote

  -- Santtu Söderholm

  This paragraph starts a new block quote at the same level
  as the previous one (as in it is not nested).

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
    TreeNodeType::Attribution { raw_text } => if raw_text.as_str() != "Santtu Söderholm" { panic!() }
    _ => panic!()
  }

  match doctree.child(2).shared_data() {
    TreeNodeType::EmptyLine { .. } => {}
    _ => panic!()
  }

  match doctree.child(3).shared_data() {
    TreeNodeType::BlockQuote { .. } => {}
    _ => panic!()
  }

  match doctree.child(3).child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
    _ => panic!()
  }
}
