/// This is ruSTLa
/// 
/// author: Santtu Söderholm
///  email: santtu.soderholm@tuni.fi

use std::{env, process, fs};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR_NAME: &'static str = env!("AUTHOR_NAME");
const AUTHOR_EMAIL: &'static str = env!("AUTHOR_EMAIL");

/// Program starting point
fn main() {
    
  copyright();
  
  let args: Vec<String> = env::args().collect();

  println!("{:?}", args); // Debug with :?

  if args.len() != 2 {
    usage();
    process::exit(1)
  }

  let arg: &String = &args[1];
  let md: fs::Metadata = match fs::metadata(&arg) {
    Ok(meta) => meta,
    Err(e) => {
      println!("\nCannot determine the type of input:\n{}", e);
      process::exit(1);
    }
  };
  if md.is_dir() {
    println!("{} is a directory", args[1]);

  } else if md.is_file(){
    println!("{} is a file", args[1]);
  }

}

/// # Copyright
/// Prints out copyright information of ruSTLa
fn copyright() {
  println!("\nThis is ruSTLa, version {}", VERSION);
  println!("©{}, {}\n", AUTHOR_NAME, AUTHOR_EMAIL);
}

/// # Usage
/// A function that prints the usage instructions
/// for ruSTLa
fn usage() {
  println!("Instructions");
  println!("============");
  println!("In order to transpile a set of documents");
  println!("point ruSTLa to a directory that contains");
  println!("and index.rst file with");
  println!("\n  $ rustla <dir>\n");
  println!("A single file can be transpiled with");
  println!("  $ rustla <file>\n");
}
