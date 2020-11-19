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

  if args_len < 2 { usage(); return Err(MainError::ArgumentError(String::from("ruSTLa needs at least one argument..."))) }

  let input: std::path::PathBuf = if let Some(path) = args.last() {
    match std::fs::canonicalize(path) {
      Ok(path) => path,
      Err(e) => {
        return Err(MainError::PathError(format!("Could not canonicalize input path: {}", e)))
      }
    }
  } else {
    unreachable!("No arguments, not even the program itself? Computer says no...")
  };

  if let Some(extension) = input.extension() {
    if let Some(extension_str) = extension.to_str() {
      if extension_str != "rst" {
        return Err(MainError::PathError(String::from("As a precaution, the source file name should have the suffix \".rst\".")))
      }
    }
  }

  let src_file_metadata: std::fs::Metadata = match std::fs::metadata(&input) {
    Ok(meta) => meta,
    Err(e) => {
      return Err(MainError::InputError(format!("Cannot determine the type of input: {}", e)))
    }
  };

  if src_file_metadata.is_dir() {

    return Err(MainError::InputError(format!("{}\n{}", "At this stage, ruSTLa is designed to work with files only.", "Please enter a valid rST file.")));

  } else if src_file_metadata.is_file() {

    let src_lines = match common::read_path_lines(&input) {
      Ok(lines) => {
        lines.map(|s|
          match s {
            Ok(string) => string,
            Err(e) => panic!("Ran into an error when reading source file into buffer.")
          }
        ).collect::<Vec<String>>()
      },
      Err(_e) => panic!("File could not be opened")
    };

    // Enter parser here...

    let mut doctree = DocTree::new(input);
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
  }

  return Ok(())
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
