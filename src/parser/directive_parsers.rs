/*!
A file that contains functions dedicated to
parsing *directives*, reStructuredText extensions.
All of these follow the same basic pattern:

1. check for directive argument
2. check for directive options and filter them for accepted ones,
3. return with a transition result, so that the `parser` might continue
    parsing the contents of the directive node.

Some small deviations from this pattern might occur,
since a directive might not accept arguments or options,
or just not have any content.

Copyright © 2020 Santtu Söderholm
*/
use std::collections::HashMap;

use crate::common::ParsingResult;
use crate::doctree::tree_node_types::TreeNodeType;
use crate::doctree::DocTree;
use crate::parser::line_cursor::LineCursor;
use crate::parser::state_machine::State;
use crate::parser::types_and_aliases::{
    InlineParsingResult, LineAdvance, PushOrPop, TransitionResult,
};
use crate::parser::Parser;
use crate::parser::converters;
use crate::parser::types_and_aliases::IndentedBlockResult;
use crate::parser::types_and_aliases::TextBlockResult;

pub fn parse_standard_admonition(
    src_lines: &[String],
    body_indent: usize,
    section_level: usize,
    first_indent: usize,
    mut doctree: DocTree,
    line_cursor: &mut LineCursor,
    admonition_type: &str,
    empty_after_marker: bool,
) -> TransitionResult {
    use crate::doctree::directives::AdmonitionType;

    let variant: AdmonitionType = match admonition_type {
        "attention" => AdmonitionType::Attention,
        "caution" => AdmonitionType::Caution,
        "danger" => AdmonitionType::Danger,
        "error" => AdmonitionType::Error,
        "hint" => AdmonitionType::Hint,
        "important" => AdmonitionType::Important,
        "note" => AdmonitionType::Note,
        "tip" => AdmonitionType::Tip,
        "warning" => AdmonitionType::Warning,
        _ => unreachable!(
            "No standard admonition type \"{}\" on line {}. Computer says no...",
            admonition_type,
            line_cursor.sum_total()
        ),
    };

    let mut arg_lines = if let Some(arg) = scan_directive_arguments(
        src_lines,
        line_cursor,
        body_indent,
        Some(first_indent),
        empty_after_marker,
    ) {
        arg
    } else {
        Vec::new()
    };

    // Try scanning for options, if first block was empty
    let (classes, name) = if let Some(mut options) =
        scan_directive_options(src_lines, line_cursor, body_indent)
    {
        (
            options.remove("class"),
            options.remove("name"),
        )
    } else {
        (None, None)
    };

    // Read in the rest of the admonition contents...
    let offset = match Parser::read_indented_block(
        src_lines,
        line_cursor.relative_offset(),
        false,
        true,
        Some(body_indent),
        Some(body_indent),
        false) {
            IndentedBlockResult::Ok {mut lines, minimum_indent, offset, blank_finish } => {
            arg_lines.append(&mut lines);
            offset
        },
        _ => return TransitionResult::Failure {
            message: format!("Error when reading in the contents of \"{}\" around line {}. Computer says no...", variant.to_string(), line_cursor.sum_total()),
            doctree: doctree
        }
    };

    // Create admonition node...
    let admonition_data = TreeNodeType::Admonition {
        content_indent: body_indent,
        classes: classes,
        name: name,
        variant: variant.clone(),
    };

    // Focus on the created node...
    doctree = match doctree.push_data_and_focus(admonition_data) {
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

    // Start nested parse inside admonition...
    let (doctree, nested_state_stack) = match Parser::new(
        &arg_lines,
        doctree,
        body_indent,
        line_cursor.sum_total(),
        State::Admonition,
        section_level,
    ).parse() {
        ParsingResult::EOF {
            doctree,
            state_stack,
        } => (doctree, state_stack),
        ParsingResult::EmptyStateStack {
            doctree,
            state_stack,
        } => (doctree, state_stack),
        ParsingResult::Failure { message, doctree } => {
            return TransitionResult::Failure {
                message: format!(
                    "Error when parsing a \"{}\" on line {}: {}",
                    variant,
                    line_cursor.sum_total(),
                    message
                ),
                doctree: doctree,
            }
        }
    };

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Push(nested_state_stack),
        line_advance: LineAdvance::Some(offset),
    }
}

/// Much like `parse_standard_admonition`, except
/// 1. first checks that the admonition contains an argument,
/// 2. then checks for possible options and
/// 3. focuses on the admonition itself.
pub fn parse_generic_admonition(
    src_lines: &[String],
    mut doctree: DocTree,
    line_cursor: &mut LineCursor,
    empty_after_marker: bool,
    body_indent: usize,
    first_indent: Option<usize>,
) -> TransitionResult {

    let argument = if let Some(arg) =
        scan_directive_arguments(src_lines, line_cursor, body_indent, first_indent, empty_after_marker)
    {
        arg
    } else {
        return TransitionResult::Failure {
            message: format!("General admonition on line {} does not contain a compulsory title argument. Computer says no...", line_cursor.sum_total()),
            doctree: doctree
        };
    };

    let directive_options =
        scan_directive_options(src_lines, line_cursor, body_indent);

    let (classes, name) = if let Some(mut options) = directive_options {
        (
            options.remove("class"),
            options.remove("name")
        )
    } else {
        (None, None)
    };

    let admonition_data = TreeNodeType::Admonition {
        content_indent: body_indent,
        classes: classes,
        name: name,
        variant: crate::doctree::directives::AdmonitionType::Admonition {
            title: argument.join(" "),
        },
    };

    doctree = match doctree.push_data_and_focus(admonition_data) {
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

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Push(vec![State::Admonition]),
        line_advance: LineAdvance::None,
    }
}

pub fn parse_image(
    src_lines: &[String],
    mut doctree: DocTree,
    line_cursor: &mut LineCursor,
    empty_after_marker: bool,
    body_indent: usize,
    first_indent: Option<usize>,
) -> TransitionResult {

    let argument = if let Some(arg) =
        scan_directive_arguments(src_lines, line_cursor, body_indent, first_indent, empty_after_marker)
    {
        arg
    } else {
        return TransitionResult::Failure {
            message: format!(
                "Image on line {} does not contain a compulsory image URI. Computer says no...",
                line_cursor.sum_total()
            ),
            doctree: doctree,
        };
    };

    let (alt, height, width, scale, align, target, classes, name) =
        if let Some(mut options) = scan_directive_options(src_lines, line_cursor, body_indent) {
            (
                options.remove("alt"),
                options.remove("height"),
                options.remove("width"),
                options.remove("scale"),
                options.remove("align"),
                options.remove("target"),
                options.remove("class"),
                options.remove("name")
            )
        } else {
            (None, None, None, None, None, None, None, None)
        };

    let image_data = TreeNodeType::Image {
        uri: argument.join(""),
        alt: alt,
        height: if let Some(h) = &height {
            converters::str_to_length(h)
        } else {
            None
        },
        width: if let Some(w) = &width {
            converters::str_to_length(w)
        } else {
            None
        },
        scale: if let Some(scale) = &scale {
            converters::str_to_percentage(scale)
        } else {
            None
        },
        align: if let Some(a) = &align {
            converters::str_to_html_alignment(a)
        } else {
            None
        },
        target: target,
        name: name,
        class: classes,
        inline: false
    };

    doctree = match doctree.push_data(image_data) {
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

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Neither,
        line_advance: LineAdvance::None,
    }
}

