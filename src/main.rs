#![allow(dead_code, unused_variables, unused_imports)]
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

use std::io::BufRead;
use std::{env, fs, path, io};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR_NAME: &'static str = env!("AUTHOR_NAME");
const AUTHOR_EMAIL: &'static str = env!("AUTHOR_EMAIL");
const AUTHOR_YEAR: &'static str = env!("AUTHOR_YEAR");


/// Program starting point
fn main() -> Result<(), ()>{
    
  copyright();
  
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    usage();
    return Err(())
  }

  let path: path::PathBuf = match fs::canonicalize(&args[1]) {
    Ok(p) => p,
    Err(e) => {
      eprintln!("Could not resolve file path:\n{}",e);
      return Err(())
    }
  };

  let md: fs::Metadata = match fs::metadata(&path) {
    Ok(meta) => meta,
    Err(e) => {
      eprintln!("\nCannot determine the type of input:\n{}", e);
      return Err(())
    }
  };

  if md.is_dir() {
    println!("{:?} is a directory", path);
    println!("At this stage, ruSTLa is designed to work with");
    println!("files only. Please enter a valid rST file.");
    return Err(());

  } else if md.is_file() {

    let src_lines = match common::read_path_lines(&path) {
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

    use crate::common::OutputStream;
    doctree.write_to_larst(OutputStream::StdOut);
  }

  return Ok(())
}

/// # `has_toctree`
/// Checks the file contents `fc`
/// for the substring `.. toctree::`
fn has_toctree (fc: &String) -> bool{
  println!("Checking for toctree...");
  if fc.contains(".. toctree::") {
    println!("Toctree found...");
    true
  } else {
    println!("No toctree to be seen...");
    false
  }
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
