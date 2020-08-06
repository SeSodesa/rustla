/// ## test_hyperlink_targets
/// A submodule for tests related to parsing hyperlink targets.

use super::*;

#[cfg(test)]


#[test]
fn footnote_01 () {

  let src = String::from("
  .. [1] Here is a paragraph
     with body indent.

     * Bullet list inside foonote

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.child(1).get_data() {
    TreeNodeType::Footnote { .. } => (),
    _ => panic!()
  }

  match doctree.child(1).child(0).get_data() {
    TreeNodeType::Paragraph => (),
    _ => panic!()
  }

  match doctree.child(1).child(1).get_data() {
    TreeNodeType::EmptyLine => (),
    _ => panic!()
  }

  match doctree.child(1).child(2).get_data() {
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

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.child(1).get_data() {
    TreeNodeType::Footnote { .. } => (),
    _ => panic!()
  }

  match doctree.child(1).child(0).get_data() {
    TreeNodeType::Paragraph => (),
    _ => panic!()
  }

  match doctree.child(2).get_data() {
    TreeNodeType::Footnote { .. } => (),
    _ => panic!()
  }

  match doctree.child(2).child(0).get_data() {
    TreeNodeType::Paragraph => (),
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

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.child(1).get_data() {
    TreeNodeType::Footnote {label, ..} => {
      if label == "*" {} else {panic!()}
    }
    _ => panic!()
  }

  match doctree.child(3).get_data() {
    TreeNodeType::Footnote {label, ..} => {
      if label == "‡" {} else {panic!()}
    }
    _ => panic!()
  }

  match doctree.child(11).get_data() {
    TreeNodeType::Footnote {label, ..} => {
      if label == "**" {} else {panic!()}
    }
    _ => panic!()
  }

  match doctree.child(13).get_data() {
    TreeNodeType::Footnote {label, ..} => {
      if label == "‡‡" {} else {panic!()}
    }
    _ => panic!()
  }

  match doctree.child(21).get_data() {
    TreeNodeType::Footnote {label, ..} => {
      if label == "***" {} else {panic!()}
    }
    _ => panic!()
  }

  match doctree.child(29).get_data() {
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

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.child(1).get_data() {
    TreeNodeType::Footnote { label, target, ..} => {
      if label == "2" && target == "2" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.child(2).get_data() {
    TreeNodeType::Footnote { label, target, ..} => {
      if label == "1" && target == "test-with-mixed" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.child(3).get_data() {
    TreeNodeType::Footnote { label, target, ..} => {
      if label == "3" && target == "3" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.child(4).get_data() {
    TreeNodeType::Footnote { label, target, ..} => {
      if label == "4" && target == "second" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.child(5).get_data() {
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

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match &doctree.child(1).get_data() {
    TreeNodeType::Footnote { label, target, .. } => {
      if label == "2" && target == "2" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.child(2).get_data() {
    TreeNodeType::Footnote { label, target, .. } => {
      if label == "1" && target == "test-with-mixed" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.child(3).get_data() {
    TreeNodeType::Footnote { label, target, .. } => {
      if label == "*" && target == "*" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.child(3).child(0).get_data() {
    TreeNodeType::Footnote { label, target, .. } => {
      if label == "3" && target == "nested" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.child(4).get_data() {
    TreeNodeType::Footnote { label, target, .. } => {
      if label == "†" && target == "†" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.child(5).get_data() {
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

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match &doctree.child(1).get_data() {
    TreeNodeType::Citation {label, .. } => {
      if !(label == "CIT2005") { panic!() }
    }
     _=> panic!()
  }

  match &doctree.child(1).child(0).get_data() {
    TreeNodeType::Paragraph => {}
     _=> panic!()
  }
}


#[test]
fn citation_02 () {

  let src = String::from("
  .. [one] aaa
      .. [two] bbb
        .. [three] ccc

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match &doctree.child(1).get_data() {
    TreeNodeType::Citation {label, .. } => {
      if !(label == "one") { panic!() }
    }
     _=> panic!()
  }

  match &doctree.child(1).child(0).get_data() {
    TreeNodeType::Paragraph => {}
     _=> panic!()
  }

  match &doctree.child(2).get_data() {
    TreeNodeType::Citation {label, .. } => {
      if !(label == "three") { panic!() }
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

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  assert_eq!(doctree.child(2).shared_target_label(), "target1--target2");
}


#[test]
fn hyperlink_target_02 () {
  
  let src = String::from("
  * This here is a bulleted list

    .. _internal-target-referencing-below-item:

    .. _another-target-referencing-below-item:
  
  * The above internal targets that belong to the
    previous list item should reference this item.

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  assert_eq!(doctree.child(1).child(1).shared_target_label(), "internal-target-referencing-below-item--another-target-referencing-below-item");
}


#[test]
fn hyperlink_target_03 () {

  let src = String::from("
  .. _an-external-hyperlink: https://www.address.fi//

  .. _indirect_hyperlink: an-external-hyperlink_


  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  eprintln!("Doctree targets: {:#?}\n", doctree.shared_targets());
  eprintln!("Doctree references: {:#?}\n", doctree.shared_references());

  match doctree.child(1).get_data() {
    TreeNodeType::ExternalHyperlinkTarget { uri, target, .. } => {
      if target != "an-external-hyperlink" || uri != "https://www.address.fi//" {
        eprintln!("Target: {:#?}\nURI: {:#?}\n", target, uri);
        panic!()
      }
    }
    _ => panic!()
  }

  match doctree.child(3).get_data() {
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

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  eprintln!("Doctree targets: {:#?}", doctree.shared_targets());
  eprintln!("Doctree references: {:#?}", doctree.shared_references());

  match doctree.child(1).get_data() {
    TreeNodeType::ExternalHyperlinkTarget { target, uri, .. } => {
      if target != "[[-ANON-LABEL-1-]]" || uri != "https://www.address.fi//" {
        eprintln!("Target: {:#?}\nURI: {:#?}\n", target, uri);
        panic!()
      }
    }
    _ => panic!()
  }

  match doctree.child(3).get_data() {
    TreeNodeType::IndirectHyperlinkTarget { target, indirect_target, .. } => {
      if target != "[[-ANON-LABEL-2-]]" || indirect_target != "[[-ANON-LABEL-1-]]" {
        eprintln!("Target: {:#?}\nIndirect target: {:#?}\n", target, indirect_target);
        panic!()
      }
    }
    _ => panic!()
  }
}
