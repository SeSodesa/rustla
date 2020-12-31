/*!
Contains the transition function for parsing attributions inside block quotes.

Copyright © 2020 Santtu Söderholm
*/

use super::*;

/// A function that generates attribution nodes inside a block quote.
/// An attribution ends block quotes, so encountering one makes the parser focus on the parent of the current node.
pub fn attribution(
    src_lines: &Vec<String>,
    base_indent: usize,
    section_level: &mut usize,
    line_cursor: &mut LineCursor,
    doctree: Option<DocTree>,
    captures: &regex::Captures,
    pattern_name: &Pattern,
) -> TransitionResult {
    let mut doctree = doctree.unwrap();

    let match_len = captures.get(0).unwrap().as_str().chars().count() + base_indent;
    let attribution_line_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;

    match Parser::parent_indent_matches(doctree.shared_data(), attribution_line_indent) {
        IndentationMatch::JustRight => {
            // Attempt to create attribution node inside current block quote and focus on the parent node

            let current_line = if let Some(line) = src_lines.get(line_cursor.relative_offset()) {
                line
            } else {
                panic!("Found an attribution marker on line {} but the line doesn't exist? Computer says no...", line_cursor.sum_total())
            };

            let line_after_marker = Parser::line_suffix(current_line, match_len - base_indent);

            let empty_after_marker = line_after_marker.as_str().trim().is_empty();

            let first_indent = if empty_after_marker {
                None
            } else {
                Some(match_len)
            };

            let next_indent = if let Some((indent, offset)) =
                Parser::indent_on_subsequent_lines(src_lines, line_cursor.relative_offset() + 1)
            {
                if offset == 0 && indent >= attribution_line_indent {
                    Some(indent)
                } else {
                    Some(match_len)
                }
            } else {
                Some(match_len)
            };

            let (attribution_string, offset) = if let Ok((lines, _, offset, _)) =
                Parser::read_indented_block(
                    src_lines,
                    Some(line_cursor.relative_offset()),
                    Some(true),
                    Some(true),
                    next_indent,
                    first_indent,
                    true,
                ) {
                (lines.join(" ").trim().to_string(), offset)
            } else {
                panic!(
                    "Could not read comment block on line {}...",
                    line_cursor.sum_total()
                )
            };

            doctree = match doctree.push_data(TreeNodeType::Attribution {
                raw_text: attribution_string,
            }) {
                Ok(tree) => tree,
                Err(tree) => {
                    return TransitionResult::Failure {
                        message: format!(
                            "Node insertion error on line {}. Computer says no...",
                            line_cursor.sum_total()
                        ),
                        doctree: tree,
                    }
                }
            };
            doctree = doctree.focus_on_parent();

            TransitionResult::Success {
                doctree: doctree,
                push_or_pop: PushOrPop::Pop,
                line_advance: LineAdvance::Some(offset),
            }
        }
        IndentationMatch::TooMuch => {
            // Create another block quote
            doctree = match doctree.push_data_and_focus(TreeNodeType::BlockQuote {
                body_indent: attribution_line_indent,
            }) {
                Ok(tree) => tree,
                Err(tree) => {
                    return TransitionResult::Failure {
                        message: format!(
                            "Node insertion error on line {}. Computer says no...",
                            line_cursor.sum_total()
                        ),
                        doctree: tree,
                    }
                }
            };

            return TransitionResult::Success {
                doctree: doctree,
                push_or_pop: PushOrPop::Push(vec![State::BlockQuote]),
                line_advance: LineAdvance::None,
            };
        }
        _ => {
            doctree = doctree.focus_on_parent();
            TransitionResult::Success {
                doctree: doctree,
                push_or_pop: PushOrPop::Pop,
                line_advance: LineAdvance::None,
            }
        }
    }
}
