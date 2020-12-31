/*!
A submodule dedicated to parsing functions of the `StateMachine::AplusQuestionnaire` state.

Copyright © 2020 Santtu Söderholm
*/

use super::*;

/// A function for reading in blocks of intermediate text (rST paragraphs) between questionnaire questions.
pub fn parse_aplus_questionnaire_text(
    src_lines: &Vec<String>,
    base_indent: usize,
    section_level: &mut usize,
    line_cursor: &mut LineCursor,
    doctree: Option<DocTree>,
    captures: &regex::Captures,
    pattern_name: &Pattern,
) -> TransitionResult {
    let mut doctree = doctree.unwrap();

    let detected_indent = captures.get(1).unwrap().as_str().chars().count();

    match Parser::parent_indent_matches(doctree.shared_data(), detected_indent) {
        IndentationMatch::JustRight => {
            let start_line = line_cursor.relative_offset();
            let indent_allowed = true;
            let remove_indent = true;
            let alignment = Some(detected_indent);
            let (block_lines, offset) = if let Ok((lines, offset)) = Parser::read_text_block(
                src_lines,
                start_line,
                indent_allowed,
                remove_indent,
                alignment,
                true
            ) {
                (lines, offset)
            } else {
                panic!("Error when reading intermediate text in A+ questionnaire on line {}. Computer says no...", line_cursor.sum_total())
            };

            let inline_nodes = match Parser::inline_parse(block_lines.join("\n"), None, line_cursor) {
                InlineParsingResult::Nodes(nodes) => nodes,
                _ => panic!(
                    "Could not parse intermediate questionnaire text on line {} for inline nodes. Computer says no...",
                    line_cursor.sum_total()
                )
            };

            let paragraph = TreeNodeType::Paragraph {
                indent: detected_indent,
            };
            doctree = match doctree.push_data_and_focus(paragraph) {
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
            for node in inline_nodes {
                doctree = match doctree.push_data(node) {
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
            }
            doctree = doctree.focus_on_parent();
            return TransitionResult::Success {
                doctree: doctree,
                push_or_pop: PushOrPop::Neither,
                line_advance: LineAdvance::Some(1),
            };
        }
        _ => {
            doctree = doctree.focus_on_parent();
            return TransitionResult::Success {
                doctree: doctree,
                push_or_pop: PushOrPop::Pop,
                line_advance: LineAdvance::None,
            };
        }
    }
}

/// Parses an A+ questionnaire directive. Works as an entry point between the parser and the functions
/// `crate::parser::directive_parsers::{parse_aplus_pick_any, parse_aplus_pick_one, parse_aplus_freetext}`.
pub fn parse_aplus_questionnaire_directive(
    src_lines: &Vec<String>,
    base_indent: usize,
    section_level: &mut usize,
    line_cursor: &mut LineCursor,
    doctree: Option<DocTree>,
    captures: &regex::Captures,
    pattern_name: &Pattern,
) -> TransitionResult {
    let mut doctree = doctree.unwrap();

    let detected_marker_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
    let detected_directive_label = captures
        .get(2)
        .unwrap()
        .as_str()
        .split_whitespace()
        .collect::<String>()
        .to_lowercase();
    let detected_first_indent = captures.get(0).unwrap().as_str().chars().count() + base_indent;

    let empty_after_marker: bool = {
        let line = src_lines.get(line_cursor.relative_offset()).unwrap(); // Unwrapping is not a problem here.

        match line.char_indices().nth(detected_first_indent) {
            Some((index, _)) => line[index..].trim().is_empty(),
            None => true,
        }
    };

    let (body_indent, body_offset) =
        match Parser::indent_on_subsequent_lines(src_lines, line_cursor.relative_offset() + 1) {
            Some((indent, offset)) => (indent + base_indent, offset),
            None => (detected_first_indent, 0), // EOF encountered => stay on same line
        };

    match Parser::parent_indent_matches(doctree.shared_data(), detected_marker_indent) {
        IndentationMatch::JustRight => match detected_directive_label.as_str() {
            "pick-one" => directive_parsers::parse_aplus_pick_one(
                src_lines,
                doctree,
                line_cursor,
                detected_first_indent,
                body_indent,
                empty_after_marker,
            ),
            "pick-any" => directive_parsers::parse_aplus_pick_any(
                src_lines,
                doctree,
                line_cursor,
                detected_first_indent,
                body_indent,
                empty_after_marker,
            ),
            "freetext" => directive_parsers::parse_aplus_freetext(
                src_lines,
                doctree,
                line_cursor,
                detected_first_indent,
                body_indent,
                empty_after_marker,
            ),
            _ => {
                doctree = doctree.focus_on_parent();
                return TransitionResult::Success {
                    doctree: doctree,
                    push_or_pop: PushOrPop::Pop,
                    line_advance: LineAdvance::None,
                };
            }
        },
        _ => {
            doctree = doctree.focus_on_parent();
            return TransitionResult::Success {
                doctree: doctree,
                push_or_pop: PushOrPop::Pop,
                line_advance: LineAdvance::None,
            };
        }
    }
}
