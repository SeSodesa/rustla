/// ## Table parsers
///
/// This submodule contains the table parsing functions,
/// that the state machine uses as helpers in constructing tables.
///
/// author: Santtu Söderholm <santtu.soderholm@tuni.fi>

use crate::parser::Parser;
use crate::parser::line_cursor::LineCursor;


#[derive(Debug)]
/// A data structure that will be returned once the table parsing has been completed, or if it fails.
/// Provides a variant for successfully constructed tables and an error formalformed tables.
///
/// Both grid and simple rST tables result in the data structure of column widhts,
/// header rows and body rows. Each row is a vector of cells.
///
pub enum TableResult {
    CompleteTable {
        col_widths: Vec<u32>,
        head_rows: Vec<Row>,
        body_rows: Vec<Row>
    },
    MalformedTableError
}

impl TableResult {}

#[derive(Debug)]
pub struct Cell {
    vspan: u32,
    hspan: u32,
    content_offset: u32,
    text_lines: Vec<String>
}

impl Cell {

    /// The `Cell` constructor.
    pub fn new(vspan: u32, hspan: u32, content_offset: u32, text_lines: Vec<String>) -> Self {
        Self {
            vspan: vspan,
            hspan: hspan,
            content_offset:content_offset,
            text_lines: text_lines
        }
    }
}

/// A type alias for a vector of table cells.
type Row = Vec<Cell>;

/// Implementation of the table parsing functions for the `Parser` type.
impl Parser {

    /// Parses a grid table, returning a `TableResult`.
    pub fn parse_grid_table (table_lines: Vec<String>) -> TableResult {
        todo!()
    }


    /// Parses a simple table into a `TableResult`.
    pub fn parse_simple_table (table_string: Vec<String>) -> TableResult {
        todo!()
    }

    /// Retrieves the lines containing a grid table from the source line vector.
    pub fn isolate_grid_table (src_lines: &Vec<String>, line_cursor: &LineCursor) -> TableIsolationResult {

        let start_line = line_cursor.relative_offset();
        let indent_allowed = true;
        let remove_indent = true;
        let alignment = if let Some(line) = src_lines.get(line_cursor.relative_offset()) {
            line.chars().take_while(|c| c.is_whitespace()).count()
        } else {
            return TableIsolationResult::EndOfInput
        };

        let (mut lines, offset) = if let Ok((lines, offset)) = Parser::read_text_block(src_lines, start_line, indent_allowed, remove_indent, Some(alignment)) {
            (lines, offset)
        } else {
            return TableIsolationResult::EndOfInput
        };

        // Check if the last line of lines matches the table bottom pattern and if not,
        // pop lines until it is found.
        while let Some(line) = lines.last_mut() {
            if let Some(capts) = crate::parser::automata::GRID_TABLE_TOP_AND_BOT_AUTOMATON.captures(line) {
                break
            } else {
                if let None = lines.pop() {
                    return TableIsolationResult::EmptyTable
                }
            }
        }

        // Kept popping and met the table top line...
        if lines.len() == 1 { return TableIsolationResult::EmptyTable }

        TableIsolationResult::Table(lines)
    }


    /// Retrieves the lines containing a simple table from the source line vector.
    pub fn isolate_simple_table (src_lines: &Vec<String>) {
        todo!()
    }
}

pub enum TableIsolationResult {
    Table(Vec<String>),
    EndOfInput,
    EmptyTable
}
