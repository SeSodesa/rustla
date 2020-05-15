const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR_NAME: &'static str = env!("AUTHOR_NAME");
const AUTHOR_EMAIL: &'static str = env!("AUTHOR_EMAIL");

fn main() {
    
  copyright();

}

fn copyright() {
  println!("\nThis is ruSTLa, version {}\n", VERSION);
  println!("©{}, {}\n", AUTHOR_NAME, AUTHOR_EMAIL);
}
