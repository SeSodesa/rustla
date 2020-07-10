/// ## enumerated_list
/// A submodule for `Statemachine::EnumeratedList` related transition functions.

use super::*;

pub fn enumerator (src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>, PushOrPop, LineAdvance), &'static str> {
  todo!()
}
