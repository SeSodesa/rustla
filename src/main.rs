#![allow(dead_code, unused_variables)]
/// # This is ruSTLa
///
/// ruSTLa is the Rust implementation of the rSTLa or resTructuredText to LaTeX parser.
/// It is intended to function as the counterpart to the LarST, or LaTeX to reStructuredText
/// parser, written by Tomi Janhunen.
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi


mod rustla_options;
mod parser;
use parser::Parser;
mod doctree;
use doctree::DocTree;
mod common;
mod utf8_to_latex;

use std::io::BufRead;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR_NAME: &'static str = env!("AUTHOR_NAME");
const AUTHOR_EMAIL: &'static str = env!("AUTHOR_EMAIL");
const AUTHOR_YEAR: &'static str = env!("AUTHOR_YEAR");


/// Program starting point
fn main() -> Result<(),MainError> {

  copyright();

  let args: Vec<String> = std::env::args().collect();
  let args_len = args.len();

  let rustla_options = crate::rustla_options::ruSTLaOptions::new(&args);

  let (src_lines, path): (Vec<String>, std::path::PathBuf) = if let Some(path) = args.last() {

    if args_len == 1 { // Handle no arguments first

      let mut src_lines = Vec::new();
      let stdin = std::io::stdin();
      for line in stdin.lock().lines() {
        match line {
          Ok(line) => src_lines.push(line),
          Err(e) => return Err(MainError::InputError(format!("Error when reading stdin: {}", e)))
        }
      }

      (src_lines, std::path::PathBuf::new())

    } else if let Ok(pathbuf) = std::fs::canonicalize(path) {

      let line_iter = match crate::common::read_path_lines(&pathbuf) {
        Ok(lines) => lines,
        Err(e) => return Err(MainError::PathError(format!("Could not split file into lines: {}", e)))
      };

      let mut src_lines: Vec<String> = Vec::new();
      for line in line_iter {
        match line {
          Ok(line) => src_lines.push(line),
          Err(e) => return Err(MainError::InputError(String::from("Could not construct a line vector from input...")))
        }
      }

      (src_lines, pathbuf)

    } else {

      let mut src_lines = Vec::new();
      let stdin = std::io::stdin();
      for line in stdin.lock().lines() {
        match line {
          Ok(line) => src_lines.push(line),
          Err(e) => return Err(MainError::InputError(format!("Error when reading stdin: {}", e)))
        }
      }
      (src_lines, std::path::PathBuf::new())
    }
  } else {
    unreachable!("No arguments, not even the program itself? Computer says no...")
  };

  // Enter parser here...

  let mut doctree = DocTree::new(path);
  let mut parser = Parser::new(src_lines, doctree, None, 0, None, 0);
  
  use common::ParsingResult;

  doctree = match parser.parse() {
    ParsingResult::EOF { doctree, .. } | ParsingResult::EmptyStateStack { doctree, .. } => doctree,
    ParsingResult::Failure { message, doctree } => {
      eprintln!("Parsing error: {}", message);
      doctree
    }
  };

  doctree = doctree.perform_restructuredtext_transforms();
  doctree.write_to_larst(&rustla_options);

  Ok(())
}


/// # Copyright
/// Prints out copyright information of ruSTLa
fn copyright() {
  eprintln!("\nThis is ruSTLa, version {}", VERSION);
  eprintln!("© {} {},\n{}\n", AUTHOR_NAME, AUTHOR_YEAR, AUTHOR_EMAIL);
}

/// # Usage
/// A function that prints the usage instructions
/// for ruSTLa
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
enum MainError {
  Ok,
  PathError(String),
  InputError(String),
  ParseError(String),
  PrintError(String),
  ArgumentError(String)
}
