/*!
A submodule for transition functions common to all states, such as blank line detection.

(c) Santtu Söderholm <santtu.soderholm@tuni.fi>
*/

use super::*;

/// Simply adds an empty line to the children of the curren node.
pub fn empty_line(
    src_lines: &Vec<String>,
    base_indent: usize,
    section_level: &mut usize,
    line_cursor: &mut LineCursor,
    doctree: Option<DocTree>,
    captures: &regex::Captures,
    pattern_name: &PatternName,
) -> TransitionResult {
    let doctree = doctree.unwrap();

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Neither,
        line_advance: LineAdvance::Some(1),
    }
}
