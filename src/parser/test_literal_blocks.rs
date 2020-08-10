/// ## test_literal_block
/// A submodule for testing the parsing of literal blocks of text|code.

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

  todo!()
}