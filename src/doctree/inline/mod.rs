/// This module contains `Inline` level node specifications.

use super::*;


pub struct Text {
  
}

pub struct Emphasis {
  text: String,

}

pub struct StrongEmphasis {
  text: String,
}

pub struct Literal {
  text: String,
}

pub struct Reference {
  text: String,
}

pub struct FootnoteReference {
  text: String,
}

pub struct CitationReference {
  text: String,
}

pub struct SubstitutionReference {
  text: String,
}

pub struct TitleReference {
  text: String,
}

pub struct Abbreviation {
  text: String,
}

pub struct Acronym {
  text: String,
}

pub struct SuperScript {
  text: String,
}

pub struct SubScript {
  text: String,
}

pub struct Math {
  text: String,
}

pub struct Image {
  text: String,
}

pub struct Inline {
  text: String,
}

pub struct Problematic {
  text: String,
}
pub struct Generated {
  text: String,
}

