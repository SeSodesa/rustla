/// ## enumerated_list
/// A submodule for `Statemachine::EnumeratedList` related transition functions.

use super::*;

pub fn enumerator (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>, PushOrPop, LineAdvance), &'static str> {

  let mut tree_wrapper = doctree.unwrap();

  let detected_enumerator_indent = captures.get(1).unwrap().as_str().chars().count();
  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count();
  let detected_enum_str = captures.get(2).unwrap().as_str();

  let (detected_delims, mut detected_kind) = if let PatternName::Enumerator { delims, kind} = pattern_name {
    (*delims, *kind)
  } else {
    return Err("No enumerator inside enumerator transition method.\nWhy...?\n")
  };

  let mut detected_enum_as_usize = match detected_kind {

    EnumKind::Arabic => {
      detected_enum_str.parse::<usize>().unwrap() // Standard library has implemented conversions from str to integers
    }

    EnumKind::LowerAlpha | EnumKind::UpperAlpha => {
      if let Some(num) = Parser::alpha_to_usize(detected_enum_str) {
        num
      } else {
        return Err("Couldn't convert given alphabet to an integer...\n")
      }
    }

    EnumKind::LowerRoman => {
      if let Some(num) = Parser::lower_roman_to_usize(detected_enum_str) {
        num
      } else {
        return Err("Couldn't convert lower-case Roman numeral to an integer...\n")
      }
    }

    EnumKind::UpperRoman => {
      if let Some(num) = Parser::lower_roman_to_usize(detected_enum_str) {
        num
      } else {
        return Err("Couldn't convert upper-case Roman numeral to an integer...\n")
      }
    }
  };

  eprintln!("Detected enumerator type pair ({:#?}, {:#?}) as {:#?}...\n", detected_delims, detected_kind, detected_enum_as_usize);

  let (list_delims, list_kind, list_start_index, list_item_number,list_enumerator_indent, list_text_indent) = match &mut tree_wrapper.tree.node.data {
    TreeNodeType::EnumeratedList { delims, kind, start_index, n_of_items, enumerator_indent, latest_text_indent } => (delims, kind, start_index, n_of_items, enumerator_indent, latest_text_indent),
    _ => return Err("Not focused on EnumeratedList...\n")
  };

  if detected_enum_str == "i" && *list_item_number == 0 {
    // LowerRoman list at our hands
    detected_kind = EnumKind::LowerRoman;
    detected_enum_as_usize = 1;
  } else if detected_enum_str == "I" && *list_item_number == 0 {
    // UpperRoman list at our hands
    detected_kind = EnumKind::LowerRoman;
    detected_enum_as_usize = 1;
  }

  // Matching detected parameters against corresponding list ones and proceeding accordingly 
  match (detected_delims, detected_kind, &detected_enumerator_indent, &detected_text_indent) {

    (delims, kind, enum_indent, text_indent) if delims == *list_delims && kind == *list_kind && enum_indent == list_enumerator_indent && detected_enum_as_usize == *list_item_number + 1 => {

      // All parameters are the same, so this ListItem is a direct child of the current EnumeratedList.
      // Create a new ListItem node, focus on it and push a ListItem state on top of the parser stack.

      eprintln!("Found list item belonging to current list...\n");

      match &mut tree_wrapper.tree.node.data {
        TreeNodeType::EnumeratedList {n_of_items, latest_text_indent, ..} => {
          *n_of_items += 1;
          *latest_text_indent = *text_indent;
        },
        _ => return Err("Only enumerated lists keep track of the number of item nodes in them...\n")
      }


      // Read indented block here
      let block = match Parser::read_indented_block(src_lines, Some(*current_line), Some(true), None, Some(*text_indent), Some(*text_indent)) {
        Ok((lines, min_indent, line_offset, blank_finish)) => {
          if min_indent != *text_indent {
            return Err("Indent of list item block was less than given.")
          }
          lines.join("\n")
        }
        Err(e) => {
          eprintln!("{}", e);
          return Err("Error when reading list item block.\n")
        }
      };

      // Pass text to inline parser as a string
      let mut inline_nodes = if let Some(children) = Parser::inline_parse(block, current_line) {
        children
      } else {
        Vec::new()
      };

      let paragraph_node_data = TreeNodeType::Paragraph;
      let mut paragraph_node = TreeNode::new(paragraph_node_data);
      paragraph_node.append_children(&mut inline_nodes);

      let item_node_data = TreeNodeType::EnumeratedListItem {delims: delims, kind: kind, index_in_list: detected_enum_as_usize, enumerator_indent: *enum_indent, text_indent: *text_indent};

      eprintln!("List node data: {:#?}\n", tree_wrapper.tree.node.data);

      eprintln!("Item node data: {:#?}\n", item_node_data);

      let mut item_node = TreeNode::new(item_node_data);

      item_node.push_child(paragraph_node);

      tree_wrapper.tree.push_child(item_node);

      tree_wrapper.tree = match tree_wrapper.tree.focus_on_last_child() {
        Ok(tree)  => tree,
        Err(tree) => return Err("Couldn't focus on enumerated list item...\n")
      };

      let next_state = StateMachine::ListItem;

      Ok( ( Some(tree_wrapper), Some(next_state), PushOrPop::Push, LineAdvance::Some(1) ) )

    }

    (delims, kind, enum_indent, text_indent) if enum_indent == list_text_indent => {

      // Enumerator indent is on the same level as the current list indentation
      // => A sublist has been detected, so push another EnumeratedList state on top of
      // the parser state stack, create a new EnumeratedList node as a child of
      // the current one and focus on it.

      eprintln!("Sublist enumerator detected...\n");
      todo!()
    }

    (delims, kind, enum_indent, text_indent) if detected_delims != *list_delims || detected_kind != *list_kind  || enum_indent < list_enumerator_indent => {

      // Unmatching enumerator or less indent
      // => This enumerator is either a part of a superlist, or a different list on the same level
      // => Pop from machine stack and try parsing at a lower nesting level.

      tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
        Ok(tree) => tree,
        Err(tree) => return Err("Couldn't focus on list parent...\n")
      };

      Ok( ( Some(tree_wrapper), None, PushOrPop::Pop, LineAdvance::None ) )

    }

    _ => {
      eprintln!("No specific instruction for found detected enumerator parameters.\nSimply POPping from stack in hopes of the previous state knowing better...\n");
      return Ok( ( Some(tree_wrapper), None, PushOrPop::Pop, LineAdvance::None ) )
    }
  }

}
