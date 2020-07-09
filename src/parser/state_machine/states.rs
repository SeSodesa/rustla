/// This submodule contains the states and related transitions of
/// the parser state machine.

//pub mod body;

use super::*;
use crate::doctree::{self, TreeNode, TreeNodeType, EnumeratorType, structural_nodes, body_nodes, inline_nodes};


/// ### Body
/// A state for detecting and parsing the first lines
/// of different types of rST text blocks. Transitions to
/// other states for handling the following lines
/// of the block are handled by the `TransitionMethod`s
/// in this state.
pub struct Body  {
  pub transitions: &'static Vec<Transition>
}

impl std::fmt::Debug for Body {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    f.debug_struct("Body").finish()
  }
}


impl Body  {

  /// ### new
  /// A `Body` state constructor.
  pub fn new() -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get(&StateMachine::Body).unwrap()
    }
  }


  /// ### empty_line
  /// Simply adds an empty line to the children of the curren node.
  pub fn empty_line (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>, PushOrPop, LineAdvance), &'static str>  {

    let mut tree_wrapper = doctree.unwrap();

    let node = TreeNode::new(TreeNodeType::EmptyLine);

    tree_wrapper.tree.push_child(node);

    Ok((Some(tree_wrapper), None, PushOrPop::Neither, LineAdvance::Some(1)))

  }


  /// ### bullet
  /// The transition method for matching bullets in `Body` state.
  /// Causes the parser to push a new machine in the state
  /// `BulletList` on top of its machine stack. Leaves the reponsibility
  /// of the actual parsing to that state.
  pub fn bullet (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>, PushOrPop, LineAdvance), &'static str> {

    let mut tree_wrapper = doctree.unwrap();

    let bullet = captures.get(2).unwrap().as_str().chars().next().unwrap();
    let bullet_indent = captures.get(1).unwrap().as_str().chars().count();
    let text_indent = captures.get(0).unwrap().as_str().chars().count();

    let bullet_list_data = TreeNodeType::BulletList{bullet: bullet, bullet_indent:bullet_indent, text_indent: text_indent};

    let list_node = TreeNode::new(bullet_list_data);

    tree_wrapper.tree.node.push_child(list_node);

    tree_wrapper.tree = match tree_wrapper.tree.focus_on_last_child() {
      Ok(child_zipper) => child_zipper,
      Err(node_itself) => {
        return Err("An error occurred when adding a child to the current node.\n");
      }
    };

    let next_state = StateMachine::BulletList;

    Ok( ( Some(tree_wrapper), Some(next_state), PushOrPop::Push, LineAdvance::None))

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
  pub fn enumerator (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>, PushOrPop, LineAdvance), &'static str> {
    
    let mut tree_wrapper = doctree.unwrap();

    let detected_enumerator_indent = captures.name("indent").unwrap().as_str().chars().count();
    let detected_text_indent = captures.get(0).unwrap().as_str().chars().count();

    const ENUMERATOR_NAMES: [&str; 15] = [
      "arabic_parens", "lower_alpha_parens", "upper_alpha_parens", "lower_roman_parens", "upper_roman_parens",
      "arabic_rparen", "lower_alpha_rparen", "upper_alpha_rparen", "lower_roman_rparen", "upper_roman_rparen",
      "arabic_period", "lower_alpha_period", "upper_alpha_period", "lower_roman_period", "upper_roman_period",
    ];

    let mut opt_enumerator_type: Option<EnumeratorType> = None;
    for enum_type in ENUMERATOR_NAMES.iter() {

      let enumerator_candidate = captures.name(enum_type);

      if let Some(enumerator) = enumerator_candidate {
        opt_enumerator_type = match *enum_type {
          "arabic_parens"       =>  Some(EnumeratorType::ParensArabic),
          "lower_alpha_parens"  =>  Some(EnumeratorType::ParensLowerAlpha),
          "upper_alpha_parens"  =>  Some(EnumeratorType::ParensUpperAlpha),
          "lower_roman_parens"  =>  Some(EnumeratorType::ParensLowerRoman),
          "upper_roman_parens"  =>  Some(EnumeratorType::ParensUpperRoman),
          "arabic_rparen"       =>  Some(EnumeratorType::RParenArabic),
          "lower_alpha_rparen"  =>  Some(EnumeratorType::RParenLowerAlpha),
          "upper_alpha_rparen"  =>  Some(EnumeratorType::RParenUpperAlpha),
          "lower_roman_rparen"  =>  Some(EnumeratorType::RParenLowerRoman),
          "upper_roman_rparen"  =>  Some(EnumeratorType::RParenUpperRoman),
          "arabic_period"       =>  Some(EnumeratorType::PeriodArabic),
          "lower_alpha_period"  =>  Some(EnumeratorType::PeriodLowerAlpha),
          "upper_alpha_period"  =>  Some(EnumeratorType::PeriodUpperAlpha),
          "lower_roman_period"  =>  Some(EnumeratorType::PeriodLowerRoman),
          "upper_roman_period"  =>  Some(EnumeratorType::PeriodUpperRoman),
          _                     =>  unreachable!()
        };
        break
      } 
    };

    let enumerator_type = if let Some(enumerator) = opt_enumerator_type {
      enumerator
    } else {
      return Err("Enumerator detected but no known enumerator type!\n")
    };
    
    let node_data = TreeNodeType::EnumeratedList {
      enum_type: enumerator_type,
      enumerator_indent: detected_enumerator_indent,
      text_indent: detected_text_indent,
    };

    let list_node = TreeNode::new(node_data);

    tree_wrapper.tree.push_child(list_node);

    tree_wrapper.tree = match tree_wrapper.tree.focus_on_last_child() {
      Ok(tree)  => tree,
      Err(tree) => return Err("Couldn't focus on enumerated list at body level...\n")
    };

    let enumerated_state = StateMachine::Body;

    Ok( ( Some(tree_wrapper), Some(enumerated_state), PushOrPop::Push, LineAdvance::None ) )

  }


  pub fn paragraph (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>, PushOrPop, LineAdvance), &'static str> {

    let mut tree_wrapper = doctree.unwrap();
    let indent = captures.get(1).unwrap().as_str().chars().count();

    let block = match Parser::read_indented_block(src_lines, Some(*current_line), Some(true), None, Some(indent), None) {
      Ok((lines, min_indent, line_offset, blank_finish)) => {
        lines.join("\n")
      }
      Err(e) => {
        eprintln!("{}", e);
        return Err("Error when reading paragraph block in Body.\n")
      }
    };

    // Pass text to inline parser as a string
    let inline_parser = MachineWithState::<Inline>::from(MachineWithState::new());

    let mut inline_nodes = if let Some(children) = inline_parser.parse(block, current_line) {
      children
    } else {
      return Err("Couldn't parse paragraph for inline nodes\n")
    };

    let data = TreeNodeType::Paragraph;

    let paragraph_node = TreeNode::new(data);

    tree_wrapper.tree.push_child(paragraph_node);

    tree_wrapper.tree = match tree_wrapper.tree.focus_on_last_child() {
      Ok(child) => child,
      Err(node_itself) => return Err("Couldn't focus on child paragraph\n")
    };

    tree_wrapper.tree.append_children(&mut inline_nodes);

    tree_wrapper.tree = match tree_wrapper.tree.focus_on_parent() {
      Ok(parent) => parent,
      Err(node_self) => return Err("Couldn't move focus to paragraph parent...\n")
    };

    return Ok((Some(tree_wrapper), None, PushOrPop::Neither, LineAdvance::Some(1)))

  }

}

