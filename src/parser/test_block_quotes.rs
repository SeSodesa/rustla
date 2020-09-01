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


#[test]
fn block_quote_03 () {

  let src = String::from("
  This is a paragraph inside a block quote.
  Indentation determines quotation level.
  The below attribution does not end this block quote,
  as it is indented relative to this block quote level.

    -- Santtu Söderholm inside a nested block quote

  This paragraph continues the first block quote,
  as the above attribution ends the nested block quote
  it also ended up triggering.

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
    TreeNodeType::Attribution { raw_text } => if raw_text.as_str() != "Santtu Söderholm inside a nested block quote" { panic!() }
    _ => panic!()
  }

  match doctree.child(1).child(3).shared_data() {
    TreeNodeType::EmptyLine { .. } => {}
    _ => panic!()
  }

  match doctree.child(1).child(4).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
    _ => panic!()
  }
}


#[test]
fn block_quote_04 () {

  let src = String::from("
  Below is a multi-line attribution

  -- Santtu Söderholm
    and company

The attribution will be combined into a single line,
at least for now.
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

  match doctree.child(1).child(2).shared_data() {
    TreeNodeType::Attribution { raw_text } => if raw_text.as_str() != "Santtu Söderholm and company" { panic!() }
    _ => panic!()
  }

  match doctree.child(2).shared_data() {
    TreeNodeType::EmptyLine => {}
    _ => panic!()
  }

  match doctree.child(3).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
    _ => panic!()
  }
}


#[test]
fn block_quote_05 () {

  let src = String::from("
  Below is a multi-line attribution

  -- Santtu Söderholm
 and company with too little indentation on the second line.
 This indented block actually ends up inside another less indented
 block quote as a paragraph.

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

  match doctree.child(1).child(2).shared_data() {
    TreeNodeType::Attribution { raw_text } => assert_eq!(raw_text, "Santtu Söderholm"),
    _ => panic!()
  }

  match doctree.child(2).shared_data() {
    TreeNodeType::BlockQuote { .. } => {}
    _ => panic!()
  }

  match doctree.child(2).child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
    _ => panic!()
  }
}
