/// ## test_aplus_questionnaire
///
/// A submodule for testing A+ questionnaire directrives.
///
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;
use std::path::PathBuf;

#[cfg(test)]


#[test]
fn aplus_questionnaire_01 () {

  let src = String::from("
.. questionnaire:: 1 A
  :submissions: 4
  :points-to-pass: 0

  This is a questionnaire with the key `1` that grants at maximum 70 points
  of difficulty A. Students can make at most 4 submissions.
  This exercise is marked passed when 0 points are reached (the default).

  .. pick-one:: 10
    :required:

    What is 1+1?

    a. 1
    *b. 2
    c. 3

    !b § Count again!
    b § That is correct!
    c § Too much

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(PathBuf::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  todo!()
}