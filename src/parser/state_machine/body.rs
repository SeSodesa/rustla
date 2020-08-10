/// ## body
/// This module contains the transition functions related to `StateMachine::Body`.

use super::*;


/// ### bullet
/// The transition method for matching bullets in `Body` state.
/// Causes the parser to push a new machine in the state
/// `BulletList` on top of its machine stack. Leaves the reponsibility
/// of the actual parsing to that state.
pub fn bullet (src_lines: &Vec<String>, base_indent: &usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();

  let detected_bullet = captures.get(2).unwrap().as_str().chars().next().unwrap();
  let detected_bullet_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count() + base_indent;

  let sublist_data = TreeNodeType::BulletList {
    bullet: detected_bullet,
    bullet_indent: detected_bullet_indent,
    text_indent: detected_text_indent,
  };

  if parent_indent_matches(tree_wrapper.get_node_data(), detected_bullet_indent) {
    tree_wrapper = tree_wrapper.push_data_and_focus(sublist_data);
    return TransitionResult::Success {
      doctree: tree_wrapper,
      next_states: Some(vec![StateMachine::BulletList]),
      push_or_pop: PushOrPop::Push,
      line_advance: LineAdvance::None,
    }
  } else {
    tree_wrapper = tree_wrapper.focus_on_parent();
    return TransitionResult::Success {
      doctree: tree_wrapper,
      next_states: None,
      push_or_pop: PushOrPop::Pop,
      line_advance: LineAdvance::None,
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
pub fn enumerator (src_lines: &Vec<String>, base_indent: &usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

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
    //latest_text_indent: detected_text_indent,
  };

  if parent_indent_matches(tree_wrapper.get_node_data(), detected_enumerator_indent) {
    tree_wrapper = tree_wrapper.push_data_and_focus(list_node_data);
    return TransitionResult::Success {
      doctree: tree_wrapper,
      next_states: Some(vec![StateMachine::EnumeratedList]),
      push_or_pop: PushOrPop::Push,
      line_advance: LineAdvance::None,
    }
  } else {
    tree_wrapper = tree_wrapper.focus_on_parent();
    return TransitionResult::Success {
      doctree: tree_wrapper,
      next_states: None,
      push_or_pop: PushOrPop::Pop,
      line_advance: LineAdvance::None,
    }
  }
}


/// ### field_marker
/// A transitioin function for handling detected field markers in a state that generates body type nodes.
pub fn field_marker (src_lines: &Vec<String>, base_indent: &usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();

  let detected_marker_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;

  let list_node_data = TreeNodeType::FieldList {
    marker_indent: detected_marker_indent
  };

  // Match against the parent node. Only document root ignores indentation;
  // inside any other container it makes a difference.
  if parent_indent_matches(tree_wrapper.get_node_data(), detected_marker_indent) {
    tree_wrapper = tree_wrapper.push_data_and_focus(list_node_data);
    return TransitionResult::Success {
      doctree: tree_wrapper,
      next_states: Some(vec![StateMachine::FieldList]),
      push_or_pop: PushOrPop::Push,
      line_advance: LineAdvance::None,
    }
  } else {
    tree_wrapper = tree_wrapper.focus_on_parent();
    return TransitionResult::Success {
      doctree: tree_wrapper,
      next_states: None,
      push_or_pop: PushOrPop::Pop,
      line_advance: LineAdvance::None,
    }
  }
}


/// ### footnote
/// A transition function for generating footnotes
pub fn footnote (src_lines: &Vec<String>, base_indent: &usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();

  // Detected parameters...
  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count() + base_indent;
  let detected_marker_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
  let detected_label_str = captures.get(2).unwrap().as_str();

  let detected_body_indent = if let Some(line) = src_lines.get(line_cursor.relative_offset() + 1) {
    if line.trim().is_empty() {
      detected_text_indent
    } else {
      let indent = line.chars().take_while(|c| c.is_whitespace()).count() + base_indent;
      if indent < detected_marker_indent + 3 {
        detected_text_indent
      } else {
        indent
      }
    }
  } else {
    detected_text_indent
  };

  let detected_kind = if let PatternName::Footnote { kind } = pattern_name {
    kind
  } else {
    panic!("No footnote type information inside footnote transition function.\nComputer says no...\n")
  };

  let (label, target) = if let Some( label_and_target ) = detected_footnote_label_to_ref_label(&tree_wrapper, pattern_name, detected_label_str) {
    (label_and_target.0, label_and_target.1)
  } else {
    return TransitionResult::Failure {
      message: String::from("Cound not transform a footnote marker into a label--target-pair.\nComputer says no...\n")
    }
  };

  // Match against the parent node. Only document root ignores indentation;
  // inside any other container it makes a difference.
  if parent_indent_matches(tree_wrapper.get_node_data(), detected_marker_indent) {

    let footnote_data = TreeNodeType::Footnote {
      body_indent: detected_body_indent,
      kind: *detected_kind,
      label: label.clone(),
      target: target.clone()
    };
    tree_wrapper = tree_wrapper.push_data_and_focus(footnote_data);

    let (doctree, offset, state_stack) = match Parser::parse_first_node_block(tree_wrapper, src_lines, base_indent, line_cursor, detected_body_indent, Some(detected_text_indent), StateMachine::Footnote) {
      Some((doctree, nested_parse_offset, state_stack)) => (doctree, nested_parse_offset, state_stack),
      None => return TransitionResult::Failure {message: format!("Could not parse the first block of footnote on line {:#?}.\nComputer says no...\n", line_cursor.sum_total())}
    };

    tree_wrapper = doctree;

  return TransitionResult::Success {
    doctree: tree_wrapper,
    next_states: Some(state_stack),
    push_or_pop: PushOrPop::Push,
    line_advance: LineAdvance::Some(offset),
  }
  } else {
    tree_wrapper = tree_wrapper.focus_on_parent();
    return TransitionResult::Success {
      doctree: tree_wrapper,
      next_states: None,
      push_or_pop: PushOrPop::Pop,
      line_advance: LineAdvance::None,
    }
  }
}


/// ### citation
/// A transition function for generating citations
pub fn citation (src_lines: &Vec<String>, base_indent: &usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();

  // Detected parameters...
  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count() + base_indent;
  let detected_marker_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
  let detected_label_str = captures.get(2).unwrap().as_str();

  let detected_body_indent = if let Some(line) = src_lines.get(line_cursor.relative_offset() + 1) {
    if line.trim().is_empty() {
      detected_text_indent
    } else {
      let indent = line.chars().take_while(|c| c.is_whitespace()).count() + base_indent;
      if indent < detected_marker_indent + 3 { detected_text_indent } else { indent }
    }
  } else {
    detected_text_indent
  };

  eprintln!("marker: {}", detected_marker_indent);
  eprintln!("text: {}", detected_text_indent);
  eprintln!("body: {}\n", detected_body_indent);

  // Match against the parent node. Only document root ignores indentation;
  // inside any other container it makes a difference.
  if parent_indent_matches(tree_wrapper.get_node_data(), detected_body_indent) {

    let citation_data = TreeNodeType::Citation {
      body_indent: detected_body_indent,
      label: detected_label_str.to_string(),
    };
    tree_wrapper = tree_wrapper.push_data_and_focus(citation_data);

    let (doctree, offset, state_stack) = match Parser::parse_first_node_block(tree_wrapper, src_lines, base_indent, line_cursor, detected_body_indent, Some(detected_text_indent), StateMachine::Citation) {
      Some((doctree, nested_parse_offset, state_stack)) => (doctree, nested_parse_offset, state_stack),
      None => return TransitionResult::Failure {message: format!("Could not parse the first block of footnote on line {:#?}.\nComputer says no...\n", line_cursor.sum_total())}
    };

    tree_wrapper = doctree;

    return TransitionResult::Success {
      doctree: tree_wrapper,
      next_states: Some(state_stack),
      push_or_pop: PushOrPop::Push,
      line_advance: LineAdvance::Some(offset),
    }
  } else {

    tree_wrapper = tree_wrapper.focus_on_parent();
    return TransitionResult::Success {
      doctree: tree_wrapper,
      next_states: None,
      push_or_pop: PushOrPop::Pop,
      line_advance: LineAdvance::None,
    }
  }
}


/// ### hyperlink_target
/// Parses a hyperlink target into a node.
pub fn hyperlink_target (src_lines: &Vec<String>, base_indent: &usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut doctree = doctree.unwrap();

  // Detected parameters
  // Here we check which type of target we are dealing with:
  // 1. internal
  // 2. external or
  // 3. indirect
  // in addition to the usual identation and such.
  let detected_marker_indent = captures.get(1).unwrap().as_str().chars().count();
  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count();
  let detected_target_label = captures.get(2).unwrap().as_str();

  // Check for anonymous target
  let label_as_string = if detected_target_label == "_" {
    doctree.next_anon_target_label()
  } else {
    detected_target_label.to_string()
  };

  let detected_body_indent = if let Some(line) = src_lines.get(line_cursor.relative_offset() + 1) {
    if line.trim().is_empty() {
      detected_text_indent
    } else {
      let indent = line.chars().take_while(|c| c.is_whitespace()).count() + base_indent;
      if indent < detected_marker_indent + 3 { detected_text_indent } else { indent }
    }
  } else {
    detected_text_indent
  };

  if parent_indent_matches(doctree.get_node_data(), detected_marker_indent) {

    // Read in the following block of text here and parse it to find out the type of hyperref target in question

    let (block_string, offset): (String, usize) = match Parser::read_indented_block(src_lines, Some(line_cursor.relative_offset()), Some(true), Some(true), Some(detected_body_indent), Some(detected_text_indent), false) {
      Ok(( block, _, offset, _)) => (block.join("\n").chars().filter(|c| !c.is_whitespace()).collect(), offset),
      Err(e) => {
        return TransitionResult::Failure { message: e }
      }
    };

    eprintln!("Block string: {:#?}\n", block_string);

    if block_string.is_empty() { // ... the target is internal

      // We simply add the detected label into the queue of internal target labels and proceed with parsing in the current state.
      // Should a non-internal target or other type of target node be detected next,
      // this set of labels will be set to reference that node.

      doctree.push_to_internal_target_stack(label_as_string);

      doctree.print_internal_labels();

      return TransitionResult::Success {
        doctree: doctree,
        next_states: None,
        push_or_pop: PushOrPop::Neither,
        line_advance: LineAdvance::Some(1), // Jump to the next line so we don't just keep trying to parse the same internal target.
      }
    }

    let node_type: TreeNodeType = match Parser::inline_parse(block_string, Some(doctree), line_cursor) {
      
      InlineParsingResult::DoctreeAndNodes(altered_doctree, nodes_data) => {

        doctree = altered_doctree;

        eprintln!("Target nodes: {:#?}\n", nodes_data);

        if nodes_data.len() != 1 {
          return TransitionResult::Failure {
            message: String::from("Hyperlink targets should only contain a single node.\nComputer says no...\n")
          }
        }

        match nodes_data.get(0) {

          Some(TreeNodeType::AbsoluteURI { text })  |  Some(TreeNodeType::StandaloneEmail { text })  =>  {

            TreeNodeType::ExternalHyperlinkTarget {
              uri: text.clone(),
              target: label_as_string,
              marker_indent: detected_marker_indent
            }
          }

          Some(TreeNodeType::Reference { target_label, displayed_text }) =>  {

            TreeNodeType::IndirectHyperlinkTarget {
              target: label_as_string,
              indirect_target: target_label.clone(),
              marker_indent: detected_marker_indent
            }
          }

          _ => panic!("Hyperlink target didn't match any known types.\nComputer says no...\n")
        }
      }
      _ => panic!("Inline parser failed when parsing a hyperlink target on line {}\n.Computer says no...\n", line_cursor.sum_total())
    };

    let node = TreeNode::new(node_type, doctree.node_count(), None);

    doctree.push_child(node);

    return TransitionResult::Success {
      doctree: doctree,
      next_states: None,
      push_or_pop: PushOrPop::Neither,
      line_advance: LineAdvance::Some(1)
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


/// ### directive
/// A transition function for parsing directives in a state that recognizes body elements.
pub fn directive (src_lines: &Vec<String>, base_indent: &usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut doctree = doctree.unwrap();

  let detected_marker_indent = captures.get(1).unwrap().as_str().chars().count();
  let detected_directive_label = captures.get(2).unwrap().as_str().split_whitespace().collect::<String>().to_lowercase();
  let detected_text_indent = captures.get(0).unwrap().as_str().chars().count();
  let detected_body_indent = if let Some(line) = src_lines.get(line_cursor.relative_offset() + 1) {
    if line.trim().is_empty() {
      detected_text_indent
    } else {
      let indent = line.chars().take_while(|c| c.is_whitespace()).count() + base_indent;
      if indent < detected_marker_indent + 1 { detected_text_indent } else { indent }
    }
  } else { detected_text_indent };

  if parent_indent_matches(doctree.get_node_data(), detected_marker_indent) {

    match detected_directive_label.as_str() {

      "attention" | "caution" | "danger" | "error" | "hint" | "important" | "note" | "tip" | "warning" => {

        todo!("Parse admonition here...")
      }

      "image" => {

        todo!("Parse image here...")
      }

      "figure" => {

        todo!("Parse figure here...")
      }

      "topic" => {

        todo!("Parse topic here...")
      }

      "sidebar" => {

        todo!("Parse sidebar here...")
      }

      "line-block" => {

        todo!("Parse line block here...")
      }

      "parsed-literal" => {

        todo!("Parse literal block with inlin elements here...")
      }


      "code" => {

        todo!("Parse code block here...")
      }


      "math" => {
        todo!("Parse display math block here...")
      }

      "rubric" => {

        todo!("Parse rubric here...")
      }

      "epigraph" => {

        todo!("Parse epigraph here...")
      }

      "highlights" => {

        todo!("Parse highlights here...")
      }

      "pull-quote" => {

        todo!("Parse pull-quote here...")
      }

      "compound" => {

        todo!("Parse compound paragraph here...")
      }

      "container" => {

        todo!("parse container here...")
      }

      "table" => {

        todo!("Parse table here...")
      }

      "csv-table" => {

        todo!("Parse CSV table here...")
      }

      "list-table" => {
        todo!("Parse list table here...")
      }

      // DOCUMENT PARTS

      "contents" => {

        todo!("Parse table of contents here...")
      }

      "sectnum" | "section-numbering" => {

        todo!("Parse automatic section number generator here...")
      }

      "header" => {

        todo!("Parse header here...")
      }

      "footer" => {

        todo!("Parse footer here...")
      }

      "target-notes" => {

        todo!("Parse target footnotes here...")
      }

      "footnotes" => {

        unimplemented!("Footnotes (plural) directive is mentioned in the rST specification but is not implemented yet.")
      }

      "citations" => {

        unimplemented!("Citations (plural) directive is mentioned in the rST specification but is not implemented yet.")
      }

      "meta" => {

        todo!("Parse HTML metadata here...")
      }

      // MISCELLANEOUS

      "include" => {

        todo!("Include document here...")
      }

      "raw" => {

        todo!("Parse raw data pass-through here...")
      }

      "class" => {

        todo!("Assign specific CSS class to a block here...")
      }

      "role" => {

        todo!("Create a new interpreted text here...")
      }

      "default-role" => {

        todo!("Assign a default role to interpreted text here...")
      }

      "title" => {

        todo!("Assign a document metadata title here...")
      }

      "restructuredtext-test-directive" => {

        todo!("Only for test purposes...")
      }

      // A+ SPECIFIC DIRECTIVES

      "questionnaire" => {

        todo!("Parse graded and feedback questionnaires here...")
      }

      "submit" => {

        todo!("Parse submittable (external) exercises here...")
      }

      "toctree" => {

        todo!("Parse round metadata here...")
      }

      "ae-input" => {

        todo!("Parse active element input here...")
      }

      "ae-output" => {

        todo!("Parse active element output here...")
      }

      "hidden_block" => {

        todo!("Parse hidden block here...")
      }

      "point-of-interest" => {

        todo!("Parse point of interest here...")
      }

      "annotated" => {

        todo!("Parse annotated code blocks here...")
      }

      "lineref-code-block" => {

        todo!("Parse code blocks with line references here...")
      }

      "repl-res-count-reset" => {

        todo!("Parse an interactiv REPL session here...")
      }

      "acos-submit" => {

        todo!("parse submitttable ACOS exercise here...")
      }

      "div" => {

        todo!("Assign CSS classes (div) to text blocks here...")
      }

      "styled-topic" => {

        todo!("Parse styled topic here...")
      }

      // A+ MEDIA DIRECTIVES

      "story" => {

        todo!("Story embedded in a iframe...")
      }

      "jsvee" => {

        todo!("JSVee program visualization...")
      }

      "youtube" => {

        todo!("Youtube video...")
      }

      "local-video" => {

        todo!("A local video...")
      }

      "embedded-page" => {

        todo!("An embedded page...")
      }

      _ => todo!("Return the unknown or not yet implemented directive as a literal block...")
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


/// ### paragraph
/// A function that handles the parsing of paragraphs of text.
pub fn paragraph (src_lines: &Vec<String>, base_indent: &usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut tree_wrapper = doctree.unwrap();
  let detected_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;

  // Check if we are inside a node that cares about indentation
  if parent_indent_matches(tree_wrapper.get_node_data(), detected_indent) {
    
    tree_wrapper = tree_wrapper.push_data_and_focus(TreeNodeType::Paragraph { indent: detected_indent });

    let relative_indent = detected_indent - base_indent;
  
    let mut block = match Parser::read_text_block(src_lines, line_cursor.relative_offset(), true, true, Some(relative_indent)) {
      Ok((lines, line_offset)) => {
        lines.join("\n").trim_end().to_string()
      }
      Err(e) => {
        eprintln!("{}", e);
        return TransitionResult::Failure {
          message: String::from("Error when reading lines of text of a supposed paragraph block.\nComputer says no...\n")
        }
      }
    };

    lazy_static! {
      /// There are two kinds of literal block indicators:
      /// 1. preceded by whitespace
      /// 2. not preceded by whitespace
      /// 
      /// In the first case, both `::`s will be removed. In the second case, only the first one will disappear.
      static ref LITERAL_BLOCK_INDICATOR: Regex = Regex::new(r"(\s{0,1}|\S)::").unwrap();
    }

    let literal_block_next: bool = if let Some(capts) = LITERAL_BLOCK_INDICATOR.captures(block.as_str()) {

      eprintln!("Captures: {:#?}\n", capts.get(1));

      // Remove literal block indicator from paragraph
      let indicator_len = if capts.get(1).unwrap().as_str().trim().is_empty() {
        "::".chars().count()
      } else {
        ":".chars().count()
      };

      for _ in 0..indicator_len {
        if let None = block.pop() {
          return TransitionResult::Failure { // This should not ever be triggered
            message: format!("Tried removing a literal block indicator from a paragraph starting on line {} but failed.\nComputer says no...\n", line_cursor.sum_total())
          }
        }
      }
      true
    } else { false };

    eprintln!("Lietral block next: {}\n", literal_block_next);
  
    // Pass text to inline parser as a string
    tree_wrapper = if let InlineParsingResult::DoctreeAndNodes(mut returned_doctree, nodes_data) = Parser::inline_parse(block, Some(tree_wrapper), line_cursor) {


      for data in nodes_data {
        returned_doctree = returned_doctree.push_data(data);
      }

      returned_doctree.focus_on_parent()

    } else {
      return TransitionResult::Failure {
        message: String::from("Couldn't parse paragraph for inline nodes\n")
      }
    };

    if literal_block_next {
      return TransitionResult::Success {
        doctree: tree_wrapper,
        next_states: Some(vec![StateMachine::LiteralBlock]),
        push_or_pop: PushOrPop::Neither,
        line_advance: LineAdvance::Some(1),
      }
    } else {
      return TransitionResult::Success {
        doctree: tree_wrapper,
        next_states: None,
        push_or_pop: PushOrPop::Neither,
        line_advance: LineAdvance::Some(1),
      }
    }
  } else {
    tree_wrapper = tree_wrapper.focus_on_parent();
    return TransitionResult::Success {
      doctree: tree_wrapper,
      next_states: None,
      push_or_pop: PushOrPop::Pop,
      line_advance: LineAdvance::None,
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

    TreeNodeType::FieldListItem {body_indent, .. } | TreeNodeType::Footnote {body_indent, ..}
    | TreeNodeType::Citation {body_indent, ..} => {
      if relevant_detected_indent == *body_indent { true } else { false }
    },

    // Add other cases here...

    _ => false
  }

}


/// ### foonote_label_to_int
/// Converts a foonote label into a label--target-pair based on the current state of `DocTree.foonote_data`,
/// if possible. Returns an `Option`al pair `(label, target)` if successful.
pub fn detected_footnote_label_to_ref_label (doctree: &DocTree, pattern_name: &PatternName, detected_label_str: &str) -> Option<(String, String)> {

  use std::convert::TryFrom;

  if let PatternName::Footnote { kind } = pattern_name {
    match kind {

      FootnoteKind::Manual => {

        // In this case the doctree is simply asked whether it has a reference
        // with this name. If yes, the user is warned of a duplicate label,
        // but otherwise no special action is taken.

        return Some((detected_label_str.to_string(), detected_label_str.to_string()))
      }

      FootnoteKind::AutoNumbered => {

        // Here we iterate the set of all possible `u32` values
        // and once a number that has not been used as a label is found,
        // it is returned.

        // TODO: retrieve a start value from doctree, so iteration doesn't have to start from 1...

        for n in 1..=EnumAsInt::MAX {

          eprintln!("{}", n);

          let n_str = n.to_string();
          if doctree.has_target_label(n_str.as_str()) {
            continue
          }
          return Some( (n_str.clone(), n_str) )
        }
        eprintln!("All possible footnote numbers in use.\nComputer says no...\n");
        return None
      }

      FootnoteKind::SimpleRefName => {

        // Same as with automatically numbered footnotes, check if this has already a number representation
        // in the doctree and if not, return it.

        for n in 1..=EnumAsInt::MAX {

          let n_str = n.to_string();
          if doctree.has_target_label(n_str.as_str()) {
            continue
          }
          return Some( (n_str.clone(), detected_label_str.to_string()) )
        }
        eprintln!("All possible footnote numbers in use.\nComputer says no...\n");
        return None
      }

      FootnoteKind::AutoSymbol => {

        // Generate a label from crate::common::FOONOTE_SYMBOLS based on the number of autosymbol footnotes
        // entered into the document thus far.

        use crate::common::FOOTNOTE_SYMBOLS; // Import constant locally

        let n = doctree.n_of_symbolic_footnotes() as usize; // No overflow checks with as...

        let n_of_symbols = FOOTNOTE_SYMBOLS.len();

        let passes = n / n_of_symbols;
        let index = n % n_of_symbols;
        let symbol: &char = match FOOTNOTE_SYMBOLS.get(index) {
          Some(symb) => symb,
          None       => {
            eprintln!("No footnote symbol with index {}!\n", index);
            panic!()
          }
        };

        let label: String = vec![*symbol; passes + 1].iter().collect();
        return Some( (label.clone(), label) )
      }
    }
  } else {
    eprintln!("No footnote pattern inside a footnote transition function.\nComputer says no...\n");
    None
  }
}
