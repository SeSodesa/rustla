use super::*;

use crate::doctree::TreeNodeType;

#[cfg(test)]


#[test]
fn inline_parse_01 () {

  let src = String::from("This is a string with\n a ``literal``, **strong emphasis** and normal text");
  let mut lc = LineCursor::new(0,0);

  let nodes = match Parser::inline_parse(src, None, &mut lc, &mut 0) {
    Some(nodes) => nodes,
    None => panic!("No nodes to be found!")
  };

  eprintln!("{:#?}", nodes);

  assert_eq!(
    if let TreeNodeType::Literal{text} = &nodes[12].data {
      text.as_str()
    } else {panic!()},
    "literal"
  );

  assert_eq!(
    if let TreeNodeType::StrongEmphasis{text} = &nodes[15].data {
      text.as_str()
    } else {panic!()},
    "strong emphasis"
  );

}


#[test]
fn inline_parse_02 () {

  let src = String::from("This is a string with a simple-reference+with:punctuation__\nand a `phrase reference`_");

  let mut lc = LineCursor::new(0,0);

  let nodes = match Parser::inline_parse(src,  None, &mut lc, &mut 0) {
    Some(nodes) => nodes,
    None => panic!("No nodes to be found!")
  };

  eprintln!("{:#?}", nodes);

  assert_eq!(
    if let TreeNodeType::Reference{target_label} = &nodes[12].data {
      target_label.as_str()
    } else {panic!()},
    "simple-reference+with:punctuation"
  );

  assert_eq!(
    if let TreeNodeType::Reference{target_label} = &nodes[18].data {
      target_label.as_str()
    } else {panic!()},
    "phrase reference"
  );

}



#[test]
fn inline_parse_03 () {

  let src = String::from("Here is a simple-reference_ to an _`inline target.`");
  let mut lc = LineCursor::new(0,0);

  let nodes = match Parser::inline_parse(src, None, &mut lc, &mut 0) {
    Some(nodes) => nodes,
    None => panic!("No nodes to be found!")
  };

  eprintln!("{:#?}", nodes);

  assert_eq!(
    if let TreeNodeType::Reference{target_label} = &nodes[6].data {
      target_label.as_str()
    } else {panic!()},
    "simple-reference"
  );

  assert_eq!(
    if let TreeNodeType::InlineTarget{target_label} = &nodes[12].data {
      target_label.as_str()
    } else {panic!()},
    "inline target."
  );

}



#[test]
fn inline_parse_04 () {

  let src = String::from("Here is a |substitution reference|_ to an _`inline target.`");
  let mut lc = LineCursor::new(0,0);

  let nodes = match Parser::inline_parse(src, None, &mut lc, &mut 0) {
    Some(nodes) => nodes,
    None => panic!("No nodes to be found!")
  };

  eprintln!("{:#?}", nodes);

  assert_eq!(
    if let TreeNodeType::SubstitutionReference{target_label} = &nodes[6].data {
      target_label.as_str()
    } else {panic!()},
    "substitution reference"
  );

}


#[test]
fn inline_parse_05 () {

  let src = String::from("This is an absolute URI: https://john.harry.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top");
  let mut lc = LineCursor::new(0,0);

  let nodes = match Parser::inline_parse(src, None, &mut lc, &mut 0) {
    Some(nodes) => nodes,
    None => panic!("No nodes to be found!")
  };

  eprintln!("{:#?}", nodes);

  assert_eq!(
    if let TreeNodeType::AbsoluteURI{text} = &nodes[10].data {
      text.as_str()
    } else {panic!("Absolute URI not found!")},
    "https://john.harry.doe@www.example.com:123/forum/questions/?tag=networking&order=newest#top"
  );

}


#[test]
fn inline_parse_06 () {

  let src = String::from("This is an email address: john.harry.doe@www.example.com");
  let mut lc = LineCursor::new(0,0);

  let nodes = match Parser::inline_parse(src, None, &mut lc, &mut 0) {
    Some(nodes) => nodes,
    None => panic!("No nodes to be found!")
  };

  eprintln!("{:#?}", nodes);

  assert_eq!(
    if let TreeNodeType::StandaloneEmail{text} = &nodes[10].data {
      text.as_str()
    } else {panic!()},
    "mailto:john.harry.doe@www.example.com"
  );

}
