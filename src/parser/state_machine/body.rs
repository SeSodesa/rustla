/// ## body
/// This module contains the transition functions related to `StateMachine::Body`.

use super::*;


/// ### bullet
/// The transition method for matching bullets in `Body` state.
/// Causes the parser to push a new machine in the state
/// `BulletList` on top of its machine stack. Leaves the reponsibility
/// of the actual parsing to that state.
pub fn bullet (src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();

  let detected_bullet = captures.get(2).unwrap().as_str().chars().next().unwrap();
  let detected_bullet_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count() + base_indent;

  let sublist_data = TreeNodeType::BulletList {
    bullet: detected_bullet,
    bullet_indent: detected_bullet_indent,
    text_indent: detected_text_indent,
  };

  if parent_indent_matches(&tree_wrapper.tree.node.data, detected_bullet_indent) {
    tree_wrapper = tree_wrapper.push_and_focus(sublist_data);
    return TransitionResult::Success {
      doctree: tree_wrapper,
      next_state: Some(StateMachine::BulletList),
      push_or_pop: PushOrPop::Push,
      line_advance: LineAdvance::None,
      nested_state_stack: None
    }
  } else {
    tree_wrapper = tree_wrapper.focus_on_parent();
    return TransitionResult::Success {
      doctree: tree_wrapper,
      next_state: None,
      push_or_pop: PushOrPop::Pop,
      line_advance: LineAdvance::None,
      nested_state_stack: None
    }
  }

}


/// ### enumerator
/// Transition method for matching enumerators in the `Body` state.
/// Attempts to create a new enumerated list node and focus on it,
/// while at the same time pushing a new `EnumeratedList` state on
/// top of the parser machine stack.
/// 
/// This does not yet parse the first detected list item.
/// That responsibility is on the corresponding enumerator method
/// of the `EnumeratedList` state.
pub fn enumerator (src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();

  let detected_enumerator_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count() + base_indent;
  let detected_enum_str = captures.get(2).unwrap().as_str();

  let (detected_delims, detected_kind) = if let PatternName::Enumerator { delims, kind} = pattern_name {
    (*delims, *kind)
  } else {
    return TransitionResult::Failure {
      message: String::from("No enumerator inside enumerator transition method.\nWhy...?\n")
    }
  };

  let (detected_enum_as_usize, detected_kind) = match Parser::enum_str_to_int_and_kind(detected_enum_str, &detected_kind, &detected_kind, false, None, None) {
    Some((int, kind)) => (int, kind),
    None => return TransitionResult::Failure {
      message: String::from("Unknown enumerator type detected...?\n")
    }
  };

  let list_node_data = TreeNodeType::EnumeratedList {
    delims: detected_delims,
    kind: detected_kind,
    start_index: detected_enum_as_usize,
    n_of_items: 0,
    enumerator_indent: detected_enumerator_indent,
    latest_text_indent: detected_text_indent,
  };

  if parent_indent_matches(&tree_wrapper.tree.node.data, detected_enumerator_indent) {
    tree_wrapper = tree_wrapper.push_and_focus(list_node_data);
    return TransitionResult::Success {
      doctree: tree_wrapper,
      next_state: Some(StateMachine::EnumeratedList),
      push_or_pop: PushOrPop::Push,
      line_advance: LineAdvance::None,
      nested_state_stack: None
    }
  } else {
    tree_wrapper = tree_wrapper.focus_on_parent();
    return TransitionResult::Success {
      doctree: tree_wrapper,
      next_state: None,
      push_or_pop: PushOrPop::Pop,
      line_advance: LineAdvance::None,
      nested_state_stack: None
    }
  }

}


/// ### field_marker
/// A transitioin function for handling detected field markers in a state that generates body type nodes.
pub fn field_marker (src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();

  let detected_marker_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;

  let list_node_data = TreeNodeType::FieldList {
    marker_indent: detected_marker_indent
  };

  // Match against the parent node. Only document root ignores indentation;
  // inside any other container it makes a difference.
  if parent_indent_matches(&tree_wrapper.tree.node.data, detected_marker_indent) {
    tree_wrapper = tree_wrapper.push_and_focus(list_node_data);
    return TransitionResult::Success {
      doctree: tree_wrapper,
      next_state: Some(StateMachine::FieldList),
      push_or_pop: PushOrPop::Push,
      line_advance: LineAdvance::None,
      nested_state_stack: None
    }
  } else {
    tree_wrapper = tree_wrapper.focus_on_parent();
    return TransitionResult::Success {
      doctree: tree_wrapper,
      next_state: None,
      push_or_pop: PushOrPop::Pop,
      line_advance: LineAdvance::None,
      nested_state_stack: None
    }
  }
}


