/// This is a test module for the Lexer
/// regexes.

use super::*;

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
    for (key, val) in TITLE_RE_MAP.iter() {
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
    for (key, val) in TITLE_RE_MAP.iter() {
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
