/// ## test_inline_parsing
/// Unit tests related to parsing inline text.
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;

#[cfg(test)]


#[test]
fn literal_and_strong_emphasis_01 () {

  let src = String::from("This is a string with\n a ``literal``, **strong emphasis** and normal text");
  let mut lc = LineCursor::new(0,0);

  let nodes = match Parser::inline_parse(src, None, &mut lc) {
    InlineParsingResult::Nodes(nodes) => nodes,
    _ => panic!("No nodes to be found!")
  };

  eprintln!("{:#?}", nodes);

  assert_eq!(
    if let TreeNodeType::Literal{text} = &nodes[12] {
      text.as_str()
    } else { panic!() },
    "literal"
  );

  assert_eq!(
    if let TreeNodeType::StrongEmphasis{text} = &nodes[15] {
      text.as_str()
    } else { panic!() },
    "strong emphasis"
  );

}


#[test]
fn references_01 () {

  let src = String::from("This is a string with a simple-reference+with:punctuation_\nand a `phrase reference`_");

  let mut lc = LineCursor::new(0,0);

  let nodes = match Parser::inline_parse(src,  None, &mut lc) {
    InlineParsingResult::Nodes(nodes) => nodes,
    _ => panic!("No nodes to be found!")
  };

  eprintln!("{:#?}", nodes);

  assert_eq!(
    if let TreeNodeType::Reference{target_label, ..} = &nodes[12] {
      target_label.as_str()
    } else {panic!()},
    "simple-reference+with:punctuation"
  );

  assert_eq!(
    if let TreeNodeType::Reference{target_label, ..} = &nodes[18] {
      target_label.as_str()
    } else {panic!()},
    "phrase reference"
  );

}



#[test]
fn references_02 () {

  let src = String::from("Here is a simple-reference_ and a `not so simple refereNce`_.");
  let mut lc = LineCursor::new(0,0);

  let nodes = match Parser::inline_parse(src, None, &mut lc) {
    InlineParsingResult::Nodes(nodes) => nodes,
    _ => panic!("No nodes to be found!")
  };

  eprintln!("{:#?}", nodes);

  assert_eq!(
    if let TreeNodeType::Reference{target_label, ..} = &nodes[6] {
      target_label.as_str()
    } else {panic!()},
    "simple-reference"
  );

  if let TreeNodeType::Reference{target_label, displayed_text} = &nodes[12] {
    assert_eq!(displayed_text.as_str(), "not so simple refereNce");
    assert_eq!(target_label.as_str(), "not so simple reference");
  } else {panic!()}

}



#[test]
fn substitution_ref_01 () {

  let src = String::from(
r#"
This is a simple |substitution reference|.  It will be replaced by
the processing system.

This is a combination |substitution and hyperlink reference|_.  In
addition to being replaced, the replacement text or element will
refer to the "substitution and hyperlink reference" target.
"#);
  let mut lc = LineCursor::new(0,0);

  let nodes = match Parser::inline_parse(src, None, &mut lc) {
    InlineParsingResult::Nodes(nodes) => nodes,
    _ => panic!("No nodes to be found!")
  };

  eprintln!("{:#?}", nodes);

  if let TreeNodeType::SubstitutionReference{substitution_label, target_label} = &nodes[9] {
    assert_eq!(substitution_label.as_str(), "substitution reference");
    assert_eq!(target_label.as_deref(), None);
    
  } else { panic!() };

  if let TreeNodeType::SubstitutionReference{substitution_label, target_label} = &nodes[36] {
    assert_eq!(substitution_label.as_str(), "substitution and hyperlink reference");
    assert_eq!(target_label.as_deref(), Some("substitution and hyperlink reference"));
    
  } else { panic!() };

}


#[test]
fn inline_parse_05 () {

  let src = String::from("This is an absolute URI: https://john.harry.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top");
  let mut lc = LineCursor::new(0,0);

  let nodes = match Parser::inline_parse(src, None, &mut lc) {
    InlineParsingResult::Nodes(nodes) => nodes,
    _ => panic!("No nodes to be found!")
  };

  eprintln!("{:#?}", nodes);

  assert_eq!(
    if let TreeNodeType::AbsoluteURI{text} = &nodes[10] {
      text.as_str()
    } else {panic!("Absolute URI not found!")},
    "https://john.harry.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top"
  );

}


#[test]
fn inline_parse_06 () {

  let src = String::from("This is an email address: john.harry.doe@www.example.com");
  let mut lc = LineCursor::new(0,0);

  let nodes = match Parser::inline_parse(src, None, &mut lc) {
    InlineParsingResult::Nodes(nodes) => nodes,
    _ => panic!("No nodes to be found!")
  };

  eprintln!("{:#?}", nodes);

  assert_eq!(
    if let TreeNodeType::StandaloneEmail{text} = &nodes[10] {
      text.as_str()
    } else {panic!()},
    "john.harry.doe@www.example.com"
  );

}


use std::path::PathBuf;

#[test]
fn quoted_markup_01 () {

  let src = String::from(r#"
Paragraph with quoted markup: "**strong emphasis**",
<*text in italics*>, (`a phrase reference with automatic labeling`__),
maybe a -simple-reference__- as well.

"#).lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(PathBuf::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  todo!()
}