impl From<MachineWithState<Body>> for MachineWithState<BulletList> {
  fn from(machine: MachineWithState<Body>) -> Self {
    MachineWithState {
      state: BulletList::new()
    }
  }
}

/// ### BulletList
/// A transition to this state is made if a `BulletList`
/// is detected in state `Body`. Handles subsequent
/// `BulletList` items.
pub struct BulletList {
  pub transitions: &'static Vec<Transition>
}


impl std::fmt::Debug for BulletList {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    f.debug_struct("BulletList").finish()
  }
}


impl BulletList {

  /// ### new
  /// A `BulletList` state constructor.
  pub fn new () -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get(&StateMachine::BulletList).unwrap()
    }
  }


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
        let inline_parser = MachineWithState::<Inline>::from(MachineWithState::new());

        let mut inline_nodes = if let Some(children) = inline_parser.parse(block, current_line) {
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

        let list_item_state = StateMachine::ListItem;

        return Ok((Some(tree_wrapper), Some(list_item_state), PushOrPop::Push, LineAdvance::Some(1)))

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

}

/// ### ListItem
/// A state for recognizing other beginning list items (enumerators or bullets)
/// and paragraphs. A single list item consists of paragraphs of text, and detection
/// of any other types of items will trigger a transition to a previous state in the stack.
pub struct ListItem {
  pub transitions: &'static Vec<Transition>
}

