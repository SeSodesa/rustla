/// ## test_math_blocks
/// 
/// A submodule for testing math blocks.
/// 
/// author: Santtu Söderholm
/// email: santtu.soderholm@tuni.fi

use super::*;

#[cfg(test)]


#[test]
fn math_block_01 () {

  let src = String::from(r"
.. math::
  :name: name
  :class: class

  \alpha + \beta = \gamma

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  match doctree.shared_child(1).shared_data() {
    TreeNodeType::MathBlock { block_text, name, class } => {
      assert_eq!(block_text.as_str(), r"\alpha + \beta = \gamma");
      assert_eq!(name.as_ref().unwrap().as_str(), r"name");
      assert_eq!(class.as_ref().unwrap().as_str(), r"class");
    }
    _ => panic!()
  }
}


#[test]
fn math_block_02 () {

  let src = String::from(r#"
.. math::
  :name: name
  :class: class

  (1) \alpha + \beta = \gamma
  \delta

  Another block with math (printed as "Anotherblockwithmath" in LaTeX)

  "#).lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  match doctree.shared_child(1).shared_data() {
    TreeNodeType::MathBlock { block_text, name, class } => {
      assert_eq!(block_text.as_str(), r"(1) \alpha + \beta = \gamma \delta");
      assert_eq!(name.as_ref().unwrap().as_str(), r"name");
      assert_eq!(class.as_ref().unwrap().as_str(), r"class");
    }
    _ => panic!()
  }

  match doctree.shared_child(2).shared_data() {
    TreeNodeType::MathBlock { block_text, name, class } => {
      assert_eq!(block_text.as_str(), r#"Another block with math (printed as "Anotherblockwithmath" in LaTeX)"#);
      assert_eq!(name.as_ref().unwrap().as_str(), r"name");
      assert_eq!(class.as_ref().unwrap().as_str(), r"class");
    }
    _ => panic!()
  }
}
