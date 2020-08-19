/// ## directive_parsers
/// 
/// A submodule that contains functions dedicated to
/// parsing *directives*, reStructuredText extensions.
/// 
/// Author: Santtu Söderholm
/// email: santtu.soderholm@tuni.fi

use super::*;

impl Parser {

  /// ### COMMON_OPTIONS
  /// All directives support these options, even if they might override them.
  const COMMON_OPTIONS: &'static [&'static str] = &["name", "class"];


  pub fn parse_standard_admonition (src_lines: &Vec<String>, base_indent: usize,  mut section_level: usize, directive_marker_line_indent: usize, doctree: DocTree, line_cursor: &mut LineCursor, admonition_type: &str) -> TransitionResult {

    let content_indent = match Self::indent_on_subsequent_lines(src_lines, line_cursor.relative_offset() + 1) {
      Some(indent) => indent,
      None => directive_marker_line_indent
    };


    match admonition_type {

      "attention" => {

      }
      "caution" => {

      }
      "danger" => {

      }
      "error" => {

      }
      "hint" => {

      }
      "important" => {

      }
      "note" => {

      }
      "tip" => {

      }
      "warning" => {

      }
      _ => unreachable!()
    }

    let (doctree, offset, state_stack) = match Parser::parse_first_node_block(doctree, src_lines, &base_indent, line_cursor, directive_marker_line_indent, None, StateMachine::ListItem, &mut section_level) {
      Some((doctree, nested_parse_offset, state_stack)) => (doctree, nested_parse_offset, state_stack),
      None => return TransitionResult::Failure {message: format!("Could not parse the first block {} list item on line {:#?}", admonition_type, line_cursor.sum_total())}
    };
    todo!()
  }


  pub fn parse_generic_admonition () {
    todo!()
  }


  pub fn parse_image () {
    todo!()
  }


  pub fn parse_figure () {
    todo!()
  }


  pub fn parse_topic () {
    todo!()
  }


  pub fn parse_sidebar () {
    todo!()
  }


  pub fn parse_line_block () {
    todo!()
  }


  pub fn parse_parsed_literal () {
    todo!()
  }


  pub fn parse_code () {
    todo!()
  }


  pub fn parse_math () {
    todo!()
  }


  pub fn parse_rubric () {
    todo!()
  }


  pub fn parse_epigraph () {
    todo!()
  }


  pub fn parse_highlights () {
    todo!()
  }


  pub fn parse_pull_quote () {
    todo!()
  }


  pub fn parse_compound () {
    todo!()
  }


  pub fn parse_container () {

  }


  pub fn parse_table () {
    todo!()
  }


  pub fn parse_csv_table () {
    todo!()
  }


  pub fn parse_list_table () {
    todo!()
  }


  pub fn parse_contents () {
    todo!()
  }


  pub fn parse_section_numbering () {
    todo!()
  }


  pub fn parse_header_or_footer () {
    todo!()
  }


  pub fn parse_target_notes () {
    todo!()
  }


  pub fn parse_footnotes () {
    todo!()
  }


  pub fn parse_citations () {
    todo!()
  }


  pub  fn parse_meta () {
    todo!()
  }


  pub fn parse_include () {
    todo!()
  }


  pub fn parse_raw () {
    todo!()
  }


  pub fn parse_class () {
    todo!()
  }


  pub fn parse_role () {
    todo!()
  }


  pub fn parse_default_role () {
    todo!()
  }


  pub fn parse_title () {
    todo!()
  }


  pub fn restucturetext_test_directive () {
    todo!()
  }


  pub fn parse_aplus_questionnaire () {
    todo!()
  }


  pub fn parse_aplus_submit () {
    todo!()
  }


  pub fn parse_aplus_toctree () {
    todo!()
  }


  pub fn parse_aplus_active_element_input () {
    todo!()
  }


  pub fn parse_apkus_active_element_output () {
    todo!()
  }


  pub fn parse_aplus_hidden_block () {
    todo!()
  }


  pub fn parse_aplus_point_of_interest () {
    todo!()
  }


  pub fn parse_aplus_annotated () {
    todo!()
  }


  pub fn parse_aplus_lineref_codeblock () {
    todo!()
  }


  pub fn parse_aplus_repl_res_count_reset () {
    todo!()
  }


  pub fn parse_aplus_acos_submit () {
    todo!()
  }


  pub fn parse_aplus_div () {
    todo!()
  }


  pub fn parse_aplus_styled_topic () {
    todo!()
  }


  pub fn parse_aplus_story () {
    todo!()
  }


  pub fn parse_aplus_jsvee () {
    todo!()
  }


  pub fn parse_aplus_youtube () {
    todo!()
  }


  pub fn parse_aplus_local_video () {
    todo!()
  }


  pub fn parse_aplus_embedded_page () {
    todo!()
  }


  // ---------
  //  HELPERS
  // ---------

  /// ### indent_on_subsequent_lines
  /// Scans the source lines until it finds a non-empty line and returns the `Option`al indent of it.
  fn indent_on_subsequent_lines (src_lines: &Vec<String>, start_line: usize) -> Option<usize> {

    let mut current_line = start_line;
    loop {
      if let Some(line) = src_lines.get(current_line) {
        if line.trim().is_empty() {
          current_line += 1;
          continue
        } else {
          break Some(line.chars().take_while(|c| c.is_whitespace()).count())
        }
      } else {
        break None
      }
    }
  }


  /// ### scan_directive_options
  /// Scans the lines following the directive marker for something resembling a field list,
  /// and attempts to scan the contents of the list into an `Option`al `HashMap` of directive
  /// option names and values. The calling directive parser will handle their validation,
  /// as different directives have different options available to them.
  /// 
  /// An empty line separates directive options from the directive content, so encountering one
  /// will terminate the scan. This means that the options have to start of the line following
  /// the directive marker.
  fn scan_directive_options (src_lines: Vec<String>, line_cursor: &mut LineCursor, body_indent: usize) -> Option<HashMap<String, String>>{

    use crate::parser::state_machine::FIELD_MARKER_RE;

    let mut current_line = line_cursor.relative_offset() + 1;

    let mut option_map: HashMap<String, String> = HashMap::new();

    while let Some(line) = src_lines.get(current_line) {

      if line.trim().is_empty() { break } // End of option list

      if let Some(captures) = FIELD_MARKER_RE.captures(line) {
        let line_indent = captures.get(1).unwrap().as_str().chars().count();
        if line_indent < body_indent { panic!("Found a directive option list item with too little indent on line {}. Computer says no...", line_cursor.sum_total()) } // panic for now
        let option_key = captures.get(2).unwrap().as_str().trim();

        let option_val_indent = captures.get(0).unwrap().as_str().chars().count();
        let option_val = line.chars().skip(option_val_indent).collect::<String>().as_str().trim().to_string(); // Allocations galore...

        if let Some(val) = option_map.insert(option_key.to_string(), option_val) {
          eprintln!("Duplicate directive option on line {}\n", line_cursor.sum_total() + current_line)
        }
      } else {
        break // Found a line not conforming to field list item syntax 
      }
      current_line += 1;
    }

    if option_map.is_empty() { None } else { Some(option_map) }
  }
}

use std::collections::HashMap;
