/// This submodule contains tests for the inline actions of the lexer.

use super::super::*;

#[test]
fn lex_code () {

  let src = r"asdsadas ``some code``  ";

  let pos = &mut Pos::new();

  let toks = Lexer::new(src, pos, State::Inline).lex();

  println!("{:?}", toks);

  assert_eq!(toks[0].t_type, TokenType::Text);
  assert_eq!(toks[1].t_type, TokenType::Code);
  assert_eq!(toks[2].t_type, TokenType::Text);
  assert_eq!(toks[1].lexeme, "some code");

}

#[test]
fn phrase_reference_01 () {

  let src = r"asdsadas ``some code``  
  asdsadsadsad `alias <link>`__";

  let pos = &mut Pos::new();

  let toks = Lexer::new(src, pos, State::Inline).lex();

  println!("{:?}", toks);

  assert_eq!(toks[4].t_type, TokenType::LinkAlias);
  assert_eq!(toks[5].t_type, TokenType::Hyperlink);

}

#[test]
fn phrase_reference_02 () {

  let src = r"asdsadas ``some code``  
  asdsadsadsad `target`__ adsadsadsadasds
  ffasfsa";

  let pos = &mut Pos::new();

  let toks = Lexer::new(src, pos, State::Inline).lex();

  println!("{:?}", toks);

  assert_eq!(toks[4].t_type, TokenType::TargetReference);

}
