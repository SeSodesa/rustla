/// ## body
/// This module contains the transition functions related to `StateMachine::Body`.
///
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi
use super::*;

/// ### bullet
/// The transition method for matching bullets in `Body` state.
/// Causes the parser to push a new machine in the state
/// `BulletList` on top of its machine stack. Leaves the reponsibility
/// of the actual parsing to that state.
pub fn bullet(
    src_lines: &Vec<String>,
    base_indent: usize,
    section_level: &mut usize,
    line_cursor: &mut LineCursor,
    doctree: Option<DocTree>,
    captures: &regex::Captures,
    pattern_name: &PatternName,
) -> TransitionResult {
    let mut tree_wrapper = doctree.unwrap();

    let detected_bullet = captures.get(2).unwrap().as_str().chars().next().unwrap();
    let detected_bullet_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
    let detected_text_indent = captures.get(0).unwrap().as_str().chars().count() + base_indent;

    let sublist_data = TreeNodeType::BulletList {
        bullet: detected_bullet,
        bullet_indent: detected_bullet_indent,
        text_indent: detected_text_indent,
    };

    match Parser::parent_indent_matches(tree_wrapper.shared_node_data(), detected_bullet_indent) {
        IndentationMatch::JustRight => {
            tree_wrapper = match tree_wrapper.push_data_and_focus(sublist_data) {
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
                doctree: tree_wrapper,
                push_or_pop: PushOrPop::Push(vec![State::BulletList]),
                line_advance: LineAdvance::None,
            };
        }
        IndentationMatch::TooMuch => {
            tree_wrapper = match tree_wrapper.push_data_and_focus(TreeNodeType::BlockQuote {
                body_indent: detected_bullet_indent,
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
                doctree: tree_wrapper,
                push_or_pop: PushOrPop::Push(vec![State::BlockQuote]),
                line_advance: LineAdvance::None,
            };
        }
        _ => {
            tree_wrapper = tree_wrapper.focus_on_parent();
            return TransitionResult::Success {
                doctree: tree_wrapper,
                push_or_pop: PushOrPop::Pop,
                line_advance: LineAdvance::None,
            };
        }
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
pub fn enumerator(
    src_lines: &Vec<String>,
    base_indent: usize,
    section_level: &mut usize,
    line_cursor: &mut LineCursor,
    doctree: Option<DocTree>,
    captures: &regex::Captures,
    pattern_name: &PatternName,
) -> TransitionResult {
    let mut doctree = doctree.unwrap();

    let detected_enumerator_indent =
        captures.get(1).unwrap().as_str().chars().count() + base_indent;
    let detected_text_indent = captures.get(0).unwrap().as_str().chars().count() + base_indent;
    // let detected_enum_str = captures.get(2).unwrap().as_str();

    let (detected_number, detected_kind, detected_delims) =
        match Parser::enum_captures_to_int_kind_and_delims(&captures, None, false, None, None) {
            Some((number, kind, delims)) => (number, kind, delims),
            None => return TransitionResult::Failure {
                message: format!(
                    "Could not convert a list enumerator to number on line {}. Computer says no...",
                    line_cursor.sum_total()
                ),
                doctree: doctree,
            },
        };

    // Ceck validity of list item
    doctree = match Parser::is_enumerated_list_item(
        doctree,
        src_lines,
        line_cursor,
        captures,
        section_level,
        base_indent,
        detected_enumerator_indent,
        detected_number,
        detected_kind,
        detected_delims,
        pattern_name,
        None,
        false,
        None,
        None,
    ) {
        Ok(doctree) => doctree,
        Err(transition_result) => return transition_result,
    };

    let list_node_data = TreeNodeType::EnumeratedList {
        delims: detected_delims,
        kind: detected_kind,
        start_index: detected_number, //detected_enum_as_usize,
        n_of_items: 0,
        enumerator_indent: detected_enumerator_indent,
    };

    match Parser::parent_indent_matches(doctree.shared_node_data(), detected_enumerator_indent) {
        IndentationMatch::JustRight => {
            doctree = match doctree.push_data_and_focus(list_node_data) {
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
                push_or_pop: PushOrPop::Push(vec![State::EnumeratedList]),
                line_advance: LineAdvance::None,
            };
        }
        IndentationMatch::TooMuch => {
            doctree = match doctree.push_data_and_focus(
                TreeNodeType::BlockQuote {
                    body_indent: detected_enumerator_indent,
                }
            ) {
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
            return TransitionResult::Success {
                doctree: doctree,
                push_or_pop: PushOrPop::Pop,
                line_advance: LineAdvance::None,
            };
        }
    }
}

/// ### field_marker
/// A transitioin function for handling detected field markers in a state that generates body type nodes.
pub fn field_marker(
    src_lines: &Vec<String>,
    base_indent: usize,
    section_level: &mut usize,
    line_cursor: &mut LineCursor,
    doctree: Option<DocTree>,
    captures: &regex::Captures,
    pattern_name: &PatternName,
) -> TransitionResult {
    let mut tree_wrapper = doctree.unwrap();

    let detected_marker_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;

    let list_node_data = TreeNodeType::FieldList {
        marker_indent: detected_marker_indent,
    };

    // Match against the parent node. Only document root ignores indentation;
    // inside any other container it makes a difference.
    match Parser::parent_indent_matches(tree_wrapper.shared_node_data(), detected_marker_indent) {
        IndentationMatch::JustRight => {
            tree_wrapper = match tree_wrapper.push_data_and_focus(list_node_data) {
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
                doctree: tree_wrapper,
                push_or_pop: PushOrPop::Push(vec![State::FieldList]),
                line_advance: LineAdvance::None,
            };
        }
        IndentationMatch::TooMuch => {
            tree_wrapper = match tree_wrapper.push_data_and_focus(
                TreeNodeType::BlockQuote {
                    body_indent: detected_marker_indent,
                }
            ) {
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
                doctree: tree_wrapper,
                push_or_pop: PushOrPop::Push(vec![State::BlockQuote]),
                line_advance: LineAdvance::None,
            };
        }
        _ => {
            return TransitionResult::Success {
                doctree: tree_wrapper.focus_on_parent(),
                push_or_pop: PushOrPop::Pop,
                line_advance: LineAdvance::None,
            };
        }
    }
}

/// ### footnote
/// A transition function for generating footnotes
pub fn footnote(
    src_lines: &Vec<String>,
    base_indent: usize,
    section_level: &mut usize,
    line_cursor: &mut LineCursor,
    doctree: Option<DocTree>,
    captures: &regex::Captures,
    pattern_name: &PatternName,
) -> TransitionResult {
    let mut doctree = doctree.unwrap();

    // Detected parameters...
    let detected_text_indent = captures.get(0).unwrap().as_str().chars().count() + base_indent;
    let detected_marker_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
    let detected_label_str = captures.get(2).unwrap().as_str();

    let detected_body_indent = if let Some(line) = src_lines.get(line_cursor.relative_offset() + 1)
    {
        if line.trim().is_empty() {
            detected_text_indent
        } else {
            let indent = line.chars().take_while(|c| c.is_whitespace()).count() + base_indent;
            if indent < detected_marker_indent + 3 {
                detected_text_indent
            } else {
                indent
            }
        }
    } else {
        detected_text_indent
    };

    let detected_kind = if let PatternName::Footnote(kind) = pattern_name {
        kind
    } else {
        return TransitionResult::Failure {
            message: format!("No footnote type information inside footnote transition function. Computer says no..."),
            doctree: doctree
        };
    };

    let (label, target) = if let Some(label_and_target) =
        detected_footnote_label_to_ref_label(&doctree, pattern_name, detected_label_str)
    {
        (label_and_target.0, label_and_target.1)
    } else {
        return TransitionResult::Failure {
            message: format!("Cound not transform a footnote marker into a label--target-pair on line {}. Computer says no...", line_cursor.sum_total()),
            doctree: doctree
        };
    };

    // Match against the parent node. Only document root ignores indentation;
    // inside any other container it makes a difference.
    match Parser::parent_indent_matches(doctree.shared_node_data(), detected_marker_indent) {
        IndentationMatch::JustRight => {
            let footnote_data = TreeNodeType::Footnote {
                body_indent: detected_body_indent,
                kind: *detected_kind,
                label: label.clone(),
                target: target.clone(),
            };
            doctree = match doctree.push_data_and_focus(footnote_data) {
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

            let (doctree, offset, state_stack) = match Parser::parse_first_node_block(doctree, src_lines, base_indent, line_cursor, detected_body_indent, Some(detected_text_indent), State::Footnote, section_level, false) {
                Ok((parsing_result, offset)) => if let ParsingResult::EOF { doctree, state_stack } | ParsingResult::EmptyStateStack { doctree, state_stack } = parsing_result {
                    (doctree, offset, state_stack)
                } else {
                    unreachable!(
                        "Returned from a nested parsing session on line {} without necessary information. Computer says no...",
                        line_cursor.sum_total()
                    )
                },
                Err(ParsingResult::Failure { message, doctree }) => return TransitionResult::Failure {
                    message: format!("Looks like footnote on line {} has no content. Computer says no...", line_cursor.sum_total()),
                    doctree: doctree
                },
                _ => unreachable!("Parsing first node block on line {} resulted in unknown combination of return values. Computer says no...", line_cursor.sum_total())
            };

            return TransitionResult::Success {
                doctree: doctree,
                push_or_pop: PushOrPop::Push(state_stack),
                line_advance: LineAdvance::Some(offset),
            };
        }
        IndentationMatch::TooMuch => {
            doctree = match doctree.push_data_and_focus(TreeNodeType::BlockQuote {
                body_indent: detected_marker_indent,
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
            return TransitionResult::Success {
                doctree: doctree.focus_on_parent(),
                push_or_pop: PushOrPop::Pop,
                line_advance: LineAdvance::None,
            };
        }
    }
}

/// ### citation
/// A transition function for generating citations
pub fn citation(
    src_lines: &Vec<String>,
    base_indent: usize,
    section_level: &mut usize,
    line_cursor: &mut LineCursor,
    doctree: Option<DocTree>,
    captures: &regex::Captures,
    pattern_name: &PatternName,
) -> TransitionResult {
    let mut tree_wrapper = doctree.unwrap();

    // Detected parameters...
    let detected_text_indent = captures.get(0).unwrap().as_str().chars().count() + base_indent;
    let detected_marker_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
    let detected_label_str = captures.get(2).unwrap().as_str();

    let detected_body_indent = if let Some(line) = src_lines.get(line_cursor.relative_offset() + 1)
    {
        if line.trim().is_empty() {
            detected_text_indent
        } else {
            let indent = line.chars().take_while(|c| c.is_whitespace()).count() + base_indent;
            if indent < detected_marker_indent + 3 {
                detected_text_indent
            } else {
                indent
            }
        }
    } else {
        detected_text_indent
    };

    use crate::common::normalize_refname;

    // Match against the parent node. Only document root ignores indentation;
    // inside any other container it makes a difference.
    match Parser::parent_indent_matches(tree_wrapper.shared_node_data(), detected_marker_indent) {
        IndentationMatch::JustRight => {
            let citation_data = TreeNodeType::Citation {
                body_indent: detected_body_indent,
                label: normalize_refname(detected_label_str),
            };
            tree_wrapper = match tree_wrapper.push_data_and_focus(citation_data) {
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

            let (doctree, offset, state_stack) = match Parser::parse_first_node_block(tree_wrapper, src_lines, base_indent, line_cursor, detected_body_indent, Some(detected_text_indent), State::Citation, section_level,false) {
                Ok((parsing_result, offset)) => if let ParsingResult::EOF { doctree, state_stack } | ParsingResult::EmptyStateStack { doctree, state_stack } = parsing_result {
                    (doctree, offset, state_stack)
                } else {
                    unreachable!(
                        "Returned from a nested parsing session on line {} without necessary information. Computer says no...",
                        line_cursor.sum_total()
                    )
                },
                Err(ParsingResult::Failure { message, doctree }) => return TransitionResult::Failure {
                    message: format!("Looks like citation on line {} has no content. Computer says no...", line_cursor.sum_total()),
                    doctree: doctree
                },
                _ => unreachable!("Parsing first node block on line {} resulted in unknown combination of return values. Computer says no...", line_cursor.sum_total())
            };

            tree_wrapper = doctree;

            return TransitionResult::Success {
                doctree: tree_wrapper,
                push_or_pop: PushOrPop::Push(state_stack),
                line_advance: LineAdvance::Some(offset),
            };
        }
        IndentationMatch::TooMuch => {
            tree_wrapper = match tree_wrapper.push_data_and_focus(TreeNodeType::BlockQuote {
                body_indent: detected_marker_indent,
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
                doctree: tree_wrapper,
                push_or_pop: PushOrPop::Push(vec![State::BlockQuote]),
                line_advance: LineAdvance::None,
            };
        }
        _ => {
            return TransitionResult::Success {
                doctree: tree_wrapper.focus_on_parent(),
                push_or_pop: PushOrPop::Pop,
                line_advance: LineAdvance::None,
            };
        }
    }
}

/// ### hyperlink_target
/// Parses a hyperlink target into a node.
pub fn hyperlink_target(
    src_lines: &Vec<String>,
    base_indent: usize,
    section_level: &mut usize,
    line_cursor: &mut LineCursor,
    doctree: Option<DocTree>,
    captures: &regex::Captures,
    pattern_name: &PatternName,
) -> TransitionResult {

    let mut doctree = doctree.unwrap();

    // Detected parameters
    let detected_marker_indent = captures.get(1).unwrap().as_str().chars().count();
    let detected_text_indent = captures.get(0).unwrap().as_str().chars().count();
    let detected_target_label = captures.get(2).unwrap().as_str();

    // Check for anonymous target
    let label_as_string = if detected_target_label == "_" {
        doctree.next_anon_target_label()
    } else {
        crate::common::normalize_refname(detected_target_label)
    };

    let detected_body_indent = if let Some(line) = src_lines.get(line_cursor.relative_offset() + 1)
    {
        if line.trim().is_empty() {
            detected_text_indent
        } else {
            let indent = line.chars().take_while(|c| c.is_whitespace()).count() + base_indent;
            if indent < detected_marker_indent + 3 {
                detected_text_indent
            } else {
                indent
            }
        }
    } else {
        detected_text_indent
    };

    match Parser::parent_indent_matches(doctree.shared_node_data(), detected_marker_indent) {

        IndentationMatch::JustRight => {
            // Read in the following block of text here and parse it to find out the type of hyperref target in question

            let (block_string, offset): (String, usize) = match Parser::read_indented_block(
                src_lines,
                Some(line_cursor.relative_offset()),
                Some(true),
                Some(true),
                Some(detected_body_indent),
                Some(detected_text_indent),
                false,
            ) {
                Ok((block, _, offset, _)) => (
                    block
                        .join("\n")
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect(),
                    offset,
                ),
                Err(e) => {
                    return TransitionResult::Failure {
                        message: e,
                        doctree: doctree,
                    }
                }
            };

            // Here we check which type of target we are dealing with:
            // 1. internal
            // 2. external or
            // 3. indirect
            // in addition to the usual identation and such.
            if block_string.is_empty() {
                // ... the target is internal

                // We simply add the detected label into the queue of internal target labels and proceed with parsing in the current state.
                // Should a non-internal target or other type of target node be detected next,
                // this set of labels will be set to reference that node.

                doctree.push_to_internal_target_stack(label_as_string);

                return TransitionResult::Success {
                    doctree: doctree,
                    push_or_pop: PushOrPop::Neither,
                    line_advance: LineAdvance::Some(1), // Jump to the next line so we don't just keep trying to parse the same internal target.
                };
            }

            let node_type: TreeNodeType = match Parser::inline_parse(block_string, Some(&mut doctree), line_cursor) {

                InlineParsingResult::Nodes(nodes_data) => {

                    if nodes_data.len() != 1 {
                        return TransitionResult::Failure {
                            message: format!("Hyperlink targets should only contain a single node. Computer says no on line {}...", line_cursor.sum_total()),
                            doctree: doctree
                        }
                    }

                    match nodes_data.get(0) {

                        // Indirect target
                        Some(TreeNodeType::Reference { reference, displayed_text }) =>  {

                            use crate::common::Reference;

                            match reference {
                                Reference::Internal (ref_str) => TreeNodeType::IndirectHyperlinkTarget {
                                    target: label_as_string,
                                    indirect_target: match reference {
                                        Reference::Internal(ref_str) => ref_str.to_string(),
                                        Reference::URI(ref_str) | Reference::EMail(ref_str) => return TransitionResult::Failure {
                                        message: format!("Wrong type of inline node when parsing an indirect target {} on line {}. Computer says no...", ref_str, line_cursor.sum_total()),
                                        doctree: doctree
                                        },
                                    },
                                    marker_indent: detected_marker_indent
                                },
                                Reference::URI (ref_str) => TreeNodeType::ExternalHyperlinkTarget {
                                    marker_indent: detected_marker_indent,
                                    target: label_as_string,
                                    uri: ref_str.to_owned()
                                },
                                Reference::EMail (ref_str) => TreeNodeType::ExternalHyperlinkTarget {
                                    marker_indent: detected_marker_indent,
                                    target: label_as_string,
                                    uri: ref_str.to_owned()
                                }
                            }
                        }

                        _ => return TransitionResult::Failure {
                            message: format!("Hyperlink target on line {} didn't match any known types. Computer says no...", line_cursor.sum_total()),
                            doctree: doctree
                        }
                    }
                }

                _ => panic!("Inline parser failed when parsing a hyperlink target on line {} .Computer says no...", line_cursor.sum_total())

            };

            let node = TreeNode::new(node_type, doctree.node_count(), None, None);

            match doctree.push_child(node) {
                Ok(()) => {}
                Err(node) => {
                    return TransitionResult::Failure {
                        message: format!(
                            "Node insertion error on line {}. Computer says no...",
                            line_cursor.sum_total()
                        ),
                        doctree: doctree,
                    }
                }
            };

            return TransitionResult::Success {
                doctree: doctree,
                push_or_pop: PushOrPop::Neither,
                line_advance: LineAdvance::Some(1),
            };
        }

        IndentationMatch::TooMuch => {
            doctree = match doctree.push_data_and_focus(
                TreeNodeType::BlockQuote {
                    body_indent: detected_marker_indent,
                }
            ) {
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
            return TransitionResult::Success {
                doctree: doctree.focus_on_parent(),
                push_or_pop: PushOrPop::Pop,
                line_advance: LineAdvance::None,
            };
        }
    }
}

/// ### directive
/// A transition function for parsing directives in a state that recognizes body elements.
pub fn directive(
    src_lines: &Vec<String>,
    base_indent: usize,
    section_level: &mut usize,
    line_cursor: &mut LineCursor,
    doctree: Option<DocTree>,
    captures: &regex::Captures,
    pattern_name: &PatternName,
) -> TransitionResult {
    let mut doctree = doctree.unwrap();

    let detected_marker_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
    let detected_directive_label = captures
        .get(2)
        .unwrap()
        .as_str()
        .to_lowercase()
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

    match Parser::parent_indent_matches(doctree.shared_node_data(), detected_marker_indent) {
        IndentationMatch::JustRight => {
            match detected_directive_label.as_str() {
                "attention" | "caution" | "danger" | "error" | "hint" | "important" | "note"
                | "tip" | "warning" => Parser::parse_standard_admonition(
                    src_lines,
                    body_indent,
                    *section_level,
                    detected_first_indent,
                    doctree,
                    line_cursor,
                    detected_directive_label.as_str(),
                    empty_after_marker,
                ),

                "admonition" => Parser::parse_generic_admonition(
                    src_lines,
                    doctree,
                    line_cursor,
                    empty_after_marker,
                    Some(detected_first_indent),
                ),

                "image" => Parser::parse_image(
                    src_lines,
                    doctree,
                    line_cursor,
                    empty_after_marker,
                    Some(detected_first_indent),
                ),

                "figure" => Parser::parse_figure(
                    src_lines,
                    doctree,
                    line_cursor,
                    base_indent,
                    empty_after_marker,
                    body_indent,
                    Some(detected_first_indent),
                    *section_level,
                ),

                "topic" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "sidebar" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "line-block" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "parsed-literal" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "code" => Parser::parse_code(
                    src_lines,
                    doctree,
                    line_cursor,
                    base_indent,
                    empty_after_marker,
                    Some(detected_first_indent),
                    *section_level,
                ),

                "math" => Parser::parse_math_block(
                    src_lines,
                    doctree,
                    line_cursor,
                    base_indent,
                    empty_after_marker,
                    detected_first_indent,
                    *section_level,
                ),

                "rubric" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "epigraph" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "highlights" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "pull-quote" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "compound" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "container" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "table" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "csv-table" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "list-table" => Parser::parse_list_table(
                    src_lines,
                    doctree,
                    line_cursor,
                    base_indent,
                    empty_after_marker,
                    Some(detected_first_indent),
                    body_indent,
                    *section_level,
                ),

                // DOCUMENT PARTS
                "contents" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "sectnum" | "section-numbering" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "header" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "footer" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "target-notes" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "footnotes" => {
                    unimplemented!("Footnotes (plural) directive is mentioned in the rST specification but is not implemented yet.")
                }

                "citations" => {
                    unimplemented!("Citations (plural) directive is mentioned in the rST specification but is not implemented yet.")
                }

                "meta" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                // MISCELLANEOUS
                "include" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "raw" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "class" => Parser::parse_class(
                    src_lines,
                    doctree,
                    line_cursor,
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                    *section_level,
                ),

                "role" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "default-role" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "title" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "restructuredtext-test-directive" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                // SPHINX-SPECIFIC DIRECTIVES
                "toctree" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "versionadded" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "versionchanged" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "deprecated" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "seealso" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "centered" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "hlist" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "highlight" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "code-block" | "sourcecode" => Parser::parse_sphinx_code_block(
                    src_lines,
                    doctree,
                    line_cursor,
                    base_indent,
                    empty_after_marker,
                    body_indent,
                    Some(detected_first_indent),
                ),

                "literalinclude" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "glossary" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "sectionauthor" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "codeauthor" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "index" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "only" => Parser::parse_sphinx_only(
                    src_lines,
                    doctree,
                    line_cursor,
                    base_indent,
                    empty_after_marker,
                    detected_first_indent,
                    body_indent,
                    *section_level,
                ),

                "tabularcolumns" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "productionlist" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                // A+-SPECIFIC DIRECTIVES
                "questionnaire" => Parser::parse_aplus_questionnaire(
                    src_lines,
                    doctree,
                    line_cursor,
                    base_indent,
                    empty_after_marker,
                    detected_first_indent,
                    body_indent,
                ),

                "submit" => Parser::parse_aplus_submit(
                    src_lines,
                    doctree,
                    line_cursor,
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "ae-input" => Parser::parse_aplus_active_element_input(
                    src_lines,
                    doctree,
                    line_cursor,
                    base_indent,
                    empty_after_marker,
                    detected_first_indent,
                    body_indent,
                ),

                "ae-output" => Parser::parse_aplus_active_element_output(
                    src_lines,
                    doctree,
                    line_cursor,
                    base_indent,
                    empty_after_marker,
                    detected_first_indent,
                    body_indent,
                ),

                "hidden_block" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "point-of-interest" => Parser::parse_aplus_point_of_interest(
                    src_lines,
                    doctree,
                    line_cursor,
                    base_indent,
                    empty_after_marker,
                    detected_first_indent,
                    body_indent,
                    *section_level,
                ),

                "annotated" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "lineref-code-block" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "repl-res-count-reset" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "acos-submit" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "div" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "styled-topic" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                // A+ MEDIA DIRECTIVES
                "story" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "jsvee" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "youtube" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "local-video" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                "embedded-page" => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),

                _ => Parser::parse_unknown_directive(
                    doctree,
                    src_lines,
                    line_cursor,
                    detected_directive_label.as_str(),
                    detected_first_indent,
                    body_indent,
                    empty_after_marker,
                ),
            }
        }
        IndentationMatch::TooMuch => {
            doctree = match doctree.push_data_and_focus(TreeNodeType::BlockQuote {
                body_indent: detected_marker_indent,
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
            return TransitionResult::Success {
                doctree: doctree.focus_on_parent(),
                push_or_pop: PushOrPop::Pop,
                line_advance: LineAdvance::None,
            };
        }
    }
}

/// ### comment
/// A function for parsing reST comments.
pub fn comment(
    src_lines: &Vec<String>,
    base_indent: usize,
    section_level: &mut usize,
    line_cursor: &mut LineCursor,
    doctree: Option<DocTree>,
    captures: &regex::Captures,
    pattern_name: &PatternName,
) -> TransitionResult {
    let mut doctree = doctree.unwrap();

    let match_len = captures.get(0).unwrap().as_str().chars().count() + base_indent;
    let detected_marker_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;
    let next_line_indent = if let Some(line) = src_lines.get(line_cursor.relative_offset() + 1) {
        if line.trim().is_empty() {
            match_len
        } else {
            let indent = line.chars().take_while(|c| c.is_whitespace()).count() + base_indent;
            if indent < detected_marker_indent + 1 {
                match_len
            } else {
                indent
            }
        }
    } else {
        match_len
    };

    match Parser::parent_indent_matches(doctree.shared_node_data(), detected_marker_indent) {
        IndentationMatch::JustRight => {
            let (comment_string, offset) = match Parser::read_indented_block(
                src_lines,
                Some(line_cursor.relative_offset()),
                Some(false),
                Some(true),
                Some(next_line_indent - base_indent),
                Some(match_len - base_indent),
                false,
            ) {
                Ok((lines, _, offset, d)) => (lines.join("\n").trim().to_string(), offset),
                Err(e) => {
                    return TransitionResult::Failure {
                        message: format!(
                            "Could not read comment on line {}: {}",
                            line_cursor.sum_total(),
                            e
                        ),
                        doctree: doctree,
                    }
                }
            };

            let comment_data = if comment_string.is_empty() {
                TreeNodeType::Comment { text: None }
            } else {
                TreeNodeType::Comment {
                    text: Some(comment_string),
                }
            };

            return TransitionResult::Success {
                doctree: match doctree.push_data(comment_data) {
                    Ok(doctree) => doctree,
                    Err(doctree) => {
                        return TransitionResult::Failure {
                            message: format!(
                                "Error when reading comment on line {}. Computer says no...",
                                line_cursor.sum_total()
                            ),
                            doctree: doctree,
                        }
                    }
                },
                push_or_pop: PushOrPop::Neither,
                line_advance: LineAdvance::Some(offset),
            };
        }
        IndentationMatch::TooMuch => {
            doctree = match doctree.push_data_and_focus(TreeNodeType::BlockQuote {
                body_indent: detected_marker_indent,
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
            return TransitionResult::Success {
                doctree: doctree.focus_on_parent(),
                push_or_pop: PushOrPop::Pop,
                line_advance: LineAdvance::None,
            };
        }
    }
}

/// ### text
/// A function that handles the parsing of blocks that start with text.
/// This includes paragraphs, but also underlined titles.
/// These are detected via line lookahead.
pub fn text(
    src_lines: &Vec<String>,
    base_indent: usize,
    section_level: &mut usize,
    line_cursor: &mut LineCursor,
    doctree: Option<DocTree>,
    captures: &regex::Captures,
    pattern_name: &PatternName,
) -> TransitionResult {
    let mut doctree = doctree.unwrap();
    let detected_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;

    let next_line = src_lines.get(line_cursor.relative_offset() + 1);

    if next_line.is_some() {
        let next_line_str = next_line.unwrap();

        if let Some(line_capts) = LINE_RE.captures(next_line_str) {
            // Underlined section title
            if detected_indent > 0 {
                return TransitionResult::Failure {
                    message: format!(
                        "Found indented underlined section on line {}. Computer says no...",
                        line_cursor.sum_total()
                    ),
                    doctree: doctree,
                };
            }

            let line_char = next_line_str.chars().next().unwrap();
            let section_style = SectionLineStyle::Under(line_char);
            let title_text = src_lines.get(line_cursor.relative_offset()).unwrap().trim();
            let section_data = doctree.new_section_data(title_text, section_style);

            if let TreeNodeType::Section { level, .. } = section_data {
                let detected_level = level;

                match doctree.shared_data() {
                    TreeNodeType::Document { .. } => {
                        doctree = match doctree.push_data_and_focus(section_data) {
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
                        *section_level = detected_level;
                    }

                    TreeNodeType::Section { level, .. } => {
                        if detected_level <= *level {
                            *section_level = *level;
                            doctree = doctree.focus_on_parent();
                            return TransitionResult::Success {
                                doctree: doctree,
                                push_or_pop: PushOrPop::Pop,
                                line_advance: LineAdvance::None,
                            };
                        } else {
                            *section_level = detected_level;
                            doctree =
                                match doctree.push_data_and_focus(section_data) {
                                    Ok(tree) => tree,
                                    Err(tree) => return TransitionResult::Failure {
                                        message: format!(
                                            "Node insertion error on line {}. Computer says no...",
                                            line_cursor.sum_total()
                                        ),
                                        doctree: tree,
                                    },
                                };
                        }
                    }

                    _ => {
                        doctree = doctree.focus_on_parent();

                        if let TreeNodeType::Section { level, .. } = doctree.shared_data() {
                            *section_level = *level;
                        }

                        return TransitionResult::Success {
                            doctree: doctree,
                            push_or_pop: PushOrPop::Pop,
                            line_advance: LineAdvance::None,
                        };
                    }
                }
                return TransitionResult::Success {
                    doctree: doctree,
                    push_or_pop: PushOrPop::Push(vec![State::Section]),
                    line_advance: LineAdvance::Some(2), // Jump over the section underline
                };
            }
        }

        if let Some(text_capts) = TEXT_RE.captures(next_line_str) {
            // Paragraph or definition list item. Determine based on indentation.

            let next_line_indent =
                text_capts.get(1).unwrap().as_str().chars().count() + base_indent;

            if next_line_indent == detected_indent {
                // Paragraph

                return parse_paragraph(
                    src_lines,
                    base_indent,
                    line_cursor,
                    doctree,
                    detected_indent,
                );
            } else if next_line_indent > detected_indent {
                // Definition list item

                match Parser::parent_indent_matches(doctree.shared_data(), detected_indent) {
                    IndentationMatch::JustRight => {
                        doctree = match doctree.push_data_and_focus(TreeNodeType::DefinitionList {
                            term_indent: detected_indent,
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
                            push_or_pop: PushOrPop::Push(vec![State::DefinitionList]),
                            line_advance: LineAdvance::None,
                        };
                    }
                    IndentationMatch::TooMuch => {
                        doctree = match doctree.push_data_and_focus(TreeNodeType::BlockQuote {
                            body_indent: detected_indent,
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
                        return TransitionResult::Success {
                            doctree: doctree.focus_on_parent(),
                            push_or_pop: PushOrPop::Pop,
                            line_advance: LineAdvance::None,
                        };
                    }
                }
            } else {
                // Paragraph line unaligned with previous lines => syntax error

                return parse_paragraph(
                    src_lines,
                    base_indent,
                    line_cursor,
                    doctree,
                    detected_indent,
                );
            }
        } else {
            return parse_paragraph(
                src_lines,
                base_indent,
                line_cursor,
                doctree,
                detected_indent,
            );
        }
    } else {
        // End of input, so parse current line as a paragraph and leave it at that.
        return parse_paragraph(
            src_lines,
            base_indent,
            line_cursor,
            doctree,
            detected_indent,
        );
    }
}

/// ### line
pub fn line(
    src_lines: &Vec<String>,
    base_indent: usize,
    section_level: &mut usize,
    line_cursor: &mut LineCursor,
    doctree: Option<DocTree>,
    captures: &regex::Captures,
    pattern_name: &PatternName,
) -> TransitionResult {
    let mut doctree = doctree.unwrap();

    /// #### TRANSITION_LINE_LENGTH
    /// The minimum length of a transition line.
    const TRANSITION_LINE_LENGTH: usize = 4;

    let detected_line = captures.get(1).unwrap().as_str();
    let detected_line_char = detected_line.chars().next().unwrap();
    let detected_line_length = detected_line.trim_end().chars().count();

    let current_line = line_cursor.relative_offset();

    let previous_line = if let Some(num) = usize::checked_sub(current_line, 1) {
        src_lines.get(current_line - 1)
    } else {
        None
    };

    let next_line = if let Some(num) = usize::checked_add(current_line, 1) {
        src_lines.get(current_line + 1)
    } else {
        None
    };

    let at_doc_root = if let TreeNodeType::Document { .. } = doctree.shared_node_data() {
        true
    } else {
        false
    };
    let at_input_start = previous_line.is_none();
    let at_input_end = next_line.is_none();

    if at_input_end {
        return TransitionResult::Failure {
      message: format!("Discovered a transition or an incomplete section at the end of (nested) input on line {}. Computer says no...", line_cursor.sum_total()),
      doctree: doctree
    };
    }

    match (previous_line, next_line) {
    (Some(p_line), Some(n_line)) => {
      if p_line.trim().is_empty() && n_line.trim().is_empty() && detected_line_length >= TRANSITION_LINE_LENGTH {

        // A transition can only exist as a child of a document or a section, so a TransitionResult::Success is returned on insertion failure.
        // TODO: add more TransitionResult variants to allow for detection of incompatible parents and children?
        doctree = match doctree.push_data(TreeNodeType::Transition) {
          Ok(tree) => tree,
          Err(tree) => return TransitionResult::Success {
            doctree: tree.focus_on_parent(),
            push_or_pop: PushOrPop::Pop,
            line_advance: LineAdvance::None
          }
        };

        return TransitionResult::Success {
          doctree: doctree,
          push_or_pop: PushOrPop::Neither,
          line_advance: LineAdvance::Some(2) // jump over the empty line following the transition
        }

      } else if TEXT_RE.is_match(n_line) {
        // A possible section title.
        // Check next line for line pattern and its length.

        if let Some(next_next_line) = src_lines.get(line_cursor.relative_offset() + 2) {

          if let Some(capts) = LINE_RE.captures(next_next_line) {

            let next_line_len = n_line.trim_end().chars().count(); // title text line
            let next_next_line_char = next_next_line.trim_end().chars().next().unwrap();
            let next_next_line_len = next_next_line.trim_end().chars().count();

            if detected_line_char == next_next_line_char && detected_line_length == next_next_line_len && next_line_len <= detected_line_length {
              // generate a section.

              let section_line_style = SectionLineStyle::OverAndUnder(detected_line_char);
              let section_data = doctree.new_section_data(n_line.trim(), section_line_style);

              if let TreeNodeType::Section { level, .. } = section_data {

                let detected_level = level;

                match doctree.shared_data() {

                  TreeNodeType::Document { .. } => {
                    doctree = match doctree.push_data_and_focus(section_data) {
                      Ok(tree) => tree,
                      Err(tree) => return TransitionResult::Failure {
                        message: format!("Node insertion error on line {}. Computer says no...", line_cursor.sum_total()),
                        doctree: tree
                      }
                    };
                    *section_level = detected_level;
                  }

                  TreeNodeType::Section { level, .. } => {

                    if detected_level <= *level {
                      *section_level = *level;
                      doctree = doctree.focus_on_parent();
                      return TransitionResult::Success {
                        doctree: doctree,
                        push_or_pop: PushOrPop::Pop,
                        line_advance: LineAdvance::None
                      }
                    } else {
                      *section_level = detected_level;
                      doctree = match doctree.push_data_and_focus(section_data) {
                        Ok(tree) => tree,
                        Err(tree) => return TransitionResult::Failure {
                          message: format!("Node insertion error on line {}. Computer says no...", line_cursor.sum_total()),
                          doctree: tree
                        }
                      };
                    }
                  }

                  _ => {
                    doctree = doctree.focus_on_parent();

                    if let TreeNodeType::Section{level, .. } = doctree.shared_data() {
                      *section_level = *level;
                    }

                    return TransitionResult::Success {
                      doctree: doctree,
                      push_or_pop: PushOrPop::Pop,
                      line_advance: LineAdvance::None
                    }
                  }
                }
                return TransitionResult::Success {
                  doctree: doctree,
                  push_or_pop: PushOrPop::Push(vec![State::Section]),
                  line_advance: LineAdvance::Some(3) // Jump over the section underline
                }
              } else {
                return TransitionResult::Failure {
                  message: format!("No generated section where one was expected on line {}. Computer says no...", line_cursor.sum_total()),
                  doctree: doctree
                }
              }

            } else {
              return TransitionResult::Failure {
                message: format!("Found a section with unmatching over- and underline lengths or characters on line {}. Computer says no...", line_cursor.sum_total()),
                doctree: doctree
              }
            }

          } else {
            return TransitionResult::Failure {
              message: format!("Found section-like construct without underline on line {}. Computer says no...", line_cursor.sum_total()),
              doctree: doctree
            }
          }

        } else {
          return TransitionResult::Failure {
            message: format!("Found something akin to an section title but no underline at the end of input on line {}. Computer says no...", line_cursor.sum_total()),
            doctree: doctree
          }
        }

      } else if captures.get(0).unwrap().as_str().trim() == "::" {
          // Empty paragraph
          return parse_paragraph(src_lines, base_indent, line_cursor, doctree, 0)
      } else {
        return TransitionResult::Failure {
          message: format!("Unknown line construct on line {}. Computer says no...", line_cursor.sum_total()),
          doctree: doctree
        }
      }
    }

    (None, Some(n_line)) => {
      if TEXT_RE.is_match(n_line) {
        // A possible section title.
        // Check next line for line pattern and its length.

        if let Some(next_next_line) = src_lines.get(line_cursor.relative_offset() + 2) {

          if let Some(capts) = LINE_RE.captures(next_next_line) {

            let next_line_len = n_line.trim_end().chars().count(); // title text line
            let next_next_line_char = next_next_line.trim_end().chars().next().unwrap();
            let next_next_line_len = next_next_line.trim_end().chars().count();

            if detected_line_char == next_next_line_char && detected_line_length == next_next_line_len && next_line_len <= detected_line_length {
              // generate a section.

              let section_line_style = SectionLineStyle::OverAndUnder(detected_line_char);
              let section_data = doctree.new_section_data(n_line.trim(), section_line_style);

              if let TreeNodeType::Section { level, .. } = section_data {

                let detected_level = level;

                match doctree.shared_data() {

                  TreeNodeType::Document { .. } => {
                    doctree = match doctree.push_data_and_focus(section_data) {
                      Ok(tree) => tree,
                      Err(tree) => return TransitionResult::Failure {
                        message: format!("Node insertion error on line {}. Computer says no...", line_cursor.sum_total()),
                        doctree: tree
                      }
                    };
                    *section_level = detected_level;
                  }

                  TreeNodeType::Section { level, .. } => {

                    if detected_level <= *level {
                      *section_level = *level;
                      doctree = doctree.focus_on_parent();
                      return TransitionResult::Success {
                        doctree: doctree,
                        push_or_pop: PushOrPop::Pop,
                        line_advance: LineAdvance::None
                      }
                    } else {
                      *section_level = detected_level;
                      doctree = match doctree.push_data_and_focus(section_data) {
                        Ok(tree) => tree,
                        Err(tree) => return TransitionResult::Failure {
                          message: format!("Node insertion error on line {}. Computer says no...", line_cursor.sum_total()),
                          doctree: tree
                        }
                      };
                    }
                  }

                  _ => {
                    doctree = doctree.focus_on_parent();

                    if let TreeNodeType::Section{level, .. } = doctree.shared_data() {
                      *section_level = *level;
                    }

                    return TransitionResult::Success {
                      doctree: doctree,
                      push_or_pop: PushOrPop::Pop,
                      line_advance: LineAdvance::None
                    }
                  }
                }
                return TransitionResult::Success {
                  doctree: doctree,
                  push_or_pop: PushOrPop::Push(vec![State::Section]),
                  line_advance: LineAdvance::Some(3) // Jump over the section underline
                }
              } else {
                return TransitionResult::Failure {
                  message: format!("No generated section where one was expected on line {}. Computer says no...", line_cursor.sum_total()),
                  doctree: doctree
                }
              }

            } else {
              return TransitionResult::Failure {
                message: format!("Found a section with unmatching over- and underline lengths or characters on line {}. Computer says no...", line_cursor.sum_total()),
                doctree: doctree
              }
            }

          } else {
            return TransitionResult::Failure {
              message: format!("Found section-like construct without underline on line {}. Computer says no...", line_cursor.sum_total()),
              doctree: doctree
            }
          }

        } else {
          return TransitionResult::Failure {
            message: format!("Found something akin to an section title but no underline at the end of input on line {}. Computer says no...", line_cursor.sum_total()),
            doctree: doctree
          }
        }

      } else if captures.get(0).unwrap().as_str().trim() == "::" {
        // Empty paragraph
        return parse_paragraph(src_lines, base_indent, line_cursor, doctree, 0)
      } else {
        return TransitionResult::Failure {
          message: format!("No known pattern during a line transition on line {}. Computer says no...", line_cursor.sum_total()),
          doctree: doctree
        }
      }
    }

    _ => return TransitionResult::Failure {
      message: format!("Found a transition-like construct on line {}, but no existing previous or next line. Computer says no...", line_cursor.sum_total()),
      doctree: doctree
    }
  }
}

// ==================
//  Helper functions
// ==================

/// ### foonote_label_to_int
/// Converts a foonote label into a label--target-pair based on the current state of `DocTree.foonote_data`,
/// if possible. Returns an `Option`al pair `(label, target)` if successful.
pub fn detected_footnote_label_to_ref_label(
    doctree: &DocTree,
    pattern_name: &PatternName,
    detected_label_str: &str,
) -> Option<(String, String)> {
    use crate::common::normalize_refname;

    let normalized_name = normalize_refname(detected_label_str);

    if let PatternName::Footnote(kind) = pattern_name {
        match kind {
            FootnoteKind::Manual => {
                // In this case the doctree is simply asked whether it has a reference
                // with this name. If yes, the user is warned of a duplicate label,
                // but otherwise no special action is taken.

                return Some((normalized_name.clone(), normalized_name));
            }

            FootnoteKind::AutoNumbered => {
                // Here we iterate the set of all possible `u32` values
                // and once a number that has not been used as a label is found,
                // it is returned.

                // TODO: retrieve a start value from doctree, so iteration doesn't have to start from 1...

                for n in 1..=EnumAsInt::MAX {
                    let n_str = n.to_string();
                    if doctree.has_target_label(n_str.as_str()) {
                        continue;
                    }
                    return Some((n_str.clone(), n_str));
                }
                eprintln!("All possible footnote numbers in use. Computer says no...");
                return None;
            }

            FootnoteKind::SimpleRefName => {
                // Same as with automatically numbered footnotes, check if this has already a number representation
                // in the doctree and if not, return it.

                for n in 1..=EnumAsInt::MAX {
                    let n_str = n.to_string();
                    if doctree.has_target_label(n_str.as_str()) {
                        continue;
                    }
                    return Some((n_str, normalized_name));
                }
                eprintln!("All possible footnote numbers in use. Computer says no...");
                return None;
            }

            FootnoteKind::AutoSymbol => {
                // Generate a label from crate::common::FOONOTE_SYMBOLS based on the number of autosymbol footnotes
                // entered into the document thus far.

                use crate::common::FOOTNOTE_SYMBOLS; // Import constant locally

                let n = doctree.n_of_symbolic_footnotes() as usize; // No overflow checks with as...

                let n_of_symbols = FOOTNOTE_SYMBOLS.len();

                let passes = n / n_of_symbols;
                let index = n % n_of_symbols;
                let symbol: &char = match FOOTNOTE_SYMBOLS.get(index) {
                    Some(symb) => symb,
                    None => {
                        eprintln!("No footnote symbol with index {}!", index);
                        return None;
                    }
                };

                let label: String = vec![*symbol; passes + 1].iter().collect();
                return Some((label.clone(), label));
            }
        }
    } else {
        eprintln!("No footnote pattern inside a footnote transition function. Computer says no...");
        None
    }
}

/// ### parse_paragraph
/// A helper for parsing a paragraph node.
fn parse_paragraph(
    src_lines: &Vec<String>,
    base_indent: usize,
    line_cursor: &mut LineCursor,
    mut doctree: DocTree,
    detected_indent: usize,
) -> TransitionResult {
    match Parser::parent_indent_matches(doctree.shared_node_data(), detected_indent) {
        IndentationMatch::JustRight => {
            let relative_indent = detected_indent - base_indent;

            let mut block = match Parser::read_text_block(
                src_lines,
                line_cursor.relative_offset(),
                true,
                true,
                Some(relative_indent),
            ) {
                Ok((lines, line_offset)) => lines.join("\n").trim_end().to_string(),
                Err(e) => {
                    return TransitionResult::Failure {
                        message: format!("Error on line {}: {}", line_cursor.sum_total(), e),
                        doctree: doctree,
                    }
                }
            };

            lazy_static! {
              /// There are two kinds of literal block indicators:
              /// 1. preceded by whitespace
              /// 2. not preceded by whitespace
              ///
              /// In the first case, both `::`s will be removed. In the second case, only the first one will disappear.
              static ref LITERAL_BLOCK_INDICATOR: Regex = Regex::new(r"(\s{0,1}|\S)::$").unwrap();
            }

            let literal_block_next: bool = if let Some(capts) =
                LITERAL_BLOCK_INDICATOR.captures(block.as_str())
            {
                // Remove literal block indicator from paragraph
                let indicator_len = if capts.get(1).unwrap().as_str().trim().is_empty() {
                    "::".chars().count()
                } else {
                    ":".chars().count()
                };

                for _ in 0..indicator_len {
                    if let None = block.pop() {
                        return TransitionResult::Failure { // This should not ever be triggered
              message: format!("Tried removing a literal block indicator from a paragraph starting on line {} but failed. Computer says no...", line_cursor.sum_total()),
              doctree: doctree
            };
                    }
                }
                true
            } else {
                false
            };

            doctree = match doctree.push_data_and_focus(TreeNodeType::Paragraph {
                indent: detected_indent,
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

            // Pass text to inline parser as a string
            doctree = match Parser::inline_parse(block, Some(&mut doctree), line_cursor) {
                InlineParsingResult::Nodes(nodes_data) => {
                    for data in nodes_data {
                        doctree = match doctree.push_data(data) {
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

                    doctree.focus_on_parent()
                }
                InlineParsingResult::NoNodes => {
                    doctree = doctree.focus_on_parent();
                    match doctree.pop_child() {
                        Some(child) => {
                            eprintln!(
                                "Removing an empty paragraph from the tree on line {}...",
                                line_cursor.sum_total()
                            );
                        }
                        None => {}
                    };
                    doctree
                }
            };

            if literal_block_next {
                return TransitionResult::Success {
                    doctree: doctree,
                    push_or_pop: PushOrPop::Push(vec![State::LiteralBlock]),
                    line_advance: LineAdvance::Some(1),
                };
            } else {
                return TransitionResult::Success {
                    doctree: doctree,
                    push_or_pop: PushOrPop::Neither,
                    line_advance: LineAdvance::Some(1),
                };
            }
        }

        IndentationMatch::TooMuch => {
            doctree = match doctree.push_data_and_focus(TreeNodeType::BlockQuote {
                body_indent: detected_indent,
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
            return TransitionResult::Success {
                doctree: doctree.focus_on_parent(),
                push_or_pop: PushOrPop::Pop,
                line_advance: LineAdvance::None,
            };
        }
    }
}
