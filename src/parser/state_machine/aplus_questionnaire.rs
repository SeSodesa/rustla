/// ## aplus_questionnaire
///
/// A submodule dedicated to parsing functions of the `StateMachine::AplusQuestionnaire` state.
///
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;


/// ### aplus_questionnaire_text
///
/// A function for reading in blocks of intermediate text (rST paragraphs) between questionnaire questions.
pub fn aplus_questionnaire_text (src_lines: &Vec<String>, base_indent: usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult  {

  let mut doctree = doctree.unwrap();

  let detected_indent = captures.get(1).unwrap().as_str().chars().count();

  match Parser::parent_indent_matches(doctree.shared_data(), detected_indent) {
    IndentationMatch::JustRight => {

      let start_line = line_cursor.relative_offset();
      let indent_allowed = true;
      let remove_indent = true;
      let alignment = Some(detected_indent);
      let (block_lines, offset) = if let Ok((lines, offset)) = Parser::read_text_block(src_lines, start_line, indent_allowed, remove_indent, alignment) {
        (lines, offset)
      } else {
        panic!("Error when reading intermediate text in A+ questionnaire on line {}. Computer says no...", line_cursor.sum_total())
      };

      let inline_nodes = match Parser::inline_parse(block_lines.join("\n"), None, line_cursor) {
        InlineParsingResult::Nodes(nodes) => nodes,
        _ => panic!("Cound not parse intermediate questionnaire text on line {} for inline nodes. Computer says no...", line_cursor.sum_total())
      };

      let paragraph = TreeNodeType::Paragraph { indent: detected_indent };
      doctree = doctree.push_data_and_focus(paragraph);
      for node in inline_nodes {
        doctree = doctree.push_data(node);

      }
      doctree = doctree.focus_on_parent();
      return TransitionResult::Success {
        doctree: doctree,
        next_states: None,
        push_or_pop: PushOrPop::Neither,
        line_advance: LineAdvance::Some(1)
      };
    }
    _ => {
      doctree = doctree.focus_on_parent();
      return TransitionResult::Success {
        doctree: doctree,
        next_states: None,
        push_or_pop: PushOrPop::Pop,
        line_advance: LineAdvance::None,
      }
    }
  }
}

/// ### aplus_pick_one
///
/// A `pick-one` type questionnaire question parser.
pub fn aplus_pick_one (src_lines: &Vec<String>, base_indent: usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult  {
  todo!()
}


/// ### aplus_pick_any
///
/// A `pick-any` type questionnaire question parser.
pub fn aplus_pick_any (src_lines: &Vec<String>, base_indent: usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult  {
  todo!()
}


/// ### aplus_freetext
///
/// A `freetext` type questionnaire question parser.
pub fn aplus_freetext (src_lines: &Vec<String>, base_indent: usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult  {
  todo!()
}