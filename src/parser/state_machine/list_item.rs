/// ## list_item
/// A submodule for function related to `StateMachine::ListItem` transition functions.

use super::*;


/// ### bullet
/// A bullet detected within a `ListItem` state either signifies a start of a new superlist or a sublist of the current list.
pub fn bullet (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>, PushOrPop, LineAdvance), &'static str> {
  
  let mut tree_wrapper = doctree.unwrap();

  eprintln!("{:#?}", tree_wrapper.tree.node.data);

  let (list_item_bullet, list_item_bullet_indent, list_item_text_indent) = match tree_wrapper.tree.node.data {
    TreeNodeType::BulletListItem{bullet, bullet_indent, text_indent} => (bullet, bullet_indent, text_indent),
    _ => return Err("Not focused on list item.\nCannot ask for bullet and indentation.\n")
  };

  let detected_bullet = captures.get(2).unwrap().as_str().chars().next().unwrap();
  let detected_bullet_indent = captures.get(1).unwrap().as_str().chars().count();
  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count();

  // Match against bullet and indentation.

  match (detected_bullet, detected_bullet_indent, detected_text_indent) {

    (bullet, b_indent, t_indent) if bullet == list_item_bullet && b_indent == list_item_bullet_indent => {

      // If they are the same, we have detected another list item on the same level
      // and need to move back to parent list so it might be appended.

      tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
        Ok(tree) => tree,
        Err(tree) => return Err("Couldn't focus on parent bullet list")
      };
      
      eprintln!("{:#?}\n", tree_wrapper.tree.node.data);
      return Ok( ( Some(tree_wrapper), None, PushOrPop::Pop, LineAdvance::None ) )

    }

    (bullet, b_indent, t_indent) if t_indent < list_item_text_indent => {
      // Indentation less than that of the current list item => probably a parent
      // list item was detected => need to move focus to said list and pop from
      // parser machine stack until corresponding level of nesting is reached.

      tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
        Ok(tree) => tree,
        Err(tree) => return Err("Couldn't focus on parent bullet list")
      };

      eprintln!("{:#?}\n", tree_wrapper.tree.node.data);
      return Ok( ( Some(tree_wrapper), None, PushOrPop::Pop, LineAdvance::None ) )

    }

    (bullet, b_indent, t_indent) if b_indent >= list_item_text_indent => {
      // Indent greater than that of the current item means a sublist has started,
      // again, assuming that it aligns with the left edge of the list item.

      tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
        Ok(tree) => tree,
        Err(tree) => return Err("Couldn't focus on parent bullet list")
      };

      eprintln!("{:#?}\n", tree_wrapper.tree.node.data);

      return Ok( ( Some(tree_wrapper), None, PushOrPop::Pop, LineAdvance::None ) )
    }

    _ => {
      eprint!("No action for such (bullet, bullet indent, text indent) = ({}, {}, {}) combination.\n", detected_bullet, detected_bullet_indent, detected_text_indent);
      return Err("")
    }
  }
}


/// ### enumerator
/// An enumerator detected within a `ListItem` state either signifies a start of a new superlist or a sublist of the current list.
pub fn enumerator (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>, PushOrPop, LineAdvance), &'static str> {

  let mut tree_wrapper = doctree.unwrap();

  eprintln!("{:#?}", tree_wrapper.tree.node.data);

  let (list_item_bullet_indent, list_item_text_indent) = match tree_wrapper.tree.node.data {
    TreeNodeType::EnumeratedListItem{enumerator_indent, text_indent, ..} => (enumerator_indent, text_indent),
    _ => return Err("Not focused on enumerator list item.\nCannot ask for enumerator and indentation...\n")
  };

  let detected_enumerator_indent = captures.get(1).unwrap().as_str().chars().count();
  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count();
  let detected_enum_str = captures.get(2).unwrap().as_str();

  let (detected_delims, detected_kind) = if let PatternName::Enumerator { delims, kind} = pattern_name {
    (*delims, *kind)
  } else {
    return Err("No enumerator inside enumerator transition method.\nWhy...?\n")
  };

  tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
    Ok(tree) => tree,
    Err(tree) => return Err("Couldn't focus on parent bullet list")
  };

  eprintln!("{:#?}\n", tree_wrapper.tree.node.data);

  return Ok( ( Some(tree_wrapper), None, PushOrPop::Pop, LineAdvance::None ) )

}

/// ### paragraph
/// Direct child nodes of list items may only be paragraphs.
/// This function parses each paragraph in a list item for inline nodes.
/// A paragraph must have at least the same level of indentation as the containing list item,
/// otherwise is it interpreted as ending the current list item.
pub fn paragraph (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>, PushOrPop, LineAdvance), &'static str> {
  
  let mut tree_wrapper = doctree.unwrap();

  let (item_bullet_indent, item_text_indent) = match tree_wrapper.tree.node.data {
    TreeNodeType::BulletListItem{bullet_indent, text_indent, ..} => (bullet_indent, text_indent),
    TreeNodeType::EnumeratedListItem{enumerator_indent, text_indent, ..} => ( enumerator_indent, text_indent),
    _ => return Err("Failed to retrieve bullet list item indentation info when parsing a paragraph.\n")
  };

  let detected_par_indent = captures.get(1).unwrap().as_str().chars().count();

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

      tree_wrapper.tree.push_child(paragraph_node);
      

      Ok( ( Some(tree_wrapper), None, PushOrPop::Neither, LineAdvance::Some(1) ) )
    }

    indent if indent < item_text_indent => {

      // Less indentation means the paragraph is not a part of this
      // nested list. Possibly a continuation of the previous list item.
      // Focus on parent and pop from stack.
    
      tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
        Ok(tree) => tree,
        Err(tree) => return Err("Bullet list outer paragraph detected, but no parent?\n")
      };

      Ok( ( Some(tree_wrapper), None, PushOrPop::Pop, LineAdvance::None ) )

    }

    indent if indent > item_text_indent => {

      // More indentation might mean that there is a literal block as a part of this list item.
      todo!()
    }

    _ => todo!() // The other options still need to be figured out

  }
}
