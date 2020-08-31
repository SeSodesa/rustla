/// ## block_quote
/// Contains the transition function for parsing attributions inside block quotes.

use super::*;

/// ### attribution
/// A function that generates attribution nodes inside a block quote.
/// An attribution ends block quotes, so encoutnering one makes the parser focus on its parent.
pub fn attribution (src_lines: &Vec<String>, base_indent: &usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let mut doctree = doctree.unwrap();

  let match_len = captures.get(0).unwrap().as_str().chars().count() + *base_indent;
  let attribution_line_indent = captures.get(1).unwrap().as_str().chars().count() + *base_indent;


  match Parser::parent_indent_matches(doctree.shared_data(), attribution_line_indent) {

    IndentationMatch::JustRight => {
      // Attempt to create attribution node inside current block quote and focus on the parent node

      let current_line = if let Some(line) = src_lines.get(line_cursor.relative_offset()) {
        line
      } else {
        panic!("Found an attribution marker on line {} but the line doesn't exist? Computer says no...", line_cursor.sum_total())
      };

      let line_after_marker = Parser::line_suffix(current_line, match_len - base_indent);

      eprintln!("{}\n", line_after_marker);

      let empty_after_marker = line_after_marker.as_str().trim().is_empty();

      eprintln!("{}\n", empty_after_marker);

      let first_indent = if empty_after_marker { None } else { Some(match_len)  };

      eprintln!("{:#?}\n", first_indent);

      let next_indent = if let Some((indent, offset)) = Parser::indent_on_subsequent_lines(src_lines, line_cursor.relative_offset() + 1) {
        if offset == 0 { Some(indent) } else { Some(match_len) }
      } else {
        Some(match_len)
      };

      let (attribution_string, offset) = if let Ok((lines, _, offset, _)) = Parser::read_indented_block(src_lines, Some(line_cursor.relative_offset()), Some(true), Some(true), next_indent, first_indent, true) {

        eprintln!("{:#?}\n", lines);

        (lines.join("\n").trim().to_string(), offset)
      } else {
        panic!("Could not read comment block on line {}...", line_cursor.sum_total())
      };

      doctree = doctree.push_data(TreeNodeType::Attribution { raw_text: attribution_string });
      doctree = doctree.focus_on_parent();

      TransitionResult::Success {
        doctree: doctree,
        next_states: None,
        push_or_pop: PushOrPop::Pop,
        line_advance: LineAdvance::Some(offset)
      }
    }
    IndentationMatch::TooMuch => {
      // Create another block quote
      doctree = doctree.push_data_and_focus(TreeNodeType::BlockQuote { body_indent: attribution_line_indent });
      return TransitionResult::Success {
        doctree: doctree,
        next_states: Some(vec![StateMachine::BlockQuote]),
        push_or_pop: PushOrPop::Push,
        line_advance: LineAdvance::None,
      }
    }
    _ => {
      doctree = doctree.focus_on_parent();
      TransitionResult::Success {
        doctree: doctree,
        next_states: None,
        push_or_pop: PushOrPop::Pop,
        line_advance: LineAdvance::None
      }
    }
  }
}