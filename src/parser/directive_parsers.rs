/// ## directive_parsers
/// 
/// A submodule that contains functions dedicated to
/// parsing *directives*, reStructuredText extensions.
/// 
/// Author: Santtu Söderholm
/// email: santtu.soderholm@tuni.fi

use std::collections::HashMap;

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

    let admonition_data = TreeNodeType::Admonition {
      content_indent: content_indent,
      classes: classes,
      name: name,
      variant: variant
    };

    doctree = doctree.push_data_and_focus(admonition_data);



    let (doctree, offset, state_stack) = match Parser::parse_first_node_block(doctree, src_lines, &base_indent, line_cursor, content_indent, Some(first_line_indent), StateMachine::Admonition, &mut section_level, false) {
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
        eprintln!("Admonition preceding line {} received unknown options.\nIgnoring those...\n", line_cursor.sum_total())
      }
      let classes = options.remove("class");
      let name = options.remove("name");

      (classes, name)
    } else {
      (None, None)
    };

    let admonition_data = TreeNodeType::Admonition {
      content_indent: content_indent,
      classes: classes,
      name: name,
      variant: doctree::directives::AdmonitionDirective::Admonition {
        title: argument
      }
    };

    doctree = doctree.push_data_and_focus(admonition_data);
    
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
        eprintln!("Image preceding line {} received unknown options.\nIgnoring those...\n", line_cursor.sum_total())
      }

      let alt = options.remove("alt");
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

    let image_data = TreeNodeType::Image {
      uri: argument,
      alt:    alt,
      height: height,
      width:  width,
      scale:  scale,
      align:  align,
      target: target,
      name:   name,
      class:  classes,
    };

    doctree = doctree.push_data(image_data);

    TransitionResult::Success {
      doctree: doctree,
      next_states: None,
      push_or_pop: PushOrPop::Neither,
      line_advance: LineAdvance::None
    }
  }


  pub fn parse_figure (src_lines: &Vec<String>, mut doctree: DocTree, line_cursor: &mut LineCursor, base_indent: usize, empty_after_marker: bool, first_indent: Option<usize>, section_level: usize) -> TransitionResult {

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

    let (alt, height, width, scale, align, target, classes, name, figwidth, figclass) = if let Some(mut options) = directive_options {
      if !Self::all_options_recognized(&options, &["alt","height", "width", "scale", "align", "target", "class", "name", "figwidth", "figclass"]) {
        eprintln!("Image preceding line {} received unknown options.\nIgnoring those...\n", line_cursor.sum_total())
      }

      let alt = options.remove("alt");
      let height = options.remove("height");
      let width = options.remove("width");
      let scale = options.remove("scale");
      let align = options.remove("align");
      let target = options.remove("target");
      let classes = options.remove("class");
      let name = options.remove("name");
      let figwidth = options.remove("figwidth");
      let figclass = options.remove("figclass");

      (alt, height, width, scale, align, target, classes, name, figwidth, figclass)
    } else {
      (None, None, None, None, None, None, None, None, None, None)
    };

    // Construct the contained image
    let image = TreeNodeType::Image {
      uri: argument,

      alt: alt,
      height:height,
      width: width,
      scale: scale,
      align: None, // Image does not have alignenment inside a figure.
      target: target,
      class: classes,
      name: name,
    };

    let figure = TreeNodeType::Figure {
      body_indent: content_indent,
      align: align,
      figclass: figclass,
      figwidth: figwidth
    };

    // Add figure node to tree and focus on it
    doctree = doctree.push_data_and_focus(figure);

    // Add image to figure
    doctree = doctree.push_data(image);


    // Transition to figure state to scan for a possible caption
    // (a simple paragraph). If an empty comment is encountered,
    // interpret it as a missing caption and move on to
    // parsing figure legend contents.

    let (lines, offset) = if let Ok((lines, _, offset, _)) = Parser::read_indented_block(src_lines, Some(line_cursor.relative_offset()), Some(false), Some(true), Some(content_indent), None, false) {
      (lines, offset)
    } else {
      panic!("Could not read the legend contents of the figure on line {}. Computer says no...", line_cursor.sum_total())
    };

    let current_node_id = doctree.current_node_id();

    let (mut doctree, nested_state_stack) = match Parser::new(lines, doctree, Some(content_indent), line_cursor.sum_total(), Some(StateMachine::Figure), section_level).parse() {
      ParsingResult::EOF { doctree, state_stack } => (doctree, state_stack),
      ParsingResult::EmptyStateStack { doctree, state_stack } => (doctree, state_stack),
      ParsingResult::Failure { message } => panic!("{}", message)
    };

    use common::TraversalType;

    // Ensure we are focused on the figure that started the above nested parsing session.
    // This might still be buggy in situations where a figure was constructed inside of a figure.
    // Might need to be TreeNodeType AND ID-based instead.
    // doctree = doctree.walk(TraversalType::ID(current_node_id));

    // if let TreeNodeType::Figure { .. } = doctree.shared_data() {  } else { panic!("Not focused on parent figure after nested parsing session. Computer says no...") };

    let first_child_data = doctree.mut_child(1).mut_data();

    // if let TreeNodeType::Paragraph { indent } = first_child_data {
    //   *first_child_data = TreeNodeType::Caption { indent: *indent }
    // }

    TransitionResult::Success {
      doctree: doctree,
      next_states: Some(nested_state_stack),
      push_or_pop: PushOrPop::Push,
      line_advance: LineAdvance::Some(offset),
    }
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


  /// ### parse_code
  /// 
  /// The "code" directive parser.
  pub fn parse_code (src_lines: &Vec<String>, mut doctree: DocTree, line_cursor: &mut LineCursor, base_indent: usize, empty_after_marker: bool, first_indent: Option<usize>, section_level: usize) -> TransitionResult {

    // Fetch content indentation and option|content offset from directive marker line
    let (content_indent, content_offset) = match Self::indent_on_subsequent_lines(src_lines, line_cursor.relative_offset() + 1) {
      Some( (indent, offset ) ) => (indent, offset),
      None => panic!("Image on line {} could not be scanned for body indentation. Computer says no...", line_cursor.sum_total())
    };
    let language = if let Some(arg) = Self::scan_directive_arguments(src_lines, line_cursor, first_indent, empty_after_marker) {
      Some(arg)
    } else {
      None
    };

    let directive_options = Self::scan_directive_options(src_lines, line_cursor, content_indent);

    let (classes, name, number_lines) = if let Some(mut options) = directive_options {
      if !Self::all_options_recognized(&options, &["class", "name", "number-lines"]) {
        eprintln!("Code block preceding line {} received unknown options.\nIgnoring those...\n", line_cursor.sum_total())
      }

      let classes = options.remove("class");
      let name = options.remove("name");
      let number_lines = options.remove("number-lines");

      (classes, name, number_lines)
    } else {
      (None, None, None)
    };

    let (lines, offset) = if let Ok((lines, _, offset, _)) = Parser::read_indented_block(src_lines, Some(line_cursor.relative_offset()), Some(false), Some(true), Some(content_indent), None, false) {
      (lines, offset)
    } else {
      panic!("Could not read the code block on line {}. Computer says no...", line_cursor.sum_total())
    };

    let code_block = TreeNodeType::Code {

      text: lines.join("\n"),

      language: language,
      number_lines: number_lines,
      class: classes,
      name: name
    };

    doctree = doctree.push_data(code_block);

    TransitionResult::Success {
      doctree: doctree,
      next_states: None,
      push_or_pop: PushOrPop::Neither,
      line_advance: LineAdvance::Some(offset)
    }
  }


  /// ### parse_math_block
  /// 
  /// The display math parser. Content blocks separated by a blank lines are put in adjacent math blocks.
  pub fn parse_math_block (src_lines: &Vec<String>, mut doctree: DocTree, line_cursor: &mut LineCursor, base_indent: usize, empty_after_marker: bool, first_indent: usize, section_level: usize) -> TransitionResult {

    let math_after_marker =  if ! empty_after_marker {
      if let Some(line) = src_lines.get(line_cursor.relative_offset()) {
        Some(Parser::line_suffix(line, first_indent))
      } else {
        unreachable!("On line {} with a marker but found no line?", line_cursor.sum_total())
      }
    } else {
      None
    };

    // Fetch content indentation and option|content offset from directive marker line
    let (content_indent, content_offset) = match Self::indent_on_subsequent_lines(src_lines, line_cursor.relative_offset() + 1) {
      Some( (indent, offset ) ) => (indent, offset),
      None => panic!("Math block on line {} could not be scanned for body indentation. Computer says no...", line_cursor.sum_total())
    };

    line_cursor.increment_by(content_offset + 1);

    let directive_options = Self::scan_directive_options(src_lines, line_cursor, content_indent);

    let (classes, name) = if let Some(mut options) = directive_options {
      if !Self::all_options_recognized(&options, &["class", "name"]) {
        eprintln!("Math block preceding line {} received unknown options.\nIgnoring those...\n", line_cursor.sum_total())
      }

      let classes = options.remove("class");
      let name = options.remove("name");

      (classes, name)
    } else {
      (None, None)
    };

    if let Some(math) = math_after_marker {
      doctree = doctree.push_data(TreeNodeType::MathBlock { block_text: math, class: classes, name: name });
      return TransitionResult::Success {
        doctree: doctree,
        next_states: None,
        push_or_pop: PushOrPop::Neither,
        line_advance: LineAdvance::None
      }
    }

    let (lines, offset) = if let Ok((lines, _, offset, _)) = Parser::read_indented_block(src_lines, Some(line_cursor.relative_offset()), Some(false), Some(true), Some(content_indent), None, true) {
      (lines, offset)
    } else {
      panic!("Could not read the math block on line {}. Computer says no...", line_cursor.sum_total())
    };

    // Scan lines for blocks separated by blank lines
    let blocks = {

      let mut blocks = Vec::new();
      let mut block = String::new();

      eprintln!("{:#?}", lines);

      for line in lines.iter() {

        if line.trim().is_empty() && !block.trim().is_empty() {
          blocks.push(block); block = String::new()
        } else if line.trim().is_empty() && block.trim().is_empty() {
          continue
        }

        block = block + "\n" + line;
      }

      blocks
    };

    if blocks.is_empty() { panic!("Tried reading a math block on line {} but didn't find any actual content. Computer says no...", line_cursor.sum_total()) }

    for block in blocks {
      doctree = doctree.push_data(TreeNodeType::MathBlock { block_text: block.trim().to_string(), name: name.clone(), class: classes.clone() })
    }
    
    TransitionResult::Success {
      doctree: doctree,
      next_states: None,
      push_or_pop: PushOrPop::Neither,
      line_advance: LineAdvance::Some(offset)
    }
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


// ========================
//  Sphinx-specific directives
// ========================

  pub fn parse_sphinx_toctree () {
    todo!()
  }

  pub fn parse_sphinx_versionadded () {
    todo!()
  }

  pub fn parse_sphinx_versionchanged () {
    todo!()
  }

  pub fn parse_sphinx_deprecated () {
    todo!()
  }

  pub fn parse_sphinx_seealso () {
    todo!()
  }

  pub fn parse_sphinx_centered () {
    todo!()
  }

  pub fn parse_sphinx_hlist () {
    todo!()
  }

  pub fn parse_sphinx_highlight () {
    todo!()
  }

  pub fn parse_sphinx_code_block () {
    todo!()
  }

  pub fn parse_sphinx_literalinclude () {
    todo!()
  }

  pub fn parse_sphinx_glossary () {
    todo!()
  }

  pub fn parse_sphinx_sectionauthor () {
    todo!()
  }

  pub fn parse_sphinx_codeauthor () {
    todo!()
  }

  pub fn parse_sphinx_index () {
    todo!()
  }

  pub fn parse_sphinx_only () {
    todo!()
  }

  pub fn parse_sphinx_tabularcolumns () {
    todo!()
  }

  pub fn parse_sphinx_math_block () {
    todo!()
  }

  pub fn parse_sphinx_productionlist () {
    todo!()
  }


// ========================
//  A+-specific directives
// ========================

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


  pub fn parse_aplus_active_element_output () {
    todo!()
  }


  pub fn parse_aplus_hidden_block () {
    todo!()
  }


  pub fn parse_aplus_point_of_interest (src_lines: &Vec<String>, mut doctree: DocTree, line_cursor: &mut LineCursor, base_indent: usize, empty_after_marker: bool, first_indent: usize, body_indent: usize, section_level: usize) -> TransitionResult {

    const RECOGNIZED_OPTIONS: &[&str] = &[
      "id", "previous", "next", "hidden", "class", "height",
      "columns", "bgimg", "not_in_slides", "not_in_book", "no_poi_box"
    ];

    let title = Parser::scan_directive_arguments(src_lines, line_cursor, Some(first_indent), empty_after_marker);
    let options = Parser::scan_directive_options(src_lines, line_cursor, body_indent);

    // Read recognized options
    let (id, previous, next, hidden, class, height, columns, bgimg, not_in_slides, not_in_book, no_poi_box) = if let Some(mut options) = options {
      if !Self::all_options_recognized(&options, RECOGNIZED_OPTIONS) {
        eprintln!("A+ point of interest block preceding line {} received unknown options.\nIgnoring those...\n", line_cursor.sum_total())
      }

      let id = options.remove("id");
      let previous = options.remove("previous");
      let next = options.remove("next");
      let hidden = options.remove("hidden");
      let class = options.remove("class");
      let height = options.remove("height");
      let columns = options.remove("columns");
      let bgimg = options.remove("bgimg");
      let not_in_slides = options.remove("not_in_slides");
      let not_in_book = options.remove("not_in_book");
      let no_poi_box = options.remove("no_poi_box");

      (id, previous, next, hidden, class, height, columns, bgimg, not_in_slides, not_in_book, no_poi_box)
    } else {
      (None, None, None, None, None, None, None, None, None, None, None)
    };

    let poi_node = TreeNodeType::AplusPOI {
      title: if let Some(title) = title { title } else { "".to_string() },
      body_indent: body_indent,

      id: id,
      previous: previous,
      next: next,
      hidden: hidden,
      class: class,
      height: height,
      columns: columns,
      bgimg: bgimg,
      not_in_slides: not_in_slides,
      not_in_book: not_in_book,
      no_poi_box: no_poi_box
    };

    doctree = doctree.push_data(poi_node);

    TransitionResult::Success {
      doctree: doctree,
      next_states: Some(vec![StateMachine::Body]), // PoI contains body nodes
      push_or_pop: PushOrPop::Push,
      line_advance: LineAdvance::None
    }
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


  /// ### parse_unknown_directive
  /// 
  /// Parses unknown directive blocks as literal text.
  pub fn parse_unknown_directive (mut doctree: DocTree, src_lines: &Vec<String>, line_cursor: &LineCursor, first_line_indent: usize, body_indent: usize) -> TransitionResult {

    let (unknown_directive_as_text, offset) = match Parser::read_indented_block(src_lines, Some(line_cursor.relative_offset()), Some(false), Some(true), Some(body_indent), Some(first_line_indent), false) {
      Ok((lines, _, offset, _)) => {
        eprintln!("{:#?}\n", lines);
        (lines.join("\n"), offset)
      },
      Err(message) => panic!("Error when reading an unknown directive as literal text: {}", message)
    };

    let literal_node = TreeNodeType::LiteralBlock { text: unknown_directive_as_text };

    doctree = doctree.push_data(literal_node);

    TransitionResult::Success {
      doctree: doctree,
      next_states: None,
      push_or_pop: PushOrPop::Neither,
      line_advance: LineAdvance::Some(offset)
    }
  }


  // ---------
  //  HELPERS
  // ---------

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
      line_cursor.increment_by(1);
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
        line.chars().skip_while(|c| c.is_whitespace()).collect::<String>().as_str().trim().to_string()
      };

      eprintln!("Line: {:#?}\n", line_without_indent);
      eprintln!("Field marker matches: {}\n", FIELD_MARKER_RE.is_match(line_without_indent.as_str()));

      if line_without_indent.as_str().trim().is_empty() || FIELD_MARKER_RE.is_match(line_without_indent.as_str()) {
        break
      }

      argument_lines.push(line_without_indent);
      line_cursor.increment_by(1);
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

        eprintln!("Option value: {:#?}\n", option_val);

        if let Some(val) = option_map.insert(option_key.to_string(), option_val.to_string()) {
          eprintln!("Duplicate directive option on line {}\n", line_cursor.sum_total())
        }
      } else {
        ended_with_blank = false;
        break // Found a line not conforming to field list item syntax
      }
      line_cursor.increment_by(1);
    }

    if option_map.is_empty() { None } else {
      if ended_with_blank { line_cursor.increment_by(1) }
      Some(option_map)
    }
  }


  /// ### all_options_recognized
  /// 
  /// Checks that a given hashmap only contains recognized option keys,
  /// based on a given `&str` array slice.
  fn all_options_recognized (option_map: &HashMap<String, String>, recognized_keys: &[&str]) -> bool {

    let mut option_iter = option_map.keys();
    let mut recognized_iter = recognized_keys.iter();

    // All option keys should be found in recognized keys
    option_iter.all( |option_key| recognized_iter.any(|recognized_key| option_key == recognized_key) )
  }
}
