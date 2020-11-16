/// ## aplus
///
/// A submodule that contains transition methods specific to creating A+ nodes.
///
/// author: Santtu Söderholm
/// email:   santtu.soderholm@tuni.fi

use super::*;


/// ### aplus_col_break
pub fn aplus_col_break (src_lines: &Vec<String>, base_indent: usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: &regex::Captures, pattern_name: &PatternName) -> TransitionResult  {

  let mut doctree = doctree.unwrap();

  let detected_marker_indent = captures.get(1).unwrap().as_str().chars().count();

  match Parser::parent_indent_matches(doctree.shared_node_data(), detected_marker_indent) {

    IndentationMatch::JustRight | IndentationMatch::DoesNotMatter => {
      doctree = match doctree.push_data(TreeNodeType::AplusColBreak) {
        Ok(tree) => tree,
        Err(tree) => return TransitionResult::Failure {
          message: format!("Node insertion error on line {}. Computer says no...", line_cursor.sum_total()),
          doctree: tree
        }
      };
      TransitionResult::Success {
        doctree: doctree,
        next_states: None,
        push_or_pop: PushOrPop::Neither,
        line_advance: LineAdvance::Some(1)
      }
    }
    IndentationMatch::TooLittle => {
      eprintln!("Detected a column break on line {} with too little indentation. Interpreting as paragraph...", line_cursor.sum_total());
      doctree = doctree.focus_on_parent();
      TransitionResult::Success {
        doctree: doctree,
        next_states: None,
        push_or_pop: PushOrPop::Pop,
        line_advance: LineAdvance::None
      }
    }
    IndentationMatch::TooMuch => {
      doctree = match doctree.push_data_and_focus(TreeNodeType::BlockQuote { body_indent: detected_marker_indent }) {
        Ok(tree) => tree,
        Err(tree) => return TransitionResult::Failure {
          message: format!("Node insertion error on line {}. Computer says no...", line_cursor.sum_total()),
          doctree: tree
        }
      };
      return TransitionResult::Success {
        doctree: doctree,
        next_states: Some(vec![State::BlockQuote]),
        push_or_pop: PushOrPop::Push,
        line_advance: LineAdvance::None,
      }
    }
  }
}
