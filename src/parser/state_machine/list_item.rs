/// ## list_item
/// A submodule for function related to `StateMachine::ListItem` transition functions.

use super::*;


/// ### bullet
/// A bullet detected within a `ListItem` state either signifies a start of a new superlist or a sublist of the current list.
pub fn bullet (src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {
  
  let mut tree_wrapper = doctree.unwrap();

  eprintln!("{:#?}", tree_wrapper.tree.node.data);

  let (list_item_bullet, list_item_bullet_indent, list_item_text_indent) = match tree_wrapper.tree.node.data {
    TreeNodeType::BulletListItem{bullet, bullet_indent, text_indent} => (bullet, bullet_indent, text_indent),
    _ => return TransitionResult::Failure {
      message: String::from("Not focused on list item.\nCannot ask for bullet and indentation.\n")
    }
  };

  let detected_bullet = captures.get(2).unwrap().as_str().chars().next().unwrap();
  let detected_bullet_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count() + base_indent;

  // Match against bullet and indentation.

  match (detected_bullet, detected_bullet_indent, detected_text_indent) {

    (bullet, b_indent, t_indent) if b_indent == list_item_text_indent => {

      // Bullet indent equal to list item text indent means there is a sublist under this list item.
      // Push a new BulletList state on top of the parser stack, a new BulletListNode as the child of the
      // current ListItem node and parse from current line forward.

      let list_node_data = TreeNodeType::BulletList{bullet: bullet, bullet_indent: b_indent, text_indent: t_indent};

      let list_node = TreeNode::new(list_node_data);

      tree_wrapper.tree.push_child(list_node);

      tree_wrapper.tree = match tree_wrapper.tree.focus_on_last_child() {
        Ok(tree) => tree,
        Err(..) => return TransitionResult::Failure {
          message: String::from("Couldn't focus on child bullet list under list item...\n")
        }
      };

      TransitionResult::Success {
        doctree: tree_wrapper,
        next_state: Some(StateMachine::BulletList),
        push_or_pop: PushOrPop::Push,
        line_advance: LineAdvance::None
      }

    }

    _ => {

      // There is a sublist, only if the indentation matches. Otherwise we will assume that
      // The list does not belong to this list item and reverse up the tree and parser state stack.

      tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
        Ok(tree) => tree,
        Err(tree) => tree, // at root
      };

      TransitionResult::Success {
        doctree: tree_wrapper,
        next_state: None,
        push_or_pop: PushOrPop::Pop,
        line_advance: LineAdvance::None
      }

    }
  }
}


/// ### enumerator
/// An enumerator detected within a `ListItem` state either signifies a start of a new superlist or a sublist of the current list.
pub fn enumerator (src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();

  eprintln!("{:#?}", tree_wrapper.tree.node.data);

  let (list_item_bullet_indent, list_item_text_indent) = match tree_wrapper.tree.node.data {
    TreeNodeType::EnumeratedListItem{enumerator_indent, text_indent, ..} => (enumerator_indent, text_indent),
    _ => return TransitionResult::Failure {
      message: String::from("Not focused on enumerator list item.\nCannot ask for enumerator and indentation...\n")
    }
  };

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

  let item_text_indent = match tree_wrapper.tree.node.data {
    TreeNodeType::BulletListItem{text_indent, ..} => text_indent,
    TreeNodeType::EnumeratedListItem{text_indent, ..} => text_indent,
    _ => return TransitionResult::Failure {
      message: String::from("Not focused on list item inside a list item function.\nWhy...?\n")
    }
  };

  match detected_enumerator_indent {

    enum_indent if enum_indent == item_text_indent => {

      // Detected marker indent matches with the text indentation of the current list item.
      // A new sublist has started, so act accordingly.

      eprintln!("Sublist with indetation {:#?} detected...\n", enum_indent);

      let (list_number, list_kind) = match Parser::enum_str_to_int_and_kind(detected_enum_str, &detected_kind, None) {
        Some((int, kind)) => (int, kind),
        None => return TransitionResult::Failure {
          message: String::from("Couldn't parse sublist item enumerator...\n")
        }
      };

      let list_node_data = TreeNodeType::EnumeratedList{
        delims: detected_delims,
        kind: list_kind,
        enumerator_indent: detected_enumerator_indent,
        latest_text_indent: detected_text_indent,
        n_of_items: 0,
        start_index: list_number
      };

      let list_node = TreeNode::new(list_node_data);

      tree_wrapper.tree.push_child(list_node);

      tree_wrapper.tree = tree_wrapper.tree.focus_on_last_child().unwrap();

      TransitionResult::Success {
        doctree: tree_wrapper,
        next_state: Some(StateMachine::EnumeratedList),
        push_or_pop: PushOrPop::Push,
        line_advance: LineAdvance::None
      }

    }

    _ => {

      // Not sublist. Pop from parser stack and focus on parent list node.

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

/// ### paragraph
/// Direct child nodes of list items may only be paragraphs.
/// This function parses each paragraph in a list item for inline nodes.
/// A paragraph must have at least the same level of indentation as the containing list item,
/// otherwise is it interpreted as ending the current list item.
pub fn paragraph (src_lines: &Vec<String>, base_indent: &usize, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {
  
  let mut tree_wrapper = doctree.unwrap();

  let (item_bullet_indent, item_text_indent) = match tree_wrapper.tree.node.data {
    TreeNodeType::BulletListItem{bullet_indent, text_indent, ..} => (bullet_indent, text_indent),
    TreeNodeType::EnumeratedListItem{enumerator_indent, text_indent, ..} => ( enumerator_indent, text_indent),
    _ => return TransitionResult::Failure {
      message: String::from("Failed to retrieve list item indentation info when parsing a paragraph inside one.\n")
    }
  };

  let detected_par_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;

  match detected_par_indent {

    t_indent if t_indent == item_text_indent => {

      // If the indentation of the paragraph matches with the text
      // indentation of the list item, the paragraph is a valid part of the item
      // and can be added to it.

      let mut paragraph_node = TreeNode::new(TreeNodeType::Paragraph);

      // Read indented block here
      let block = match Parser::read_indented_block(src_lines, Some(*current_line), Some(true), None, Some(t_indent), None) {
        Ok((lines, min_indent, line_offset, blank_finish)) => {
          if min_indent != item_text_indent {
            return TransitionResult::Failure {
              message: String::from("Indent of list item block was less than given.\n")
            }
          }
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

      // Add inline nodes to Paragraph node
      paragraph_node.append_children(&mut inline_nodes);

      tree_wrapper.tree.push_child(paragraph_node);
      
      TransitionResult::Success {
        doctree: tree_wrapper,
        next_state: None,
        push_or_pop: PushOrPop::Neither,
        line_advance: LineAdvance::Some(1)
      }

    }

    indent if indent < item_text_indent => {

      // Less indentation means the paragraph is not a part of this
      // nested list. Possibly a continuation of the previous list item.
      // Focus on parent and pop from stack.
    
      tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
        Ok(tree) => tree,
        Err(tree) => return TransitionResult::Failure {
          message: String::from("Bullet list outer paragraph detected, but no parent?\n")
        }
      };

      TransitionResult::Success {
        doctree: tree_wrapper,
        next_state: None,
        push_or_pop: PushOrPop::Pop,
        line_advance: LineAdvance::None
      }

    }

    indent if indent > item_text_indent => {

      // More indentation might mean that there is a literal block as a part of this list item.
      todo!()
    }

    _ => todo!() // The other options still need to be figured out

  }
}
