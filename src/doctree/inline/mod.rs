/// This module contains `Inline` level node specifications.

use super::*;


pub struct Text {
  id: usize,
  text: String
}

pub struct Emphasis {
  id: usize,
  text: String,

}

pub struct StrongEmphasis {
  id: usize,
  text: String,
}

pub struct Literal {
  id: usize,
  text: String,
}

pub struct Reference {
  id: usize,
  text: String,
}

pub struct FootnoteReference {
  id: usize,
  text: String,
}

pub struct CitationReference {
  id: usize,
  text: String,
}

pub struct SubstitutionReference {
  id: usize,
  text: String,
}

pub struct TitleReference {
  id: usize,
  text: String,
}

pub struct Abbreviation {
  id: usize,
  text: String,
}

pub struct Acronym {
  id: usize,
  text: String,
}

pub struct SuperScript {
  id: usize,
  text: String,
}

pub struct SubScript {
  id: usize,
  text: String,
}

pub struct Math {
  id: usize,
  text: String,
}

pub struct Image {
  id: usize,
  text: String,
}

pub struct Inline {
  id: usize,
  text: String,
}

pub struct Problematic {
  id: usize,
  text: String,
}
pub struct Generated {
  id: usize,
  text: String,
}

