/// This module contains `Inline` level node specifications.

use super::*;

#[derive(Debug)]
pub struct Text {
  pub text: String
}

#[derive(Debug)]
pub struct Emphasis {

  text: String,
}

#[derive(Debug)]
pub struct StrongEmphasis {
  pub text: String,
}

#[derive(Debug)]
pub struct Literal {
  text: String,
}

#[derive(Debug)]
pub struct Reference {

  text: String,
}

#[derive(Debug)]
pub struct FootnoteReference {

  text: String,
}

#[derive(Debug)]
pub struct CitationReference {

  text: String,
}

#[derive(Debug)]
pub struct SubstitutionReference {

  text: String,
}

#[derive(Debug)]
pub struct TitleReference {

  text: String,
}

#[derive(Debug)]
pub struct Abbreviation {

  text: String,
}

#[derive(Debug)]
pub struct Acronym {

  text: String,
}

#[derive(Debug)]
pub struct SuperScript {

  text: String,
}

#[derive(Debug)]
pub struct SubScript {

  text: String,
}

#[derive(Debug)]
pub struct Math {

  text: String,
}

#[derive(Debug)]
pub struct Image {

  text: String,
}

#[derive(Debug)]
pub struct Inline {

  text: String,
}

#[derive(Debug)]
pub struct Problematic {

  text: String,
}

#[derive(Debug)]
pub struct Generated {

  text: String,
}