pub fn parse_figure(
    src_lines: &[String],
    mut doctree: DocTree,
    line_cursor: &mut LineCursor,
    base_indent: usize,
    empty_after_marker: bool,
    body_indent: usize,
    first_indent: Option<usize>,
    section_level: usize,
) -> TransitionResult {
    let argument = if let Some(arg) =
        scan_directive_arguments(src_lines, line_cursor, body_indent, first_indent, empty_after_marker)
    {
        arg
    } else {
        return TransitionResult::Failure {
    message: format!("Figure on line {} does not contain a compulsory image URI. Computer says no...", line_cursor.sum_total()),
    doctree: doctree
    };
    };

    let (alt, height, width, scale, align, target, classes, name, figwidth, figclass) =
        if let Some(mut options) =
            scan_directive_options(src_lines, line_cursor, body_indent)
        {
            (
                options.remove("alt"),
                options.remove("height"),
                options.remove("width"),
                options.remove("scale"),
                options.remove("align"),
                options.remove("target"),
                options.remove("class"),
                options.remove("name"),
                options.remove("figwidth"),
                options.remove("figclass")
            )
        } else {
            (None, None, None, None, None, None, None, None, None, None)
        };

    // Construct the contained image
    let image = TreeNodeType::Image {
        uri: argument.join(""),

        alt: alt,
        height: if let Some(h) = height {
            converters::str_to_length(&h)
        } else {
            None
        },
        width: if let Some(w) = width {
            converters::str_to_length(&w)
        } else {
            None
        },
        scale: if let Some(scale) = &scale {
            converters::str_to_percentage(scale)
        } else {
            None
        },
        align: None, // Image does not have alignenment inside a figure.
        target: target,
        class: classes,
        name: None, // Sphinx patch moved "name" to containing figure node
        inline: false
    };

    let figure = TreeNodeType::Figure {
        body_indent: body_indent,
        align: if let Some(a) = &align {
            converters::str_to_horizontal_alignment(a)
        } else {
            None
        },
        figclass: figclass,
        figwidth: if let Some(w) = &figwidth {
            converters::str_to_length(w)
        } else {
            None
        },
        name: if let Some(refname) = &name {
            Some(crate::common::normalize_refname(refname))
        } else {
            None
        }
    };

    // Add figure node to tree and focus on it
    doctree = match doctree.push_data_and_focus(figure) {
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

    // Add image to figure
    doctree = match doctree.push_data(image) {
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

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Push(vec![State::Figure]),
        line_advance: LineAdvance::None,
    }
}

pub fn parse_topic() {
    todo!()
}

pub fn parse_sidebar() {
    todo!()
}

pub fn parse_line_block() {
    todo!()
}

pub fn parse_parsed_literal() {
    todo!()
}

/// The "code" directive parser.
pub fn parse_code(
    src_lines: &[String],
    mut doctree: DocTree,
    line_cursor: &mut LineCursor,
    base_indent: usize,
    empty_after_marker: bool,
    body_indent: usize,
    first_indent: Option<usize>,
    section_level: usize,
) -> TransitionResult {

    let language = if let Some(arg) =
        scan_directive_arguments(src_lines, line_cursor, body_indent, first_indent, empty_after_marker)
    {
        Some(arg.join(""))
    } else {
        None
    };

    let directive_options =
        scan_directive_options(src_lines, line_cursor, body_indent);

    let (classes, name, number_lines) = if let Some(mut options) = directive_options {
        (
            options.remove("class"),
            options.remove("name"),
            options.remove("number-lines"),
        )
    } else {
        (None, None, None)
    };

    let (lines, offset) = if let IndentedBlockResult::Ok {lines, minimum_indent, offset, blank_finish } = Parser::read_indented_block(
        src_lines,
        line_cursor.relative_offset(),
        false,
        true,
        Some(body_indent),
        None,
        false,
    ) {
        (lines, offset)
    } else {
        return TransitionResult::Failure {
            message: format!(
                "Could not read the code block on line {}. Computer says no...",
                line_cursor.sum_total()
            ),
            doctree: doctree,
        };
    };

    let code_block = TreeNodeType::Code {
        text: lines.join("\n"),
        language: language,
        number_lines: number_lines,
        class: classes,
        name: name,
    };

    doctree = match doctree.push_data(code_block) {
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

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Neither,
        line_advance: LineAdvance::Some(offset),
    }
}

/// The display math parser. Content blocks separated by a blank lines are put in adjacent math blocks.
pub fn parse_math_block(
    src_lines: &[String],
    mut doctree: DocTree,
    line_cursor: &mut LineCursor,
    body_indent: usize,
    empty_after_marker: bool,
    first_indent: usize,
) -> TransitionResult {

    let math_after_marker = scan_directive_arguments(
        src_lines,
        line_cursor,
        body_indent,
        Some(first_indent),
        empty_after_marker
    );

    let (classes, name, nowrap, label) = if let Some(mut options)
        = scan_directive_options(src_lines, line_cursor, body_indent)
    {
        (
            options.remove("class"),
            options.remove("name"),
            // These were added by Sphinx
            options.remove("nowrap"),
            options.remove("label"),
        )
    } else {
        (None, None, None, None)
    };

    // If an equation was given as an argument, quit early
    if let Some(math) = math_after_marker {
        doctree = match doctree.push_data(
            TreeNodeType::MathBlock {
                math_block: math.join("\n"),
                class: classes,
                name: name,
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
            push_or_pop: PushOrPop::Neither,
            line_advance: LineAdvance::None,
        };
    }

    // If no equation as argument, try reading block contents as multiple equations...
    let (lines, offset) = match Parser::read_indented_block(
        src_lines,
        line_cursor.relative_offset(),
        false,
        true,
        Some(body_indent),
        Some(body_indent),
        false
    ) {
        IndentedBlockResult::Ok {lines, minimum_indent, offset, blank_finish } => (lines, offset),
        _ => return TransitionResult::Failure {
            message: format!(
                "Could not read the math block on line {}. Computer says no...",
                line_cursor.sum_total()
            ),
            doctree: doctree,
        }
    };

    // Scan lines for blocks separated by blank lines
    let blocks = {
        let mut blocks = Vec::new();
        let mut block = String::new();

        for line in lines.iter() {
            if ! line.trim().is_empty() {
                block = block + "\n" + line;
            } else if line.trim().is_empty() && !block.trim().is_empty() {
                blocks.push(block);
                block = String::new();
            } else {
                continue
            }
        }

        if ! block.trim().is_empty() {
            blocks.push(block)
        }

        blocks
    };

    if blocks.is_empty() {
        return TransitionResult::Failure {
            message: format!("Tried reading a math block on line {} but didn't find any actual content. Computer says no...", line_cursor.sum_total()),
            doctree: doctree
        };
    }

    // A counter for separating refnames of multiple blocks generated by the same directive
    let mut refname_counter = if blocks.len() == 1 {
        None
    } else {
        Some(0)
    };
    for block in blocks {
        doctree = match doctree.push_data(
            TreeNodeType::MathBlock {
                math_block: block.trim().to_string(),
                name: if let Some(name) = &name {
                    match &mut refname_counter {
                        Some(counter) => {
                            *counter += 1;
                            Some(name.clone() + &(counter.to_string()))
                        },
                        None => Some(name.clone())
                    }
                } else {
                    None
                },
                class: classes.clone(),
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
    }

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Neither,
        line_advance: LineAdvance::Some(offset),
    }
}

pub fn parse_rubric() {
    todo!()
}

pub fn parse_epigraph() {
    todo!()
}

pub fn parse_highlights() {
    todo!()
}

pub fn parse_pull_quote() {
    todo!()
}

pub fn parse_compound() {
    todo!()
}

pub fn parse_container() {}

pub fn parse_table() {
    todo!()
}

pub fn parse_csv_table() {
    todo!()
}

pub fn parse_list_table(
    src_lines: &[String],
    mut doctree: DocTree,
    line_cursor: &mut LineCursor,
    base_indent: usize,
    empty_after_marker: bool,
    first_indent: Option<usize>,
    body_indent: usize,
    section_level: usize,
) -> TransitionResult {

    let table_title = if let Some(title) = scan_directive_arguments(
        src_lines,
        line_cursor,
        body_indent,
        first_indent,
        empty_after_marker,
    ) {
        title.join(" ")
    } else {
        String::new()
    };

    let (header_rows, stub_columns, width, widths, class, name, align) =
        if let Some(mut options) = scan_directive_options
        (src_lines, line_cursor, body_indent) {
            (
                options.remove("header-rows"),
                options.remove("stub-columns"),
                options.remove("width"),
                options.remove("widths"),
                options.remove("class"),
                options.remove("name"),
                options.remove("align"),
            )
        } else {
            (None, None, None, None, None, None, None)
        };

    use crate::common::{HorizontalAlignment, MetricType, TableColWidths};

    let list_table_node = TreeNodeType::ListTable {
        body_indent: body_indent,

        title: if !table_title.is_empty() {
            Some(table_title)
        } else {
            None
        },
        widths: if let Some(widths) = widths {
            if widths.as_str().trim() == "auto" {
                Some(TableColWidths::Auto)
            } else {
                let widths = widths
                    .split_whitespace()
                    .filter(
                        |s| ! s.is_empty()
                    )
                    .map(
                        |int|
                            if let Ok(result) = int.parse::<f64>() {
                                result
                            } else {
                                panic!(
                                    "Tried converting a list table column width \"{}\" into a integer on line {} but failed. Computer says no...",
                                    int,
                                    line_cursor.sum_total()
                                );
                            }
                    )
                    .collect::<Vec<f64>>();
                if widths.len() == 0 {
                    None
                } else {
                    Some(TableColWidths::Columns(widths))
                }
            }
        } else {
            None
        },
        width: if let Some(width) = width {
            if let Some(length) = converters::str_to_length(&width) {
                Some(MetricType::Lenght(length))
            } else if let Some(percentage) = converters::str_to_percentage(&width) {
                Some(crate::common::MetricType::Percentage(percentage))
            } else {
                None
            }
        } else {
            None
        },
        header_rows: if let Some(num) = header_rows {
            if let Ok(result) = num.parse::<u32>() {
                Some(result)
            } else {
                eprintln!(
                    "Could not parse list-table header-rows setting to integer on line {}...",
                    line_cursor.sum_total()
                );
                None
            }
        } else {
            None
        },
        stub_columns: if let Some(num) = stub_columns {
            if let Ok(result) = num.parse::<u32>() {
                Some(result)
            } else {
                eprintln!(
                    "Could not parse list-table stub-columns setting to integer on line {}...",
                    line_cursor.sum_total()
                );
                None
            }
        } else {
            None
        },
        align: if let Some(alignment) = align {
            match alignment.as_str() {
                "left" => Some(HorizontalAlignment::Left),
                "center" => Some(HorizontalAlignment::Center),
                "right" => Some(HorizontalAlignment::Right),
                _ => {
                    eprintln!("Found an alignment setting for list table on line {}, but setting not valid...", line_cursor.sum_total());
                    None
                }
            }
        } else {
            None
        },
    };

    Parser::skip_empty_lines(src_lines, line_cursor);

    doctree = match doctree.push_data_and_focus(list_table_node) {
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

    let (lines, offset) = if let IndentedBlockResult::Ok {lines, minimum_indent, offset, blank_finish } = Parser::read_indented_block(
        src_lines,
        line_cursor.relative_offset(),
        false,
        true,
        Some(body_indent),
        None,
        false,
    ) {
        (lines, offset)
    } else {
        return TransitionResult::Failure {
            message: format!("Could not read the legend contents of the figure on line {}. Computer says no...", line_cursor.sum_total()),
            doctree: doctree
        };
    };

    let (mut doctree, mut nested_state_stack) = match Parser::new(
        &lines,
        doctree,
        body_indent,
        line_cursor.sum_total(),
        State::ListTable,
        section_level,
    ).parse() {
        ParsingResult::EOF {
            doctree,
            state_stack,
        } => (doctree, state_stack),
        ParsingResult::EmptyStateStack {
            doctree,
            state_stack,
        } => (doctree, state_stack),
        ParsingResult::Failure { message, doctree } => {
            return TransitionResult::Failure {
                message: format!(
                    "Error when parsing a list-table on line {}: {}",
                    line_cursor.sum_total(),
                    message
                ),
                doctree: doctree,
            };
        }
    };

    // Focus back on list-table
    while nested_state_stack.len() > 1 {
        nested_state_stack.pop();
        doctree = doctree.focus_on_parent()
    }

    if let TreeNodeType::ListTable { .. } = doctree.shared_data() {
        // A-Ok
    } else {
        return TransitionResult::Failure {
            message: format!("Not focused on list-table after parsing its contents starting on line {}. Computer says no...", line_cursor.sum_total()),
            doctree: doctree
        };
    };

    // Check largest number of columns and validate list at the same time

    let n_of_columns = {
        let mut max_cols: u32 = 0;

        if let Some(children) = doctree.shared_children() {
            if let Some(child_list) = children.get(0) {
                if let TreeNodeType::BulletList { .. } = child_list.shared_data() {
                    if let Some(list_items) = child_list.shared_children() {
                        // Go over sublists and count the number of children in them
                        for list_item in list_items {
                            if let Some(children) = list_item.shared_children() {
                                if let Some(nested_list) = children.get(0) {
                                    if let TreeNodeType::BulletList { .. } = nested_list.shared_data() {
                                        if let Some(items) = nested_list.shared_children() {
                                            let row_entry_count = items
                                                .iter()
                                                .filter(
                                                    |item| {
                                                        if let TreeNodeType::BulletListItem { .. } = item.shared_data() {
                                                            true
                                                        } else {
                                                            false
                                                        }
                                                    }
                                                )
                                                .count() as u32;
                                            max_cols = std::cmp::max(max_cols, row_entry_count);
                                        } else {
                                            return TransitionResult::Failure {
                                                message: format!("Second level list has no children inside list-table before line {}. Computer says no...", line_cursor.sum_total()),
                                                doctree: doctree
                                            };
                                        }
                                    } else {
                                        return TransitionResult::Failure {
                                            message: format!("No second level bullet list inside list-table before line {}. Computer says no...", line_cursor.sum_total()),
                                            doctree: doctree
                                        };
                                    }
                                } else {
                                    return TransitionResult::Failure {
                                        message: format!("List item in list-table on line {} does not contain children. Computer says no...", line_cursor.sum_total()),
                                        doctree: doctree
                                    };
                                }
                            } else {
                                return TransitionResult::Failure {
                                    message: format!("First level list item inside list-table on line {} has no children. Computer says no...", line_cursor.sum_total()),
                                    doctree: doctree
                                };
                            }
                        }
                    } else {
                        return TransitionResult::Failure {
                            message: format!("Bullet list in list-table on line {} cannot have children? Computer says no...", line_cursor.sum_total()),
                            doctree: doctree
                        };
                    }
                } else {
                    return TransitionResult::Failure {
                        message: format!("First child if list-table on line {} is not a bullet list. Computer says no...", line_cursor.sum_total()),
                        doctree: doctree
                    };
                }
            } else {
                return TransitionResult::Failure {
                    message: format!(
                        "List-table on line {} has no children. Computer says no...",
                        line_cursor.sum_total()
                    ),
                    doctree: doctree,
                };
            }
        } else {
            return TransitionResult::Failure {
                message: format!(
                    "List-table before line {} cannot have children? Computer says no...",
                    line_cursor.sum_total()
                ),
                doctree: doctree,
            };
        }
        max_cols
    };

    // Set column widths, if not yet set

    if let TreeNodeType::ListTable { widths, .. } = doctree.mut_node_data() {
        if widths.is_none() {
            *widths = Some(
                TableColWidths::Columns(
                    std::iter::repeat(1.0 / n_of_columns as f64)
                        .take(n_of_columns as usize)
                        .collect(),
                )
            )
        }
    } else {
        return TransitionResult::Failure {
            message: format!("Not focused on list-table before line {}, after validating said table. Computer says no...", line_cursor.sum_total()),
            doctree: doctree
        };
    }

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Push(vec![State::ListTable]),
        line_advance: LineAdvance::Some(offset),
    }
}

pub fn parse_contents() {
    todo!()
}

pub fn parse_section_numbering() {
    todo!()
}

pub fn parse_header_or_footer() {
    todo!()
}

pub fn parse_target_notes() {
    todo!()
}

pub fn parse_footnotes() {
    todo!()
}

pub fn parse_citations() {
    todo!()
}

pub fn parse_meta() {
    todo!()
}

/// Parses in "include" directive for its argument and options.
/// Generates an "include" node in the parse tree with the given options.
pub fn parse_include(
    src_lines: &[String],
    mut doctree: DocTree,
    line_cursor: &mut LineCursor,
    first_indent: usize,
    body_indent: usize,
    empty_after_marker: bool,
    section_level: usize,
) -> TransitionResult {

    let uri = match scan_directive_arguments(src_lines, line_cursor, body_indent, Some(first_indent), empty_after_marker) {
        Some(arg) => arg.join(""),
        None => return TransitionResult::Failure {
            message: format!("Include directive on line {} did not have a file URI as an argument.", line_cursor.sum_total()),
            doctree: doctree
        }
    };

    let (
        start_line,
        end_line,
        start_after,
        end_before,
        literal,
        code,
        number_lines,
        encoding,
        tab_width,
        name,
        class
    ) = if let Some(mut options) = scan_directive_options
    (src_lines, line_cursor, body_indent) {
        let start_line = if let Some(option) = options.remove("start-line") {
            match option.parse::<u32>() {
                Ok(num) => Some(num),
                Err(_) => None
            }
        } else {
            None
        };
        let end_line = if let Some(option) = options.remove("end-line") {
            match option.parse::<u32>() {
                Ok(num) => Some(num),
                Err(_) => None
            }
        } else {
            None
        };
        let start_after = if let Some(option) = options.remove("start-after") {
            match option.parse::<u32>() {
                Ok(num) => Some(num),
                Err(_) => None
            }
        } else {
            None
        };
        let end_before = if let Some(option) = options.remove("end-before") {
            match option.parse::<u32>() {
                Ok(num) => Some(num),
                Err(_) => None
            }
        } else {
            None
        };
        let literal = if let Some(option) = options.remove("literal") {
            true
        } else {
            false
        };
        let code = if let Some(language) = options.remove("code") {
            if language.trim().is_empty() {
                Some(None)
            } else {
                Some(Some(language))
            }
        } else {
            None
        };
        let number_lines = if let Some(option) = options.remove("code") {
            if option.trim().is_empty() {
                Some(None)
            } else {
                match option.parse::<u32>() {
                    Ok(number) => Some(Some(number)),
                    Err(_) => Some(None)
                }
            }
        } else {
            None
        };
        let encoding = if let Some(encoding) = options.remove("encoding") {
            Some(encoding)
        } else {
            None
        };
        let tab_width = if let Some(option) = options.remove("tab-width") {
            match option.parse::<u32>() {
                Ok(number) => Some(number),
                Err(_) => None
            }
        } else {
            None
        };
        let name = if let Some(option) = options.remove("name") {
            Some(option)
        } else {
            None
        };
        let class = if let Some(option) = options.remove("class") {
            Some(option)
        } else {
            None
        };

        (start_line, end_line, start_after, end_before, literal, code, number_lines, encoding, tab_width, name, class)

    } else {
        (None, None, None, None, false, None, None, None, None, None, None)
    };

    let include_node_data = TreeNodeType::Include {
        uri: uri,
        start_line: start_line,
        end_line: end_line,
        start_after: start_after,
        end_before: end_before,
        literal: literal,
        code: code,
        number_lines: number_lines,
        encoding: encoding,
        tab_width: tab_width,
        name: name,
        class: class
    };

    doctree = match doctree.push_data(include_node_data) {
        Ok(tree) => tree,
        Err(tree) => return TransitionResult::Failure {
            message: format!("Node insertion error on line {}.", line_cursor.sum_total()),
            doctree: tree
        }
    };

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Neither,
        line_advance: LineAdvance::None
    }
}

pub fn parse_raw() {
    todo!()
}

pub fn parse_class(
    src_lines: &[String],
    mut doctree: DocTree,
    line_cursor: &mut LineCursor,
    first_indent: usize,
    body_indent: usize,
    empty_after_marker: bool,
    section_level: usize,
) -> TransitionResult {
    let classes = if let Some(classes) = scan_directive_arguments(
        src_lines,
        line_cursor,
        body_indent,
        Some(first_indent),
        empty_after_marker,
    ) {
        classes
            .iter()
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    } else {
        return TransitionResult::Failure {
            message: format!(
                "Class directive on line {} doesn't provide any classes. Computer says no...",
                line_cursor.sum_total()
            ),
            doctree: doctree,
        };
    };

    let class_node = TreeNodeType::Class {
        body_indent: body_indent,
        classes: classes,
    };

    doctree = match doctree.push_data_and_focus(class_node) {
        Ok(tree) => tree,
        Err(tree) => {
            return TransitionResult::Failure {
                message: format!(
                    "Failed to push class node to tree on line {}...",
                    line_cursor.sum_total()
                ),
                doctree: tree,
            }
        }
    };

    let (lines, offset) = if let IndentedBlockResult::Ok {lines, minimum_indent, offset, blank_finish } = Parser::read_indented_block(
        src_lines,
        line_cursor.relative_offset(),
        false,
        true,
        Some(body_indent),
        None,
        false,
    ) {
        (lines, offset)
    } else {
        return TransitionResult::Failure {
            message: format!(
                "Could not parse class contents starting from line {}. Computer says no...",
                line_cursor.sum_total()
            ),
            doctree: doctree,
        };
    };

    let (doctree, nested_state_stack) = match Parser::new(
        &lines,
        doctree,
        body_indent,
        line_cursor.sum_total(),
        State::Body,
        section_level,
    ).parse() {
        ParsingResult::EOF {
            doctree,
            state_stack,
        } => (doctree, state_stack),
        ParsingResult::EmptyStateStack {
            doctree,
            state_stack,
        } => (doctree, state_stack),
        ParsingResult::Failure { message, doctree } => {
            return TransitionResult::Failure {
                message: format!(
                    "Error when parsing a class on line {}: {}",
                    line_cursor.sum_total(),
                    message
                ),
                doctree: doctree,
            }
        }
    };

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Push(nested_state_stack),
        line_advance: LineAdvance::None,
    }
}

pub fn parse_role() {
    todo!()
}

pub fn parse_default_role() {
    todo!()
}

pub fn parse_title() {
    todo!()
}

pub fn restucturetext_test_directive() {
    todo!()
}

// ========================
//  Sphinx-specific directives
// ========================

pub fn parse_sphinx_toctree() {
    todo!()
}

pub fn parse_sphinx_versionadded() {
    todo!()
}

pub fn parse_sphinx_versionchanged() {
    todo!()
}

pub fn parse_sphinx_deprecated() {
    todo!()
}

pub fn parse_sphinx_seealso() {
    todo!()
}

pub fn parse_sphinx_centered() {
    todo!()
}

pub fn parse_sphinx_hlist() {
    todo!()
}

pub fn parse_sphinx_highlight() {
    todo!()
}

/// A parser for the Sphinx-specific `code-block` directive. See https://www.sphinx-doc.org/en/master/usage/restructuredtext/directives.html#directive-code-block
/// for explanations of different settings and arguments.
pub fn parse_sphinx_code_block(
    src_lines: &[String],
    mut doctree: DocTree,
    line_cursor: &mut LineCursor,
    base_indent: usize,
    empty_after_marker: bool,
    body_indent: usize,
    first_indent: Option<usize>,
) -> TransitionResult {
    // Read directive argument: the formal language (should be recognized by Pygments)
    let formal_language = if let Some(arg) = scan_directive_arguments(
        src_lines,
        line_cursor,
        body_indent,
        first_indent,
        empty_after_marker,
    ) {
        arg.join("")
    } else {
        String::from("python") // the Sphinx "highlight_language" setting default
    };

    // Read the settings...
    let (linenos, lineno_start, emphasize_lines, caption, name, dedent, force) =
        if let Some(mut settings) = scan_directive_options(
            src_lines, line_cursor, body_indent
        ){
            let mut linenos = if let Some(linenos) = settings.remove("linenos") {
                true
            } else {
                false
            };
            let lineno_start = if let Some(start_line) = settings.remove("lineno-start") {
                if let Ok(number) = start_line.parse::<usize>() {
                    linenos = true;
                    Some(number)
                } else {
                    None
                }
            } else {
                None
            };
            let emphasize_lines = if let Some(line_numbers) = settings.remove("emphasize-lines")
            {
                let emph_lines = line_numbers
                    .split(",")
                    .filter(|s| !s.trim().is_empty())
                    .map(|s| s.trim())
                    .filter_map(|s| s.parse::<usize>().ok())
                    .collect::<Vec<usize>>();

                Some(emph_lines)
            } else {
                None
            };
            let caption = settings.remove("caption");
            let name = if let Some(refname) = settings.remove("name") {
                Some(crate::common::normalize_refname(&refname))
            } else {
                None
            };
            let dedent = if let Some(dedent) = settings.remove("dedent") {
                if let Ok(dedent) = dedent.parse::<usize>() {
                    Some(dedent)
                } else {
                    None
                }
            } else {
                None
            };
            let force = if let Some(force) = settings.remove("force") {
                true
            } else {
                false
            };

            (
                linenos,
                lineno_start,
                emphasize_lines,
                caption,
                name,
                dedent,
                force,
            )
        } else {
            (false, None, None, None, None, None, false)
        };

    // Construct node from settings and read content...

    let (code_text, offset) = match Parser::read_indented_block(
        src_lines,
        line_cursor.relative_offset(),
        false,
        true,
        Some(body_indent),
        None,
        false,
    ) {
        IndentedBlockResult::Ok {mut lines, minimum_indent, offset, blank_finish } => {
            // Remove empty lines from front
            lines = lines
                .iter()
                .skip_while(|line| line.is_empty())
                .map(|s| s.to_string())
                .collect();

            // Remove empty lines from back
            while let Some(line) = lines.last_mut() {
                if line.is_empty() {
                    lines.pop();
                } else {
                    break;
                }
            }

            (lines.join("\n") + "\n", offset)
        }
        _ => {
            return TransitionResult::Failure {
                message: format!(
                    "Error when parsing a Sphinx code block on line {}.",
                    line_cursor.sum_total(),
                ),
                doctree: doctree,
            }
        }
    };

    let code_block_data = TreeNodeType::SphinxCodeBlock {
        language: formal_language,
        linenos: linenos,
        lineno_start: lineno_start,
        emphasize_lines: emphasize_lines,
        caption: caption,
        name: name,
        dedent: dedent,
        force: force,
        code_text: code_text,
    };

    doctree = match doctree.push_data(code_block_data) {
        Ok(tree) => tree,
        Err(tree) => {
            return TransitionResult::Failure {
                message: format!(
                    "Erro when parsing Sphinx code block on line {}. Computer says no...",
                    line_cursor.sum_total()
                ),
                doctree: tree,
            }
        }
    };

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Neither,
        line_advance: LineAdvance::Some(offset),
    }
}

