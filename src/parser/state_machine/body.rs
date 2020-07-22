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

  // Are we focused on a node where we care about the indentation and such?
  match tree_wrapper.tree.node.data {

    TreeNodeType::BulletListItem {bullet, bullet_indent, text_indent } => {

      match (detected_bullet, detected_bullet_indent, detected_text_indent) {
        (detected_bullet, detected_bullet_indent, detected_text_indent)
        if detected_bullet_indent == text_indent => {

          // Focused on list node and sublist detected...

          let sublist_data = TreeNodeType::BulletList {
            bullet: detected_bullet,
            bullet_indent: detected_bullet_indent,
            text_indent: detected_text_indent,
          };

          tree_wrapper.tree = tree_wrapper.tree.push_and_focus(sublist_data).unwrap();

          return TransitionResult::Success {
            doctree: tree_wrapper,
            next_state: Some(StateMachine::BulletList),
            push_or_pop: PushOrPop::Push,
            line_advance: LineAdvance::None,
            nested_state_stack: None
          }

        }

        (_,_,_) => { // Not a sublist so must be a parent list or some such

          tree_wrapper.tree = tree_wrapper.tree.focus_on_parent().unwrap();

          return TransitionResult::Success {
            doctree: tree_wrapper,
            next_state: None,
            push_or_pop: PushOrPop::Pop,
            line_advance: LineAdvance::None,
            nested_state_stack: None
          }

        }
      }

    }

    _ => () // No troublesome data found
      
  }

  let bullet_list_data = TreeNodeType::BulletList{
    bullet: detected_bullet,
    bullet_indent: detected_bullet_indent,
    text_indent: detected_text_indent
  };

  tree_wrapper.tree = match tree_wrapper.tree.push_and_focus(bullet_list_data) {
    Ok(tree) => tree,
    Err(..) => return TransitionResult::Failure {
      message: String::from("Couldn't focus on bullet list...\n")
    }
  };
    
  TransitionResult::Success {
    doctree: tree_wrapper,
    next_state: Some(StateMachine::BulletList),
    push_or_pop: PushOrPop::Push,
    line_advance: LineAdvance::None,
    nested_state_stack: None
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

  let (detected_enum_as_usize, detected_kind) = match Parser::enum_str_to_int_and_kind(detected_enum_str, &detected_kind, None) {
    Some((int, kind)) => (int, kind),
    None => return TransitionResult::Failure {
      message: String::from("Unknown enumerator type detected...?\n")
    }
  };

  eprintln!("Start index: {}\n", detected_enum_as_usize);

  let list_node_data = TreeNodeType::EnumeratedList {
    delims: detected_delims,
    kind: detected_kind,
    start_index: detected_enum_as_usize,
    n_of_items: 0,
    enumerator_indent: detected_enumerator_indent,
    latest_text_indent: detected_text_indent,
  };

  match tree_wrapper.tree.node.data {

    TreeNodeType::BulletListItem { text_indent, .. } | TreeNodeType::EnumeratedListItem { text_indent, .. } => {
      if detected_enumerator_indent == text_indent { // Contained paragraphs need to be aligned...

        tree_wrapper.tree = tree_wrapper.tree.push_and_focus(list_node_data).unwrap();

        return TransitionResult::Success {
          doctree: tree_wrapper,
          next_state: Some(StateMachine::EnumeratedList),
          push_or_pop: PushOrPop::Push,
          line_advance: LineAdvance::None,
          nested_state_stack: None
        }

      } else { // paragraph does not belong to this item

        tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
          Ok(tree)  => tree,
          Err(tree) => {
            eprintln!("INFO: focused on tree root inside transition method...\n");
            tree
          }
        };

        return TransitionResult::Success {
          doctree: tree_wrapper,
          next_state: None,
          push_or_pop: PushOrPop::Pop,
          line_advance: LineAdvance::None,
          nested_state_stack: None
        }
      }
    }

    _ => ()

  }


  tree_wrapper.tree = tree_wrapper.tree.push_and_focus(list_node_data).unwrap();

  TransitionResult::Success {
    doctree: tree_wrapper,
    next_state: Some(StateMachine::EnumeratedList),
    push_or_pop: PushOrPop::Push,
    line_advance: LineAdvance::None,
    nested_state_stack: None
  }

}


