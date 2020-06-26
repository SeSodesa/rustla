use super::*;

use crate::doctree::TreeNodeType;

#[cfg(test)]

#[test]
fn inline_parse_01 () {

  let src = String::from("This is a string with\n a ``literal``, **strong emphasis** and normal text");

  let in_machine = MachineWithState::<Inline>::from(MachineWithState::new());

  let nodes = match in_machine.parse(src, &mut 0) {
    Some(nodes) => nodes,
    None => panic!("No nodes to be found!")
  };

  eprintln!("{:#?}", nodes);

  assert_eq!(
    if let TreeNodeType::Literal(data) = &nodes[12].data {
      data.text.as_str()
    } else {panic!()},
    "literal"
  );

  assert_eq!(
    if let TreeNodeType::StrongEmphasis(data) = &nodes[15].data {
      data.text.as_str()
    } else {panic!()},
    "strong emphasis"
  );

}


#[test]
fn inline_parse_02 () {

  let src = String::from("This is a string with a reference_");

  let in_machine = MachineWithState::<Inline>::from(MachineWithState::new());

  let nodes = match in_machine.parse(src, &mut 0) {
    Some(nodes) => nodes,
    None => panic!("No nodes to be found!")
  };

  eprintln!("{:#?}", nodes);

  assert_eq!(
    if let TreeNodeType::SimpleReference(data) = &nodes[12].data {
      data.target_label.as_str()
    } else {panic!()},
    "reference"
  );

}