pub fn parse_sphinx_literalinclude() {
    todo!()
}

pub fn parse_sphinx_glossary() {
    todo!()
}

pub fn parse_sphinx_sectionauthor() {
    todo!()
}

pub fn parse_sphinx_codeauthor() {
    todo!()
}

pub fn parse_sphinx_index() {
    todo!()
}

pub fn parse_sphinx_only(
    src_lines: &[String],
    mut doctree: DocTree,
    line_cursor: &mut LineCursor,
    empty_after_marker: bool,
    first_indent: usize,
    body_indent: usize,
) -> TransitionResult {


    /// Directive `only` tags that are always known to the Sphinx parser.
    /// These work like expressions in predicate logic and can be combined with
    /// ` and `, ` or ` and grouped with parentheses.

    /// They should be included with the directive argument.
    const ALWAYS_DEFINED_TAGS: &[&str] = &["html", "latex", "text"];

    let expression = match scan_directive_arguments(src_lines, line_cursor, body_indent, Some(first_indent), empty_after_marker) {
        Some(lines) => lines.join(" "),
        None => String::new()
    };

    if expression.is_empty() {
        return TransitionResult::Failure {
            message: format!(
                r#"The expression of an "only" Sphinx directive on line {} should not be empty. Computer says no..."#,
                line_cursor.sum_total()
            ),
            doctree: doctree,
        };
    }

    let only_node = TreeNodeType::SphinxOnly {
        expression: expression,
        body_indent: body_indent,
    };

    doctree = match doctree.push_data_and_focus(only_node) {
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

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Push(vec![State::Body]),
        line_advance: LineAdvance::Some(1),
    }
}

