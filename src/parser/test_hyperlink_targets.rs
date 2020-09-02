/// ## test_hyperlink_targets
/// 
/// A submodule for tests related to parsing hyperlink targets.
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;

#[cfg(test)]


#[test]
fn footnote_01 () {

  let src = String::from("
.. [1] Here is a paragraph
    with body indent.

    * Bullet list inside foonote

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.shared_child(1).shared_data() {
    TreeNodeType::Footnote { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(1).shared_data() {
    TreeNodeType::EmptyLine => (),
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(2).shared_data() {
    TreeNodeType::BulletList { .. } => (),
    _ => panic!()
  }
}


#[test]
fn footnote_02 () {

  let src = String::from("
.. [1] Here is a paragraph

.. [1] Another footnote with the same label (and target).
        The duplicate label should generate a warning.

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.shared_child(1).shared_data() {
    TreeNodeType::Footnote { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(2).shared_data() {
    TreeNodeType::Footnote { .. } => (),
    _ => panic!()
  }

  match doctree.shared_child(2).shared_child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!()
  }
}


#[test]
fn footnote_03 () {

  let src = String::from("
.. [*] 1
.. [*] 2
.. [*] 3
.. [*] 4
.. [*] 5
.. [*] 6
.. [*] 7
.. [*] 8
.. [*] 9
.. [*] 10
.. [*] 11
.. [*] 12
.. [*] 13
.. [*] 14
.. [*] 15
.. [*] 16
.. [*] 17
.. [*] 18
.. [*] 19
.. [*] 20
.. [*] 21
.. [*] 22
.. [*] 23
.. [*] 24
.. [*] 25
.. [*] 26
.. [*] 27
.. [*] 28
.. [*] 29

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.shared_child(1).shared_data() {
    TreeNodeType::Footnote {label, ..} => {
      if label == "*" {} else {panic!()}
    }
    _ => panic!()
  }

  match doctree.shared_child(3).shared_data() {
    TreeNodeType::Footnote {label, ..} => {
      if label == "‡" {} else {panic!()}
    }
    _ => panic!()
  }

  match doctree.shared_child(11).shared_data() {
    TreeNodeType::Footnote {label, ..} => {
      if label == "**" {} else {panic!()}
    }
    _ => panic!()
  }

  match doctree.shared_child(13).shared_data() {
    TreeNodeType::Footnote {label, ..} => {
      if label == "‡‡" {} else {panic!()}
    }
    _ => panic!()
  }

  match doctree.shared_child(21).shared_data() {
    TreeNodeType::Footnote {label, ..} => {
      if label == "***" {} else {panic!()}
    }
    _ => panic!()
  }

  match doctree.shared_child(29).shared_data() {
    TreeNodeType::Footnote {label, ..} => {
      if label == "♦♦♦" {} else {panic!()}
    }
    _ => panic!()
  }
}


#[test]
fn footnote_04 () {

  let src = String::from("
.. [2] 1
.. [#test-with-mixed] 2
.. [#] 3
.. [#second] 4
.. [#] 5

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.shared_child(1).shared_data() {
    TreeNodeType::Footnote { label, target, ..} => {
      if label == "2" && target == "2" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.shared_child(2).shared_data() {
    TreeNodeType::Footnote { label, target, ..} => {
      if label == "1" && target == "test-with-mixed" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.shared_child(3).shared_data() {
    TreeNodeType::Footnote { label, target, ..} => {
      if label == "3" && target == "3" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.shared_child(4).shared_data() {
    TreeNodeType::Footnote { label, target, ..} => {
      if label == "4" && target == "second" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.shared_child(5).shared_data() {
    TreeNodeType::Footnote { label, target, ..} => {
      if label == "5" && target == "5" {} else { panic!() }
    }
    _ => panic!()
  }
}


#[test]
fn footnote_05 () {

  let src = String::from("
.. [2] 1
.. [#test-with-mixed] 2
.. [*] .. [#nested] 4
.. [*] 5
.. [2] 5

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match &doctree.shared_child(1).shared_data() {
    TreeNodeType::Footnote { label, target, .. } => {
      if label == "2" && target == "2" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.shared_child(2).shared_data() {
    TreeNodeType::Footnote { label, target, .. } => {
      if label == "1" && target == "test-with-mixed" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.shared_child(3).shared_data() {
    TreeNodeType::Footnote { label, target, .. } => {
      if label == "*" && target == "*" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.shared_child(3).shared_child(0).shared_data() {
    TreeNodeType::Footnote { label, target, .. } => {
      if label == "3" && target == "nested" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.shared_child(4).shared_data() {
    TreeNodeType::Footnote { label, target, .. } => {
      if label == "†" && target == "†" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.shared_child(5).shared_data() {
    TreeNodeType::Footnote { label, target, .. } => {
      if label == "2" && target == "2" {} else { panic!() }
    }
    _ => panic!()
  }
}


#[test]
fn citation_01 () {

  let src = String::from("
.. [CIT2005] Citation

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match &doctree.shared_child(1).shared_data() {
    TreeNodeType::Citation {label, .. } => {
      if !(label == "CIT2005") { panic!() }
    }
     _=> panic!()
  }

  match &doctree.shared_child(1).shared_child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
     _=> panic!()
  }
}


#[test]
fn citation_02 () {

  let src = String::from("
.. [one] aaa
    .. [two] This line is a part of
    the paragraph started on previous line
       .. [three] This is a citation inside a block quote

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match &doctree.shared_child(1).shared_data() {
    TreeNodeType::Citation {label, .. } => {
      if !(label == "one") { panic!() }
    }
     _=> panic!()
  }

  match &doctree.shared_child(1).shared_child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
     _=> panic!()
  }

  match &doctree.shared_child(1).shared_child(1).shared_data() {
    TreeNodeType::BlockQuote { body_indent } => {
      if *body_indent != 7 { panic!() }
    }
     _=> panic!()
  }
}


#[test]
fn hyperlink_target_01 () {
  
  let src = String::from("
.. _target1:
.. _target2:

Paragraph here. Please give me the label \"target1--target2\".

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  assert_eq!(doctree.shared_child(2).shared_target_label(), "target1--target2");
}


#[test]
fn hyperlink_target_02 () {
  
  let src = String::from("
* This here is a bulleted list

  .. _internal-target-referencing-below-item:

  .. _another-target-referencing-below-item:

* The above internal targets that belong to the
  previous list item should reference this item.

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  assert_eq!(doctree.shared_child(1).shared_child(1).shared_target_label(), "internal-target-referencing-below-item--another-target-referencing-below-item");
}


#[test]
fn hyperlink_target_03 () {

  let src = String::from("
.. _an-external-hyperlink: https://www.address.fi//

.. _indirect_hyperlink: an-external-hyperlink_


  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  eprintln!("Doctree targets: {:#?}\n", doctree.shared_targets());
  eprintln!("Doctree references: {:#?}\n", doctree.shared_references());

  match doctree.shared_child(1).shared_data() {
    TreeNodeType::ExternalHyperlinkTarget { uri, target, .. } => {
      if target != "an-external-hyperlink" || uri != "https://www.address.fi//" {
        eprintln!("Target: {:#?}\nURI: {:#?}\n", target, uri);
        panic!()
      }
    }
    _ => panic!()
  }

  match doctree.shared_child(3).shared_data() {
    TreeNodeType::IndirectHyperlinkTarget { target, indirect_target, .. } => {
      if target != "indirect_hyperlink" || indirect_target != "an-external-hyperlink" {
        eprintln!("Target: {:#?}\nIndirect target: {:#?}\n", target, indirect_target);
        panic!()
      }
    }
    _ => panic!()
  }
}


#[test]
fn hyperlink_target_04 () {

  let src = String::from("
.. __: https://www.address.fi//

.. __: anon-target-ref__

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  eprintln!("Doctree targets: {:#?}", doctree.shared_targets());
  eprintln!("Doctree references: {:#?}", doctree.shared_references());

  match doctree.shared_child(1).shared_data() {
    TreeNodeType::ExternalHyperlinkTarget { target, uri, .. } => {
      if target != "[[-ANON-LABEL-1-]]" || uri != "https://www.address.fi//" {
        eprintln!("Target: {:#?}\nURI: {:#?}\n", target, uri);
        panic!()
      }
    }
    _ => panic!()
  }

  match doctree.shared_child(3).shared_data() {
    TreeNodeType::IndirectHyperlinkTarget { target, indirect_target, .. } => {
      if target != "[[-ANON-LABEL-2-]]" || indirect_target != "[[-ANON-LABEL-1-]]" {
        eprintln!("Target: {:#?}\nIndirect target: {:#?}\n", target, indirect_target);
        panic!()
      }
    }
    _ => panic!()
  }
}
