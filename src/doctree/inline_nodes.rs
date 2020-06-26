/// This module contains `Inline` level node specifications.

use super::*;

#[derive(Debug, PartialEq)]
pub struct Text {
  pub text: String
}

#[derive(Debug, PartialEq)]
pub struct Emphasis {
  pub text: String,
}

#[derive(Debug, PartialEq)]
pub struct StrongEmphasis {
  pub text: String,
}

#[derive(Debug, PartialEq)]
pub struct Literal {
  pub text: String,
}

#[derive(Debug, PartialEq)]
pub struct Reference {
  pub target_label: String,
}

#[derive(Debug, PartialEq)]
pub struct InlineTarget {
  pub target_label: String,
}

#[derive(Debug, PartialEq)]
pub struct WhiteSpace {
  pub text: String
}

#[derive(Debug, PartialEq)]
pub struct FootnoteReference {
  text: String,
}

#[derive(Debug, PartialEq)]
pub struct CitationReference {
  text: String,
}

#[derive(Debug, PartialEq)]
pub struct SubstitutionReference {

  text: String,
}

#[derive(Debug, PartialEq)]
pub struct TitleReference {

  text: String,
}

#[derive(Debug, PartialEq)]
pub struct Abbreviation {

  text: String,
}

#[derive(Debug, PartialEq)]
pub struct Acronym {

  text: String,
}

#[derive(Debug, PartialEq)]
pub struct SuperScript {

  text: String,
}

#[derive(Debug, PartialEq)]
pub struct SubScript {

  text: String,
}

#[derive(Debug, PartialEq)]
pub struct Math {

  text: String,
}

#[derive(Debug, PartialEq)]
pub struct Image {

  text: String,
}

#[derive(Debug, PartialEq)]
pub struct Inline {

  text: String,
}

#[derive(Debug, PartialEq)]
pub struct Problematic {

  text: String,
}

#[derive(Debug, PartialEq)]
pub struct Generated {

  text: String,
}

