#[cfg(test)]
/// Tests for `Token` methods

use super::*;

#[test]
/// Tests the constructor
fn new(){
  let t = Token::new(TokenType::Test, String::from("test"), 3);
  assert_eq!(t.t_type, TokenType::Test);
}
