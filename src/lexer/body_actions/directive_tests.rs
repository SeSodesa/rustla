/// This moduel tests directive-related regexes

use super::*;
use super::super::*;
use regex::Regex;

#[cfg(test)]

#[test]
fn general_directive_match() {

  let text = r"
asdasdfsadfdsafsadf
safadsfsadfasfadsf
asdfadsf

  .. image:: images/ball1.gif
  
asfsdafsdfsafasfsf";

  let pattern = val_from_key(
    &TokenType::GeneralDirective, 
    BODY_TRANSITIONS).unwrap();

  let re = Regex::new(pattern).unwrap();
  if !re.is_match(text) {
    panic!();
  }

}
