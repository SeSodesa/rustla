/// This is a test module for the Lexer
/// regexes.

use super::*;

#[cfg(test)]

#[test]
fn overlined_title() {
  
  let title_line_symbols = [
    "=", "-", "`", ":", "'", r#"""#, "~",
    "^", "_", "*", "+", "#", "<", ">"
  ];
  let title_str = "This is a title";

  for s in title_line_symbols.iter() {
    let overlined_title
      = format!(
        "{}\n{}\n{}\n",
        s.repeat(3), title_str, s.repeat(3)
      );

    if
      ! (OVERLINED_TITLE_EQUALS.is_match(overlined_title.as_str())
      | OVERLINED_TITLE_DASH.is_match(overlined_title.as_str())
      | OVERLINED_TITLE_BACKTICK.is_match(overlined_title.as_str())
      | OVERLINED_TITLE_COLON.is_match(overlined_title.as_str())
      | OVERLINED_TITLE_SQUOTE.is_match(overlined_title.as_str())
      | OVERLINED_TITLE_DQUOTE.is_match(overlined_title.as_str())
      | OVERLINED_TITLE_TILDE.is_match(overlined_title.as_str())
      | OVERLINED_TITLE_CARET.is_match(overlined_title.as_str())
      | OVERLINED_TITLE_UNDERSCORE.is_match(overlined_title.as_str())
      | OVERLINED_TITLE_ASTERISK.is_match(overlined_title.as_str())
      | OVERLINED_TITLE_PLUS.is_match(overlined_title.as_str())
      | OVERLINED_TITLE_HASH.is_match(overlined_title.as_str())
      | OVERLINED_TITLE_LESS.is_match(overlined_title.as_str())
      | OVERLINED_TITLE_MORE.is_match(overlined_title.as_str())
      )
    {
      eprintln!("{}", overlined_title);
      panic!();
    }
  }
}


#[test]
fn underlined_title() {
  
  let title_line_symbols = [
    "=", "-", "`", ":", "'", r#"""#, "~",
    "^", "_", "*", "+", "#", "<", ">"
  ];
  let title_str = "This is a title";

  for s in title_line_symbols.iter() {
    let underlined_title
      = format!(
        "{}\n{}\n",
        title_str, s.repeat(3)
      );

    if
      ! (TITLE_EQUALS.is_match(underlined_title.as_str())
      | TITLE_DASH.is_match(underlined_title.as_str())
      | TITLE_BACKTICK.is_match(underlined_title.as_str())
      | TITLE_COLON.is_match(underlined_title.as_str())
      | TITLE_SQUOTE.is_match(underlined_title.as_str())
      | TITLE_DQUOTE.is_match(underlined_title.as_str())
      | TITLE_TILDE.is_match(underlined_title.as_str())
      | TITLE_CARET.is_match(underlined_title.as_str())
      | TITLE_UNDERSCORE.is_match(underlined_title.as_str())
      | TITLE_ASTERISK.is_match(underlined_title.as_str())
      | TITLE_PLUS.is_match(underlined_title.as_str())
      | TITLE_HASH.is_match(underlined_title.as_str())
      | TITLE_LESS.is_match(underlined_title.as_str())
      | TITLE_MORE.is_match(underlined_title.as_str())
      )
    {
      eprintln!("{}", underlined_title);
      panic!();
    }
  }
}
