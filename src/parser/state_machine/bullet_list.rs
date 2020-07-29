/// ## bullet_list
/// A submodule that contains `StateMachine::BulletList` related transition functions.

use super::*;


/// ### bullet
/// A `BulletList` version of the bullet list related
/// transition method. Differs from the `Body` state version
/// in that this detects whether a list of a different type has started
/// and acts accordingly.
pub fn bullet (src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();

  let detected_bullet = captures.get(2).unwrap().as_str().chars().next().unwrap();
  let detected_bullet_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
  let detected_text_indent = captures.get(0).unwrap().end() + base_indent;

  let (list_bullet, list_bullet_indent, list_text_indent) = match tree_wrapper.tree.node.data {
    doctree::TreeNodeType::BulletList{bullet, bullet_indent, text_indent} => (bullet, bullet_indent, text_indent),
    _ => {
      return TransitionResult::Failure {
        message: String::from("Only bullet list nodes contain bullets\nCannot compare detected bullet with parent...\n")
      }
    }
  };


  match tree_wrapper.tree.node.data {

    TreeNodeType::BulletList { bullet, bullet_indent, text_indent } => {

      if bullet == detected_bullet && bullet_indent == detected_bullet_indent && text_indent == detected_text_indent {
        // Still within same list based on indentation and bullet.
        // Create new ListItem node add a `ListItem` state on top of the state stack and proceed to
        // parse body elements on the same indentation level

        let item_node_data = TreeNodeType::BulletListItem{
          bullet: bullet,
          bullet_indent: detected_bullet_indent,
          text_indent: detected_text_indent
        };

        tree_wrapper = tree_wrapper.push_and_focus(item_node_data);

        let (doctree, offset, state_stack) = match Parser::first_list_item_block(tree_wrapper, src_lines, base_indent, current_line, detected_text_indent, None, StateMachine::ListItem) {
          Some((doctree, nested_parse_offset, state_stack)) => (doctree, nested_parse_offset, state_stack),
          None => return TransitionResult::Failure {message: format!("Could not parse the first block of list item on line {:#?}", current_line)}
        };

        tree_wrapper = doctree;

        return TransitionResult::Success {
          doctree: tree_wrapper,
          next_state: None,
          push_or_pop: PushOrPop::Push,
          line_advance: LineAdvance::Some(offset),
          nested_state_stack: Some(state_stack)
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

    _ => {
      return TransitionResult::Failure {
        message: String::from("Tried parsing a bullet list item osutide of a bullet list.\nComputer says no...\n")
      }
    }
  }
}
