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

/// ### bullet
pub fn bullet (src_lines: &Vec<String>, base_indent: &usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let doctree = doctree.unwrap().focus_on_parent();

  TransitionResult::Success {
    doctree: doctree,
    next_states: None,
    push_or_pop: PushOrPop::Pop,
    line_advance: LineAdvance::None
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
pub fn enumerator (src_lines: &Vec<String>, base_indent: &usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let doctree = doctree.unwrap().focus_on_parent();

  TransitionResult::Success {
    doctree: doctree,
    next_states: None,
    push_or_pop: PushOrPop::Pop,
    line_advance: LineAdvance::None
  }
}


/// ### field_marker
/// A transitioin function for handling detected field markers in a state that generates body type nodes.
pub fn field_marker (src_lines: &Vec<String>, base_indent: &usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let doctree = doctree.unwrap().focus_on_parent();

  TransitionResult::Success {
    doctree: doctree,
    next_states: None,
    push_or_pop: PushOrPop::Pop,
    line_advance: LineAdvance::None
  }
}


/// ### footnote
/// A transition function for generating footnotes
pub fn footnote (src_lines: &Vec<String>, base_indent: &usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let doctree = doctree.unwrap().focus_on_parent();

  TransitionResult::Success {
    doctree: doctree,
    next_states: None,
    push_or_pop: PushOrPop::Pop,
    line_advance: LineAdvance::None
  }
}


/// ### citation
/// A transition function for generating citations
pub fn citation (src_lines: &Vec<String>, base_indent: &usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let doctree = doctree.unwrap().focus_on_parent();

  TransitionResult::Success {
    doctree: doctree,
    next_states: None,
    push_or_pop: PushOrPop::Pop,
    line_advance: LineAdvance::None
  }
}


/// ### hyperlink_target
/// Parses a hyperlink target into a node.
pub fn hyperlink_target (src_lines: &Vec<String>, base_indent: &usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let doctree = doctree.unwrap().focus_on_parent();

  TransitionResult::Success {
    doctree: doctree,
    next_states: None,
    push_or_pop: PushOrPop::Pop,
    line_advance: LineAdvance::None
  }
}


/// ### directive
/// A transition function for parsing directives in a state that recognizes body elements.
pub fn directive (src_lines: &Vec<String>, base_indent: &usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let doctree = doctree.unwrap().focus_on_parent();

  TransitionResult::Success {
    doctree: doctree,
    next_states: None,
    push_or_pop: PushOrPop::Pop,
    line_advance: LineAdvance::None
  }
}


/// ### comment
/// A function for parsing reST comments.
pub fn comment (src_lines: &Vec<String>, base_indent: &usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let doctree = doctree.unwrap().focus_on_parent();

  TransitionResult::Success {
    doctree: doctree,
    next_states: None,
    push_or_pop: PushOrPop::Pop,
    line_advance: LineAdvance::None
  }
}


pub fn line (src_lines: &Vec<String>, base_indent: &usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let doctree = doctree.unwrap().focus_on_parent();

  TransitionResult::Success {
    doctree: doctree,
    next_states: None,
    push_or_pop: PushOrPop::Pop,
    line_advance: LineAdvance::None
  }
}


/// ### text
/// A transition function for parsing definition list items.
pub fn text (src_lines: &Vec<String>, base_indent: &usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult {

  let doctree = doctree.unwrap().focus_on_parent();

  return TransitionResult::Success {
    doctree: doctree,
    next_states: None,
    push_or_pop: PushOrPop::Pop,
    line_advance: LineAdvance::None
  }
}
