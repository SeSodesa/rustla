/// A regex submodule that contains title
/// related regular expressions,

mod test;

use regex::Regex;
use lazy_static::lazy_static;
use crate::lexer::token::TokenType;

// Patterns as raw strings
// =======================

/// A map (list) of TokenTypes and matching patterns 
/// This could have been cosntructed with a macro,
/// but meh...
static TITLE_RE_MAP: &[(TokenType, &'static str)] = &[
  
  // Overlined headings
  // ------------------
  (TokenType::EqualsOverlinedHeading, r"(?m)^{3,}\n[ \t]*.+\n={3,}\n"),
  (TokenType::DashOverlinedHeading, r"(?m)^-{3,}\n[ \t]*.+\n-{3,}\n"),
  (TokenType::BacktickOverlinedHeading, r"(?m)^`{3,}\n[ \t]*.+\n`{3,}\n"),
  (TokenType::ColonOverlinedHeading, r"(?m)^:{3,}\n[ \t]*.+\n:{3,}\n"),
  (TokenType::SquoteOverlinedHeading, r"(?m)^'{3,}\n[ \t]*.+\n'{3,}\n"),
  (TokenType::DquoteOverlinedHeading, r#"(?m)^"{3,}\n[ \t]*.+\n"{3,}\n"#),
  (TokenType::TildeOverlinedHeading, r"(?m)^~{3,}\n[ \t]*.+\n~{3,}\n"),
  (TokenType::CaretOverlinedHeading, r"(?m)^\^{3,}\n[ \t]*.+\n\^{3,}\n"),
  (TokenType::UnderscoreOverlinedHeading, r"(?m)^_{3,}\n[ \t]*.+\n_{3,}\n"),
  (TokenType::AsteriskOverlinedHeading, r"(?m)^\*{3,}\n[ \t]*.+\n\*{3,}\n"),
  (TokenType::PlusOverlinedHeading, r"(?m)^\+{3,}\n[ \t]*.+\n\+{3,}\n"),
  (TokenType::HashOverlinedHeading, r"(?m)^\#{3,}\n[ \t]*.+\n\#{3,}\n"),
  (TokenType::LessOverlinedHeading, r"(?m)^<{3,}\n[ \t]*.+\n<{3,}\n"),
  (TokenType::MoreOverlinedHeading, r"(?m)^>{3,}\n[ \t]*.+\n>{3,}\n"),

  // Normal headings
  // ---------------
  (TokenType::EqualsHeading, r"(?m)^.+\n={3,}\n"),
  (TokenType::DashHeading, r"(?m)^.+\n-{3,}\n"),
  (TokenType::BacktickHeading, r"(?m)^.+\n`{3,}\n"),
  (TokenType::ColonHeading, r"(?m)^.+\n:{3,}\n"),
  (TokenType::SquoteHeading, r"(?m)^.+\n'{3,}\n"),
  (TokenType::DquoteHeading, r#"(?m)^.+\n"{3,}\n"#),
  (TokenType::TildeHeading, r"(?m)^.+\n~{3,}\n"),
  (TokenType::CaretHeading, r"(?m)^.+\n\^{3,}\n"),
  (TokenType::UnderscoreHeading, r"(?m)^.+\n_{3,}\n"),
  (TokenType::AsteriskHeading, r"(?m)^.+\n\*{3,}\n"),
  (TokenType::PlusHeading, r"(?m)^.+\n\+{3,}\n"),
  (TokenType::HashHeading, r"(?m)^.+\n\#{3,}\n"),
  (TokenType::LessHeading, r"(?m)^.+\n<{3,}\n"),
  (TokenType::MoreHeading, r"(?m)^.+\n>{3,}\n"),

];

/// Title over- and underlined with =
const EQUALS_U_TITLE: &'static str
  = r"=(?m)^*.+\n={3,}";

/// Title over- and underlined with -
const DASH_U_TITLE: &'static str
  = r"(?m)^.+\n-{3,}";

/// Title over- and underlined with `
const BACKTICK_U_TITLE: &'static str
  = r"(?m)^`.+\n`{3,}";

/// Title over- and underlined with :
const COLON_U_TITLE: &'static str
  = r"(?m)^.+\n:{3,}";

/// Title over- and underlined with '
const SQUOTE_U_TITLE: &'static str
  = r"(?m)^.+\n'{3,}";

/// Title over- and underlined with "
const DQUOTE_U_TITLE: &'static str
  = r#"(?m)^.+\n"{3,}"#;

/// Title over- and underlined with ~
const TILDE_U_TITLE: &'static str
  = r"(?m)^.+\n~{3,}";
  
/// Title over- and underlined with ^
const CARET_U_TITLE: &'static str
  = r"(?m)^.+\n\^{3,}";

/// Title over- and underlined with _
/// (why why why would you ever do this?)
const UNDERSCORE_U_TITLE: &'static str
  = r"(?m)^.+\n_{3,}";

/// Title over- and underlined with *
const ASTERISK_U_TITLE: &'static str
  = r"(?m)^.+\n*{3,}";

/// Title over- and underlined with +
const PLUS_U_TITLE: &'static str
  = r"(?m)^.+\n+{3,}";

/// Title over- and underlined with #
const HASH_U_TITLE: &'static str
  = r"(?m)^.+\n\#{3,}";

/// Title over- and underlined with <
const LESS_U_TITLE: &'static str
  = r"(?m)^.+\n<{3,}";

/// Title over- and underlined with >
const MORE_U_TITLE: &'static str
  = r"(?m)^.+\n>{3,}";
