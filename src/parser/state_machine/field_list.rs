/*!A submodule that contains transition functions responsible for creating FieldListItem nodes.

Copyright © 2020 Santtu Söderholm
*/

use super::*;

/// Creates FieldListItems, if parameters such as detected indentation and such match with the parent node ones.
pub fn field_marker(
    src_lines: &[String],
    base_indent: usize,
    section_level: &mut usize,
    line_cursor: &mut LineCursor,
    mut doctree: DocTree,
    captures: &regex::Captures,
    pattern_name: &Pattern,
) -> TransitionResult {

    let indent_after_marker = captures.get(0).unwrap().as_str().chars().count() + base_indent;
    let detected_marker_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
    let detected_marker_name = captures.get(2).unwrap().as_str();
    let detected_body_indent = Parser::indent_from_next_line(
        src_lines,
        base_indent,
        detected_marker_indent,
        indent_after_marker,
        line_cursor
    );

    // Make sure we are inside a FieldList and that indentations match
    match doctree.shared_node_data() {

        TreeNodeType::FieldList { marker_indent } => {

            // Parse the marker for inline nodes
            if *marker_indent == detected_marker_indent {

                let marker_inline_nodes = if let InlineParsingResult::Nodes(nodes_data) = Parser::inline_parse(detected_marker_name.to_string(), None, line_cursor) {
                    nodes_data
                } else {
                    return TransitionResult::Failure {
                        // Should not happen in the first place, if a field marker was detected...
                        message: format!(
                            "Tried parsing a field marker on line {} for inline nodes but none found. Marker not valid... ",
                            line_cursor.sum_total()
                        ),
                        doctree: doctree
                    }
                };
                let item_node_data = TreeNodeType::FieldListItem {
                    raw_marker_name: detected_marker_name.to_string(),
                    marker_name_as_inline_nodes: marker_inline_nodes,
                    marker_indent: detected_marker_indent,
                    body_indent: detected_body_indent
                };
                doctree = match doctree.push_data_and_focus(item_node_data) {
                    Ok(tree) => tree,
                    Err(tree) => return TransitionResult::Failure {
                        message: format!("Node insertion error on line {}. Computer says no...", line_cursor.sum_total()),
                        doctree: tree
                    }
                };
                let (doctree, offset, state_stack) = match Parser::parse_first_node_block(
                    doctree,
                    src_lines,
                    base_indent,
                    line_cursor,
                    detected_body_indent,
                    Some(indent_after_marker),
                    State::ListItem,
                    section_level,
                    false
                ) {
                    Ok((parsing_result, offset)) => if let ParsingResult::EOF { doctree, state_stack } | ParsingResult::EmptyStateStack { doctree, state_stack } = parsing_result {
                        (doctree, offset, state_stack)
                    } else {
                        unreachable!("Returned from a nested parsing session on line {} without necessary information. Computer says no...", line_cursor.sum_total())
                    },
                    Err(ParsingResult::Failure { message, doctree }) => return TransitionResult::Failure {
                        message: format!("Looks like field list item on line {} has no content. Computer says no... ", line_cursor.sum_total()),
                        doctree: doctree
                    },
                    _ => unreachable!(
                        "Parsing first node block on line {} resulted in unknown combination of return values. Computer says no...",
                        line_cursor.sum_total()
                    )
                };
                return TransitionResult::Success {
                    doctree: doctree,
                    push_or_pop: PushOrPop::Push(state_stack),
                    line_advance: LineAdvance::Some(offset),
                }
            } else {
                return TransitionResult::Success {
                    doctree: doctree.focus_on_parent(),
                    push_or_pop: PushOrPop::Pop,
                    line_advance: LineAdvance::None,
                }
            }
        }
        _ => return TransitionResult::Failure {
            message: format!(
                "Attempted parsing a FieldListItem outside a FieldList on line {}. Computer says no... ",
                line_cursor.sum_total()
            ),
            doctree: doctree
        }
    }
}