impl std::fmt::Debug for ListItem {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    f.debug_struct("ListItem").finish()
  }
}

impl ListItem {

  /// ### new
  /// A `ListItem` state constructor.
  pub fn new () -> Self {
    Self{
      transitions: transitions::TRANSITION_MAP.get(&StateMachine::ListItem).unwrap()
    }
  }

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
    todo!()
  }

  /// ### paragraph
  /// Direct child nodes of list items may only be paragraphs.
  /// This function parses each paragraph in a list item for inline nodes.
  /// A paragraph must have at least the same level of indentation as the containing list item,
  /// otherwise is it interpreted as ending the current list item.
  pub fn paragraph (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>, PushOrPop, LineAdvance), &'static str> {
    
    let mut tree_wrapper = doctree.unwrap();

    let (bullet, item_bullet_indent, item_text_indent) = match tree_wrapper.tree.node.data {
      TreeNodeType::BulletListItem{bullet, bullet_indent, text_indent} => (bullet, bullet_indent, text_indent),
      _ => return Err("Failed to retrieve bullet list item info when parsing a paragraph.\n")
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
        let inline_parser = MachineWithState::<Inline>::from(MachineWithState::new());

        let mut inline_nodes = if let Some(children) = inline_parser.parse(block, current_line) {
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
}


/// ### Definition
/// A state for handling the second line of a possible
/// `DefinitionList` items.
pub struct Definition {
  pub transitions: &'static Vec<Transition>
}

impl std::fmt::Debug for Definition {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    f.debug_struct("Definition").finish()
  }
}


impl Definition {

  /// ### new
  /// A `Definition` state constructor
  pub fn new () -> Self {
    Self{
      transitions: transitions::TRANSITION_MAP.get(&StateMachine::Definition).unwrap()
    }
  }

}


/// ### DefinitionList
/// This state is transitioned to if a first line of `DefinitionList`
/// is detected. Handles the subsequent lines.
pub struct DefinitionList {
  pub transitions: &'static Vec<Transition>
}

impl std::fmt::Debug for DefinitionList {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    f.debug_struct("DefinitionList").finish()
  }
}


impl DefinitionList {

  /// ### new
  /// A `DefinitionList` state constructor.
  pub fn new () -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get(&StateMachine::DefinitionList).unwrap()
    }
  }
}

/// ### EnumeratedList
/// A state that parses the lines followed by the detection of
/// the first line of a possibly detected `EnumeratedList`.
pub struct EnumeratedList {
  pub transitions: &'static Vec<Transition>
}

impl std::fmt::Debug for EnumeratedList {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    f.debug_struct("EnumeratedList").finish()
  }
}

impl EnumeratedList {

