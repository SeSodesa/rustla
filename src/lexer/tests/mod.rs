/// Tests for the lexer module

#[cfg(test)]
use super::*;

#[test]
/// A test for the Lexer constructor
fn new() {
  let ls = "Aaa!";
  let lex = Lexer::new(ls);
  assert_eq!(lex.tokens, Vec::new());
  assert_eq!(lex.lexeme_start, 0);
  assert_eq!(lex.lookahead, 0);
  assert_eq!(lex.row, 0);
  assert_eq!(lex.col, 0);
}


#[test]
/// Tests the scanning of the entire source
fn lex_01() {
  let src = "
  
=====\ntekstiä1\n========\n


tekstiä2
^^^^
  
  ";
  let lexer = Lexer::new(src);

  println!("{}",src);

  let tokens = lexer.lex();

  println!("{:?}",tokens);

  assert_eq!(tokens[0].lexeme, "tekstiä1");
  assert_eq!(tokens[1].lexeme, "tekstiä2");

}

#[test]
fn scan_token() {
  let src = "========\ntekstiä\n=========\n";
  let mut lexer = Lexer::new(src);
  lexer.scan_token(src);

  println!("{:?}", lexer.tokens);

  assert_eq!(lexer.tokens[0].lexeme, "tekstiä");

}
