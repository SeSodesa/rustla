/// This submodule contains tests for the inline actions of the lexer.

use super::super::*;

#[test]
fn lex_code () {

  let mut src_iter = r"asdsadas ``some code``  ".chars();

  let pos = &mut Pos::new();

  let mut lexer = Lexer::new(&mut src_iter, pos, State::Inline);

  lexer.lex();

  let toks = lexer.tokens;

  println!("{:?}", toks);

  assert_eq!(toks[0].t_type, TokenType::Text);
  assert_eq!(toks[1].t_type, TokenType::Code);
  assert_eq!(toks[2].t_type, TokenType::Text);
  assert_eq!(toks[1].lexeme, "some code");

}

#[test]
fn phrase_reference_01 () {

  let mut src_iter = r"asdsadas ``some code``  
  asdsadsadsad `alias <link>`__".chars();

  let pos = &mut Pos::new();

  let mut lexer = Lexer::new(&mut src_iter, pos, State::Inline);

  lexer.lex();

  let toks = lexer.tokens;

  println!("{:?}", toks);

  assert_eq!(toks[4].t_type, TokenType::LinkAlias);
  assert_eq!(toks[5].t_type, TokenType::Hyperlink);

}

#[test]
fn phrase_reference_02 () {

  let mut src_iter = r"asdsadas ``some code``  
  asdsadsadsad `target`__ adsadsadsadasds
  ffasfsa".chars();


  let pos = &mut Pos::new();



  let mut lexer = Lexer::new(&mut src_iter, pos, State::Inline);

  lexer.lex();

  let toks = lexer.tokens;

  println!("{:?}", toks);

  assert_eq!(toks[4].t_type, TokenType::TargetReference);

}