pub fn parse_sphinx_tabularcolumns() {
    todo!()
}

pub fn parse_sphinx_math_block() {
    todo!()
}

pub fn parse_sphinx_productionlist() {
    todo!()
}

// ========================
//  A+-specific directives
// ========================

pub fn parse_aplus_questionnaire(
    src_lines: &[String],
    mut doctree: DocTree,
    line_cursor: &mut LineCursor,
    base_indent: usize,
    empty_after_marker: bool,
    first_indent: usize,
    body_indent: usize,
) -> TransitionResult {
    let (key, difficulty, max_points): (String, String, String) = match scan_directive_arguments(
            src_lines,
            line_cursor,
            body_indent,
            Some(first_indent),
            empty_after_marker,
    ) {
        Some(lines) => aplus_key_difficulty_and_max_points(lines.join(" ").as_str(), line_cursor),
        None => return TransitionResult::Failure {
            message: format!(
                "A+ questionnaire on line {} was not given arguments. Computer says no...",
                line_cursor.sum_total()
            ),
            doctree: doctree,
        }
    };

    let (
        submissions,
        points_to_pass,
        feedback,
        title,
        no_override,
        pick_randomly,
        preserve_questions_between_attempts,
        category,
        status,
        reveal_model_at_max_submissions,
        show_model,
        allow_assistant_viewing,
        allow_assistant_grading,
    ) = if let Some(mut options) = scan_directive_options(src_lines, line_cursor, body_indent) {
        (
            options.remove("submissions"),
            options.remove("points-to-pass"),
            options.remove("feedback"),
            options.remove("title"),
            options.remove("no_override"),
            options.remove("pick_randomly"),
            options.remove("preserve-questions-between-attempts"),
            options.remove("category"),
            options.remove("status"),
            options.remove("reveal-model-at-max-submissions"),
            options.remove("show-model"),
            options.remove("allow-assistant-viewing"),
            options.remove("allow-assistant-grading"),
        )
    } else {
        (
            None, None, None, None, None, None, None, None, None, None, None, None, None,
        )
    };

    use crate::common::QuizPoints;

    let questionnaire_node = TreeNodeType::AplusQuestionnaire {
        body_indent: body_indent,
        key: key,
        difficulty: if difficulty.is_empty() {
            None
        } else {
            Some(difficulty)
        },
        max_points: if let Ok(result) = max_points.parse::<QuizPoints>() {
            Some(result)
        } else {
            None
        },
        points_from_children: 0,
        submissions: submissions,
        points_to_pass: points_to_pass,
        feedback: feedback,
        title: title,
        no_override: no_override,
        pick_randomly: pick_randomly,
        preserve_questions_between_attempts: preserve_questions_between_attempts,
        category: category,
        status: status,
        reveal_model_at_max_submissions: reveal_model_at_max_submissions,
        show_model: show_model,
        allow_assistant_viewing: allow_assistant_viewing,
        allow_assistant_grading: allow_assistant_grading,
    };

    doctree = match doctree.push_data_and_focus(questionnaire_node) {
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

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Push(vec![State::AplusQuestionnaire]),
        line_advance: LineAdvance::None,
    }
}

