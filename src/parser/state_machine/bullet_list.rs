/*!
A submodule that contains `State::BulletList` related transition functions.

Copyright © 2020 Santtu Söderholm
*/

use super::*;

/// A `BulletList` version of the bullet list related
/// transition method. Differs from the `Body` state version
/// in that this detects whether a list of a different type has started
/// and acts accordingly.
pub fn bullet(
    src_lines: &Vec<String>,
    base_indent: usize,
    section_level: &mut usize,
    line_cursor: &mut LineCursor,
    mut doctree: DocTree,
    captures: &regex::Captures,
    pattern_name: &Pattern,
) -> TransitionResult {

    let detected_bullet = captures.get(2).unwrap().as_str().chars().next().unwrap();
    let detected_bullet_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
    let detected_text_indent = captures.get(0).unwrap().end() + base_indent;

    match doctree.shared_node_data() {
        TreeNodeType::BulletList { bullet, bullet_indent, text_indent } => {
            if *bullet == detected_bullet
                && *bullet_indent == detected_bullet_indent
                && *text_indent == detected_text_indent
            {
                // Still within same list based on indentation and bullet.
                // Create new ListItem node add a `ListItem` state on top of the state stack and proceed to
                // parse body elements on the same indentation level

                let item_node_data = TreeNodeType::BulletListItem{
                    bullet: *bullet,
                    bullet_indent: detected_bullet_indent,
                    text_indent: detected_text_indent
                };
                doctree = match doctree.push_data_and_focus(item_node_data) {
                    Ok(tree) => tree,
                    Err(tree) => return TransitionResult::Failure {
                        message: format!("Node insertion error on line {}. Computer says no...", line_cursor.sum_total()),
                        doctree: tree
                    }
                };
                let (doctree, offset, state_stack) = match Parser::parse_first_node_block(
                    doctree, src_lines,
                    base_indent,
                    line_cursor,
                    detected_text_indent,
                    None,
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
                        message: format!("Looks like bullet list item on line {} has no content.\nComputer says no...", line_cursor.sum_total()),
                        doctree: doctree
                    },
                    _ => unreachable!("Parsing first node block on line {} resulted in unknown combination of return values. Computer says no...", line_cursor.sum_total())
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
        _ => panic!("Tried parsing a bullet list item outside of a bullet list on line {}. Computer says no...", line_cursor.sum_total())
    }
}
