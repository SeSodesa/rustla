/// This is a submodule for testing
/// title-related things of the body elements.

use super::*;
use regex::Regex;

#[cfg(test)]


#[test]
fn overlined_titles() {

  let tls = [
    "=", "-", "`", ":", "'", r#"""#, "~",
    "^", "_", "*", "+", "#", "<", ">"
  ];
  let title = "  This is a title";
  for s in tls.iter() {
    let overlined_title
      = format!(
        "{}\n{}\n{}\n",
        s.repeat(3), title, s.repeat(3)
      );
    let mut some_re_matches:bool = false;
    for (_, val, _) in BODY_TRANSITIONS.iter() {
      let r = Regex::new(val).unwrap();
      if r.is_match(overlined_title.as_str()) {
        some_re_matches = true;
        break
      }
    }

    if !some_re_matches {
      panic!();
    }
  }
}

#[test]
fn underlined_titles() {

  let tls = [
    "=", "-", "`", ":", "'", r#"""#, "~",
    "^", "_", "*", "+", "#", "<", ">"
  ];
  let title = "  This is a title";
  for s in tls.iter() {
    let overlined_title
      = format!(
        "{}\n{}\n",
        title, s.repeat(3)
      );
    let mut some_re_matches:bool = false;
    for (_, val, _) in BODY_TRANSITIONS.iter() {
      let r = Regex::new(val).unwrap();
      if r.is_match(overlined_title.as_str()) {
        some_re_matches = true;
        break
      }
    }

    if !some_re_matches {
      panic!();
    }
  }
}
