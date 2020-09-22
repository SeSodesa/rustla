/// ## aplus_questionnaire
///
/// A submodule dedicated to parsing functions of the `StateMachine::AplusQuestionnaire` state.
///
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;


/// ### aplus_questionnaire_text
///
/// A function for reading in blocks of intermediate text (rST paragraphs) between questionnaire questions.
pub fn aplus_questionnaire_text (src_lines: &Vec<String>, base_indent: usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult  {
  todo!()
}

/// ### aplus_pick_one
///
/// A `pick-one` type questionnaire question parser.
pub fn aplus_pick_one (src_lines: &Vec<String>, base_indent: usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult  {
  todo!()
}


/// ### aplus_pick_any
///
/// A `pick-any` type questionnaire question parser.
pub fn aplus_pick_any (src_lines: &Vec<String>, base_indent: usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult  {
  todo!()
}


/// ### aplus_freetext
///
/// A `freetext` type questionnaire question parser.
pub fn aplus_freetext (src_lines: &Vec<String>, base_indent: usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, pattern_name: &PatternName) -> TransitionResult  {
  todo!()
}