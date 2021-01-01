/*!
A submodule whose only function's purpose is to replace other transition methods
and make the document tree focus on its parent and make the parser pop from its state stack.
This is useful in making the transitions from each state *complete*,
as in defined for each possible input, where it is needed.

Parsing definition lists is one of these cases, as other than
purely textual inputs need to be checked for first before interpreting
a definition term line as simple text.

Copyright © 2020 Santtu Söderholm
*/
use super::*;

/// Focuses on node parent and POPs from parser state stack.
/// Useful in situations, where a pattern should be recognized but not allowed inside the current state.
/// One such case is the definition list state, where only normal text should be allowed,
/// but other possibilities such as bullet lists beed to be eliminated first.
pub fn back_up(
    src_lines: &Vec<String>,
    base_indent: usize,
    section_level: &mut usize,
    line_cursor: &mut LineCursor,
    doctree: DocTree,
    captures: &regex::Captures,
    pattern_name: &Pattern,
) -> TransitionResult {

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Pop,
        line_advance: LineAdvance::None,
    }
}
