/// ## test_definition_lists
/// A submodule for definition list -related unit tests.

use super::*;


#[cfg(test)]


#[test]
fn definition_list_01 () {

  let src = String::from("
term 1
  Definition 1.

term 2
  Definition 2, paragraph 1.

  Definition 2, paragraph 2.

term 3 : classifier
  Definition 3.

term 4 : classifier one : classifier two
  Definition 4. 
  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  match doctree.child(1).shared_data() {
    TreeNodeType::DefinitionList { .. } => {}
    _ => panic!()
  }

  match doctree.child(1).child(0).shared_data() {
    TreeNodeType::DefinitionListItem { term, classifiers, .. } => {
      if term != "term 1" || !classifiers.is_empty() { panic!() }
    }
    _ => panic!()
  }

  match doctree.child(1).child(1).shared_data() {
    TreeNodeType::DefinitionListItem { term, classifiers, .. } => {
      if term != "term 2" || !classifiers.is_empty() { panic!() }
    }
    _ => panic!()
  }

  match doctree.child(1).child(2).shared_data() {
    TreeNodeType::DefinitionListItem { term, classifiers, .. } => {
      assert_eq!(term, "term 3");
      assert_eq!(*classifiers, vec!["classifier".to_string()]); 
    }
    _ => panic!()
  }

  match doctree.child(1).child(3).shared_data() {
    TreeNodeType::DefinitionListItem { term, classifiers, .. } => {
      assert_eq!(term, "term 4");
      assert_eq!(*classifiers, vec!["classifier one", "classifier two"].iter().map(|s| s.to_string()).collect::<Vec<String>>()); 
    }
    _ => panic!()
  }
}