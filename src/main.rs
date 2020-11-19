#![allow(dead_code, unused_variables)]
/// # This is ruSTLa
///
/// ruSTLa is the Rust implementation of the rSTLa or resTructuredText to LaTeX parser.
/// It is intended to function as the counterpart to the LarST, or LaTeX to reStructuredText
/// parser, written by Tomi Janhunen.
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi


mod parser;
use parser::Parser;
mod doctree;
use doctree::DocTree;
mod common;
mod utf8_to_latex;

use std::collections::HashMap;
use std::{env};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR_NAME: &'static str = env!("AUTHOR_NAME");
const AUTHOR_EMAIL: &'static str = env!("AUTHOR_EMAIL");
const AUTHOR_YEAR: &'static str = env!("AUTHOR_YEAR");


/// Program starting point
fn main() -> Result<(),MainError> {
    
  copyright();
  
  let args: Vec<String> = env::args().collect();
  let args_len = args.len();

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

  let option_map = read_known_options(&args);

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

    use crate::common::OutputStream;
    let out_stream = if let Some(stream) = option_map.get("output-stream") {
      match stream.as_str() {
        "stdout" => OutputStream::StdOut,
        "stderr" => OutputStream::StdOut,
        "file" => OutputStream::File,
        _ => OutputStream::StdOut,
      }
    } else { OutputStream::StdOut };
    doctree.write_to_larst(out_stream);
  }

  return Ok(())
}


/// Scans a vector of the command line arguments of ruSTLa for known options and their values.
/// Unknown options and/or values are simply ignored.
/// Any recognized options and their values are stored in a HashMap.
fn read_known_options (args: &Vec<String>) -> HashMap<String, String> {

  const KNOWN_OPTIONS: [(&str, &[&str]); 1] = [
    ("--output-stream", &["stdout", "stderr", "file"]),
  ];

  let mut arg_index = 0usize;
  let args_len = args.len();

  let mut option_map: HashMap<String, String> = HashMap::new();

  loop {

    let arg = if let Some(arg) = args.get(arg_index) { arg } else { break };

    // Check for known options and act accordingly
    match arg.as_str() {

      "output-stream" => {

        let option_name = "output-stream";

        let option_value = if let Some(val) = args.get(arg_index + 1) { val } else {
          eprintln!("The option \"{}\" has no value. Ignoring the option...", option_name);
          break
        };
        match option_value.as_str() {
          "stdout" | "stderr" | "file" => if let Some(val) = option_map.insert(option_name.to_string(), option_value.to_string()) {
            eprintln!("Option \"{}\" given more than once. Overriding last value \"{}\" with a new one, \"{}\"...", option_name, val, option_value);
            arg_index += 2;
          } else {
            arg_index += 2;
          },
          _ => {
            eprintln!("Unknown value for the option \"{}\". Trying to interpret it as a different option...", option_name);
            arg_index += 1;
            continue
          }
        }
      }

      _ => { arg_index += 1; }
    }

    if arg_index >= args_len { break }
  };

  option_map
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
