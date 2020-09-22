/// ## unknown_transitions
/// 
/// A submodule whose transition functions for all patterns all make the document tree
/// focus on its parent and the parser pop from its state stack.
/// This is useful in making the transitions from each state *complete*,
/// as in defined for each possible input, where it is needed.
/// 
/// Parsing definition lists is one of these cases, as other than
/// purely textual inputs need to be checked for first before interpreting
/// a definition term line as simple text.
/// 
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;


pub fn back_up (src_lines: &Vec<String>, base_indent: usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let doctree = doctree.unwrap().focus_on_parent();

  TransitionResult::Success {
    doctree: doctree,
    next_states: None,
    push_or_pop: PushOrPop::Pop,
    line_advance: LineAdvance::None
  }
}