  /// ### new
  /// An `EnumeratedList` state constructor.
  pub fn new () -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get(&StateMachine::EnumeratedList).unwrap()
    }
  }

}

/// ### ExplicitMarkup
/// A state for parsing explicit markup followed by the detection
/// of a first such item.
pub struct ExplicitMarkup {
  pub transitions: &'static Vec<Transition>
}

impl std::fmt::Debug for ExplicitMarkup {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    f.debug_struct("ExplicitMarkup").finish()
  }
}


impl ExplicitMarkup {

  /// ### new
  /// An `ExplicitMarkup` state constructor.
  pub fn new () -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get(&StateMachine::ExplicitMarkup).unwrap()
    }
  }

}

/// ### ExtensionOptions
/// A state for parsing directive option fields.
pub struct ExtensionOptions {
  pub transitions: &'static Vec<Transition>
}

impl std::fmt::Debug for ExtensionOptions {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    f.debug_struct("ExtensionOptions").finish()
  }
}


impl ExtensionOptions {

  /// ### new
  /// An `ExtenstionOptions` state constructor.
  pub fn new () -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get(&StateMachine::ExtensionOptions).unwrap()
    }
  }

}

/// ### FieldList
/// A state for parsing subsequent fields in a field list.
pub struct FieldList {
  pub transitions: &'static Vec<Transition>
}

impl std::fmt::Debug for FieldList {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    f.debug_struct("FieldList").finish()
  }
}

impl FieldList {

  /// ### new
  /// An `FieldList` state constructor.
  pub fn new () -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get(&StateMachine::FieldList).unwrap()
    }
  }

}



/// ### Line
/// A state for parsing a detected `Line` (section titles and such).
pub struct Line {
  pub transitions: &'static Vec<Transition>
}

impl std::fmt::Debug for Line {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    f.debug_struct("Line").finish()
  }
}


impl Line {

  /// ### new
  /// An `Line` state constructor.
  pub fn new () -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get(&StateMachine::Line).unwrap()
    }
  }

}

/// ### LineBlock
/// A state for parsing subsequent lines of a line block.
pub struct LineBlock {
  pub transitions: &'static Vec<Transition>
}

impl std::fmt::Debug for LineBlock {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    f.debug_struct("LineBlock").finish()
  }
}

impl LineBlock{

  /// ### new
  /// An `LineBlock` state constructor.
  pub fn new () -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get(&StateMachine::LineBlock).unwrap()
    }
  }

}


/// ### A state for  parsing subsequent option list items.
pub struct OptionList {
  pub transitions: &'static Vec<Transition>
}

impl std::fmt::Debug for OptionList {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    f.debug_struct("OptionList").finish()
  }
}

impl OptionList {

  /// ### new
  /// An `OptionList` state constructor.
  pub fn new () -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get(&StateMachine::OptionList).unwrap()
    }
  }

}

/// ### RFC2822Body
/// A state for parsing body items that follow the RFC2822 specification.
pub struct RFC2822Body {
  pub transitions: &'static Vec<Transition>
}

/// ### RFC2822List
/// A state for parsing list items that follow the RFC2822 specification.
pub struct RFC2822List {
  pub transitions: &'static Vec<Transition>
}

/// ### SubstitutionDef
/// A state for parsing substitution definitions
pub struct SubstitutionDef {
  pub transitions: &'static Vec<Transition>
}

impl std::fmt::Debug for SubstitutionDef {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    f.debug_struct("SubstitutionDef").finish()
  }
}

impl SubstitutionDef {

  /// ### new
  /// A `SubstitutioDef` state constructor
  pub fn new() -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get(&StateMachine::SubstitutionDef).unwrap()
    }
  }

}

/// ### Text
/// A state for parsing generic text.
pub struct Text {
  pub transitions: &'static Vec<Transition>
}

impl std::fmt::Debug for Text {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    f.debug_struct("Text").finish()
  }
}

