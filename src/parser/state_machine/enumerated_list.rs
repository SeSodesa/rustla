/// ## enumerated_list
/// A submodule for `Statemachine::EnumeratedList` related transition functions.

use super::*;

pub fn enumerator (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();

  let (list_delims, list_kind, list_start_index, list_item_number,list_enumerator_indent, list_text_indent) = match &mut tree_wrapper.tree.node.data {
    TreeNodeType::EnumeratedList { delims, kind, start_index, n_of_items, enumerator_indent, latest_text_indent } => (delims, kind, start_index, n_of_items, enumerator_indent, latest_text_indent),
    _ => return TransitionResult::Failure {
      message: String::from("Not focused on EnumeratedList...\n")
    }
  };

  let detected_enumerator_indent = captures.get(1).unwrap().as_str().chars().count();
  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count();
  let detected_enum_str = captures.get(2).unwrap().as_str();

  let (detected_delims, detected_kind) = if let PatternName::Enumerator { delims, kind} = pattern_name {
    (*delims, *kind)
  } else {
    return TransitionResult::Failure {
      message: String::from("No enumerator inside enumerator transition method.\nWhy...?\n")
    }
  };

  let (detected_enum_as_usize, detected_kind) = match Parser::enum_str_to_int_and_kind(detected_enum_str, &detected_kind, None) {
    Some((int, kind)) => (int, kind),
    None => return TransitionResult::Failure {
      message: String::from("Unknown enumerator type detected...?\n")
    }
  };

  eprintln!("Detected enumerator type pair ({:#?}, {:#?}) as {:#?}...\n", detected_delims, detected_kind, detected_enum_as_usize);

  // Matching detected parameters against corresponding list ones and proceeding accordingly 
  match (detected_delims, detected_kind, &detected_enumerator_indent, &detected_text_indent) {

    (delims, kind, enum_indent, text_indent) if delims == *list_delims && kind == *list_kind && enum_indent == list_enumerator_indent && detected_enum_as_usize == *list_item_number + 1 => {

      // All parameters are the same, so this ListItem is a direct child of the current EnumeratedList.
      // Create a new ListItem node, focus on it and push a ListItem state on top of the parser stack.

      match &mut tree_wrapper.tree.node.data {
        TreeNodeType::EnumeratedList {n_of_items, latest_text_indent, ..} => {
          *n_of_items += 1;
          *latest_text_indent = *text_indent;
        },
        _ => return TransitionResult::Failure {
          message: String::from("Only enumerated lists keep track of the number of item nodes in them...\n")
        }
      }

      let item_node_data = TreeNodeType::EnumeratedListItem {
        delims: delims,
        kind: kind,
        index_in_list: detected_enum_as_usize,
        enumerator_indent: *enum_indent,
        text_indent: *text_indent
      };

      tree_wrapper.tree = tree_wrapper.tree.push_and_focus(item_node_data).unwrap();

      // Read indented block here
      let block = match Parser::read_indented_block(src_lines, Some(*current_line), Some(true), None, Some(*text_indent), Some(*text_indent)) {
        Ok((lines, min_indent, line_offset, blank_finish)) => {
          lines.join("\n")
        }
        Err(e) => {
          eprintln!("{}", e);
          return TransitionResult::Failure {
            message: String::from("Error when reading list item block.\n")
          }
        }
      };

      // Pass text to inline parser as a string
      let mut inline_nodes = if let Some(children) = Parser::inline_parse(block, current_line) {
        children
      } else {
        Vec::new()
      };

      let mut paragraph_node = TreeNode::new(TreeNodeType::Paragraph);
      paragraph_node.append_children(&mut inline_nodes);

      tree_wrapper.tree.push_child(paragraph_node);

      let next_state = StateMachine::ListItem;

      TransitionResult::Success {
        doctree: tree_wrapper,
        next_state: Some(StateMachine::ListItem),
        push_or_pop: PushOrPop::Push,
        line_advance: LineAdvance::Some(1)
      }

    }

    _ => {
      eprintln!("No specific instruction for found detected enumerator parameters.\nSimply POPping from stack in hopes of the previous state knowing better...\n");

      tree_wrapper.tree = tree_wrapper.tree.focus_on_parent().unwrap();

      TransitionResult::Success {
        doctree: tree_wrapper,
        next_state: None,
        push_or_pop: PushOrPop::Pop,
        line_advance: LineAdvance::None
      }

    }
  }

}


