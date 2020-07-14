/// ## bullet_list
/// A submodule that contains `StateMachine::BulletList` related transition functions.

use super::*;


/// ### bullet
/// A `BulletList` version of the bullet list related
/// transition method. Differs from the `Body` state version
/// in that this detects whether a list of a different type has started
/// and acts accordingly.
pub fn bullet (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>, PushOrPop, LineAdvance), &'static str> {

  let mut tree_wrapper = doctree.unwrap();

  eprintln!("{:#?}\n", tree_wrapper.tree.node.data);

  let detected_item_bullet = captures.get(2).unwrap().as_str().chars().next().unwrap();
  let detected_bullet_indent = captures.get(1).unwrap().as_str().chars().count();
  let detected_text_indent = captures.get(0).unwrap().end();

  let (list_bullet, list_bullet_indent, list_text_indent) = match tree_wrapper.tree.node.data {
    doctree::TreeNodeType::BulletList{bullet, bullet_indent, text_indent} => (bullet, bullet_indent, text_indent),
    _ => {
      return Err("Only bullet list nodes contain bullets\nCannot compare detected bullet with parent...\n")
    }
  };

  // If bullet and indentation match with current list node, continue with current list.
  // Else check for possible sublist or need to break out of current list and act accordingly.
  match (detected_item_bullet, detected_bullet_indent, detected_text_indent) {

    (bullet, b_indent, t_indent) if bullet == list_bullet && b_indent == list_bullet_indent => {

      // Still within same list based on indentation and bullet.
      // Create new ListItem node add a `ListItem` state on top of the state stack and proceed to
      // parse body elements on the same indentation level

      let mut item_node = doctree::TreeNode::new(TreeNodeType::BulletListItem{bullet: bullet, bullet_indent: b_indent, text_indent: t_indent});
      let mut paragraph_node = doctree::TreeNode::new(TreeNodeType::Paragraph);

      // Read indented block here
      let block = match Parser::read_indented_block(src_lines, Some(*current_line), Some(true), None, Some(t_indent), Some(t_indent)) {
        Ok((lines, min_indent, line_offset, blank_finish)) => {
          if min_indent != t_indent {
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

      // Add inline nodes to Paragraph node
      paragraph_node.append_children(&mut inline_nodes);

      item_node.push_child(paragraph_node);

      tree_wrapper.tree.push_child(item_node);

      // Focus on the ListItem node after pushing it to the current bullet list
      // tree_wrapper.tree.push_child(item_node);
      tree_wrapper.tree = match tree_wrapper.tree.focus_on_last_child() {
        Ok(tree_zipper) =>tree_zipper,
        Err(node_itself) => {
          return Err("No child of type ListItem to be focused on.\n")
        }
      };

      let next_state = StateMachine::ListItem;

      return Ok((Some(tree_wrapper), Some(next_state), PushOrPop::Push, LineAdvance::Some(1)))

    },

    (bullet, b_indent, t_indent) if bullet != list_bullet && t_indent == list_text_indent => {

      // If bullet doesn't match but indent is the same, we have another list on the same level
      //   => simply move focus back to parent (body or another list) so the new list might be appended to it

      tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
        Ok(parent) => parent,
        Err(node_itself) => {
          return Err("Encountered list on same level but couldn't focus on list parent.\n")
        }
      };

      return Ok((Some(tree_wrapper), None, PushOrPop::Neither, LineAdvance::None))

    },

    (bullet, b_indent, t_indent) if b_indent < list_bullet_indent => {

      // Less indent after discovering a bullet means a sublist has ended,
      // regardless of bullet type.
      // Move focus back to parent and pop from machine stack.

      tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
        Ok(parent) => parent,
        Err(node_itself) => {
          return Err("Encountered a list item with less indent but couldn't focus on list parent.\n")
        }
      };

      return Ok((Some(tree_wrapper), None, PushOrPop::Pop, LineAdvance::None))

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
          return Err("An error occurred when shifting focus to sublist.\n");
        }
      };

      eprintln!("{:#?}\n", tree_wrapper.tree.node.data);

      return Ok((Some(tree_wrapper), Some(list_machine), PushOrPop::Push, LineAdvance::None))

    }

    _ => {
      return Err("No action for this type of bullet--indent combination")
    }
  }

}
