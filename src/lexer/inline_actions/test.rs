/// This submodule contains tests for the inline actions of the lexer.

use super::*;

#[test]
fn lex_code () {

  let src = r"asdsadas ``some code``  ";

  let toks = Lexer::new(src, State::Inline).lex();

  println!("{:?}", toks);

  assert_eq!(toks[0].t_type, TokenType::Text);
  assert_eq!(toks[1].t_type, TokenType::Code);
  assert_eq!(toks[2].t_type, TokenType::Text);
  assert_eq!(toks[1].lexeme, "some code");

}
