/// ## enumerated_list
/// A submodule for `Statemachine::EnumeratedList` related transition functions.

use super::*;

pub fn enumerator (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>, PushOrPop, LineAdvance), &'static str> {

  let mut tree_wrapper = doctree.unwrap();

  let detected_enumerator_indent = captures.name("indent").unwrap().as_str().chars().count();
  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count();
  let detected_enumerator_type = if let Some(enumerator) = StateMachine::check_enumerator_type(&captures) {
    enumerator
  } else {
    return Err("No match for enumerator in known enumerators.\nWhy inside transition function...?\n")
  };

  let (list_enumerator_type, list_enumerator_indent, list_text_indent) = match &tree_wrapper.tree.node.data {
    TreeNodeType::EnumeratedList { enum_type, enumerator_indent, text_indent } => (enum_type, enumerator_indent, text_indent),
    _ => return Err("Not focused on EnumeratedList...\n")
  };

  // Matching detected parameters against corresponding list ones and proceeding accordingly 
  match (&detected_enumerator_type, &detected_enumerator_indent, &detected_text_indent) {

    (enum_type, enum_indent, text_indent) if enum_type == list_enumerator_type && enum_indent == list_enumerator_indent && text_indent == list_text_indent => {

      // All parameters are the same, so this ListItem is a direct child of the current EnumeratedList.
      // Create a new ListItem node, focus on it and push a ListItem state on top of the parser stack. 
      todo!()
    }

    (enum_type, enum_indent, text_indent) if enum_indent == list_text_indent => {

      // Enumerator indent is on the same level as the current list indentation
      // => A sublist has been detected, so push another EnumeratedList state on top of
      // the parser state stack, created a new EnumeratedList node as a child of
      // the current oneand focus on it.
      todo!()
    }

    (enum_type, enum_indent, text_indent) if enum_type != list_enumerator_type || enum_indent < list_enumerator_indent => {
      
      // Unmatching enumerator or less indent
      // => This enumerator is either a part of a superlist, or a different list on the same level
      // => Pop from machine stack and try parsing at a lower nesting level.
      todo!()
    }

    _ => return Ok( ( Some(tree_wrapper), None, PushOrPop::Pop, LineAdvance::None ) )
  }

}