/// ### footnote
/// A transition function for generating footnotes
pub fn footnote (src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();


  // Detected parameters...
  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count() + base_indent;
  let detected_marker_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
  let detected_label_str = captures.get(2).unwrap().as_str();

  let detected_body_indent = if let Some(line) = src_lines.get(*current_line + 1) {
    if line.is_empty() {
      detected_text_indent
    } else {
      line.chars().take_while(|c| !c.is_whitespace()).count() + base_indent
    }
  } else {
    detected_text_indent
  };

  let detected_label_as_int = if let Some( int) = detected_footnote_label_to_ref_label(&tree_wrapper, pattern_name, detected_label_str) {
    int
  } else {
    return TransitionResult::Failure {
      message: String::from("Cound not transform a footnote marker into an integer.\nComputer says no...\n")
    }
  };

  let footnote_data = TreeNodeType::Footnote {
    body_indent: detected_body_indent,
    label: detected_label_str.to_string(),
  };


  // Match against the parent node. Only document root ignores indentation;
  // inside any other container it makes a difference.
  if parent_indent_matches(&tree_wrapper.tree.node.data, detected_marker_indent) {
    tree_wrapper = tree_wrapper.push_and_focus(footnote_data);
    return TransitionResult::Success {
      doctree: tree_wrapper,
      next_state: Some(StateMachine::Footnote),
      push_or_pop: PushOrPop::Push,
      line_advance: LineAdvance::None,
      nested_state_stack: None
    }
  } else {
    tree_wrapper = tree_wrapper.focus_on_parent();
    return TransitionResult::Success {
      doctree: tree_wrapper,
      next_state: None,
      push_or_pop: PushOrPop::Pop,
      line_advance: LineAdvance::None,
      nested_state_stack: None
    }
  }
}


/// ### citation
/// A transition function for generating citations
pub fn citation (src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {
  todo!()
}


/// ### directive
/// A transition function for parsing directives in a state that recognizes body elements.
pub fn directive (src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {
  todo!()
}


/// ### paragraph
/// A function that handles the parsing of paragraphs of text.
pub fn paragraph (src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();
  let detected_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;

  let paragraph_data = TreeNodeType::Paragraph;
  let mut paragraph_node = TreeNode::new_from_id_ref(paragraph_data, &mut tree_wrapper.node_count);

  let relative_indent = detected_indent - base_indent;

  let block = match Parser::read_text_block(src_lines, *current_line, true, true, Some(relative_indent)) {
    Ok((lines, line_offset)) => {
      lines.join("\n")
    }
    Err(e) => {
      eprintln!("{}", e);
      return TransitionResult::Failure {
        message: String::from("Error when reading paragraph block in Body.\n")
      }
    }
  };

  // Pass text to inline parser as a string
  let mut inline_nodes = if let Some(children) = Parser::inline_parse(block, current_line, &mut tree_wrapper.node_count) {
    children
  } else {
    return TransitionResult::Failure {
      message: String::from("Couldn't parse paragraph for inline nodes\n")
    }
  };

  // Add inline nodes to paragraph...
  paragraph_node.append_children(&mut inline_nodes);

  // Check if we are inside a node that cares about indentation
  if parent_indent_matches(&tree_wrapper.tree.node.data, detected_indent) {
    tree_wrapper.tree.push_child(paragraph_node);
    return TransitionResult::Success {
      doctree: tree_wrapper,
      next_state: None,
      push_or_pop: PushOrPop::Neither,
      line_advance: LineAdvance::Some(1),
      nested_state_stack: None
    }
  } else {
    tree_wrapper = tree_wrapper.focus_on_parent();
    return TransitionResult::Success {
      doctree: tree_wrapper,
      next_state: None,
      push_or_pop: PushOrPop::Pop,
      line_advance: LineAdvance::None,
      nested_state_stack: None
    }
  }

}


// ==================
//  Helper functions
// ==================


/// ### parent_indent_matches
/// Checks the indentation of the given parent (current) node and whether the relevant detected indent matches with it.
/// If the indent matches, we can push to the current node and focus on the new node. Otherwise
fn parent_indent_matches (parent_data: &TreeNodeType, relevant_detected_indent: usize) -> bool {

  // Match against the parent node. Only document root ignores indentation;
  // inside any other container it makes a difference.
  match parent_data {

    TreeNodeType::Root { .. } => true,

    TreeNodeType::BulletListItem {text_indent, .. } | TreeNodeType::EnumeratedListItem { text_indent, .. } => {
      if relevant_detected_indent == *text_indent { true } else { false }
    }

    TreeNodeType::FieldListItem {body_indent, .. } => {
      if relevant_detected_indent == *body_indent { true } else { false }
    },

    // Add other cases here...

    _ => false
  }

}


/// ### foonote_label_to_int
/// Converts a foonote label into an ordinal based on the current state of `DocTree.foonote_data`,
/// if possible. Returns the `Option`al integer, if successful.
pub fn detected_footnote_label_to_ref_label (doctree: &DocTree, pattern_name: &PatternName, detected_label_str: &str) -> Option<(EnumAsInt, String)> {

  use std::convert::TryFrom;

  if let PatternName::Footnote { kind } = pattern_name {
    match kind {
      FootnoteKind::Manual => {

        // In this case the doctree is simply asked whether it has a reference
        // with this name. If yes, the user is warned of a duplicate label,
        // but otherwise no special action is taken.

        todo!()
      }

      FootnoteKind::AutoNumbered => {

        // Here we iterate the set of all possible `u32` values
        // and once a number that has not been used as a label is found,
        // it is returned.

        while let Some(n) = (1..EnumAsInt::MAX).next() {

          // If n is not in the tree, return it

        }

        todo!()
      }

      FootnoteKind::SimpleRefName => {

        // Same as with manual footnotes, check if this has already a number representation
        // in the doctree and if not, return it.

        todo!()
      }

      FootnoteKind::AutoSymbol => {

        // Generate a label from crate::common::FOONOTE_SYMBOLS based on the number of autosymbol footnotes
        // entered into the document thus far.

        todo!()
      }
    }
  } else {
    eprintln!("No footnote pattern inside a footnote transition function.\nComputer says no...\n");
    None
  }
}
