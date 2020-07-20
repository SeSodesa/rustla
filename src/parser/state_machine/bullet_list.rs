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

  let detected_item_bullet = captures.get(2).unwrap().as_str().chars().next().unwrap();
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

  // If bullet and indentation match with current list node, continue with current list.
  // Else check for possible sublist or need to break out of current list and act accordingly.
  match (detected_item_bullet, detected_bullet_indent, detected_text_indent) {

    (bullet, b_indent, t_indent) if bullet == list_bullet && b_indent == list_bullet_indent => {

      // Still within same list based on indentation and bullet.
      // Create new ListItem node add a `ListItem` state on top of the state stack and proceed to
      // parse body elements on the same indentation level

      let item_node_data = TreeNodeType::BulletListItem{
        bullet: bullet,
        bullet_indent: b_indent,
        text_indent: t_indent
      };

      tree_wrapper.tree = tree_wrapper.tree.push_and_focus(item_node_data).unwrap();

      tree_wrapper = match Parser::first_list_item_block(tree_wrapper, src_lines, base_indent, current_line, t_indent) {
        Some(doctree) => doctree,
        None => return TransitionResult::Failure {message: format!("Could not parse the first block of list item on line {:#?}", current_line)}
      };

      let next_state = StateMachine::ListItem;

      return TransitionResult::Success {
        doctree: tree_wrapper,
        next_state: Some(StateMachine::ListItem),
        push_or_pop: PushOrPop::Push,
        line_advance: LineAdvance::None
      }

    },

    (bullet, b_indent, t_indent) if bullet != list_bullet && t_indent == list_text_indent => {

      // If bullet doesn't match but indent is the same, we have another list on the same level
      //   => simply move focus back to parent (body or another list) so the new list might be appended to it

      tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
        Ok(parent) => parent,
        Err(node_itself) => {
          return TransitionResult::Failure {
            message: String::from("Encountered list on same level but couldn't focus on list parent.\n")
          }
        }
      };

      return TransitionResult::Success {
        doctree: tree_wrapper,
        next_state: None,
        push_or_pop: PushOrPop::Neither,
        line_advance: LineAdvance::None
      }

    },

    (bullet, b_indent, t_indent) if b_indent < list_bullet_indent => {

      // Less indent after discovering a bullet means a sublist has ended,
      // regardless of bullet type.
      // Move focus back to parent and pop from machine stack.

      tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
        Ok(parent) => parent,
        Err(node_itself) => {
          return TransitionResult::Failure {
            message: String::from("Encountered a list item with less indent but couldn't focus on list parent.\n")
          }
        }
      };

      return TransitionResult::Success {
        doctree: tree_wrapper,
        next_state: None,
        push_or_pop: PushOrPop::Pop,
        line_advance: LineAdvance::None
      }

    },

    (bullet, b_indent, t_indent) if b_indent == list_text_indent => {

      // More indent after discovering a bullet means a sublist has started,
      // regardless of bullet type.
      // Create an entirely new bullet list node, focus on it, add it to the children of the current list
      // and have the parser push a new bullet machine on top of the
      // parser stack to signify an increase in nesting level.

      let bullet_list_data = TreeNodeType::BulletList{bullet: bullet, bullet_indent: b_indent, text_indent: t_indent};

      let list_node = TreeNode::new(bullet_list_data);

      let list_machine = StateMachine::BulletList;

      tree_wrapper.tree.push_child(list_node);

      // Move focus to the nested list node
      tree_wrapper.tree = match tree_wrapper.tree.focus_on_last_child() {
        Ok(child_zipper) => child_zipper,
        Err(node_itself) => {
          return TransitionResult::Failure {
            message: String::from("An error occurred when shifting focus to sublist.\n")
          }
        }
      };

      eprintln!("{:#?}\n", tree_wrapper.tree.node.data);

      return TransitionResult::Success {
        doctree: tree_wrapper,
        next_state: Some(StateMachine::BulletList),
        push_or_pop: PushOrPop::Push,
        line_advance: LineAdvance::None
      }

    }

    _ => {
      return TransitionResult::Failure {
        message: String::from("No action for this type of bullet--indent combination\n")
      }
    }
  }

}
