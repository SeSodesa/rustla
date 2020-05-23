/// This submodule contains the
/// set of possible states the lexer
/// could be in


#[derive(Debug, PartialEq)]
/// ### State
/// This enum lists the possible states the
/// lexer could be in. reStructuredText
/// contains only body and inline elements,
/// so we stick with those.
pub enum State {
  Body,
  Inline
}