impl Text {

  /// ### new
  /// A `Text` state constructor
  pub fn new() -> Self {
    Self {
      transitions: transitions::TRANSITION_MAP.get(&StateMachine::Text).unwrap()
    }
  }

}


/// ### Inline
/// A state for inline parsing. This state is different from the other ones,
/// as it is not used at the `Parser` level. Instead,
/// individual transition methods may initiate a
/// `MachineWithState<Inline>` for parsing an inline block of text.
pub struct Inline {
  pub transitions: &'static Vec<(PatternName, regex::Regex, InlineParsingMethod)>
}

impl std::fmt::Debug for Inline {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
    f.debug_struct("Inline").finish()
  }
}

impl Inline {

  /// ### new
  /// An `Inline` state constructor
  pub fn new() -> Self{
    Self {
      transitions: &COMPILED_INLINE_TRANSITIONS
    }
  }


  /// ### paired_delimiter
  /// Parses inline text elements that have simple opening
  /// and closing delimiters such as `**strong emphasis**` or ``` ``literal_text`` ```.
  pub fn paired_delimiter (pattern_name: PatternName, captures: &regex::Captures) -> (TreeNode, usize) {
    
    let content = captures.get(1).unwrap();

    let data = String::from(content.as_str());

    let node = match pattern_name {
      PatternName::StrongEmphasis => TreeNode::new(TreeNodeType::StrongEmphasis{text: data}),
      PatternName::Emphasis => TreeNode::new(TreeNodeType::Emphasis{text: data}),
      PatternName::Literal => TreeNode::new(TreeNodeType::Literal{text: data}),
      PatternName::InlineTarget => TreeNode::new(TreeNodeType::InlineTarget{target_label: data}),
      _ => panic!("No such paired delimiter type!")
    };

    assert!(node.children.is_empty());

    let match_len = captures.get(0).unwrap().as_str().chars().count();

    (node, match_len)

  }


  /// ### whitespace
  /// Parses inline whitespace
  pub fn whitespace(pattern_name: PatternName, captures: &regex::Captures) -> (TreeNode, usize) {

    let content = captures.get(0).unwrap();

    let data = TreeNodeType::WhiteSpace{text: String::from(content.as_str())};

    let node = TreeNode::new(data);

    let match_len = content.as_str().chars().count();

    (node, match_len)

  }


