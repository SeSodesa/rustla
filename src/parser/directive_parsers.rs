/// ## directive_parsers
/// 
/// A submodule that contains functions dedicated to
/// parsing *directives*, reStructuredText extensions.
/// 
/// Author: Santtu Söderholm
/// email: santtu.soderholm@tuni.fi

use super::*;
use crate::doctree::directives::DirectiveNode;

impl Parser {

  /// ### COMMON_OPTIONS
  /// All directives support these options, even if they might override them.
  const COMMON_OPTIONS: &'static [&'static str] = &["name", "class"];


  pub fn parse_standard_admonition (src_lines: &Vec<String>, base_indent: usize, mut section_level: usize, directive_marker_line_indent: usize, mut doctree: DocTree, line_cursor: &mut LineCursor, admonition_type: &str, empty_after_marker: bool) -> TransitionResult {

    use crate::doctree::directives::AdmonitionDirective;

    let variant: AdmonitionDirective = match admonition_type {

      "attention" => AdmonitionDirective::Attention,
      "caution"   => AdmonitionDirective::Caution,
      "danger"    => AdmonitionDirective::Danger,
      "error"     => AdmonitionDirective::Error,
      "hint"      => AdmonitionDirective::Hint,
      "important" => AdmonitionDirective::Important,
      "note"      => AdmonitionDirective::Note,
      "tip"       => AdmonitionDirective::Tip,
      "warning"   => AdmonitionDirective::Warning,
      _           => unreachable!("No standard admonition type \"{}\" on line {}. Computer says no...", admonition_type, line_cursor.sum_total())
    };

    // Fetch content indentation and option|content offset from directive marker line
    let (content_indent, content_offset) = match Self::indent_on_subsequent_lines(src_lines, line_cursor.relative_offset() + 1) {
      Some( (indent, offset ) ) => (indent, offset),
      None => (directive_marker_line_indent, 0)
    };

    *line_cursor.relative_offset_mut_ref() += 1; // jump to next line

    let (first_block_lines, first_line_indent) = if empty_after_marker {

      // Jump to next contiguous block of text and read it

      match Parser::read_text_block(src_lines, line_cursor.relative_offset(), true, false, Some(content_indent)) {
        Ok((lines, _)) => (lines, content_indent),
        Err(e) => panic!("{}", e)
      }
    } else {

      // Read the indented block of text starting on the same line as the directive marker

      match Parser::read_indented_block(src_lines, Some(line_cursor.relative_offset()), Some(true), Some(false), Some(content_indent), Some(directive_marker_line_indent), false) {
        Ok((lines, _, offset, _)) => (lines, directive_marker_line_indent),
        Err(e) => panic!("{}", e)
      }
    };

    let directive_options = Self::scan_directive_options(src_lines, line_cursor, content_indent);

    let (classes, name) = if let Some(mut options) = directive_options {
      if !Self::all_options_recognized(&options, Self::COMMON_OPTIONS) {
        eprintln!("Admonition on line {} received unknown options.\nIgnoring those...\n", line_cursor.sum_total())
      }
      let classes = options.remove("class");
      let name = options.remove("name");

      (classes, name)
    } else {
      (None, None)
    };

    let admonition_data = DirectiveNode::Admonition {
      content_indent: content_indent,
      classes: classes,
      name: name,
      variant: variant
    };

    doctree = doctree.push_data_and_focus(TreeNodeType::Directive(admonition_data));



    let (doctree, offset, state_stack) = match Parser::parse_first_node_block(doctree, src_lines, &base_indent, line_cursor, content_indent, Some(first_line_indent), StateMachine::Admonition, &mut section_level) {
      Some((doctree, nested_parse_offset, state_stack)) => (doctree, nested_parse_offset, state_stack),
      None => return TransitionResult::Failure {message: format!("Looks like {} admonition on line {:#?} has no content.\nComputer says no...\n", admonition_type, line_cursor.sum_total())}
    };
   
    TransitionResult::Success {
      doctree: doctree,
      next_states: Some(state_stack),
      push_or_pop: PushOrPop::Push,
      line_advance: LineAdvance::Some(offset) // Jump over empty line separating options from contents
    }
  }


  /// ### parse_generic admonition
  /// 
  /// Much like `parse_standard_admonition`, except
  /// 1. first checks that the admonition contains an argument,
  /// 2. then checks for possible options and
  /// 3. focuses on the admonition itself.
  pub fn parse_generic_admonition (src_lines: &Vec<String>, mut doctree: DocTree, line_cursor: &mut LineCursor, empty_after_marker: bool, first_indent: Option<usize>) -> TransitionResult {

    // Fetch content indentation and option|content offset from directive marker line
    let (content_indent, content_offset) = match Self::indent_on_subsequent_lines(src_lines, line_cursor.relative_offset() + 1) {
      Some( (indent, offset ) ) => (indent, offset),
      None => panic!("Admonition on line {} could not be scanned for body indentation. Computer says no...", line_cursor.sum_total())
    };

    let argument = if let Some(arg) = Self::scan_directive_arguments(src_lines, line_cursor, first_indent, empty_after_marker) {
      arg
    } else {
      panic!("General admonition on line {} does not contain a compulsory title argument. Computer says no...", line_cursor.sum_total())
    };

    let directive_options = Self::scan_directive_options(src_lines, line_cursor, content_indent);

    let (classes, name) = if let Some(mut options) = directive_options {
      if !Self::all_options_recognized(&options, Self::COMMON_OPTIONS) {
        eprintln!("Admonition on line {} received unknown options.\nIgnoring those...\n", line_cursor.sum_total())
      }
      let classes = options.remove("class");
      let name = options.remove("name");

      (classes, name)
    } else {
      (None, None)
    };

    let admonition_data = DirectiveNode::Admonition {
      content_indent: content_indent,
      classes: classes,
      name: name,
      variant: doctree::directives::AdmonitionDirective::Admonition {
        title: argument
      }
    };

    doctree = doctree.push_data_and_focus(TreeNodeType::Directive(admonition_data));
    
    TransitionResult::Success {
      doctree: doctree,
      next_states: Some(vec![StateMachine::Admonition]),
      push_or_pop: PushOrPop::Push,
      line_advance: LineAdvance::None
    }
  }


  pub fn parse_image (src_lines: &Vec<String>, mut doctree: DocTree, line_cursor: &mut LineCursor, empty_after_marker: bool, first_indent: Option<usize>) -> TransitionResult {

    use crate::doctree::directives::ImageDirective;

    // Fetch content indentation and option|content offset from directive marker line
    let (content_indent, content_offset) = match Self::indent_on_subsequent_lines(src_lines, line_cursor.relative_offset() + 1) {
      Some( (indent, offset ) ) => (indent, offset),
      None => panic!("Image on line {} could not be scanned for body indentation. Computer says no...", line_cursor.sum_total())
    };

    let argument = if let Some(arg) = Self::scan_directive_arguments(src_lines, line_cursor, first_indent, empty_after_marker) {
      arg
    } else {
      panic!("Image on line {} does not contain a compulsory image URI. Computer says no...", line_cursor.sum_total())
    };

    let directive_options = Self::scan_directive_options(src_lines, line_cursor, content_indent);

    let (alt, height, width, scale, align, target, classes, name) = if let Some(mut options) = directive_options {
      if !Self::all_options_recognized(&options, &["alt","height", "width", "scale", "align", "target", "class", "name"]) {
        eprintln!("Image on line {} received unknown options.\nIgnoring those...\n", line_cursor.sum_total())
      }

      let alt = options.remove("class");
      let height = options.remove("height");
      let width = options.remove("width");
      let scale = options.remove("scale");
      let align = options.remove("align");
      let target = options.remove("target");
      let classes = options.remove("class");
      let name = options.remove("name");

      (alt, height, width, scale, align, target, classes, name)
    } else {
      (None, None, None, None, None, None, None, None)
    };

    let image_data = TreeNodeType::Directive (
      DirectiveNode::Image (
        ImageDirective::Image {
          alt:    alt,
          height: height,
          width:  width,
          scale:  scale,
          align:  align,
          target: target,
          name:   name,
          class:  classes,
        }
      )
    );

    doctree = doctree.push_data(image_data);

    TransitionResult::Success {
      doctree: doctree,
      next_states: None,
      push_or_pop: PushOrPop::Neither,
      line_advance: LineAdvance::None
    }
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
  fn indent_on_subsequent_lines (src_lines: &Vec<String>, start_line: usize) -> Option<(usize, usize)> {

    let mut current_line = start_line;
    loop {
      if let Some(line) = src_lines.get(current_line) {
        if line.trim().is_empty() {
          current_line += 1;
          continue
        } else {
          break Some(
            (line.chars().take_while(|c| c.is_whitespace()).count(), current_line - start_line)
          )
        }
      } else {
        break None
      }
    }
  }


  /// ### scan_directive_arguments
  /// 
  /// Reads the first block of text of a directive,
  /// until an empty line or something like a list of options
  /// (recognized by the automaton `FIELD_MAKRER_RE`) is encountered.
  /// Combines the lines into a single string and `Option`ally returns it.
  /// If no arguments are found, returns `None`.
  /// 
  /// In case the directive starts on the same line as the directive marker,
  /// allows specifying first and block indents separately.
  /// `first_indent` (on the first line) or `block_indent` are ignored on each line.
  fn scan_directive_arguments (src_lines: &Vec<String>, line_cursor: &mut LineCursor, first_indent: Option<usize>, empty_after_marker: bool) -> Option<String> {

    use crate::parser::state_machine::FIELD_MARKER_RE;

    // The vector containing references to the argument lines.
    let mut argument_lines: Vec<String> = Vec::new();
    let mut on_marker_line = true;

    // Jump to next line if line after directive marker is empty
    if empty_after_marker {
      *line_cursor.relative_offset_mut_ref() += 1;
      on_marker_line = false;
    }

    while let Some(line) = src_lines.get(line_cursor.relative_offset()) {

      // Each collect allocates, but what the heck, it works.
      let line_without_indent: String = if on_marker_line {
        match first_indent {
          Some(indent) => {
            on_marker_line = false;
            line.chars().skip(indent).collect()
          }
          _ => panic!("On directive marker line {} but couldn't skip the marker to parse line contents. Computer says no...", line_cursor.sum_total())
        }
      } else {
        line.chars().skip_while(|c| c.is_whitespace()).collect()
      };

      eprintln!("Line: {:#?}\n", line_without_indent);
      eprintln!("Field marker matches: {}\n", FIELD_MARKER_RE.is_match(line_without_indent.as_str()));

      if line_without_indent.as_str().trim().is_empty() || FIELD_MARKER_RE.is_match(line_without_indent.as_str()) {
        break
      }

      argument_lines.push(line_without_indent);
      *line_cursor.relative_offset_mut_ref() += 1;
    };

    if argument_lines.is_empty() { None } else { Some(argument_lines.join(" ")) }
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
  fn scan_directive_options (src_lines: &Vec<String>, line_cursor: &mut LineCursor, body_indent: usize) -> Option<HashMap<String, String>>{

    use crate::parser::state_machine::FIELD_MARKER_RE;

    let mut option_map: HashMap<String, String> = HashMap::new();

    let mut ended_with_blank: bool = false;

    while let Some(line) = src_lines.get(line_cursor.relative_offset()) {

      eprintln!("Line: {:#?}", line);

      if line.trim().is_empty() { ended_with_blank = true; eprintln!("Ended with blank: {}\n", ended_with_blank); break } // End of option list

      if let Some(captures) = FIELD_MARKER_RE.captures(line) {
        let line_indent = captures.get(1).unwrap().as_str().chars().count();
        if line_indent != body_indent {  break } // Option lists need to be aligned
        let option_key = captures.get(2).unwrap().as_str().trim();

        let option_val_indent = captures.get(0).unwrap().as_str().chars().count();
        let index = match line.char_indices().nth(option_val_indent) {
          Some((index, _)) => index,
          None => panic!("Looks like a directive option might not have a value on line {}...", line_cursor.sum_total())
        };
        let option_val = line[index..].trim();

        if let Some(val) = option_map.insert(option_key.to_string(), option_val.to_string()) {
          eprintln!("Duplicate directive option on line {}\n", line_cursor.sum_total())
        }
      } else {
        ended_with_blank = false;
        break // Found a line not conforming to field list item syntax
      }
      *line_cursor.relative_offset_mut_ref() += 1;
    }

    if option_map.is_empty() { None } else {
      if ended_with_blank { *line_cursor.relative_offset_mut_ref() += 1 }
      Some(option_map)
    }
  }


  fn all_options_recognized (option_map: &HashMap<String, String>, recognized_keys: &[&str]) -> bool {

    let mut option_iter = option_map.keys();
    let mut recognized_iter = recognized_keys.iter();

    // All option keys should be found in recognized keys
    option_iter.all( |option_key| recognized_iter.any(|recognized_key| option_key == recognized_key) )
  }
}

use std::collections::HashMap;
