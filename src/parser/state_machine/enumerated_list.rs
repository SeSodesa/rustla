/// ## enumerated_list
/// A submodule for `Statemachine::EnumeratedList` related transition functions.

use super::*;

pub fn enumerator (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>, PushOrPop, LineAdvance), &'static str> {

  let mut tree_wrapper = doctree.unwrap();

  let detected_enumerator_indent = captures.get(1).unwrap().as_str().chars().count();
  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count();
  let detected_enum_str = captures.get(2).unwrap().as_str();

  let detected_enum_type = if let Some(enum_type) = pattern_name.as_enum_type() {
    enum_type
  } else {
    return Err("No matching enum type for given enumerator pattern...\n")
  };

  let detected_enum_as_u32 = match detected_enum_type {

    EnumeratorType::ArabicParens | EnumeratorType::ArabicRParen | EnumeratorType::ArabicPeriod => {
      detected_enum_str.parse::<u32>().unwrap() // Standard library has implemented conversion from str to u32
    }

    EnumeratorType::LowerAlphaParens | EnumeratorType::LowerAlphaRParen | EnumeratorType::LowerAlphaPeriod
    | EnumeratorType::UpperAlphaParens | EnumeratorType::UpperAlphaRParen | EnumeratorType::UpperAlphaPeriod => {
      if let Some(num) = Parser::alpha_to_u32(detected_enum_str) {
        num
      } else {
        return Err("Couldn't convert alphabet to an integer...\n")
      }
    }

    EnumeratorType::LowerRomanParens | EnumeratorType::LowerRomanRParen | EnumeratorType::LowerRomanPeriod => {
      if let Some(num) = Parser::lower_roman_to_u32(detected_enum_str) {
        num
      } else {
        return Err("Couldn't convert lower-case Roman numeral to an integer...\n")
      }
    }

    EnumeratorType::UpperRomanParens | EnumeratorType::UpperRomanRParen | EnumeratorType::UpperRomanPeriod => {
      if let Some(num) = Parser::lower_roman_to_u32(detected_enum_str) {
        num
      } else {
        return Err("Couldn't convert upper-case Roman numeral to an integer...\n")
      }
    }
  };

  let (list_enumerator_type, list_item_number,list_enumerator_indent, list_text_indent) = match &mut tree_wrapper.tree.node.data {
    TreeNodeType::EnumeratedList { enum_type, items, enumerator_indent, text_indent } => (enum_type, items, enumerator_indent, text_indent),
    _ => return Err("Not focused on EnumeratedList...\n")
  };

  // Matching detected parameters against corresponding list ones and proceeding accordingly 
  match (&detected_enum_type, &detected_enumerator_indent, &detected_text_indent) {

    (enum_type, enum_indent, text_indent) if enum_type == list_enumerator_type && enum_indent == list_enumerator_indent && text_indent == list_text_indent => {

      // All parameters are the same, so this ListItem is a direct child of the current EnumeratedList.
      // Create a new ListItem node, focus on it and push a ListItem state on top of the parser stack.


      todo!()
    }

    (enum_type, enum_indent, text_indent) if enum_indent == list_text_indent => {

      // Enumerator indent is on the same level as the current list indentation
      // => A sublist has been detected, so push another EnumeratedList state on top of
      // the parser state stack, created a new EnumeratedList node as a child of
      // the current one and focus on it.
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
