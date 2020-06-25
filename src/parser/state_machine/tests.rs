use super::*;

use crate::doctree::TreeNodeType;

#[cfg(test)]

#[test]
fn inline_parse_01 () {

  let src = String::from("This is a string with\n **strong emphasis** and normal text");

  let in_machine = MachineWithState::<Inline>::from(MachineWithState::new());

  let nodes = match in_machine.parse(src, &mut 0) {
    Some(nodes) => nodes,
    None => panic!("No nodes to be found!")
  };

  eprintln!("{:#?}", nodes);

  assert_eq!(
    if let TreeNodeType::Text(data) = &nodes[0].data {
      data.text.as_str()
    } else {panic!()},
    "This is a string with"
  );

  assert_eq!(
    if let TreeNodeType::Text(data) = &nodes[1].data {
      data.text.as_str()
    } else {panic!()},
    "\n"
  );

  assert_eq!(
    if let TreeNodeType::StrongEmphasis(data) = &nodes[3].data {
      data.text.as_str()
    } else {panic!()},
    "strong emphasis"
  );

}