fn check_item_for_inner_enumerator (src_lines: &Vec<String>, current_line: usize, item_text_indent: usize) -> Option<(usize, usize, EnumDelims, EnumKind)> {

  const ENUMERATOR_PATTERNS: [(PatternName, &str); 15] = [
    (PatternName::Enumerator{delims: EnumDelims::Parens, kind: EnumKind::Arabic}, StateMachine::ARABIC_PARENS_PATTERN),
    (PatternName::Enumerator{delims: EnumDelims::RParen, kind: EnumKind::Arabic}, StateMachine::ARABIC_RPAREN_PATTERN),
    (PatternName::Enumerator{delims: EnumDelims::Period, kind: EnumKind::Arabic}, StateMachine::ARABIC_PERIOD_PATTERN),

    (PatternName::Enumerator{delims: EnumDelims::Parens, kind: EnumKind::LowerAlpha}, StateMachine::LOWER_ALPHA_PARENS_PATTERN),
    (PatternName::Enumerator{delims: EnumDelims::RParen, kind: EnumKind::LowerAlpha}, StateMachine::LOWER_ALPHA_RPAREN_PATTERN),
    (PatternName::Enumerator{delims: EnumDelims::Period, kind: EnumKind::LowerAlpha}, StateMachine::LOWER_ALPHA_PERIOD_PATTERN),

    (PatternName::Enumerator{delims: EnumDelims::Parens, kind: EnumKind::UpperAlpha}, StateMachine::UPPER_ALPHA_PARENS_PATTERN),
    (PatternName::Enumerator{delims: EnumDelims::RParen, kind: EnumKind::UpperAlpha}, StateMachine::UPPER_ALPHA_RPAREN_PATTERN),
    (PatternName::Enumerator{delims: EnumDelims::Period, kind: EnumKind::UpperAlpha}, StateMachine::UPPER_ALPHA_PERIOD_PATTERN),

    (PatternName::Enumerator{delims: EnumDelims::Parens, kind: EnumKind::LowerRoman}, StateMachine::LOWER_ROMAN_PARENS_PATTERN),
    (PatternName::Enumerator{delims: EnumDelims::RParen, kind: EnumKind::LowerRoman}, StateMachine::LOWER_ALPHA_RPAREN_PATTERN),
    (PatternName::Enumerator{delims: EnumDelims::Period, kind: EnumKind::LowerRoman}, StateMachine::LOWER_ROMAN_PERIOD_PATTERN),

    (PatternName::Enumerator{delims: EnumDelims::Parens, kind: EnumKind::UpperRoman}, StateMachine::UPPER_ROMAN_PARENS_PATTERN),
    (PatternName::Enumerator{delims: EnumDelims::RParen, kind: EnumKind::UpperRoman}, StateMachine::UPPER_ROMAN_RPAREN_PATTERN),
    (PatternName::Enumerator{delims: EnumDelims::Period, kind: EnumKind::UpperRoman}, StateMachine::UPPER_ROMAN_PERIOD_PATTERN),
  ];


  lazy_static::lazy_static! {
    static ref COMPILED_ENUM_PATTERNS: Vec<(PatternName, regex::Regex)> = {

      let mut compiled_patterns = Vec::with_capacity(ENUMERATOR_PATTERNS.len());

      for (pattern_name, pattern) in ENUMERATOR_PATTERNS.iter() {
        compiled_patterns.push((*pattern_name, regex::Regex::new(pattern).unwrap()));
      }

      compiled_patterns

    };
  }

  // Drain current line of parent list enumerator to test for nested enumerator
  let mut chars = src_lines[current_line].chars();
  for _ in 0..item_text_indent {
    chars.next();
  }
  let line_without_enum = chars.as_str();

  // Match current line against possible enumerators and return
  // prematurely with indent (for reading the contained paragraph)
  // and type if a match is found
  for (pattern_name, regexp) in COMPILED_ENUM_PATTERNS.iter() {

    let capts: regex::Captures = if let Some(capts) = regexp.captures(line_without_enum) {
      capts // match found
    } else {
      continue
    };

    let detected_enum_indent = capts.get(1).unwrap().as_str().chars().count() + item_text_indent;
    let detected_text_indent = capts.get(0).unwrap().as_str().chars().count() + item_text_indent;
    let detected_enum_str = capts.get(2).unwrap().as_str();

    let (detected_delims, detected_kind) = match pattern_name {
      PatternName::Enumerator{delims, kind} => (delims, kind),
      _ => return None // Shouldn't happen if patter was matched
    };

    let (enum_str_as_int, enum_kind) = match Parser::enum_str_to_int_and_kind(detected_enum_str, detected_kind, None) {
      Some((num, kind)) => (num, kind),
      None => return None // Shouldn't happen, if pattern was matched
    };

  }
  
  // No matches => no sublist on same line.
  None
}

enum NestedListResult {
  Some {
    enum_indent: usize,
    text_indent: usize,
    delims: EnumDelims,
    kind: EnumKind,
    
  },
  None
}
