/// This is ruSTLa
/// 
/// author: Santtu Söderholm
///  email: santtu.soderholm@tuni.fi

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR_NAME: &'static str = env!("AUTHOR_NAME");
const AUTHOR_EMAIL: &'static str = env!("AUTHOR_EMAIL");

fn main() {
    
  copyright();
  usage();

}

fn copyright() {
  println!("\nThis is ruSTLa, version {}\n", VERSION);
  println!("©{}, {}\n", AUTHOR_NAME, AUTHOR_EMAIL);
}

fn usage() {
  println!("Instructions");
  println!("============");
  println!("In order to transpile a set of documents");
  println!("point ruSTLa to a directory that contains");
  println!("and index.rst file with");
  println!("\n  $ rustla <dir>\n");
  println!("A single file cab be transpiled with");
  println!("  $ rustla <file>");
}
