/// ## test_literal_block
/// A submodule for testing the parsing of literal blocks of text|code.
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;

#[cfg(test)]

#[test]
fn literal_block_01 () {

  let src = String::from("
   ::  
    
   > This is a literal block of text,
   > indicated by the \"::\" at the end of last paragraph.

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.child(3).shared_data() {
    TreeNodeType::LiteralBlock { text } => {
      assert_eq!(text.as_str(), "This is a literal block of text,\nindicated by the \"::\" at the end of last paragraph.")
    }
    _ => panic!()
  }
}


#[test]
fn literal_block_02 () {

  let src = String::from(r#"
   ::  
    
      An indented literal block with
      multiple lines

        Even more indent here.

          And even more...
      Return to original level of indentation

    This line ends the literal block, as its indentation is on the same level
    as that of the literal block indicator "::".

  "#);

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.child(3).shared_data() {
    TreeNodeType::LiteralBlock { text } => {
      assert_eq!(text.as_str(), "An indented literal block with\nmultiple lines\n\n  Even more indent here.\n\n    And even more...\nReturn to original level of indentation\n")
    }
    _ => panic!()
  }
}