  /// ### reference
  /// Parses reference type inline elements based on their pattern name.
  pub fn reference(pattern_name: PatternName, captures: &regex::Captures) -> (TreeNode, usize) {

    let whole_match = captures.get(0).unwrap();


    let data = match pattern_name {
      PatternName::SimpleRef | PatternName::PhraseRef => {
        let target_label = captures.get(1).unwrap();
        TreeNodeType::Reference{target_label: String::from(target_label.as_str())}
      },
      PatternName::FootNoteRef => {
        let target_label = captures.get(1).unwrap();
        TreeNodeType::FootnoteReference{target_label: String::from(target_label.as_str())}
      },
      PatternName::SubstitutionRef => {
        let target_label = captures.get(1).unwrap();
        TreeNodeType::SubstitutionReference{target_label: String::from(target_label.as_str())}
      },
      PatternName::StandaloneHyperlink => {

        let mut is_valid = true;

        const MISSING: &str = "!!!MISSING!!!";

        // Retrieving the relevant parts of the URI as &str
        let scheme = if let Some(scheme) = captures.name("scheme") {
          scheme.as_str()
        } else {
          MISSING
        };

        eprintln!("Scheme: {:#?}", scheme);

        match scheme {
          MISSING => {
            let email = if let Some(email) = captures.name("email") {
              email.as_str()
            } else {
              MISSING
            };
    
            eprintln!("Email: {:#?}", email);

            // If no email when missing a scheme, simply return match as string
            if email == MISSING {
              let match_str = whole_match.as_str();
              let data = TreeNodeType::Text{text: String::from(whole_match.as_str())};
              let text_node = TreeNode::new(data);
              return (text_node, match_str.chars().count())
            }

            let match_str = whole_match.as_str();

            // If a successful email recognition, prepend a mailto scheme to email.
            TreeNodeType::StandaloneEmail{text: format!("{}{}", "mailto:", match_str)}

          }

          _ => {

            let authority = if let Some(authority) = captures.name("authority") {
              authority.as_str()
            } else {
              MISSING
            };
            let userinfo = if let Some(userinfo) = captures.name("userinfo") {
              userinfo.as_str()
            } else {
              MISSING
            };
            let host = if let Some(host) = captures.name("host") {
              host.as_str()
            } else {
              MISSING
            };
            let port = if let Some(port) = captures.name("port") {
              port.as_str()
            } else {
              MISSING
            };
    
            eprintln!("Authority: {:#?}", authority);
            eprintln!("  userinfo: {:#?}", userinfo);
            eprintln!("  host: {:#?}", host);
            eprintln!("  port: {:#?}", port);
    
            let path = if let Some(path) = captures.name("path")  {
              path.as_str()
            } else {
              MISSING
            };
    
            eprintln!("path: {:#?}", path);
    
            let query = if let Some(query) = captures.name("query") {
              query.as_str()
            } else {
              MISSING
            };
    
            eprintln!("query: {:#?}", query);
    
            let fragment = if let Some(fragment) = captures.name("fragment") {
              fragment.as_str()
            } else {
              MISSING
            };
    
            eprintln!("fragment: {:#?}", fragment);

            // Validity checks

            if authority != MISSING  {
              let has_slash = if let Some(c) = path.chars().next() {
                eprintln!("First char of path is {}\n", c);
                
                let mut has_slash: bool = false;
                if c == '/' {
                  has_slash = true;
                }
                has_slash

              } else {
                false
              };

              if !has_slash {
                eprintln!("URI {}\nhas an autority field and a path that doesn't start with a '/'...\n  => URI invalid\n", whole_match.as_str());
                is_valid = false;
              }
            }

            // If URI is valid, return it as URI, else as text
            if is_valid {
              TreeNodeType::AbsoluteURI{text: String::from(whole_match.as_str())}
            } else {
              TreeNodeType::Text{text: String::from(whole_match.as_str())}
            }

          }
        }
      }
      _ => panic!("No such reference pattern.\n")
    };

    let node = TreeNode::new(data);

    let match_len = whole_match.as_str().chars().count();

    (node, match_len)
  }


  /// ### text
  /// Parses inline text elements that have simple opening
  /// and closing delimiters such as `**strong emphasis**` or ``` ``literal_text`` ```.
  pub fn text (pattern_name: PatternName, captures: &regex::Captures) -> (TreeNode, usize) {

    let content = captures.get(1).unwrap();

    let match_len = content.as_str().chars().count();

    let data = String::from(content.as_str());

    let node = TreeNode::new(TreeNodeType::Text{text: data});

    assert!(node.children.is_empty());

    (node, match_len)

  }

}


/// ### Failure
/// A failure state, which is entered if no match in current state is found.
#[derive(Debug)]
pub struct Failure;



impl From<MachineWithState<Body>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<Body>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<BulletList>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<BulletList>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<Definition>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<Definition>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<DefinitionList>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<DefinitionList>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<EnumeratedList>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<EnumeratedList>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<ExplicitMarkup>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<ExplicitMarkup>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<ExtensionOptions>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<ExtensionOptions>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<FieldList>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<FieldList>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<Line>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<Line>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<LineBlock>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<LineBlock>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<OptionList>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<OptionList>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<SubstitutionDef>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<SubstitutionDef>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}

impl From<MachineWithState<Text>> for MachineWithState<Failure> {
  fn from(machine: MachineWithState<Text>) -> Self {
    MachineWithState {
      state: Failure
    }
  }
}
