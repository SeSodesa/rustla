/// ## enumerated_list
/// A submodule for `Statemachine::EnumeratedList` related transition functions.
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;

pub fn enumerator (src_lines: &Vec<String>, base_indent: usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut doctree = doctree.unwrap();

  let (list_delims, list_kind, list_start_index, n_of_items,list_enumerator_indent) = match doctree.mut_node_data() {
    TreeNodeType::EnumeratedList { delims, kind, start_index, n_of_items, enumerator_indent } => (delims, kind, start_index, n_of_items, enumerator_indent),
    _ => return TransitionResult::Failure {
      message: format!("Not focused on enumerated list when parsing an enumerated list item on line {}. Computer says no...", line_cursor.sum_total()),
      doctree: doctree
    }
  };

  let detected_enumerator_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count() + base_indent;

  // Retrieve parent list information
  let (detected_number, detected_kind, detected_delims) = match Parser::enum_captures_to_int_kind_and_delims(&captures, Some(list_kind), true, Some(*n_of_items), Some(*list_start_index)) {
    Some((number, kind, delims)) => (number, kind, delims),
    None => return TransitionResult::Failure {
      message: format!("Could not convert a list enumerator to number on line {}. Computer saus no...", line_cursor.sum_total()),
      doctree: doctree
    }
  };

  // Check validity of enumerated list item
  if let Some(next_line) = src_lines.get(line_cursor.relative_offset() + 1) {

    let line_indent = next_line.chars().take_while(|c| c.is_whitespace()).count() + base_indent;

    if ! next_line.is_empty() && line_indent <= detected_enumerator_indent {

      eprintln!("Next line not empty or not indented...");

      return crate::parser::state_machine::body::text (src_lines, base_indent, section_level, line_cursor, Some(doctree), captures, pattern_name)

    } else if let Some(next_captures) = crate::parser::automata::ENUMERATOR_AUTOMATON.captures(next_line) {

      let (next_number, next_kind, next_delims) = match Parser::enum_captures_to_int_kind_and_delims(&next_captures, None, false, None, None) {
        Some((number, kind, delims)) => (number, kind, delims),
        None => return TransitionResult::Failure {
          message: format!("Line following line {} conformed to enumerator pattern but no valid enumerator found? Computer says no...", line_cursor.sum_total()),
          doctree: doctree
        }
      };
      if ! (next_number == detected_number + 1 && next_kind == detected_kind && next_delims == detected_delims) {
        eprintln!("Non-matching enumerator n next line...");
        return crate::parser::state_machine::body::text (src_lines, base_indent, section_level, line_cursor, Some(doctree), captures, pattern_name)
      } else {
        //
      }

    } else {
      // This is an anumerated list item
    }
  } else {
    // No next line so proceed as usual
  }

  // let (detected_delims, detected_kind) = if let PatternName::Enumerator ( delims, kind) = pattern_name {
  //   (*delims, *kind)
  // } else {
  //   return TransitionResult::Failure {
  //     message: format!("No enumerator inside enumerator transition method on line {}. Computer says no...", line_cursor.sum_total()),
  //     doctree: doctree
  //   }
  // };

  // let (detected_enum_as_usize, detected_kind) = match Parser::enum_str_to_int_and_kind(detected_enum_str, &detected_kind, &list_kind, true, Some(*n_of_items), Some(*list_start_index)) {
  //   Some((int, kind)) => (int, kind),
  //   None => return TransitionResult::Failure {
  //     message: format!("Unknown enumerator type detected on line {}. Computer says no...", line_cursor.sum_total()),
  //     doctree: doctree
  //   }
  // };

  if *list_delims == detected_delims && detected_kind == *list_kind && *list_enumerator_indent == detected_enumerator_indent && detected_number == *n_of_items + *list_start_index {

    // Modify relevant list parameters
    *n_of_items += 1;

    let item_node_data = TreeNodeType::EnumeratedListItem {
      delims: *list_delims,
      kind: detected_kind,
      index_in_list: detected_number,//detected_enum_as_usize,
      enumerator_indent: detected_enumerator_indent,
      text_indent: detected_text_indent
    };

    doctree = match doctree.push_data_and_focus(item_node_data) {
      Ok(tree) => tree,
      Err(tree) => return TransitionResult::Failure {
        message: format!("Node insertion error on line {}. Computer says no...", line_cursor.sum_total()),
        doctree: tree
      }
    };

    let (doctree, offset, state_stack) = match Parser::parse_first_node_block(doctree, src_lines, base_indent, line_cursor, detected_text_indent, None, State::ListItem, section_level, false) {
      Ok((parsing_result, offset)) => if let ParsingResult::EOF { doctree, state_stack } | ParsingResult::EmptyStateStack { doctree, state_stack } = parsing_result {
        (doctree, offset, state_stack)
      } else {
        unreachable!("Returned from a nested parsing session on line {} without necessary information. Computer says no...", line_cursor.sum_total())
      },
      Err(ParsingResult::Failure { message, doctree }) => return TransitionResult::Failure {
        message: format!("Looks like enumerated list item on line {} has no content. Computer says no...", line_cursor.sum_total()),
        doctree: doctree
      },
      _ => unreachable!("Parsing first node block on line {} resulted in unknown combination of return values. Computer says no...", line_cursor.sum_total())
    };

    return TransitionResult::Success {
      doctree: doctree,
      next_states: Some(state_stack),
      push_or_pop: PushOrPop::Push,
      line_advance: LineAdvance::Some(offset),
    }

  } else {
    doctree = doctree.focus_on_parent();

    return TransitionResult::Success {
      doctree: doctree,
      next_states: None,
      push_or_pop: PushOrPop::Pop,
      line_advance: LineAdvance::None,
    }
  }
}
