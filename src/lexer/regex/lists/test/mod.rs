/// Tests for list related regexes

use super::*;

#[cfg(test)]

#[test]
fn unnumbered_list_item() {
  let list
    = "* Tässä on lista-alkio\n* Jos toinenkin.";

  if !UNNUMBERED_LIST_RE.is_match(list) {
    panic!();
  }
}


#[test]
fn dot_numbered_list_item() {

  let dot_list
  = "1. Tässä on lista-alkio\n2. Jos toinenkin.\n";

  if !NUMBERED_LIST_DOT_RE.is_match(dot_list) {
    panic!();
  }

}

#[test]
fn lr_numbered_list() {
  let lr_list
  = "(1) Tässä on lista-alkio\n(2) Jos toinenkin.";

  if !NUMBERED_LIST_LRPAREN_RE.is_match(lr_list) {
    panic!();
  }

}

#[test]
fn r_numbered_list() {
  let r_list
  = "1) Tässä on lista-alkio\n2) Jos toinenkin.";

  if !NUMBERED_LIST_RPAREN_RE.is_match(r_list) {
    panic!();
  }
}

#[test]
fn dot_alpha_list() {

  let dot_list
  = "B. Tässä on lista-alkio\nA. Jos toinenkin.\n\n";

  if !ALPHA_LIST_DOT_RE.is_match(dot_list) {
    panic!();
  }

}

#[test]
fn lr_alpha_list() {
  let lr_list
  = "(C) Tässä on lista-alkio\n(d) Jos toinenkin.\n\n";

  if !ALPHA_LIST_LRPAREN_RE.is_match(lr_list) {
    panic!();
  }

}

#[test]
fn r_alpha_list() {
  let r_list
  = "a) Tässä on lista-alkio\nB) Jos toinenkin.\n\n";

  if !ALPHA_LIST_RPAREN_RE.is_match(r_list) {
    panic!();
  }
}