/// A `pick-one` type questionnaire question parser.
pub fn parse_aplus_pick_one(
    src_lines: &[String],
    mut doctree: DocTree,
    line_cursor: &mut LineCursor,
    first_indent: usize,
    body_indent: usize,
    empty_after_marker: bool,
) -> TransitionResult {


    /// Correct answers in `pick-one` and `pick-any` directives are marked with `*`.
    /// A `pick-any` question may have neutral options, which are marked with `?`.
    /// Neutral options are always counted as correct, whether the student selected them or not.
    /// Initially selected options may be set with `+`.
    /// The initially selected options are pre-selected when the exercise is loaded.
    /// The `+` character is written before `*` or `?` if they are combined.
    const APLUS_PICK_ONE_CHOICE_PATTERN: &'static str =
        r"^(\s*)(?P<pre_selected>\+)?(?P<correct>\*)?(?P<label>\S+)\.[ ]+(?P<answer>.+)";
    const APLUS_PICK_HINT_PATTERN: &'static str =
        r"^(\s*)(?P<show_not_answered>!)?(?P<label>\S+)[ ]*§[ ]*(?P<hint>.+)";

    use regex::{Captures, Regex};

    lazy_static::lazy_static! {
        static ref CHOICE_RE: Regex = Regex::new(APLUS_PICK_ONE_CHOICE_PATTERN).unwrap();
        static ref HINT_RE: Regex = Regex::new(APLUS_PICK_HINT_PATTERN).unwrap();
    }

    // Parsing the directive arguments

    use crate::common::QuizPoints;

    let points: QuizPoints = match scan_directive_arguments(
        src_lines,
        line_cursor,
        body_indent,
        Some(first_indent),
        empty_after_marker,
    ) {
        Some(lines) => match lines.join(" ").as_str().parse() {
            Ok(points) => points,
            Err(e) => return TransitionResult::Failure {
                message: format!("Quiz question points preceding line {} could not be parsed into an integer. Computer says no...", line_cursor.sum_total()),
                doctree: doctree
            }
        }
        None => return TransitionResult::Failure {
            message: format!(
                "No points provided for pick-one question on line {}. Computer says no...",
                line_cursor.sum_total()
            ),
            doctree: doctree,
        }
    };

    if let TreeNodeType::AplusQuestionnaire {
        points_from_children,
        ..
    } = doctree.mut_node_data() {
        *points_from_children += points;
    }

    // Parsing directive options

    let (class, required, key, dropdown) = if let Some(mut options) =
        scan_directive_options
        (src_lines, line_cursor, body_indent)
    {
        (
            options.remove("class"),
            options.remove("required"),
            options.remove("key"),
            options.remove("dropdown"),
        )
    } else {
        (None, None, None, None)
    };

    Parser::skip_empty_lines(src_lines, line_cursor);

    // Generating and focusing on node

    let pick_one_node = TreeNodeType::AplusPickOne {
        body_indent: body_indent,
        class: class,
        points: points,
        required: if required.is_some() { true } else { false },
        key: key,
        dropdown: if dropdown.is_some() { true } else { false },
    };

    doctree = match doctree.push_data_and_focus(pick_one_node) {
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

    // Check for assignment

    Parser::skip_empty_lines(src_lines, line_cursor);

    let start_line = match src_lines.get(line_cursor.relative_offset()) {
        Some(line) => line,
        None => return TransitionResult::Failure {
            message: format!(
                "Input overflow on line {} when parsing pick-one assignment. Computer says no...",
                line_cursor.sum_total()
            ),
            doctree: doctree
        }
    };

    let assignment_inline_nodes: Vec<TreeNodeType> = if !CHOICE_RE.is_match(start_line) {
        let (block_lines, offset) = match Parser::read_text_block(src_lines, line_cursor.relative_offset(),  true, true, Some(body_indent),true) {
            TextBlockResult::Ok {lines, offset } => (lines, offset),
            TextBlockResult::Err { lines, offset } => return TransitionResult::Failure {
                message: format!("Could not read pick-one assignment lines starting on line {}. Computer says no...", line_cursor.sum_total()),
                doctree: doctree
            }
        };
        let inline_nodes = match Parser::inline_parse(block_lines.join("\n"), None, line_cursor) {
            InlineParsingResult::Nodes(nodes) => nodes,
            _ => return TransitionResult::Failure {
                message: format!("Could not parse pick-one assignment for inline nodes on line {}. Computer says no...", line_cursor.sum_total()),
                doctree: doctree
            }
        };

        line_cursor.increment_by(1);

        inline_nodes
    } else {
        Vec::new()
    };

    // Add assignment node (paragraph) to tree

    Parser::skip_empty_lines(src_lines, line_cursor);

    if !assignment_inline_nodes.is_empty() {
        let assignment_node = TreeNodeType::Paragraph {
            indent: body_indent,
        };
        doctree = match doctree.push_data_and_focus(assignment_node) {
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
        for node in assignment_inline_nodes {
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
        doctree = doctree.focus_on_parent()
    }

    Parser::skip_empty_lines(src_lines, line_cursor);

    // Read question choices

    doctree = match doctree.push_data_and_focus(TreeNodeType::AplusPickChoices {
        body_indent: body_indent,
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

    while let Some(current_line) = src_lines.get(line_cursor.relative_offset()) {
        let indent = current_line
            .chars()
            .take_while(|c| c.is_whitespace())
            .count();

        if indent != body_indent {
            break;
        }

        let captures: Captures = if let Some(capts) = CHOICE_RE.captures(current_line) {
            capts
        } else {
            break;
        };

        let label = captures.name("label").unwrap().as_str().to_string();
        let pre_selected = captures.name("pre_selected");
        let correct = captures.name("correct");
        let answer = if let Some(capture) = captures.name("answer") {
            capture.as_str()
        } else {
            ""
        };

        if answer.trim().is_empty() {
            return TransitionResult::Failure {
                message: format!("Discovered a pick-one answer without content on line {}. Computer says no...", line_cursor.sum_total()),
                doctree: doctree
            };
        }

        let answer_nodes: Vec<TreeNodeType> = match Parser::inline_parse(answer.to_string(), None, line_cursor) {
            InlineParsingResult::Nodes(nodes) => nodes,
            _ => return TransitionResult::Failure {
                message: format!("Could not parse pick-one answer on line {} for inline nodes. Computer says no...", line_cursor.sum_total()),
                doctree: doctree
            }
        };

        let choice_node = TreeNodeType::AplusPickChoice {
            label: label,
            is_pre_selected: pre_selected.is_some(),
            is_correct: correct.is_some(),
            is_neutral: false, // pick-one nodes don't have this set
        };

        doctree = match doctree.push_data_and_focus(choice_node) {
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
        for node in answer_nodes {
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

        line_cursor.increment_by(1);
    }

    if doctree.n_of_children() == 0 {
        return TransitionResult::Failure {
            message: format!(
                "Found no choices for pick-one question on line {}. Computer says no...",
                line_cursor.sum_total()
            ),
            doctree: doctree,
        };
    }

    doctree = doctree.focus_on_parent();

    // Read possible hints inside the answers environment

    Parser::skip_empty_lines(src_lines, line_cursor);

    doctree = match doctree.push_data_and_focus(TreeNodeType::AplusQuestionnaireHints {
        body_indent: body_indent,
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

    while let Some(current_line) = src_lines.get(line_cursor.relative_offset()) {
        let indent = if !current_line.is_empty() {
            current_line
                .chars()
                .take_while(|c| c.is_whitespace())
                .count()
        } else {
            body_indent
        };

        if indent != body_indent {
            break;
        }

        let captures = if let Some(capts) = HINT_RE.captures(current_line) {
            capts
        } else {
            break;
        };

        let show_not_answered = captures.name("show_not_answered");
        let label = match captures.name("label") {
            Some(label) => label.as_str().to_string(),
            None => {
                return TransitionResult::Failure {
                    message: format!(
                        "No enumerator for pick-one hint on line {}. Computer says no...",
                        line_cursor.sum_total()
                    ),
                    doctree: doctree,
                }
            }
        };
        let hint: &str = if let Some(hint) = captures.name("hint") {
            hint.as_str().trim()
        } else {
            return TransitionResult::Failure {
                message: format!(
                    "No hint text for pick-one hint on line {}. Computer says no...",
                    line_cursor.sum_total()
                ),
                doctree: doctree,
            };
        };

        if hint.is_empty() {
            return TransitionResult::Failure {
                message: format!(
                    "Empty  hint text for hint on line {}. Computer says no...",
                    line_cursor.sum_total()
                ),
                doctree: doctree,
            };
        }

        let hint_nodes: Vec<TreeNodeType> = match Parser::inline_parse(hint.to_string(), None, line_cursor) {
            InlineParsingResult::Nodes(nodes) => nodes,
            _ => return TransitionResult::Failure {
                message: format!("Could not parse pick-one answer on line {} for inline nodes. Computer says no...", line_cursor.sum_total()),
                doctree: doctree
            }
        };

        if hint_nodes.is_empty() {
            return TransitionResult::Failure {
                message: format!(
                    "No inline nodes found for pick-one hint on line {}. Computer says no...",
                    line_cursor.sum_total()
                ),
                doctree: doctree,
            };
        }

        let hint_node = TreeNodeType::AplusQuestionnaireHint {
            label: label,
            show_when_not_selected: show_not_answered.is_some(),
            question_type: crate::common::AplusQuestionnaireType::PickOne,
        };

        doctree = match doctree.push_data_and_focus(hint_node) {
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
        for node in hint_nodes {
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

        line_cursor.increment_by(1);
    }

    Parser::skip_empty_lines(src_lines, line_cursor);

    doctree = doctree.focus_on_parent(); // Focus on pick-one
    doctree = doctree.focus_on_parent(); // Focus on questionnaire

    // Return with modified doctree

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Neither,
        line_advance: LineAdvance::None,
    }
}

/// A `pick-any` type questionnaire question parser.
pub fn parse_aplus_pick_any(
    src_lines: &[String],
    mut doctree: DocTree,
    line_cursor: &mut LineCursor,
    first_indent: usize,
    body_indent: usize,
    empty_after_marker: bool,
) -> TransitionResult {

    const APLUS_PICK_ANY_CHOICE_PATTERN: &'static str = r"^(\s*)(?P<pre_selected>\+)?(?:(?P<neutral>\?)|(?P<correct>\*))?(?P<label>\S+)\.[ ]+(?P<answer>.+)";
    const APLUS_PICK_HINT_PATTERN: &'static str =
        r"^(\s*)(?P<show_not_answered>!)?(?P<label>\S+)[ ]*§[ ]*(?P<hint>.+)";

    lazy_static::lazy_static! {
        static ref CHOICE_RE: regex::Regex = regex::Regex::new(APLUS_PICK_ANY_CHOICE_PATTERN).unwrap();
        static ref HINT_RE: regex::Regex = regex::Regex::new(APLUS_PICK_HINT_PATTERN).unwrap();
    }

    let points: crate::common::QuizPoints = match scan_directive_arguments(
        src_lines,
        line_cursor,
        body_indent,
        Some(first_indent),
        empty_after_marker,
    ) {
        Some(lines) => match lines.join(" ").as_str().parse() {
            Ok(points) => points,
            Err(e) => return TransitionResult::Failure {
                message: format!(
                    "Quiz question points preceding line {} could not be parsed into an integer. Computer says no...",
                    line_cursor.sum_total()
                ),
                doctree: doctree
            }
        },
        None => return TransitionResult::Failure {
            message: format!(
                "No points provided for pick-any question on line {}. Computer says no...",
                line_cursor.sum_total()
            ),
            doctree: doctree,
        }
    };

    if let TreeNodeType::AplusQuestionnaire {
        points_from_children,
        ..
    } = doctree.mut_node_data() {
        *points_from_children += points;
    }

    let (
        class,
        required,
        key,
        partial_points,
        randomized,
        correct_count,
        preserve_questions_between_attempts,
    ) = if let Some(mut options) = scan_directive_options
    (src_lines, line_cursor, body_indent) {
        (
            options.remove("class"),
            options.remove("required"),
            options.remove("key"),
            options.remove("partial-points"),
            options.remove("randomized"),
            options.remove("correct-count"),
            options.remove("preserve-questions-between-attempts"),
        )
    } else {
        (None, None, None, None, None, None, None)
    };

    let pick_any_node = TreeNodeType::AplusPickAny {
        body_indent: body_indent,
        points: points,
        class: class,
        required: if required.is_some() { true } else { false },
        key: key,
        partial_points: if partial_points.is_some() {
            true
        } else {
            false
        },
        randomized: if randomized.is_some() && correct_count.is_some() {
            true
        } else {
            false
        },
        correct_count: if randomized.is_some() && correct_count.is_some() {
            if let Ok(result) = correct_count.unwrap().parse() {
                Some(result)
            } else {
                return TransitionResult::Failure {
                    message: format!("No correct count provided for pick-any on line {} with randomization activated. Computer says no...", line_cursor.sum_total()),
                    doctree: doctree
                };
            }
        } else {
            None
        },
        preserve_questions_between_attempts: if preserve_questions_between_attempts.is_some() {
            true
        } else {
            false
        },
    };

    doctree = match doctree.push_data_and_focus(pick_any_node) {
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

    // Check for assignment

    Parser::skip_empty_lines(src_lines, line_cursor);

    let start_line = match src_lines.get(line_cursor.relative_offset()) {
        Some(line) => line,
        None => return TransitionResult::Failure {
            message: format!(
                "Input overflow on line {} when parsing pick-any assignment. Computer says no...",
                line_cursor.sum_total()
            ),
            doctree: doctree
        }
    };

    let assignment_inline_nodes: Vec<TreeNodeType> = if !CHOICE_RE.is_match(start_line) {
        let (block_lines, offset) = match Parser::read_text_block(src_lines, line_cursor.relative_offset(),  true, true, Some(body_indent), true) {
            TextBlockResult::Ok {lines, offset } => (lines, offset),
            TextBlockResult::Err {lines, offset } => return TransitionResult::Failure {
                message: format!("Could not read pick-any assignment lines starting on line {}. Computer says no...", line_cursor.sum_total()),
                doctree: doctree
            }
        };

        let inline_nodes = match Parser::inline_parse(block_lines.join("\n"), None, line_cursor) {
            InlineParsingResult::Nodes(nodes) => nodes,
            _ => return TransitionResult::Failure {
                message: format!("Could not parse pick-any assignment for inline nodes on line {}. Computer says no...", line_cursor.sum_total()),
                doctree: doctree
            }
        };

        line_cursor.increment_by(1);

        inline_nodes
    } else {
        Vec::new()
    };

    // Add assignment node (paragraph) to tree

    if !assignment_inline_nodes.is_empty() {
        let assignment_node = TreeNodeType::Paragraph {
            indent: body_indent,
        };
        doctree = match doctree.push_data_and_focus(assignment_node) {
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
        for node in assignment_inline_nodes {
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
        doctree = doctree.focus_on_parent()
    }

    Parser::skip_empty_lines(src_lines, line_cursor);

    // Read question choices

    doctree = match doctree.push_data_and_focus(TreeNodeType::AplusPickChoices {
        body_indent: body_indent,
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

    while let Some(current_line) = src_lines.get(line_cursor.relative_offset()) {
        let indent = current_line
            .chars()
            .take_while(|c| c.is_whitespace())
            .count();

        if indent != body_indent {
            break;
        }

        let captures: regex::Captures = if let Some(capts) = CHOICE_RE.captures(current_line) {
            capts
        } else {
            break;
        };

        let pre_selected = captures.name("pre_selected");
        let correct = captures.name("correct");
        let neutral = captures.name("neutral");

        let label = captures.name("label").unwrap().as_str().to_string();
        let answer = if let Some(capture) = captures.name("answer") {
            capture.as_str()
        } else {
            ""
        };

        if answer.trim().is_empty() {
            return TransitionResult::Failure {
                message: format!("Discovered a pick-any answer without content on line {}. Computer says no...", line_cursor.sum_total()),
                doctree: doctree
            };
        }

        let answer_nodes: Vec<TreeNodeType> = match Parser::inline_parse(answer.to_string(), None, line_cursor) {
            InlineParsingResult::Nodes(nodes) => nodes,
            _ => return TransitionResult::Failure {
                message: format!("Could not parse pick-any answer on line {} for inline nodes. Computer says no...", line_cursor.sum_total()),
                doctree: doctree
            }
        };

        let choice_node = TreeNodeType::AplusPickChoice {
            label: label,
            is_pre_selected: pre_selected.is_some(),
            is_correct: correct.is_some(),
            is_neutral: neutral.is_some(),
        };

        doctree = match doctree.push_data_and_focus(choice_node) {
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
        for node in answer_nodes {
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

        line_cursor.increment_by(1);
    }

    if doctree.n_of_children() == 0 {
        return TransitionResult::Failure {
            message: format!(
                "Found no choices for pick-any question on line {}. Computer says no...",
                line_cursor.sum_total()
            ),
            doctree: doctree,
        };
    }

    doctree = doctree.focus_on_parent();

    // Read possible hints inside the answers environment

    Parser::skip_empty_lines(src_lines, line_cursor);

    doctree = match doctree.push_data_and_focus(TreeNodeType::AplusQuestionnaireHints {
        body_indent: body_indent,
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

    while let Some(current_line) = src_lines.get(line_cursor.relative_offset()) {
        let indent = current_line
            .chars()
            .take_while(|c| c.is_whitespace())
            .count();

        if indent != body_indent {
            break;
        }

        let captures = if let Some(capts) = HINT_RE.captures(current_line) {
            capts
        } else {
            break;
        };

        let show_not_answered = captures.name("show_not_answered");
        let label = match captures.name("label") {
            Some(enumerator) => enumerator.as_str().to_string(),
            None => {
                return TransitionResult::Failure {
                    message: format!(
                        "No label for pick-any hint on line {}. Computer says no...",
                        line_cursor.sum_total()
                    ),
                    doctree: doctree,
                }
            }
        };
        let hint: &str = if let Some(hint) = captures.name("hint") {
            hint.as_str().trim()
        } else {
            return TransitionResult::Failure {
                message: format!(
                    "No hint text for pick-any hint on line {}. Computer says no...",
                    line_cursor.sum_total()
                ),
                doctree: doctree,
            };
        };

        if hint.is_empty() {
            return TransitionResult::Failure {
                message: format!(
                    "Empty hint text for hint on line {}. Computer says no...",
                    line_cursor.sum_total()
                ),
                doctree: doctree,
            };
        }

        let hint_nodes: Vec<TreeNodeType> = match Parser::inline_parse(hint.to_string(), None, line_cursor) {
    InlineParsingResult::Nodes(nodes) => nodes,
    _ => return TransitionResult::Failure {
        message: format!("Could not parse pick-any answer on line {} for inline nodes. Computer says no...", line_cursor.sum_total()),
        doctree: doctree
    }
    };

        if hint_nodes.is_empty() {
            return TransitionResult::Failure {
                message: format!(
                    "No inline nodes found for pick-any hint on line {}. Computer says no...",
                    line_cursor.sum_total()
                ),
                doctree: doctree,
            };
        }

        let hint_node = TreeNodeType::AplusQuestionnaireHint {
            label: label,
            show_when_not_selected: show_not_answered.is_some(),
            question_type: crate::common::AplusQuestionnaireType::PickAny,
        };

        doctree = match doctree.push_data_and_focus(hint_node) {
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
        for node in hint_nodes {
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

        line_cursor.increment_by(1);
    }

    Parser::skip_empty_lines(src_lines, line_cursor);

    doctree = doctree.focus_on_parent(); // Focus on pick-one
    doctree = doctree.focus_on_parent(); // Focus on questionnaire

    // Return with modified doctree

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Neither,
        line_advance: LineAdvance::None,
    }
}

/// A `freetext` type questionnaire question parser.
pub fn parse_aplus_freetext(
    src_lines: &[String],
    mut doctree: DocTree,
    line_cursor: &mut LineCursor,
    first_indent: usize,
    body_indent: usize,
    empty_after_marker: bool,
) -> TransitionResult {

    use crate::common::QuizPoints;

    let (points, method_string) = if let Some(arg) = scan_directive_arguments(
        src_lines,
        line_cursor,
        body_indent,
        Some(first_indent),
        empty_after_marker,
    ) {
        let arg_string = arg.join(" ");

        let mut arg_iter = arg_string.split_whitespace();
        let points_string: Option<&str> = arg_iter.next();
        let method_str = arg_iter.next();

        let points: QuizPoints = if let Some(string) = points_string {
            if let Ok(result) = string.parse() {
                result
            } else {
                return TransitionResult::Failure {
                    message: format!("Quiz freetext question points preceding line {} could not be parsed into an integer. Computer says no...", line_cursor.sum_total()),
                    doctree: doctree
                };
            }
        } else {
            return TransitionResult::Failure {
                message: format!(
                    "No points found for freetext on line {}. Computer says no...",
                    line_cursor.sum_total()
                ),
                doctree: doctree,
            };
        };
        let method_string = if let Some(string) = method_str {
            string.to_string()
        } else {
            String::new()
        };

        (points, method_string)
    } else {
        return TransitionResult::Failure {
            message: format!(
                "No points provided for freetext question on line {}. Computer says no...",
                line_cursor.sum_total()
            ),
            doctree: doctree,
        };
    };

    if let TreeNodeType::AplusQuestionnaire {
        points_from_children,
        ..
    } = doctree.mut_node_data()
    {
        *points_from_children += points;
    }

    let (class, required, key, length, height) = if let Some(mut options) = scan_directive_options
    (src_lines, line_cursor, body_indent) {
        (
            options.remove("class"),
            options.remove("required"),
            options.remove("key"),
            options.remove("length"),
            options.remove("height"),
        )
    } else {
        (None, None, None, None, None)
    };

    let freetext_node = TreeNodeType::AplusFreeText {
        body_indent: body_indent,
        points: points,
        compare_method: method_string,
        model_answer: String::new(),
        class: class,
        required: required,
        key: key,
        length: length,
        height: height,
    };

    doctree = match doctree.push_data_and_focus(freetext_node) {
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

    Parser::skip_empty_lines(src_lines, line_cursor);

    // Read in assignment

    let assignment_inline_nodes: Vec<TreeNodeType> = {
        let (block_lines, offset) = match Parser::read_text_block(src_lines, line_cursor.relative_offset(),  true, true, Some(body_indent), true) {
            TextBlockResult::Ok { lines, offset } => (lines, offset),
            TextBlockResult::Err { lines, offset } => return TransitionResult::Failure {
                message: format!("Could not read pick-any assignment lines starting on line {}. Computer says no...", line_cursor.sum_total()),
                doctree: doctree
            }
        };

        let inline_nodes = match Parser::inline_parse(block_lines.join("\n"), None, line_cursor) {
            InlineParsingResult::Nodes(nodes) => nodes,
            _ => return TransitionResult::Failure {
                message: format!("Could not parse pick-any assignment for inline nodes on line {}. Computer says no...", line_cursor.sum_total()),
                doctree: doctree
            }
        };

        line_cursor.increment_by(1);

        inline_nodes
    };

    // Add assignment node (paragraph) to tree

    let assignment_node = TreeNodeType::Paragraph {
        indent: body_indent,
    };
    doctree = match doctree.push_data_and_focus(assignment_node) {
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
    for node in assignment_inline_nodes {
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

    Parser::skip_empty_lines(src_lines, line_cursor);

    // Read in model answer

    if let Some(answer) = src_lines.get(line_cursor.relative_offset()) {
        let indent = answer.chars().take_while(|c| c.is_whitespace()).count();
        if indent != body_indent {
            return TransitionResult::Failure {
                message: format!("A+ freetext answer has incorrect indentation on line {}. Computer says no...", line_cursor.sum_total()),
                doctree: doctree
            };
        }

        if let TreeNodeType::AplusFreeText { model_answer, .. } = doctree.mut_node_data() {
            model_answer.push_str(answer.trim());
        } else {
            return TransitionResult::Failure {
                message: format!("Not focused on A+ freetext node when reading its model answer on line {}? Computer says no...", line_cursor.sum_total()),
                doctree: doctree
            };
        }

        line_cursor.increment_by(1);
    } else {
        return TransitionResult::Failure {
            message: format!("Tried scanning freetext question for correct answer but encountered end of input on line {}. Computer says no...", line_cursor.sum_total()),
            doctree: doctree
        };
    };

    // Read possible hints
    const APLUS_PICK_HINT_PATTERN: &'static str =
        r"^(\s*)(?P<show_not_answered>!)?(?P<label>.+)[ ]*§[ ]*(?P<hint>.+)";
    lazy_static::lazy_static! {
        static ref HINT_RE: regex::Regex = regex::Regex::new(APLUS_PICK_HINT_PATTERN).unwrap();
    }

    Parser::skip_empty_lines(src_lines, line_cursor);

    doctree = match doctree.push_data_and_focus(
        TreeNodeType::AplusQuestionnaireHints {
            body_indent: body_indent,
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

    while let Some(current_line) = src_lines.get(line_cursor.relative_offset()) {
        let indent = current_line
            .chars()
            .take_while(|c| c.is_whitespace())
            .count();

        if indent != body_indent {
            break;
        }

        let captures = if let Some(capts) = HINT_RE.captures(current_line) {
            capts
        } else {
            break;
        };

        let show_not_answered = captures.name("show_not_answered");
        let label = match captures.name("label") {
            Some(label) => label.as_str().trim().to_string(),
            None => {
                return TransitionResult::Failure {
                    message: format!(
                        "No text for freetext hint on line {}. Computer says no...",
                        line_cursor.sum_total()
                    ),
                    doctree: doctree,
                }
            }
        };
        let hint: &str = if let Some(hint) = captures.name("hint") {
            hint.as_str().trim()
        } else {
            return TransitionResult::Failure {
                message: format!(
                    "No hint text for freetext hint on line {}. Computer says no...",
                    line_cursor.sum_total()
                ),
                doctree: doctree,
            };
        };

        if hint.is_empty() {
            return TransitionResult::Failure {
                message: format!(
                    "Empty hint text for hint on line {}. Computer says no...",
                    line_cursor.sum_total()
                ),
                doctree: doctree,
            };
        }

        let hint_nodes: Vec<TreeNodeType> = match Parser::inline_parse(hint.to_string(), None, line_cursor) {
            InlineParsingResult::Nodes(nodes) => nodes,
            _ => return TransitionResult::Failure {
                message: format!("Could not parse freetext hint on line {} for inline nodes. Computer says no...", line_cursor.sum_total()),
                doctree: doctree
            }
        };

        if hint_nodes.is_empty() {
            return TransitionResult::Failure {
                message: format!(
                    "No inline nodes found for freetext hint on line {}. Computer says no...",
                    line_cursor.sum_total()
                ),
                doctree: doctree,
            };
        }

        let hint_node = TreeNodeType::AplusQuestionnaireHint {
            label: label,
            show_when_not_selected: show_not_answered.is_some(),
            question_type: crate::common::AplusQuestionnaireType::FreeText,
        };

        doctree = match doctree.push_data_and_focus(hint_node) {
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
        for node in hint_nodes {
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

        line_cursor.increment_by(1);
    }

    Parser::skip_empty_lines(src_lines, line_cursor);

    doctree = doctree.focus_on_parent(); // Focus on pick-one
    doctree = doctree.focus_on_parent(); // Focus on questionnaire

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Neither,
        line_advance: LineAdvance::None,
    }
}

pub fn parse_aplus_submit(
    src_lines: &[String],
    mut doctree: DocTree,
    line_cursor: &mut LineCursor,
    first_indent: usize,
    body_indent: usize,
    empty_after_marker: bool,
) -> TransitionResult {

    let (key, difficulty, max_points): (String, String, String) = if let Some(arg) =
        scan_directive_arguments(
            src_lines,
            line_cursor,
            body_indent,
            Some(first_indent),
            empty_after_marker,
        ) {
        aplus_key_difficulty_and_max_points(arg.join(" ").as_str(), line_cursor)
    } else {
        return TransitionResult::Failure {
            message: format!(
                "A+ submit exercise on line {} was not given arguments. Computer says no...",
                line_cursor.sum_total()
            ),
            doctree: doctree,
        };
    };

    Parser::skip_empty_lines(src_lines, line_cursor);

    let (
        config,
        submissions,
        points_to_pass,
        class,
        title,
        category,
        status,
        ajax,
        allow_assistant_viewing,
        allow_assistant_grading,
        quiz,
        url,
        radar_tokenizer,
        radar_minimum_match_tokens,
        lti,
        lti_resource_link_id,
        lti_open_in_iframe,
        lti_aplus_get_and_post,
    ) = if let Some(mut options) = scan_directive_options(
        src_lines, line_cursor, body_indent
    ) {
        (
            options.remove("config"),
            options.remove("submissions"),
            options.remove("points-to-pass"),
            options.remove("class"),
            options.remove("title"),
            options.remove("category"),
            options.remove("status"),
            options.remove("ajax"),
            options.remove("allow-assistant-viewing"),
            options.remove("allow-assistant-grading"),
            options.remove("quiz"),
            options.remove("url"),
            options.remove("radar-tokenizer"),
            options.remove("radar_minimum_match_tokens"),
            options.remove("lti"),
            options.remove("lti_resource_link_id"),
            options.remove("lti_open_in_iframe"),
            options.remove("lti_aplus_get_and_post"),
        )
    } else {
        (
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None, None, None,
        )
    };

    if config.is_none() {
        return TransitionResult::Failure {
    message: format!("A+ submit exercise on line {} has to specify a configuration file location via the :config: option. Computer says no...", line_cursor.sum_total()),
    doctree: doctree
    };
    }

    // Unpacking some options
    let max_points = if let Ok(result) = max_points.parse() {
        result
    } else {
        10
    };
    let points_to_pass = if let Some(ptp) = points_to_pass {
        if let Ok(result) = ptp.parse() {
            result
        } else {
            0
        }
    } else {
        0
    };

    use crate::common::AplusExerciseStatus;
    let status = if let Some(status) = status {
        match status.as_str().trim() {
            "ready" => AplusExerciseStatus::Ready,
            "unlisted" => AplusExerciseStatus::Unlisted,
            "hidden" => AplusExerciseStatus::Hidden,
            "enrollment" => AplusExerciseStatus::Enrollment,
            "enrollment_ext" => AplusExerciseStatus::EnrollmentExt,
            "maintenance" => AplusExerciseStatus::Maintenance,
            _ => AplusExerciseStatus::Unlisted,
        }
    } else {
        AplusExerciseStatus::Unlisted // Default
    };

    use crate::common::AplusRadarTokenizer;
    let tokenizer =
        if let Some(tokenizer) = radar_tokenizer {
            match tokenizer.as_str().trim() {
                "python" => AplusRadarTokenizer::Python3,
                "scala" => AplusRadarTokenizer::Scala,
                "javascript" => AplusRadarTokenizer::JavaScript,
                "css" => AplusRadarTokenizer::CSS,
                "html" => AplusRadarTokenizer::HTML,
                _ => return TransitionResult::Failure {
                    message: format!(
                        "No such tokenizer A+ submit exerciose on line {}. Computer says no...",
                        line_cursor.sum_total()
                    ),
                    doctree: doctree,
                },
            }
        } else {
            AplusRadarTokenizer::None // Default
        };

    let lti = if let Some(lti) = lti {
        lti
    } else {
        String::new()
    };

    // Crate submit node

    let submit_node = TreeNodeType::AplusSubmit {
        body_indent: body_indent,
        key: key,
        difficulty: difficulty,
        max_points: max_points,
        config: config.unwrap(),
        submissions: if let Some(submissions) = submissions {
            if let Ok(result) = submissions.parse() {
                result
            } else {
                10
            }
        } else {
            10
        },
        points_to_pass: points_to_pass,
        class: if let Some(class) = class {
            class
        } else {
            String::new()
        },
        title: if let Some(title) = title {
            title
        } else {
            String::new()
        },
        category: if let Some(category) = category {
            category
        } else {
            String::from("submit")
        },
        status: status,
        ajax: ajax.is_some(),
        allow_assistant_viewing: allow_assistant_viewing.is_some(),
        allow_assistant_grading: allow_assistant_grading.is_some(),
        quiz: quiz.is_some(),
        url: if let Some(url) = url {
            url
        } else {
            String::new()
        },
        radar_tokenizer: tokenizer, // implements Copy, so can be used below
        radar_minimum_match_tokens: if let Some(min) = radar_minimum_match_tokens {
            if let AplusRadarTokenizer::None = tokenizer {
                None
            } else {
                if let Ok(result) = min.parse() {
                    Some(result)
                } else {
                    None
                }
            }
        } else {
            None
        },
        lti: lti,
        lti_resource_link_id: if let Some(id) = lti_resource_link_id {
            id
        } else {
            String::new()
        },
        lti_open_in_iframe: lti_open_in_iframe.is_some(),
        lti_aplus_get_and_post: lti_aplus_get_and_post.is_some(),
    };

    doctree = match doctree.push_data_and_focus(submit_node) {
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

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Push(vec![State::Body]),
        line_advance: LineAdvance::None,
    }
}

pub fn parse_aplus_toctree() {
    todo!()
}

pub fn parse_aplus_active_element_input(
    src_lines: &[String],
    mut doctree: DocTree,
    line_cursor: &mut LineCursor,
    base_indent: usize,
    empty_after_marker: bool,
    first_indent: usize,
    body_indent: usize,
) -> TransitionResult {

    let key_for_input = if let Some(args) = scan_directive_arguments(
        src_lines,
        line_cursor,
        body_indent,
        Some(first_indent),
        empty_after_marker,
    ) {
        args.join(" ")
    } else {
        return TransitionResult::Failure {
    message: format!("A+ active element input before line {} has no key for output. Computer says no...", line_cursor.sum_total()),
    doctree: doctree
    };
    };

    let (title, default, class, width, height, clear, input_type, file) =
        if let Some(mut options) = scan_directive_options
        (src_lines, line_cursor, body_indent) {
            (
                options.remove("title"),
                options.remove("default"),
                options.remove("class"),
                options.remove("width"),
                options.remove("height"),
                options.remove("clear"),
                options.remove("type"),
                options.remove("file"),
            )
        } else {
            (None, None, None, None, None, None, None, None)
        };

    use crate::common::{AplusActiveElementClear, AplusActiveElementInputType};

    let ae_input_node = TreeNodeType::AplusActiveElementInput {
        key_for_input: key_for_input,
        title: title,
        default: default,
        class: class,
        width: if let Some(w) = &width {
            converters::str_to_length(w)
        } else {
            None
        },
        height: if let Some(h) = &height {
            converters::str_to_length(h)
        } else {
            None
        },
        clear: if let Some(clear) = clear {
            match clear.as_str() {
                "both" => Some(AplusActiveElementClear::Both),
                "left" => Some(AplusActiveElementClear::Left),
                "right" => Some(AplusActiveElementClear::Right),
                _ => return TransitionResult::Failure {
                    message: format!("No such clear type for A+ active element input before line {}. Computer says no...", line_cursor.sum_total()),
                    doctree: doctree
                }
            }
        } else {
            None
        },
        input_type: if let Some(input_type) = &input_type {
            if input_type == "file" {
                Some(AplusActiveElementInputType::File)
            } else if input_type == "clickable" {
                Some(AplusActiveElementInputType::Clickable)
            } else if input_type.starts_with("dropdown:") {
                let options = if let Some(options) = input_type.split(":").last() {
                    options
                } else {
                    return TransitionResult::Failure {
                        message: format!("No options for dropdown input for A+ activ element input before line {}. Computer says no...", line_cursor.sum_total()),
                        doctree: doctree
                    };
                };
                Some(crate::common::AplusActiveElementInputType::Dropdown(
                    options.to_string(),
                ))
            } else {
                return TransitionResult::Failure {
                    message: format!("No such input type for A+ active element input before line {}. Ignoring...", line_cursor.sum_total()),
                    doctree: doctree
                };
            }
        } else {
            None
        },
        file: if let (Some(input_type), Some(file)) = (input_type, file) {
            if input_type == "clickable" {
                Some(file)
            } else {
                None
            }
        } else {
            None
        },
    };

    doctree = match doctree.push_data(ae_input_node) {
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

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Neither,
        line_advance: LineAdvance::None,
    }
}

/// Parses an A+ active element output directive into the respective node
pub fn parse_aplus_active_element_output(
    src_lines: &[String],
    mut doctree: DocTree,
    line_cursor: &mut LineCursor,
    base_indent: usize,
    empty_after_marker: bool,
    first_indent: usize,
    body_indent: usize,
) -> TransitionResult {

    let key_for_output = if let Some(args) = scan_directive_arguments(
        src_lines,
        line_cursor,
        body_indent,
        Some(first_indent),
        empty_after_marker,
    ) {
        args.join(" ")
    } else {
        return TransitionResult::Failure {
            message: format!("A+ active element output before line {} has no key for output. Computer says no...", line_cursor.sum_total()),
            doctree: doctree
        };
    };

    let (
        config,
        inputs,
        title,
        class,
        width,
        height,
        clear,
        output_type,
        file,
        submissions,
        scale_size,
        status,
    ) = if let Some(mut options) = scan_directive_options(
        src_lines, line_cursor, body_indent
    ) {
        (
            options.remove("config"),
            options.remove("inputs"),
            options.remove("title"),
            options.remove("class"),
            options.remove("width"),
            options.remove("height"),
            options.remove("clear"),
            options.remove("type"),
            options.remove("file"),
            options.remove("submissions"),
            options.remove("scale-size"),
            options.remove("status")
        )
    } else {
        (
            None, None, None, None, None, None, None, None, None, None, None, None
        )
    };

    use crate::common::{
        AplusActiveElementClear, AplusActiveElementOutputType, AplusExerciseStatus,
    };

    let ae_output_node = TreeNodeType::AplusActiveElementOutput {
        key_for_output: key_for_output,
        config: if let Some(config) = config {
            config
        } else {
            return TransitionResult::Failure {
                message: format!("A+ active element output before line {} must have a set config file via the \"config\" option. Computer says no...", line_cursor.sum_total()),
                doctree: doctree
            };
        },
        inputs: if let Some(inputs) = inputs {
            inputs
        } else {
            return TransitionResult::Failure {
                message: format!("A+ active element output before line {} must have a set of inputs set via the \"inputs\" setting. Computer says no...", line_cursor.sum_total()),
                doctree: doctree
            };
        },
        title: title,
        class: class,
        width: if let Some(w) = &width {
            converters::str_to_length(w)
        } else {
            None
        },
        height: if let Some(h) = &height {
            converters::str_to_length(h)
        } else {
            None
        },
        clear: if let Some(clear) = clear {
            match clear.as_str() {
                "both"  => Some(AplusActiveElementClear::Both),
                "left"  => Some(AplusActiveElementClear::Left),
                "right" => Some(AplusActiveElementClear::Right),
                _       => None
            }
        } else {
            None
        },
        output_type: if let Some(output_type) = output_type {
            match output_type.as_str() {
                "text"  => AplusActiveElementOutputType::Text,
                "image" => AplusActiveElementOutputType::Image,
                _       => AplusActiveElementOutputType::Text
            }
        } else {
            AplusActiveElementOutputType::Text
        },
        submissions: if let Some(submissions) = submissions {
            if let Ok(result) = submissions.parse::<u32>() {
                Some(result)
            } else {
                None
            }
        } else {
            None
        },
        scale_size: if let Some(_) = scale_size {
            true
        } else {
            false
        },
        status: if let Some(status) = status {
            match status.as_str().trim() {
                "ready"     => AplusExerciseStatus::Ready,
                "unlisted"  => AplusExerciseStatus::Unlisted,
                "hidden"    => AplusExerciseStatus::Hidden,
                "enrollment"        => AplusExerciseStatus::Enrollment,
                "enrollment_ext"    => AplusExerciseStatus::EnrollmentExt,
                "maintenance"       => AplusExerciseStatus::Maintenance,
                _                   => AplusExerciseStatus::Unlisted
            }
        } else {
            AplusExerciseStatus::Unlisted
        },
    };

    doctree = match doctree.push_data(ae_output_node) {
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

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Neither,
        line_advance: LineAdvance::None,
    }
}

pub fn parse_aplus_hidden_block() {
    todo!()
}

/// Parses an A+ point of interest directive into the respective node.


/// Add support for the row and column directives introduced in November 2020.
pub fn parse_aplus_point_of_interest(
    src_lines: &[String],
    mut doctree: DocTree,
    line_cursor: &mut LineCursor,
    base_indent: usize,
    empty_after_marker: bool,
    first_indent: usize,
    body_indent: usize,
    section_level: usize,
) -> TransitionResult {

    let title = scan_directive_arguments(
        src_lines,
        line_cursor,
        body_indent,
        Some(first_indent),
        empty_after_marker,
    );

    // Read recognized options
    let (
        id,
        previous,
        next,
        hidden,
        class,
        height,
        columns,
        bgimg,
        not_in_slides,
        not_in_book,
        no_poi_box,
    ) = if let Some(mut options) = scan_directive_options(
        src_lines, line_cursor, body_indent
    ) {

        (
            options.remove("id"),
            options.remove("previous"),
            options.remove("next"),
            options.remove("hidden"),
            options.remove("class"),
            options.remove("height"),
            options.remove("columns"),
            options.remove("bgimg"),
            options.remove("not_in_slides"),
            options.remove("not_in_book"),
            options.remove("no_poi_box"),
        )
    } else {
        (
            None, None, None, None, None, None, None, None, None, None, None,
        )
    };

    let poi_node = TreeNodeType::AplusPOI {
        title: if let Some(title) = title {
            title.join(" ")
        } else {
            "".to_string()
        },
        body_indent: body_indent,

        id: id,
        previous: previous,
        next: next,
        hidden: hidden,
        class: class,
        height: if let Some(h) = &height {
            converters::str_to_length(h)
        } else {
            None
        },
        columns: columns,
        bgimg: bgimg,
        not_in_slides: not_in_slides,
        not_in_book: not_in_book,
        no_poi_box: no_poi_box,
    };

    doctree = match doctree.push_data_and_focus(poi_node) {
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

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Push(vec![State::AplusMultiCol]), // PoI contains body nodes and A+ specific column breaks
        line_advance: LineAdvance::None,
    }
}

pub fn parse_aplus_annotated() {
    todo!()
}

pub fn parse_aplus_lineref_codeblock() {
    todo!()
}

pub fn parse_aplus_repl_res_count_reset() {
    todo!()
}

pub fn parse_aplus_acos_submit() {
    todo!()
}

pub fn parse_aplus_div() {
    todo!()
}

pub fn parse_aplus_styled_topic() {
    todo!()
}

pub fn parse_aplus_story() {
    todo!()
}

pub fn parse_aplus_jsvee() {
    todo!()
}

pub fn parse_aplus_youtube() {
    todo!()
}

pub fn parse_aplus_local_video() {
    todo!()
}

pub fn parse_aplus_embedded_page() {
    todo!()
}

/// Parses unknown directive blocks as literal text.
pub fn parse_unknown_directive(
    mut doctree: DocTree,
    src_lines: &[String],
    line_cursor: &mut LineCursor,
    directive_name: &str,
    first_line_indent: usize,
    body_indent: usize,
    empty_after_marker: bool,
) -> TransitionResult {
    let argument = if let Some(arg) = scan_directive_arguments(
        src_lines,
        line_cursor,
        body_indent,
        Some(first_line_indent),
        empty_after_marker,
    ) {
        arg.join(" ").trim().to_string()
    } else {
        String::new()
    };

    let options = if let Some(options) =
        scan_directive_options
        (src_lines, line_cursor, body_indent)
    {
        options
    } else {
        HashMap::new()
    };

    let unknown_directive_data = TreeNodeType::UnknownDirective {
        directive_name: String::from(directive_name),
        argument: argument,
        options: options,
        body_indent: body_indent,
    };

    doctree = match doctree.push_data_and_focus(unknown_directive_data) {
        Ok(tree) => tree,
        Err(tree) => {
            return TransitionResult::Failure {
                message: format!(
                    "Could not add unknown directive data to doctree on line {}",
                    line_cursor.sum_total()
                ),
                doctree: tree,
            }
        }
    };

    TransitionResult::Success {
        doctree: doctree,
        push_or_pop: PushOrPop::Push(vec![State::Body]),
        line_advance: LineAdvance::None,
    }

    // let (unknown_directive_as_text, offset) = match Parser::read_indented_block(src_lines, Some(line_cursor.relative_offset()), Some(false), Some(false), Some(body_indent), Some(first_line_indent), false) {
    //   Ok((lines, _, offset, _)) => {
    //     (lines.join("\n"), offset)
    //   },
    //   Err(message) => return TransitionResult::Failure {
    //     message: format!("Error when reading an unknown directive as literal text: {}", message),
    //     doctree: doctree
    //   }
    // };

    // let literal_node = TreeNodeType::LiteralBlock { text: unknown_directive_as_text };

    // doctree = match doctree.push_data(literal_node) {
    //   Ok(tree) => tree,
    //   Err(tree) => return TransitionResult::Failure {
    //     message: format!("Node insertion error on line {}. Computer says no...", line_cursor.sum_total()),
    //     doctree: tree
    //   }
    // };

    // TransitionResult::Success {
    //   doctree: doctree,
    //   next_states: None,
    //   push_or_pop: PushOrPop::Neither,
    //   line_advance: LineAdvance::Some(offset)
    // }
}

// ---------
//  HELPERS
// ---------

/// Reads the first block of text of a directive,
/// until an empty line or something like a list of options
/// (recognized by the automaton `FIELD_MAKRER_RE`) is encountered.
/// Combines the lines into a single string and `Option`ally returns it.
/// If no arguments are found, returns `None`.

/// In case the directive starts on the same line as the directive marker,
/// allows specifying first and block indents separately.
/// `first_indent` (on the first line) or `block_indent` are ignored on each line.
fn scan_directive_arguments(
    src_lines: &[String],
    line_cursor: &mut LineCursor,
    body_indent: usize,
    first_indent: Option<usize>,
    empty_after_marker: bool,
) -> Option<Vec<String>> {
    use crate::parser::automata::FIELD_MARKER_AUTOMATON;

    // The vector containing references to the argument lines.
    let mut argument_lines: Vec<String> = Vec::new();
    let mut on_marker_line = true;

    // Jump to next line if line after directive marker is empty
    if empty_after_marker {
        line_cursor.increment_by(1);
        on_marker_line = false;
    }

    while let Some(line) = src_lines.get(line_cursor.relative_offset()) {


        // Each collect allocates, but what the heck, it works.
        let line_without_indent: String = if on_marker_line {
            match first_indent {
                Some(indent) => {
                    on_marker_line = false;
                    line.chars().skip(indent).collect()
                }
                None => panic!("On directive marker line {} but couldn't skip the marker to parse line contents. Computer says no...", line_cursor.sum_total())
            }
        } else {
            // Check for proper indentation
            if ! (line.chars().take_while(|c| c.is_whitespace()).count() == body_indent) {
                break
            };
            line.chars()
                .skip_while(|c| c.is_whitespace())
                .collect::<String>()
                .as_str()
                .trim()
                .to_string()
        };

        if line_without_indent.as_str().trim().is_empty()
            || FIELD_MARKER_AUTOMATON.is_match(line_without_indent.as_str())
        {
            break;
        }

        argument_lines.push(line_without_indent);
        line_cursor.increment_by(1);
    }

    if argument_lines.is_empty() {
        None
    } else {
        Some(argument_lines)
    }
}


/// Scans the lines following the directive marker for something resembling a field list,
/// and attempts to scan the contents of the list into an `Option`al `HashMap` of directive
/// option names and values. The calling directive parser will handle their validation,
/// as different directives have different options available to them.

/// An empty line separates directive options from the directive content, so encountering one
/// will terminate the scan. This means that the options have to start of the line following
/// the directive marker.
fn scan_directive_options(
    src_lines: &[String],
    line_cursor: &mut LineCursor,
    body_indent: usize,
) -> Option<HashMap<String, String>> {
    use crate::parser::automata::FIELD_MARKER_AUTOMATON;

    let mut option_map: HashMap<String, String> = HashMap::new();

    let mut ended_with_blank: bool = false;

    while let Some(line) = src_lines.get(line_cursor.relative_offset()) {
        if line.trim().is_empty() {
            ended_with_blank = true;
            break;
        } // End of option list

        if let Some(captures) = FIELD_MARKER_AUTOMATON.captures(line) {
            let line_indent = captures.get(1).unwrap().as_str().chars().count();
            if line_indent != body_indent {
                break;
            } // Option lists need to be aligned
            let option_key = captures.get(2).unwrap().as_str().trim();

            let option_val_indent = captures.get(0).unwrap().as_str().chars().count();
            let option_val = match line.char_indices().nth(option_val_indent) {
                Some((index, _)) => line[index..].trim(),
                None => "",
            };

            if let Some(val) = option_map.insert(option_key.to_string(), option_val.to_string())
            {
                // eprintln!("Duplicate directive option on line {}\n", line_cursor.sum_total())
            }
        } else {
            ended_with_blank = false;
            break; // Found a line not conforming to field list item syntax
        }
        line_cursor.increment_by(1);
    }

    if option_map.is_empty() {
        None
    } else {
        if ended_with_blank {
            line_cursor.increment_by(1)
        }
        Some(option_map)
    }
}

/// Parses the given `&str` for the directive key,
/// difficulty and maximum points.
/// Empty strings are returned for every missing part.
fn aplus_key_difficulty_and_max_points(
    arg_str: &str,
    line_cursor: &mut LineCursor,
) -> (String, String, String) {
    use regex::Regex;

    lazy_static::lazy_static! {
        static ref EXERCISE_ARGS_RE: Regex = Regex::new(r"^(?P<key>[a-zA-Z0-9]+)?[ ]*(?P<difficulty>[A-Z])?(?P<max_points>[0-9]+)?").unwrap();
    }

    if let Some(captures) = EXERCISE_ARGS_RE.captures(arg_str) {
        let key = if let Some(key) = captures.name("key") {
            String::from(key.as_str())
        } else {
            String::new()
        };
        let difficulty = if let Some(difficulty) = captures.name("difficulty") {
            String::from(difficulty.as_str())
        } else {
            String::new()
        };
        let max_points = if let Some(points) = captures.name("max_points") {
            String::from(points.as_str())
        } else {
            String::new()
        };

        (key, difficulty, max_points)
    } else {
        // No allocations for strings with zero size
        eprintln!("Either no arguments or invalid argument format for questionnaire preceding line {}...", line_cursor.sum_total());
        (String::new(), String::new(), String::new())
    }
}