pub fn field_marker (src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();

  let detected_marker_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;

  let list_node_data = TreeNodeType::FieldList {
    marker_indent: detected_marker_indent
  };

  // Match against the parent node. Only document root ignores indentation;
  // inside any other container it makes a difference.
  match tree_wrapper.tree.node.data {

    TreeNodeType::BulletListItem {text_indent, .. } | TreeNodeType::EnumeratedListItem { text_indent, .. } => {
      if detected_marker_indent == text_indent {
        tree_wrapper.tree = tree_wrapper.tree.push_and_focus(list_node_data).unwrap();
        return TransitionResult::Success {
          doctree: tree_wrapper,
          next_state: Some(StateMachine::FieldList),
          push_or_pop: PushOrPop::Push,
          line_advance: LineAdvance::None,
          nested_state_stack: None
        }
      } else {
        return TransitionResult::Success {
          doctree: tree_wrapper,
          next_state: None,
          push_or_pop: PushOrPop::Pop,
          line_advance: LineAdvance::None,
          nested_state_stack: None
        }
      }
    }

    TreeNodeType::FieldListItem {body_indent, .. } => {
      if detected_marker_indent == body_indent {
        tree_wrapper.tree = tree_wrapper.tree.push_and_focus(list_node_data).unwrap();
        return TransitionResult::Success {
          doctree: tree_wrapper,
          next_state: Some(StateMachine::FieldList),
          push_or_pop: PushOrPop::Push,
          line_advance: LineAdvance::None,
          nested_state_stack: None
        }
      } else {
        return TransitionResult::Success {
          doctree: tree_wrapper,
          next_state: None,
          push_or_pop: PushOrPop::Pop,
          line_advance: LineAdvance::None,
          nested_state_stack: None
        }
      }
    }

    _ => {} // else, do nothing...
  }

  // If no special container, ignore indentation and blindly push to tree
  tree_wrapper.tree = tree_wrapper.tree.push_and_focus(list_node_data).unwrap();
  TransitionResult::Success {
    doctree: tree_wrapper,
    next_state: Some(StateMachine::FieldList),
    push_or_pop: PushOrPop::Push,
    line_advance: LineAdvance::None,
    nested_state_stack: None
  }
}


pub fn paragraph (src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();
  let detected_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;

  let paragraph_data = TreeNodeType::Paragraph;

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
  let mut inline_nodes = if let Some(children) = Parser::inline_parse(block, current_line) {
    children
  } else {
    return TransitionResult::Failure {
      message: String::from("Couldn't parse paragraph for inline nodes\n")
    }
  };

  // Construct paragraph...
  let mut paragraph_node = TreeNode::new(paragraph_data);
  paragraph_node.append_children(&mut inline_nodes);

  // Check if we are inside a node that cares about indentation
  match tree_wrapper.tree.node.data {

    TreeNodeType::BulletListItem {text_indent, .. } | TreeNodeType::EnumeratedListItem {text_indent, .. } => {

      if detected_indent == text_indent { // Contained paragraphs need to be aligned...
        tree_wrapper.tree.push_child(paragraph_node);
        return TransitionResult::Success {
          doctree: tree_wrapper,
          next_state: None,
          push_or_pop: PushOrPop::Neither,
          line_advance: LineAdvance::Some(1),
          nested_state_stack: None
        }
      } else { // paragraph does not belong to this item
        tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
          Ok(tree)  => tree,
          Err(tree) => {
            eprintln!("INFO: focused on tree root inside transition method...\n");
            tree
          }
        };
        return TransitionResult::Success {
          doctree: tree_wrapper,
          next_state: None,
          push_or_pop: PushOrPop::Pop,
          line_advance: LineAdvance::None,
          nested_state_stack: None
        }
      }
    }

    TreeNodeType::FieldListItem {body_indent, .. } => {
      if detected_indent == body_indent {
        tree_wrapper.tree.push_child(paragraph_node);
        return TransitionResult::Success {
          doctree: tree_wrapper,
          next_state: None,
          push_or_pop: PushOrPop::Neither,
          line_advance: LineAdvance::Some(1),
          nested_state_stack: None
        }
      } else {
        tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
          Ok(tree)  => tree,
          Err(tree) => {
            eprintln!("INFO: focused on tree root inside transition method...\n");
            tree
          }
        };
        return TransitionResult::Success {
          doctree: tree_wrapper,
          next_state: None,
          push_or_pop: PushOrPop::Pop,
          line_advance: LineAdvance::None,
          nested_state_stack: None
        }
      }
    }

    _ => () // No troublesome indented nodes as parent, do nothing

  }

  // No troublesome nodes so simply push paragraph to current node
  tree_wrapper.tree.push_child(paragraph_node);

  TransitionResult::Success {
    doctree: tree_wrapper,
    next_state: None,
    push_or_pop: PushOrPop::Neither,
    line_advance: LineAdvance::Some(1),
    nested_state_stack: None
  }

}
