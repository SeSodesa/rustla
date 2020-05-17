/// This is ruSTLa
/// 
/// author: Santtu Söderholm
///  email: santtu.soderholm@tuni.fi

use std::{env, process, fs, path};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR_NAME: &'static str = env!("AUTHOR_NAME");
const AUTHOR_EMAIL: &'static str = env!("AUTHOR_EMAIL");
const AUTHOR_YEAR: &'static str = env!("AUTHOR_YEAR");


/// Program starting point
fn main() {
    
  copyright();
  
  let args: Vec<String> = env::args().collect();

  if args.len() != 2 {
    usage();
    process::exit(1)
  }

  let path: path::PathBuf = match fs::canonicalize(&args[1]) {
    Ok(p) => p,
    Err(e) => {
      println!("Could not resolve file path:\n{}",e);
      process::exit(1);
    }
  };

  let md: fs::Metadata = match fs::metadata(&path) {
    Ok(meta) => meta,
    Err(e) => {
      println!("\nCannot determine the type of input:\n{}", e);
      process::exit(1);
    }
  };

  if md.is_dir() {
    println!("{:?} is a directory", path);
    println!("At this stage, ruSTLa is designed to work with");
    println!("files only. Please enter a valid rST file.");
    process::exit(1);

  } else if md.is_file() {
    println!("{:?} is a file.", path);
    
    let fc: String = match fs::read_to_string(path) {
      Ok(a) => a,
      Err(b) => {
        println!("Could not read file:\n{}", b);
        process::exit(1);
      }
    };
    let htt: bool = has_toctree(&fc);

    if htt {
      // Create document tree structure,
      // then transpile the files
    } else {
      // Attempt transpiling a single file
    }
    
  }
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
  println!("\nThis is ruSTLa, version {}", VERSION);
  println!("©{} {},\n{}\n", AUTHOR_NAME, AUTHOR_YEAR, AUTHOR_EMAIL);
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
