/// This module contains sructural level node specifications.

use super::*;

pub struct Section {
  id: usize,
  children: Vec<DocNode>,
}


pub struct Topic {
  id: usize,
  children: Vec<DocNode>,
}

pub struct Transition {
  id: usize,
  children: Vec<DocNode>,
}
