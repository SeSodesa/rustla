/// This submodule contains tests for the inline actions of the lexer.

use super::*;

#[test]
fn lex_code () {

  let src = r"asdsadas ``   ";

  let toks = Lexer::new(src, State::Inline).lex();

  println!("{:?}", toks);

  panic!();

}
