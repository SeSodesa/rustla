/*!
A submodule for `State::EnumeratedList` related transition functions.

Copyright © 2020 Santtu Söderholm
*/

use super::*;

/// Parses enumerated list items inside enumerated lists.
pub fn enumerator(
    src_lines: &[String],
    base_indent: usize,
    section_level: &mut usize,
    line_cursor: &mut LineCursor,
    mut doctree: DocTree,
    captures: &regex::Captures,
    pattern_name: &Pattern,
) -> TransitionResult {

    let (list_delims, list_kind, list_start_index, n_of_items,list_enumerator_indent) = match doctree.shared_node_data() {
        TreeNodeType::EnumeratedList { delims, kind, start_index, n_of_items, enumerator_indent } => (*delims, *kind, *start_index, *n_of_items, *enumerator_indent),
        _ => return TransitionResult::Failure {
            message: format!("Not focused on enumerated list when parsing an enumerated list item on line {}. Computer says no...", line_cursor.sum_total()),
            doctree: doctree
        }
    };

    let detected_enumerator_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
    let detected_text_indent = captures.get(0).unwrap().as_str().chars().count() + base_indent;

    // Retrieve parent list information
    let (detected_number, detected_kind, detected_delims) =
        match converters::enum_captures_to_int_kind_and_delims(
            &captures,
            Some(&list_kind),
            true,
            false,
            n_of_items,
            list_start_index,
        ) {
            Some((number, kind, delims)) => (number, kind, delims),
            None => return TransitionResult::Failure {
                message: format!(
                    "Could not convert a list enumerator to number on line {}. Computer saus no...",
                    line_cursor.sum_total()
                ),
                doctree: doctree,
            },
        };

    // Ceck validity of list item
    if ! Parser::is_enumerated_list_item(
        &src_lines[..],
        line_cursor,
        captures,
        section_level,
        base_indent,
        detected_enumerator_indent,
        detected_number,
        detected_kind,
        detected_delims,
        pattern_name,
        &list_kind,
        true,
        n_of_items,
        list_start_index
    ) {
        return super::body::text(src_lines, base_indent, section_level, line_cursor, doctree, captures, pattern_name)
    };
    if list_delims == detected_delims
        && detected_kind == list_kind
        && list_enumerator_indent == detected_enumerator_indent
        && detected_number == n_of_items + list_start_index
    {
        // Modify relevant list parameters
        if let TreeNodeType::EnumeratedList { n_of_items, .. } = doctree.mut_node_data() {
            *n_of_items += 1;
        }
        let item_node_data = TreeNodeType::EnumeratedListItem {
            delims: list_delims,
            kind: detected_kind,
            index_in_list: detected_number,
            enumerator_indent: detected_enumerator_indent,
            text_indent: detected_text_indent,
        };
        doctree = match doctree.push_data_and_focus(item_node_data) {
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
        let relative_indent = detected_text_indent - base_indent;
        let (lines, offset) = match Parser::read_indented_block(
            src_lines,
            line_cursor.relative_offset(),
            false,
            true,
            Some(relative_indent),
            Some(relative_indent),
            false
        ) {
            IndentedBlockResult::Ok { lines, offset, .. } => (lines, offset),
            _ => return TransitionResult::Failure {
                message: format!(
                    "Could not read text block on line {}.",
                    line_cursor.sum_total()
                ),
                doctree: doctree
            }
        };

        let (doctree, state_stack) = match Parser::new(
            &lines,
            doctree,
            detected_text_indent,
            line_cursor.sum_total(),
            State::ListItem,
            *section_level
        ).parse() {
            ParsingResult::EOF {doctree, state_stack}
            | ParsingResult::EmptyStateStack {doctree, state_stack} => (doctree, state_stack),
            ParsingResult::Failure {doctree, message }  => return TransitionResult::Failure {
                message: format!(
                    "Nested parsing failed on line {}.",
                    line_cursor.sum_total()
                ),
                doctree: doctree
            }
        };

        return TransitionResult::Success {
            doctree: doctree,
            push_or_pop: PushOrPop::Push(state_stack),
            line_advance: LineAdvance::Some(offset),
        };
    } else {
        return TransitionResult::Success {
            doctree: doctree.focus_on_parent(),
            push_or_pop: PushOrPop::Pop,
            line_advance: LineAdvance::None,
        };
    }
}
