/*!
ruSTLa is a Rust implementation of the rSTLa or resTructuredText → LaTeX transpiler.
ruSTLa is a master's thesis project of Santtu Söderholm,
and is intended to function as the counterpart to the LarST, or LaTeX → reStructuredText
parser, written by Tomi Janhunen.

Copyright © 2020 Santtu Söderholm
*/
#![allow(dead_code, unused_variables)]
mod rustla_options;
pub mod parser;
use parser::Parser;
use parser::state_machine::State;
pub mod doctree;
use doctree::DocTree;
pub mod common;
mod utf8_to_latex;

use std::io::BufRead;

/// Program starting point.
/// Reads the input string, feeds it to the parser and prints the generated doctree.
pub fn run() -> Result<(), MainError> {
    copyright();

    let args: Vec<String> = std::env::args().collect();
    let args_len = args.len();

    let rustla_options = crate::rustla_options::ruSTLaOptions::new(&args);

    let mut src_lines = Vec::new();

    // Populate src_lines and generate a path buffer to the source
    // (empty if read from stdin)
    let path: std::path::PathBuf = if let Some(arg) = args.last() {
        if args_len == 1 {
            // Handle no arguments first
            let stdin = std::io::stdin();
            for line in stdin.lock().lines() {
                match line {
                    Ok(line) => src_lines.push(line),
                    Err(e) => {
                        return Err(MainError::InputError(format!(
                            "Error when reading stdin: {}",
                            e
                        )))
                    }
                }
            }

            std::path::PathBuf::new()
        } else if let Ok(pathbuf) = std::fs::canonicalize(arg) {
            let line_iter = match crate::common::read_path_lines(&pathbuf) {
                Ok(lines) => lines,
                Err(e) => {
                    return Err(MainError::PathError(format!(
                        "Could not split file into lines: {}",
                        e
                    )))
                }
            };
            for line in line_iter {
                match line {
                    Ok(line) => src_lines.push(line),
                    Err(e) => {
                        return Err(MainError::InputError(String::from(
                            "Could not construct a line vector from input...",
                        )))
                    }
                }
            }

            pathbuf
        } else {
            let stdin = std::io::stdin();
            for line in stdin.lock().lines() {
                match line {
                    Ok(line) => src_lines.push(line),
                    Err(e) => {
                        return Err(MainError::InputError(format!(
                            "Error when reading stdin: {}",
                            e
                        )))
                    }
                }
            }
            std::path::PathBuf::new()
        }
    } else {
        unreachable!("No arguments, not even the program itself? Computer says no...")
    };

    // Enter parser here...

    let mut doctree = DocTree::new(path);
    let mut parser = Parser::new(src_lines, doctree, 0, 0, State::Body, 0);

    use common::ParsingResult;

    doctree = match parser.parse() {
        ParsingResult::EOF { doctree, .. } | ParsingResult::EmptyStateStack { doctree, .. } => {
            doctree
        }
        ParsingResult::Failure { message, doctree } => {
            eprintln!("Parsing error: {}", message);
            doctree
        }
    };

    doctree = doctree.perform_restructuredtext_transforms();
    doctree.write_to_larst(&rustla_options);

    Ok(())
}

/// Prints out copyright information of ruSTLa
fn copyright() {
    /// Version retrieved from cargo.toml
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    /// Author name retrieved from cargo.toml
    const AUTHOR_NAME: &'static str = env!("AUTHOR_NAME");
    /// Author email retrieved from cargo.toml
    // const AUTHOR_EMAIL: &'static str = env!("AUTHOR_EMAIL");
    /// Year retrieved from cargo.toml
    const AUTHOR_YEAR: &'static str = env!("AUTHOR_YEAR");
    eprintln!("\nThis is ruSTLa, version {}", VERSION);
    eprintln!("Copyright © {} {}\n", AUTHOR_YEAR, AUTHOR_NAME);
}

/// A function that prints the usage instructions for ruSTLa
fn usage() {
    println!("Instructions");
    println!("============");
    println!("In order to transpile a document,");
    println!("point ruSTLa to an rST file with");
    println!("\n  $ rustla path/to/file.rst\n");
    println!("Capabilities to transpile an entire");
    println!("toctree will be added later.");
}

#[derive(Debug)]
/// The ways in which the `main` function can fail.
pub enum MainError {
    /// The file handed in for parsing wasn't valid.
    PathError(String),
    /// Something was amiss with the input, so it couldn't be read.
    InputError(String),
    /// The parser returned in an error state.
    ParseError(String),
    /// The doctree generated by the parser couldn't be transformed to object code.
    PrintError(String),
    /// Something was off with the command line arguments.
    ArgumentError(String),
}
