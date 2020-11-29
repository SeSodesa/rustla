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
    MalformedTableError(String)
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
    pub fn parse_grid_table (src_lines: &Vec<String>, line_cursor: &LineCursor) -> TableResult {

        let table_lines = match Self::isolate_grid_table(src_lines, line_cursor) {
            TableIsolationResult::Table(lines) => lines,
            TableIsolationResult::EmptyTable => return TableResult::MalformedTableError (
                format!("Table starting on line {} was empty.", line_cursor.sum_total())
            ),
            TableIsolationResult::EndOfInput => return TableResult::MalformedTableError (
                format!("Ran off the end of input when scanning a table starting on line {}.", line_cursor.sum_total())
            ),
        };
        
        let table_height = if let Some(line_len) = table_lines.len().checked_sub(1) {
            line_len
        } else {
            return TableResult::MalformedTableError(
                format!("Table on line {} only had a top border?", line_cursor.sum_total())
            )
        };
        let table_width = {
            match table_lines.get(0) {
                Some(line) => match line.chars().count().checked_sub(1) {
                    Some(num) => num,
                    None => return TableResult::MalformedTableError(
                        format!("The first row of grid table on line {} was only a single character long?", line_cursor.sum_total())
                    )
                }
                None => return TableResult::MalformedTableError(
                    format!("Table on line {} didn't even have a top border?", line_cursor.sum_total())
                )
            }
        };

        let mut cell_corner_coordinates = Vec::<(usize, usize)>::from([(0, 0)]);

        let done_cells = [usize::MAX].repeat(table_width);


        while let Some((top_pos, left_pos)) = cell_corner_coordinates.pop() {
            if top_pos == table_height || left_pos == table_width || top_pos <= done_cells[left_pos] {
                continue
            }

            // Scan cell next...
        }
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
