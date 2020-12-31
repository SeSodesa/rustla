/*!
A submodule that contains the functions related to parsing literal blocks of text.

Copyright © 2020 Santtu Söderholm
*/

use crate::parser::regex_patterns::Pattern;
use crate::doctree::tree_node_types::TreeNodeType;
use crate::doctree::DocTree;
use crate::parser::line_cursor::LineCursor;
use crate::parser::types_and_aliases::TransitionResult;
use crate::parser::types_and_aliases::{LineAdvance, PushOrPop};
use crate::parser::Parser;

/// A function for parsing indented literal block nodes.
pub fn literal_block(
    src_lines: &Vec<String>,
    base_indent: usize,
    section_level: &mut usize,
    line_cursor: &mut LineCursor,
    doctree: Option<DocTree>,
    captures: &regex::Captures,
    pattern_name: &Pattern,
) -> TransitionResult {
    let doctree = doctree.unwrap();

    let detected_indent = captures.get(1).unwrap().as_str().chars().count() + base_indent;

    let body_indent = if let Some(indent) = doctree.shared_data().body_indent() {
        indent
    } else {
        return TransitionResult::Failure {
            message: format!(
                "Literal block inside node that has no body indent on line {}. Computer says no...",
                line_cursor.sum_total()
            ),
            doctree: doctree,
        };
    };

    match pattern_name {

    Pattern::IndentedLiteralBlock if detected_indent > body_indent => parse_indented_literal(doctree, src_lines, line_cursor, captures, body_indent, detected_indent),
    Pattern::QuotedLiteralBlock if detected_indent == body_indent => parse_quoted_literal(doctree, src_lines, line_cursor, captures, body_indent, detected_indent),
    Pattern::QuotedLiteralBlock if detected_indent > body_indent => parse_indented_literal(doctree, src_lines, line_cursor, captures, body_indent, detected_indent),
    _ => return TransitionResult::Failure {
        message: format!("Non-literal pattern {:#?} after paragraph or wrong literal block indent ({} vs {}) on line {}. Computer says no...", pattern_name, detected_indent, body_indent, line_cursor.sum_total()),
        doctree: doctree
    }
  }
}

/// Generates a literal block node from a "quoted" block of text.
fn parse_indented_literal(
    mut doctree: DocTree,
    src_lines: &Vec<String>,
    line_cursor: &mut LineCursor,
    captures: &regex::Captures,
    body_indent: usize,
    detected_indent: usize,
) -> TransitionResult {
    // Read in a block with minimal indentation as-is with Parser::read_indented_block
    // and feed it to a LiteralBlock node.

    let (literal_string, offset): (String, usize) = if let Ok((lines, _, offset, _)) =
        Parser::read_indented_block(
            src_lines,
            Some(line_cursor.relative_offset()),
            Some(false),
            Some(true),
            Some(detected_indent),
            None,
            false,
        ) {
        (lines.join("\n"), offset)
    } else {
        return TransitionResult::Failure {
            message: format!("Error when reading an indented block of literal text on line {}. Computer says no...", line_cursor.sum_total()),
            doctree: doctree
        };
    };

    doctree = match doctree.push_data(TreeNodeType::LiteralBlock {
        text: literal_string,
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
        push_or_pop: PushOrPop::Pop,
        line_advance: LineAdvance::Some(offset),
    }
}

/// Generates a literal block node from a "quoted" block of text.
fn parse_quoted_literal(
    mut doctree: DocTree,
    src_lines: &Vec<String>,
    line_cursor: &mut LineCursor,
    captures: &regex::Captures,
    body_indent: usize,
    detected_indent: usize,
) -> TransitionResult {
    // Read in an aligned contiguous block of text and check that all its lines start with one of the symbols in
    // `common::SECTION_AND_QUOTING_CHARS`, such as a '>'.

    use crate::common::SECTION_AND_QUOTING_CHARS;

    let quote_char = if let Some(c) = captures.get(2) {
        c.as_str().chars().next().unwrap()
    } else {
        return TransitionResult::Failure {
      message: format!("Supposed quoted literal block found on line {} but no quote symbol? Computer says no...", line_cursor.sum_total()),
      doctree: doctree
    };
    };

    // Double checking that the used quotation symbol is in the accepted symbols
    let mut i = 0 as usize;
    loop {
        if let Some(c) = SECTION_AND_QUOTING_CHARS.get(i) {
            if *c == quote_char {
                break;
            } else {
                i += 1;
            }
        } else {
            return TransitionResult::Failure {
        message: format!("Unknown char '{}' used to quote literal block starting on line {}. Computer says no...", quote_char, line_cursor.sum_total()),
        doctree: doctree
      };
        }
    }

    let (literal_string, block_length) = match Parser::read_text_block(
        src_lines,
        line_cursor.relative_offset(),
        true,
        true,
        Some(detected_indent),
        true
    ) {
        Ok((mut lines, line_offset)) => {
            for line in lines.iter_mut() {
                let mut chars = line.chars();
                if let Some(c) = chars.next() {
                    if c == quote_char {
                        *line = chars.as_str().trim_start().to_string()
                    } else {
                        return TransitionResult::Failure {
              message: format!("Found mismatching line start symbol in a quoted literal block starting on line {}. Computer says no...", line_cursor.sum_total()),
              doctree: doctree
            };
                    }
                }
            }
            (lines.join("\n"), line_offset)
        }
        Err(e) => {
            eprintln!("{}", e);
            return TransitionResult::Failure {
        message: String::from("Error when reading lines of text of a supposed paragraph block. Computer says no..."),
        doctree: doctree
      };
        }
    };

    doctree = match doctree.push_data(TreeNodeType::LiteralBlock {
        text: literal_string,
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
        push_or_pop: PushOrPop::Pop,
        line_advance: LineAdvance::Some(block_length),
    }
}
