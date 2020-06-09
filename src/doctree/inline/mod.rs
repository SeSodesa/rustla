/// This module contains `Inline` level node specifications.

use super::*;

#[derive(Debug)]
pub struct Text {
  id: usize,
  text: String
}

#[derive(Debug)]
pub struct Emphasis {
  id: usize,
  text: String,

}

#[derive(Debug)]
pub struct StrongEmphasis {
  id: usize,
  text: String,
}

#[derive(Debug)]
pub struct Literal {
  id: usize,
  text: String,
}

#[derive(Debug)]
pub struct Reference {
  id: usize,
  text: String,
}

#[derive(Debug)]
pub struct FootnoteReference {
  id: usize,
  text: String,
}

#[derive(Debug)]
pub struct CitationReference {
  id: usize,
  text: String,
}

#[derive(Debug)]
pub struct SubstitutionReference {
  id: usize,
  text: String,
}

#[derive(Debug)]
pub struct TitleReference {
  id: usize,
  text: String,
}

#[derive(Debug)]
pub struct Abbreviation {
  id: usize,
  text: String,
}

#[derive(Debug)]
pub struct Acronym {
  id: usize,
  text: String,
}

#[derive(Debug)]
pub struct SuperScript {
  id: usize,
  text: String,
}

#[derive(Debug)]
pub struct SubScript {
  id: usize,
  text: String,
}

#[derive(Debug)]
pub struct Math {
  id: usize,
  text: String,
}

#[derive(Debug)]
pub struct Image {
  id: usize,
  text: String,
}

#[derive(Debug)]
pub struct Inline {
  id: usize,
  text: String,
}

#[derive(Debug)]
pub struct Problematic {
  id: usize,
  text: String,
}

#[derive(Debug)]
pub struct Generated {
  id: usize,
  text: String,
}

