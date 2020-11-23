/// ## larst_writer
/// 
/// A submodule that contains the larst writer method of the doctree,
/// and the patterns related to it.
/// 
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use std::io::Write;

use super::*;
use crate::common::AplusExerciseStatus;
// use crate::common::OutputStream;
use crate::rustla_options::ruSTLaOptions;
use crate::rustla_options::OutputStream;

const LATEX_OPTION_DELIM: &str = ",";

impl DocTree {

  /// ### write_to_larst
  /// 
  /// A function that writes a rusTLa doctree into a LarST file.
  /// 
  /// #### TODO
  /// 
  /// Add a return type such as a `Result<String, ()>` that contains the generated object code in a single string.
  /// Alternatively, pass a file pointer around and write (append) to it, returning it at the end if successful.
  pub fn write_to_larst (self, rustla_options: &ruSTLaOptions) {

    match rustla_options.shared_out_stream() {
      OutputStream::StdOut => {
        // Windows users beware. Only valid UTF-8 accepted.
        let mut stdout = std::io::stdout();
        self.tree.write_to_larst_stdout(&mut stdout);
      }
      OutputStream::File => {
        use std::fs::{File, OpenOptions};

        // Cannot write to file without knowing the
        if self.file_folder.is_empty() || self.filename_stem.is_empty() {
          panic!("Cannot write to file without knowing the location. Computer says no...")
        }

        const TEX_FILE_SUFFIX: &str = ".tex";
        const APLUS_CLASS_FILE_NAME: &str = "aplus.cls";

        let folder = &self.file_folder;
        let mut object_file_path = PathBuf::from(folder);
        let mut aplus_class_file_path = PathBuf::from(folder);
        object_file_path.push(self.filename_stem + TEX_FILE_SUFFIX);
        aplus_class_file_path.push(APLUS_CLASS_FILE_NAME);

        eprintln!("Writing output to {:#?}...", object_file_path);

        // TODO: Add check for file existence...
        let mut object_file: File = match OpenOptions::new().write(true).truncate(true).create(true).open(object_file_path) {
          Ok(file) => file,
          Err(e) => panic!("Could not open LarST file for writing purposes: {}", e)
        };

        // If object file generation was successful, generate A+ class file
        let mut aplus_class_file: File = match OpenOptions::new().write(true).truncate(true).create(true).open(aplus_class_file_path) {
          Ok(file) => file,
          Err(e) => panic!("Could not open A+ class file for writing purposes: {}", e)
        };

        match aplus_class_file.write(aplus_cls_contents().as_bytes()){
          Ok(_) => {},
          Err(_) => panic!("Could not write to A+ class file after generating object code. Computer says no...")
        };

        self.tree.write_to_larst_file(&mut object_file)
      }
    }
  }
}


impl TreeZipper {

  /// This is the actual recursive function that goes over the tree zipper and writes each node
  /// into its LarST string representation based on its `TreeNodeType`.
  /// Starts out by calling `TreeNodeType`-specific pre-order action,
  /// then recursively calls itself for the children of the node and
  /// finishes by calling a post-order action on `self`.
  fn write_to_larst_file (mut self, file_ptr: &mut std::fs::File) {

    self = self.walk_to_root(); // Start out by walking to root.

    self.shared_node().larst_pre_order_file_write(file_ptr);

    if let Some(children) = self.shared_node().shared_children() {
      for child in children {
        child.write_to_larst_file(file_ptr);
      }
    }

    self.shared_node().larst_post_order_file_write(file_ptr);
  }


  /// This is the actual recursive function that goes over the tree zipper and writes each node
  /// into its LarST string representation based on its `TreeNodeType`.
  /// Starts out by calling `TreeNodeType`-specific pre-order action,
  /// then recursively calls itself for the children of the node and
  /// finishes by calling a post-order action on `self`.
  fn write_to_larst_stdout (mut self, stdout: &mut std::io::Stdout) {

    self = self.walk_to_root(); // Start out by walking to root.

    self.shared_node().larst_pre_order_stdout_write(stdout);

    if let Some(children) = self.shared_node().shared_children() {
      for child in children {
        child.write_to_larst_stdout(stdout);
      }
    }

    self.shared_node().larst_post_order_stdout_write(stdout);
  }
}


impl TreeNode {

  /// ### write_to_larst_file
  /// Recursively writes a node and its children (and the children of those, etc.) to LarST.
  fn write_to_larst_file (&self, file_ptr: &mut std::fs::File) {

    self.larst_pre_order_file_write(file_ptr);

    if let Some(children) = self.shared_children() {
      for child in children {
        child.write_to_larst_file(file_ptr);
      }
    }

    self.larst_post_order_file_write(file_ptr);
  }


  /// Calls the pre-order LarST writer method of the contained `TreeNodeType` variant.
  /// output is directed to the given file.
  fn larst_pre_order_file_write (&self, file_ptr: &mut std::fs::File) {

    let refnames = self.ref_names_into_larst_labels();

    let pre_string = self.shared_data().larst_pre_order_string( refnames);
    match file_ptr.write(pre_string.as_bytes()) {
      Ok(_) => {},
      Err(_) => panic!("Could not write the prefix string \"{}\" to file. Computer says no...", pre_string)
    };
  }


  /// Calls the post-order LarST writer method of the contained `TreeNodeType` variant.
  /// output is directed to the given file.
  fn larst_post_order_file_write (&self, file_ptr: &mut std::fs::File) {

    let refnames = self.ref_names_into_larst_labels();

    let post_string = self.shared_data().larst_post_order_string( refnames);
    match file_ptr.write(post_string.as_bytes()){
      Ok(_) => {},
      Err(_) => panic!("Could not write the prefix string \"{}\" to file. Computer says no...", post_string)
    };
  }


  /// Recursively writes a node and its children (and the children of those, etc.) to LarST.
  /// Writes the result into `stdout`.
  fn write_to_larst_stdout (&self, stdout: &mut std::io::Stdout) {

    self.larst_pre_order_stdout_write(stdout);

    if let Some(children) = self.shared_children() {
      for child in children {
        child.write_to_larst_stdout(stdout);
      }
    }

    self.larst_post_order_stdout_write(stdout);
  }


  /// Calls the pre-order LarST writer method of the contained `TreeNodeType` variant.
  /// Sends the resulting string to stdout.
  fn larst_pre_order_stdout_write (&self, stdout: &mut std::io::Stdout) {

    let refnames = self.ref_names_into_larst_labels();
    let pre_string = self.shared_data().larst_pre_order_string( refnames);
    match stdout.write(pre_string.as_bytes()) {
      Ok(_) => {},
      Err(_) => panic!("Could not write the prefix string \"{}\" to stdout. Computer says no...", pre_string)
    };
  }


  /// Calls the post-order LarST writer method of the contained `TreeNodeType` variant.
  /// Sends the resulting string to stdout.
  fn larst_post_order_stdout_write (&self, stdout: &mut std::io::Stdout) {

    let refnames = self.ref_names_into_larst_labels();
    let post_string = self.shared_data().larst_post_order_string( refnames);
    match stdout.write(post_string.as_bytes()) {
      Ok(_) => {},
      Err(_) => panic!("Could not write the postfix string \"{}\" to stdout. Computer says no...", post_string)
    };
  }

  /// Generates a single string of LarST labels from contained reference names.
  fn ref_names_into_larst_labels (&self) -> String {
    if let Some(refnames) = self.shared_target_label() {
      let mut targets = String::new();
      for refname in refnames.iter() {
        targets += &format!("\\label{{{}}}\n", refname);
      }
      targets
    } else {
      String::new()
    }
  }
}


impl TreeNodeType {

  /// ### larst_pre_order_string
  /// 
  /// Defines the text pattern each `TreeNodeType` variant starts with.
  fn larst_pre_order_string (&self, ref_names: String) -> String {

    let pre_string = match self {
      Self::Abbreviation { .. }           => todo!(),
      Self::AbsoluteURI { text } => {
        format!(r"\url{{{}}}", text)
      },
      Self::Acronym { .. }      => todo!(),
      Self::Address             => todo!(),
      Self::Admonition { content_indent, classes, name, variant }   => {

        use crate::doctree::directives::AdmonitionType;

        match variant {
          AdmonitionType::Admonition { title } => format!("\\begin{{admonition}}\n"),
          _ => format!("\\begin{{{}}}\n", variant.to_string())
        }
      },
      Self::Attribution { raw_text,  }  => {
        format!("-- {}", raw_text)
      },
      Self::Author { .. }       => todo!(),
      Self::Authors {..}        => todo!(),
      Self::AutomaticSectionNumbering {..} => todo!(),
      Self::BlockQuote { body_indent } => {
        format!("\\begin{{quotation}}\n")
      },
      Self::BulletList { bullet, bullet_indent, text_indent } => {
        format!("\\begin{{itemize}}\n")
      },
      Self::BulletListItem{ bullet, bullet_indent, text_indent } => {
        format!("\\item ")
      },
      Self::Caption { indent } => {
        format!(r"\captionof{{figure}}{{")
      },
      Self::Citation { body_indent, label } => {
        todo!()
      },
      Self::CitationReference { displayed_text, target_label } => {
        format!(r"\hyperref[{}]{{{}}}", target_label, displayed_text)
      },
      Self::Class { .. }              => "".to_string(),
      Self::Classifier { .. }         => todo!(),
      Self::Code { text, language, name, class, number_lines } => {

        let lang = if let Some(lang) = language { format!("[{}]", lang) } else {"".to_string()};
        format!("\\begin{{codeblock}}{}\n", lang)
      },
      Self::ColSpec { .. }            => todo!(),
      Self::Comment { text }            => {
        if let Some(comment) = text {
          comment
            .lines()
            .fold(String::new(), |a, b| a + "% " + b + "\n") + "\n"
        } else {
          String::new()
        }
      },
      Self::CompoundParagraph { .. }  => todo!(),
      Self::Contact { .. }            => todo!(),
      Self::Container { .. }          => todo!(),
      Self::Copyright { .. }          => todo!(),
      Self::CSVTable { .. }           => todo!(),
      Self::Date                      => todo!(),
      Self::Decoration                => todo!(),
      Self::Definition                => todo!(),
      Self::DefinitionList { term_indent } => {
        format!("\\begin{{itemize}}\n")
      },
      Self::DefinitionListItem { term, classifiers, body_indent } => {

        let classifiers = if classifiers.is_empty() { "".to_string() } else { format!(": {}", classifiers.join(", ")) };

        format!("\\item \\textbf{{{}}}{}\n\n", term, classifiers)
      },
      Self::Description => todo!(),
      Self::DocInfo => todo!(),
      Self::DoctestBlock{ .. } => todo!(),
      Self::Document { .. }   => {
        format!(
"\
\\documentclass{{aplus}}

\\begin{{document}}\n\n")
      },
      Self::Emphasis { text } => {
        format!("\\textit{{{}}}", text)
      },
      Self::EmptyLine => {
        format!("")
      },
      Self::Entry { .. } => {
        format!("")
      },
      Self::EnumeratedList { delims, kind, start_index, n_of_items, enumerator_indent } => {
        format!("\\begin{{enumerate}}\n")
      },
      Self::EnumeratedListItem { delims, kind, index_in_list, enumerator_indent, text_indent } => {
        format!("\\item ")
      },
      Self::ExternalHyperlinkTarget { .. }      => todo!(),
      Self::Field                               => unimplemented!(),
      Self::FieldBody { .. }                    => unimplemented!(),
      Self::FieldList { marker_indent } => {
        format!("\\begin{{itemize}}\n")
      },
      Self::FieldListItem { raw_marker_name, marker_name_as_inline_nodes, .. } => {
        format!("\\item \\textbf{{{}}}\n\n", raw_marker_name)
      },
      Self::Figure { .. }                   => format!("\\begin{{center}}\n"),
      Self::Footer { .. }                   => todo!(),
      Self::Footnote { .. }                 => todo!(),
      Self::FootnoteReference { .. }        => todo!(),
      Self::Header { .. }                   => todo!(),
      Self::Generated                       => todo!(),
      Self::Image { uri, alt, height, width, scale, align, target, name, class } => {

        let mut options = String::new();

        if let Some(val) = alt { options = options + LATEX_OPTION_DELIM + val };
        if let Some(h) = height {
          let width = rst_length_to_string(h);
          options = options + "width=" + width.as_str() + LATEX_OPTION_DELIM;
        }
        if let Some(w) = width {
          let width = rst_length_to_string(w);
          options = options + "width=" + width.as_str() + LATEX_OPTION_DELIM;
        }
        if let Some(val) = scale  {
          let percentage_val = val.to_string();
          options = options + "scale=" + percentage_val.as_str() +  LATEX_OPTION_DELIM
        }
        if let Some(val) = align {
          let alignment = html_alignment_to_string(val);
          options = options + LATEX_OPTION_DELIM + &alignment
        }

        let options = if options.is_empty() { options } else { format!("[{}]", options) };

        format!("\\includegraphics{}{{{}}}\n", options, uri)
      },
      Self::IndirectHyperlinkTarget { .. }  => todo!(),
      Self::Inline { .. }                   => todo!(),
      Self::InlineTarget { .. }             => todo!(),
      Self::InterpretedText { .. }          => todo!(),
      Self::Label { .. }                    => todo!(),
      Self::Legend { .. }                   => todo!(),
      Self::Line { .. }                     => todo!(),
      Self::LineBlock { .. }                => todo!(),
      Self::ListTable { title, widths, width, header_rows, stub_columns, align, .. } => {

        let widths = if let Some(widths) = widths {
          match widths {
            TableColWidths::Auto => String::new(),
            TableColWidths::Columns(vals) => {
              let mut col_widths = Vec::<String>::with_capacity(vals.len());
              for val in vals {
                col_widths.push(format!("p{{{0:.2}\\textwidth}}", *val));
              }
              col_widths.join("")
            }
          }
        } else {
          panic!("Columns widths need to be set for all list tables. Computer says no...")
        };

        format!("\\begin{{tabular}}{{{}}}\n", widths)
      },
      Self::Literal { text }       => format!("\\texttt{{{}}}", text),
      Self::LiteralBlock { text }  => {
        use crate::utf8_to_latex::unicode_text_to_latex;
        format!("\\begin{{codeblock}}\n{}", unicode_text_to_latex(text))
      },
      Self::Math { text, class, name }                     => {
        format!(r"\({}\)", text)
      },
      Self::MathBlock { block_text, name, class } => {

        let ref_labels = if let Some(name) = name {
          let mut labels = String::new();
          let names = name.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
          for name in names.iter() {
            labels += &format!("\\label{}\n", name);
          }
          labels
        } else { String::new() };
        format!("\\begin{{equation}}\n{}\\begin{{split}}\n{}\n", ref_labels, block_text)
      },
      Self::OptionList { .. }               => todo!(),
      Self::OptionListItem { .. }           => todo!(),
      Self::OptionString { .. }             => todo!(),
      Self::Organization { .. }             => todo!(),
      Self::Paragraph { .. }                => "".to_string(),
      Self::ParsedLiteralBlock { .. }       => todo!(),
      Self::Pending { .. }                  => todo!(),
      Self::Problematic { .. }              => todo!(),
      Self::Raw { .. }                      => "\\begin{codeblock}\n".to_string(),
      Self::Reference { displayed_text, reference } => {
        // let command = if *has_embedded_uri { "href" } else { "hyperref" };
        // let parens = if *has_embedded_uri { ("{", "}") } else { ("[", "]") };
        // format!("\\{}{}{}{}{{{}}}", command, parens.0, target_label, parens.1, displayed_text)

        use crate::common::Reference;

        match &reference {
          Reference::Internal (ref_str) => if let Some(text) = displayed_text {
            format!("\\hyperref[{}]{{{}}}", ref_str, text)
          } else {
            format!("\\ref{{{}}}", ref_str)
          }
          Reference::URI (ref_str) => if let Some(text) = displayed_text {
            format!("\\href{{{}}}{{{}}}", ref_str, text)
          } else {
            format!("\\url{{{}}}", ref_str)
          }
          Reference::EMail (ref_str) => if let Some(text) = displayed_text {
            format!("\\href{{{}}}{{{}}}", ref_str, text)
          } else {
            format!("\\href{{{}}}{{{}}}", ref_str, ref_str)
          }
        }
      },
      Self::Revision { .. }                 => todo!(),
      Self::Row { .. }                      => todo!(),
      Self::Rubric { .. }                   => todo!(),
      Self::Section { title_text, level, line_style } => {

        let (command, subs) = if *level == 1 { ("chapter", "".to_string()) } else { ("section", "sub".repeat(*level - 2)) };
        let subs = "sub".repeat(*level - 1);
        format!("\\{}{}{{{}}}\n\n", subs, command, title_text)
      },
      Self::Sidebar { .. }                  => todo!(),
      Self::Status { .. }                   => todo!(),
      Self::StrongEmphasis { text }  => {
        format!("\\textbf{{{}}}", text)
      },
      Self::Subscript { text }       => {
        format!(r"\textsubscript{{{}}}", text)
      },
      Self::SubstitutionDefinition { .. }   => todo!(),
      Self::SubstitutionReference { substitution_label, target_label } => {
        todo!()
      },
      Self::Subtitle { .. }                 => todo!(),
      Self::Superscript { text }              => {
        format!(r"\textsuperscript{{{}}}", text)
      },
      Self::SystemMessage { .. }            => todo!(),
      Self::Table { .. }                    => todo!(),
      Self::Target { .. }                   => todo!(),
      Self::TBody { .. }                    => "".to_string(),
      Self::Term { .. }                     => todo!(),
      Self::Text { text }          => {
        format!("{}", text)
      },
      Self::TGroup { .. }                   => todo!(),
      Self::THead { .. }                    => todo!(),
      Self::TRow { .. }                    => "".to_string(),
      Self::Title { .. }                    => todo!(),
      Self::TitleReference { displayed_text, target_label } =>  {
        format!("\\hyperref[{}]{{{}}}", target_label, displayed_text)
      },
      Self::Topic { .. }                    => todo!(),
      Self::Transition {}                   =>  {
        format!("\\hrulefill\n")
      },
      Self::Version { .. }                  => todo!(),
      Self::WhiteSpace { text } => {
        format!("{}", text)
      },

      // ============================
      //  Sphinx specific directives
      // ============================

      Self::SphinxOnly { expression, body_indent } => {
        format!("\\begin{{only}}[{}]\n", expression)
      },

      Self::SphinxCodeBlock { language, linenos, lineno_start, emphasize_lines, caption, name, dedent, force, code_text } => {

        let mut options = Vec::<String>::new();

        if *linenos { options.push(String::from("linenos")) }
        if let Some(start_line) = lineno_start { options.push(format!("lineno-start={}", start_line.to_string() ) ) }
        if let Some(line_numbers) = emphasize_lines {
          let line_number_strings = line_numbers
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
            .join(",");
          options.push(format!("emphasize_lines={}",line_number_strings))
        }
        if let Some(caption) = caption {
          let latex_caption = crate::utf8_to_latex::unicode_text_to_latex(caption);
          options.push(format!("caption={}", latex_caption))
        }
        if let Some(name) = name {
          let normalized_refname = crate::common::normalize_refname(name);
          options.push(format!("name"))
        }
        if let Some(dedent) = dedent {
          // Remove indentation from code_text here?
        }

        if *force {
          options.push(format!("force"))
        }

        let option_string = options.join(",");

        // LarST does not support many of the given options yet, so they are not written to the resulting file...
        format!("\\begin{{codeblock}}[{}]\n{}", language, code_text)
      }

      // ========================
      //  A+ specific directives
      // ========================

      Self::AplusPOI { id, title, previous, next, hidden, class, height, columns, bgimg, not_in_slides, not_in_book, no_poi_box, .. } => {

        let mut options = String::new();

        if let Some(option) = id        { options = options + "id=" +option + LATEX_OPTION_DELIM};
        if let Some(option) = previous  { options = options + "previous=" + option + LATEX_OPTION_DELIM};
        if let Some(option) = next      { options = options + "next=" + option + LATEX_OPTION_DELIM};
        if let Some(option) = hidden    { options = options + "hidden" + LATEX_OPTION_DELIM};
        if let Some(option) = class     { options = options + "class=" + option + LATEX_OPTION_DELIM};
        if let Some(option) = height    {
          let height = rst_length_to_string(option);
          options = options + "height=" + &height + LATEX_OPTION_DELIM
        };
        if let Some(option) = columns   { options = options + "columns=" + option + LATEX_OPTION_DELIM};
        if let Some(option) = bgimg     { options = options + "bgimg=" + option + LATEX_OPTION_DELIM};
        if let Some(option) = not_in_slides      { options = options + "not_in_slides" + LATEX_OPTION_DELIM};
        if let Some(option) = not_in_book        { options = options + "not_in_book" + LATEX_OPTION_DELIM};
        if let Some(option) = no_poi_box         { options = options + "no_poi_box" + LATEX_OPTION_DELIM};

        if ! options.is_empty() { options = format!("[{}]", options.as_str()) };
        format!("\\begin{{poi}}{}{{{}}}\n\n", options, title)
      },
      Self::AplusColBreak => "\\newcol\n\n".to_string(),
      Self::AplusQuestionnaire { max_points, key, points_from_children, difficulty, submissions, points_to_pass, feedback, title, no_override, pick_randomly, preserve_questions_between_attempts, category, status, reveal_model_at_max_submissions, show_model, allow_assistant_viewing, allow_assistant_grading, .. } => {

        let max_points = if let Some(points) = max_points { points } else { points_from_children };

        format!("\\begin{{quiz}}{{{}}}{{{}}}\n", key, *max_points)
      },
      Self::AplusPickOne { points, class, required, key, dropdown, .. } => {

        let mut options = String::new();
        if let Some(option) = class { options = options + "id=" +option + LATEX_OPTION_DELIM };
        if *required { options = options + "required" + LATEX_OPTION_DELIM };
        if let Some(option) = key { options = options + "key=" + option + LATEX_OPTION_DELIM };
        // if *dropdown { options = options + "dropdown" + OPTION_DELIM };

        let options = if ! options.is_empty() { format!("[{}]", options) } else { options };

        format!("\\begin{{pick}}{}{{one}}{{{}}}\n", options, points)
      }
      Self::AplusPickAny { points, class, required, key, partial_points, randomized, correct_count, preserve_questions_between_attempts, .. } => {

        let mut options = String::new();
        if let Some(option) = class { options = options + "id=" +option + LATEX_OPTION_DELIM };
        if *required { options = options + "required" + LATEX_OPTION_DELIM };
        if let Some(option) = key { options = options + "key=" + option + LATEX_OPTION_DELIM };
        if *partial_points { options = options + "partial-points" + LATEX_OPTION_DELIM };
        if *randomized { options = options + "randomized" + LATEX_OPTION_DELIM };
        if let Some(correct_count) = correct_count { options = options + "correct-count=" + correct_count.to_string().as_str() + LATEX_OPTION_DELIM };
        if *preserve_questions_between_attempts { options = options + "preserve-questions-between-attempts" + LATEX_OPTION_DELIM };

        if ! options.is_empty() { options = format!("[{}]", options) }

        format!("\\begin{{pick}}{}{{any}}{{{}}}\n", options, points)
      },
      Self::AplusFreeText { points, compare_method, model_answer, required, class, key, length, height, .. } => {
        format!("\\begin{{freetext}}{{{}}}{{{}}}{{{}}}\n", compare_method, points, model_answer)
      },
      Self::AplusPickChoices { .. } => "\\begin{answers}\n".to_string(),
      Self::AplusPickChoice { label, is_correct, is_pre_selected, is_neutral } => {
        let is_correct = if *is_neutral { "\\undet" } else if *is_correct { "\\right" } else { "\\wrong" };
        format!("{} ", is_correct)
      },
      Self::AplusQuestionnaireHints { .. } => "".to_string(),
      Self::AplusQuestionnaireHint { label, show_when_not_selected, question_type } => {

        let show_when_not_selected = if *show_when_not_selected { "" } else { "!" };

        use crate::common::AplusQuestionnaireType;

        let reference = match question_type {
          AplusQuestionnaireType::PickOne | AplusQuestionnaireType::PickAny => format!("\\ref{{{}}}", label),
          AplusQuestionnaireType::FreeText => String::new()
        };

        format!("\\feedback{{{}{}}}{{", show_when_not_selected, reference)
      },
      Self::AplusSubmit { body_indent, key, difficulty, max_points, config, submissions, points_to_pass, class, title, category, status, ajax, allow_assistant_viewing, allow_assistant_grading, quiz, url, radar_tokenizer, radar_minimum_match_tokens, lti, lti_resource_link_id, lti_open_in_iframe, lti_aplus_get_and_post } => {

        // Read relevant options
        let mut option_string = String::new();

        option_string = if ! config.is_empty() { option_string + "config=" + config + LATEX_OPTION_DELIM } else { option_string };
        option_string = option_string + "submissions=" + submissions.to_string().as_str() + LATEX_OPTION_DELIM;
        option_string = option_string + "points-to-pass=" + points_to_pass.to_string().as_str() + LATEX_OPTION_DELIM;
        option_string = if ! class.is_empty() { option_string + "class=" + class + LATEX_OPTION_DELIM } else { option_string };
        option_string = if ! title.is_empty() { option_string + "title=" + title + LATEX_OPTION_DELIM } else { option_string };
        option_string = if ! category.is_empty() { option_string + "category=" + category + LATEX_OPTION_DELIM } else { option_string };
        option_string = match status {
          AplusExerciseStatus::Ready => option_string + "status=ready" + LATEX_OPTION_DELIM,
          AplusExerciseStatus::Unlisted => option_string + "status=unlisted" + LATEX_OPTION_DELIM,
          AplusExerciseStatus::Hidden => option_string + "status=hidden" + LATEX_OPTION_DELIM,
          AplusExerciseStatus::Enrollment => option_string + "status=enrollment" + LATEX_OPTION_DELIM,
          AplusExerciseStatus::EnrollmentExt => option_string + "status=enrollment_ext" + LATEX_OPTION_DELIM,
          AplusExerciseStatus::Maintenance => option_string + "status=maintenance" + LATEX_OPTION_DELIM,
        };
        option_string = if *ajax { option_string + "ajax" + LATEX_OPTION_DELIM } else { option_string };
        option_string = if *allow_assistant_viewing { option_string + "allow-assistant-viewing" + LATEX_OPTION_DELIM } else { option_string };
        option_string = if *allow_assistant_grading { option_string + "allow-assistant-grading" + LATEX_OPTION_DELIM } else { option_string };
        option_string = if *quiz { String::from("quiz") + LATEX_OPTION_DELIM } else { option_string };
        option_string = if ! lti.is_empty() { option_string + "lti" + LATEX_OPTION_DELIM } else { option_string };
        option_string = if ! lti_resource_link_id.is_empty() { option_string + "resource_link_id=" + lti_resource_link_id + LATEX_OPTION_DELIM } else { option_string };
        option_string = if *lti_open_in_iframe { option_string + "lti_open_in_iframe" + LATEX_OPTION_DELIM } else { option_string };
        option_string = if *lti_aplus_get_and_post { option_string + "lti_aplus_get_and_post" + LATEX_OPTION_DELIM } else { option_string };

        if ! option_string.is_empty() { option_string = format!("[{}]", option_string) }

        format!("\\begin{{submit}}{}{{{}}}{{{}}}\n", option_string, key, max_points )
      }

      Self::AplusActiveElementInput { key_for_input, title, default, class, width, height, clear, input_type, file } => {

        use crate::common::{ AplusActiveElementClear, AplusActiveElementInputType};

        let mut options = String::new();

        let title = if let Some(title) = title { title } else { "" };
        if let Some(option) = default { options = options + "default=" + option + LATEX_OPTION_DELIM  }
        if let Some(option) = class   { options = options + "class=" + option + LATEX_OPTION_DELIM  }
        if let Some(option) = width   {
          let width = rst_length_to_string(option);
          options = options + "width=" + &width + LATEX_OPTION_DELIM
        }
        if let Some(option) = height  {
          let height = rst_length_to_string(option);
          options = options + "height=" + &height + LATEX_OPTION_DELIM
        }
        if let Some(option) = clear {
          match option {
            AplusActiveElementClear::Both   => options = options + "clear=both" + LATEX_OPTION_DELIM,
            AplusActiveElementClear::Left   =>  options = options + "clear=left" + LATEX_OPTION_DELIM,
            AplusActiveElementClear::Right  =>  options = options + "clear=right" + LATEX_OPTION_DELIM,
          }
        }
        if let Some(option) = input_type {
          match option {
            AplusActiveElementInputType::File       => options = options + "type=file" + LATEX_OPTION_DELIM,
            AplusActiveElementInputType::Clickable  => options = options + "type=clickable" + LATEX_OPTION_DELIM,
            AplusActiveElementInputType::Dropdown(option_string)   => options = options + "type=dropdown:" + option_string + LATEX_OPTION_DELIM,
          }
        }
        if let (Some(input_type), Some(file)) = (input_type, file) {
          if let AplusActiveElementInputType::Clickable =  input_type {
            options = options + "file=" + file + LATEX_OPTION_DELIM;
          } else {
            eprintln!("LarST writer found an alleged file path but active element input type not \"clickable\". Ignoring...")
          }
        }

        if ! options.is_empty() { options = format!("[{}]", options) }

        format!("\\aeinput{}{{{}}}{{{}}}", options, key_for_input, title)
      },

      Self::AplusActiveElementOutput { key_for_output, config, inputs, title, class, width, height, clear, output_type, submissions, scale_size, status } => {

        let mut options = String::new();
        let title = if let Some(title) = title { title } else { "" };

        use crate::common::{ AplusActiveElementClear, AplusActiveElementOutputType};

        options = options + "config=" + config + LATEX_OPTION_DELIM;
        // options = options + "inputs=" + inputs + LATEX_OPTION_DELIM;
        if let Some(option) = class  { options = options + "class=" + option + LATEX_OPTION_DELIM }
        if let Some(option) = width  {
          let width = rst_length_to_string(option);
          options = options + "width=" + &width + LATEX_OPTION_DELIM
        }
        if let Some(option) = height {
          let height = rst_length_to_string(option);
          options = options + "height=" + &height + LATEX_OPTION_DELIM
        }
        if let Some(option) = clear {
          match option {
            AplusActiveElementClear::Both   => options = options + "clear=both" + LATEX_OPTION_DELIM,
            AplusActiveElementClear::Left   =>  options = options + "clear=left" + LATEX_OPTION_DELIM,
            AplusActiveElementClear::Right  =>  options = options + "clear=right" + LATEX_OPTION_DELIM,
          }
        }
        match output_type {
          AplusActiveElementOutputType::Text => options = options + "type=text" + LATEX_OPTION_DELIM,
          AplusActiveElementOutputType::Image => options = options + "type=image" + LATEX_OPTION_DELIM,
        }
        if let Some(option) = submissions { options = options + "submissions=" + &option.to_string() + LATEX_OPTION_DELIM }
        if *scale_size { options = options + "scale-size" + LATEX_OPTION_DELIM }
        match status {
          AplusExerciseStatus::Ready    => { options = options + "status=ready" + LATEX_OPTION_DELIM }
          AplusExerciseStatus::Unlisted => { options = options + "status=unlisted" + LATEX_OPTION_DELIM }
          AplusExerciseStatus::Hidden   => { options = options + "status=hidden" + LATEX_OPTION_DELIM }
          AplusExerciseStatus::Enrollment    => { options = options + "status=enrollment" + LATEX_OPTION_DELIM }
          AplusExerciseStatus::EnrollmentExt => { options = options + "status=enrollment-ext" + LATEX_OPTION_DELIM }
          AplusExerciseStatus::Maintenance   => { options = options + "status=maintenance" + LATEX_OPTION_DELIM }
        }

        if ! options.is_empty() { options = format!("[{}]", options) }

        format!("\\aeoutput{}{{{}}}{{{}}}{{{}}}", options, key_for_output, inputs, title)
      },

    };

    pre_string
  }

  /// ### larst_post_order_string
  /// 
  /// Defines the text pattern each `TreeNodeType` variant ends with.
  fn larst_post_order_string (&self, ref_names: String) -> String {

    let post_string = match self {
      Self::Abbreviation { .. }             => todo!(),
      Self::AbsoluteURI { .. }              => "".to_string(),
      Self::Acronym { .. }                  => todo!(),
      Self::Address                         => todo!(),
      Self::Admonition { variant, .. } => {
        
        
        use crate::doctree::directives::AdmonitionType;
        match variant {
          AdmonitionType::Admonition{title} => format!("\\end{{admonition}}\n\n"),
          _ => format!("\\end{{{}}}\n\n", variant.to_string())
        }
      },
      Self::Attribution { .. }              => "\n".to_string(),
      Self::Author { .. }                   => todo!(),
      Self::Authors { .. }                  => todo!(),
      Self::AutomaticSectionNumbering {..}  => todo!(),
      Self::BlockQuote { .. }               => "\\end{quotation}\n\n".to_string(),
      Self::BulletList { .. }               => format!("\\end{{itemize}}\n\n"),
      Self::BulletListItem{ .. }            => "".to_string(),
      Self::Caption { .. }                  => "}\n".to_string(),
      Self::Citation { .. }                 => "\n".to_string(),
      Self::CitationReference { .. }        => "".to_string(),
      Self::Class { .. }                    => "".to_string(),
      Self::Classifier { .. }               => todo!(),
      Self::Code { .. }                     => "\\end{codeblock}\n\n".to_string(),
      Self::ColSpec { .. }                  => todo!(),
      Self::Comment { .. }                  => "".to_string(),
      Self::CompoundParagraph { .. }        => "\n".to_string(),
      Self::Contact { .. }                  => todo!(),
      Self::Container { .. }                => todo!(),
      Self::Copyright { .. }                => todo!(),
      Self::CSVTable { .. }                 => todo!(),
      Self::Date                            => todo!(),
      Self::Decoration                      => todo!(),
      Self::Definition                      => todo!(),
      Self::DefinitionList { .. }           => "\\end{itemize}\n\n".to_string(),
      Self::DefinitionListItem { .. }       => "\n".to_string(),
      Self::Description                     => todo!(),
      Self::DocInfo                         => todo!(),
      Self::DoctestBlock{ .. }              => todo!(),
      Self::Document { .. }                 => "\\end{document}\n".to_string(),
      Self::Emphasis { .. }                 => "".to_string(),
      Self::EmptyLine                       => "".to_string(),
      Self::Entry { is_last }        => {
        let suffix = if *is_last { "" } else { "&\n" };
        format!("{}", suffix)
      },
      Self::EnumeratedList { .. }           => "\\end{enumerate}\n\n".to_string(),
      Self::EnumeratedListItem { .. }       => "".to_string(),
      Self::ExternalHyperlinkTarget { .. }  => "\n".to_string(),
      Self::Field                           => todo!(),
      Self::FieldBody { .. }                => todo!(),
      Self::FieldList { .. }                => "\\end{itemize}\n\n".to_string(),
      Self::FieldListItem { .. }            => "\n".to_string(),
      Self::Figure { .. }                   => "\\end{center}\n\n".to_string(),
      Self::Footer { .. }                   => todo!(),
      Self::Footnote { .. }                 => todo!(),
      Self::FootnoteReference { .. }        => todo!(),
      Self::Header { .. }                   => todo!(),
      Self::Generated                       => todo!(),
      Self::Image { .. }                    => "".to_string(),
      Self::IndirectHyperlinkTarget { .. }  => todo!(),
      Self::Inline { .. }                   => todo!(),
      Self::InlineTarget { .. }             => todo!(),
      Self::InterpretedText { .. }          => todo!(),
      Self::Label { .. }                    => todo!(),
      Self::Legend { .. }                   => "\n".to_string(),
      Self::Line { .. }                     => "\n".to_string(),
      Self::LineBlock { .. }                => "\n".to_string(),
      Self::ListTable { .. }                => "\\end{tabular}\n\n".to_string(),
      Self::Literal { .. }                  => "".to_string(),
      Self::LiteralBlock { .. }             => "\n\\end{codeblock}\n\n".to_string(),
      Self::Math { .. }                     => "".to_string(),
      Self::MathBlock { .. }                => "\\end{split}\n\\end{equation}\n\n".to_string(),
      Self::OptionList { .. }               => "\n".to_string(),
      Self::OptionListItem { .. }           => "\n".to_string(),
      Self::OptionString { .. }             => todo!(),
      Self::Organization { .. }             => todo!(),
      Self::Paragraph { .. }                => "\n\n".to_string(),
      Self::ParsedLiteralBlock { .. }       => "\n\n".to_string(),
      Self::Pending { .. }                  => todo!(),
      Self::Problematic { .. }              => todo!(),
      Self::Raw { .. }                      => "\\end{raw}\n\n".to_string(),
      Self::Reference { .. }                => "".to_string(),
      Self::Revision { .. }                 => todo!(),
      Self::Row { .. }                      => todo!(),
      Self::Rubric { .. }                   => "\n".to_string(),
      Self::Section { .. }                  => "".to_string(),
      Self::Sidebar { .. }                  => "\n".to_string(),
      Self::Status { .. }                   => todo!(),
      Self::StrongEmphasis { .. }           => "".to_string(),
      Self::Subscript { .. }                => "".to_string(),
      Self::SubstitutionDefinition { .. }   => "\n".to_string(),
      Self::SubstitutionReference { .. }    => "".to_string(),
      Self::Subtitle { .. }                 => "".to_string(),
      Self::Superscript { .. }              => "".to_string(),
      Self::SystemMessage { .. }            => todo!(),
      Self::Table { .. }                    => "\n".to_string(),
      Self::Target { .. }                   => "\n".to_string(),
      Self::TBody { .. }                    => "\n".to_string(),
      Self::Term { .. }                     => todo!(),
      Self::Text { .. }                     => "".to_string(),
      Self::TGroup { .. }                   => todo!(),
      Self::THead { .. }                    => "\n".to_string(),
      Self::TRow                            => "\\\\\n".to_string(),
      Self::Title { .. }                    => todo!(),
      Self::TitleReference { .. }           => "".to_string(),
      Self::Topic { .. }                    => todo!(),
      Self::Transition { .. }               => "\n".to_string(),
      Self::Version { .. }                  => todo!(),
      Self::WhiteSpace { .. }               => "".to_string(),


      // ============================
      //  Sphinx specific directives
      // ============================

      Self::SphinxOnly { expression, body_indent } => "\\end{only}\n\n".to_string(),
      Self::SphinxCodeBlock { .. } => String::from("\\end{codeblock}\n\n"),

      // ========================
      //  A+ specific directives
      // ========================

      Self::AplusPOI { .. } => "\\end{poi}\n\n".to_string(),
      Self::AplusColBreak => "".to_string(),
      Self::AplusQuestionnaire { .. } => "\\end{quiz}\n\n".to_string(),
      Self::AplusPickOne { .. } => "\\end{pick}\n\n".to_string(),
      Self::AplusPickAny { .. } => "\\end{pick}\n\n".to_string(),
      Self::AplusFreeText { .. } => "\\end{freetext}\n\n".to_string(),
      Self::AplusPickChoices { .. } => "\\end{answers}\n\n".to_string(),
      Self::AplusPickChoice { label, .. } => {
        let label = format!(" \\label{{{}}}", label);
        format!("{}\n", label)
      },
      Self::AplusQuestionnaireHints { .. } => "\n".to_string(),
      Self::AplusQuestionnaireHint { .. } => "}\n".to_string(),
      Self::AplusSubmit { .. } => {
        "\\end{submit}\n\n".to_string()
      }
      Self::AplusActiveElementInput { .. } => "\n\n".to_string(),
      Self::AplusActiveElementOutput { .. } => "\n\n".to_string(),

    };

    post_string
  }
}


// =========
//  HELPERS
// =========

/// ### aplus_cls_contents
///
/// Returns the contents of the LaTeX class file required by Larst projects
/// being compiled by `pdflatex` or `lualatex` as a `&'static str`.
/// The string was authored by Tomi Janhunen.
/// 
/// source: https://course-gitlab.tuni.fi/ITC/CS/larst/larstprod/-/raw/master/LarST-example/aplus.cls
/// url-date: 2020-09-17
fn aplus_cls_contents () -> &'static str {

r#"%
% The LaRST Project
%
% alpus -- Documentclass for the direct LaTeX compilation of A+ materials
%
% (c) 2019-2020 Tomi Janhunen

\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{aplus}

\LoadClass{book}
\RequirePackage{url}
\RequirePackage{graphicx}
\RequirePackage[breakable,most]{tcolorbox}
\RequirePackage{amsmath}
\RequirePackage{amssymb}
\RequirePackage{pifont}
\RequirePackage{keyval}
\RequirePackage{ifthen}
\RequirePackage{xstring}
\RequirePackage{comment}
\RequirePackage{environ}
\RequirePackage{fancyvrb}
\RequirePackage{hyperref}

% Font issues
\RequirePackage[T1]{fontenc}

% Reset page dimensions
\usepackage[nohead,nofoot,top=1in,margin=1in]{geometry}
\pagestyle{empty}

% \newcommand{\chapter}[1]{{\Huge\textbf{#1}}}

% Set fonts toward ``Read the Docs''
\usepackage[scaled]{helvet}
\renewcommand\familydefault{\sfdefault}

% No indentation
\setlength{\parindent}{0pt}
\setlength{\parskip}{0.5\baselineskip}

% Remove (sub)section numbering
% \makeatletter
% \renewcommand{\@seccntformat}[1]{} 
% \makeatother

% Unification of labels
\global\def\labelhere{}
\newcommand{\rstlabel}[1]{\global\def\labelhere{\hypertarget{#1}{}\label{#1}}}

% RST Simulations in LaTeX

\newcommand{\aplus}[2]{}

\makeatletter
\long\def\notext#1{}
\newenvironment{only}[1][foo]{%
  \ifthenelse{\equal{#1}{latex}}%
  {}{\Collect@Body\notext}
  }{}
\makeatother

\newenvironment{raw}{}{}
\RenewEnviron{raw}{}{}

\newcommand{\code}[1]{\texttt{#1}}

% Blocks of code

\makeatletter
\define@key{codeblock}{python}[]{}
\makeatother

\newcommand\innercodeblock[1][]{#1}
\newenvironment{codeblock}{ \bgroup\verbatim\innercodeblock }{ \endverbatim\egroup }
% \newenvironment{codeblock}[1][]{\begin{BVerbatim}}{\end{BVerbatim}}

% File download

\newcommand{\download}[2]{\par\texttt{#1}\footnote{\url{#2}}}
\newcommand{\rstclass}[1]{}
\newcommand{\feedback}[2]{\par\textbf{#1}. #2 \\}

\newenvironment{toggle}[1]{\textbf{#1}. }{}


% Points of interest (slide-type objects within material)

\makeatletter
\define@key{poi}{hidden}[]{}
\define@key{poi}{columns}[]{\def\poi@colums{#1}}
\define@key{poi}{id}[]{\def\poi@id{#1}}
\define@key{poi}{next}[]{\def\poi@next{#1}}
\define@key{poi}{prev}[]{\def\poi@prev{#1}}
\define@key{poi}{bgimg}[]{\def\poi@bgimg{#1}}
\makeatother

\newcommand{\newcol}{\newpage} % Semantic mismatch
\newenvironment{poi}[2][]{%
\setkeys{poi}{#1}
\par\noindent\begin{large}\begin{tcolorbox}[width=\textwidth,adjusted title=#2]%
}{%
\end{tcolorbox}\end{large}}

% Active elements

\makeatletter
\newlength{\ae@width}
\newlength{\ae@height}
\define@key{aelement}{width}[]{\def\ae@width{#1}}
\define@key{aelement}{height}[]{\def\ae@height{#1}}
\define@key{aelement}{class}[]{\def\ae@class{#1}}
\define@key{aelement}{type}[]{\def\ae@type{#1}}
\setkeys{aelement}{width=\textwidth,height=\baselineskip,type=pdf,class=left}%
\newcommand{\aeinput}[2][]{\setkeys{aelement}{#1}}
\newcommand{\aeoutput}[3][]{\setkeys{aelement}{#1}}
\makeatother

% Submission fields

\makeatletter
\define@key{submit}{config}[]{\def\sbm@config{#1}}
\define@key{submit}{submissions}[]{\def\sbm@submissions{#1}}
\define@key{submit}{points-to-pass}[]{\def\sbm@ptp{#1}}
\define@key{submit}{class}[]{\def\sbm@class{#1}}
\define@key{submit}{title}[]{\def\sbm@title{#1}}
\define@key{submit}{category}[]{\def\sbm@category{#1}}
\define@key{submit}{status}[]{\def\sbm@status{#1}}
\define@key{submit}{allow-assistant-viewing}[]{\def\sbm@viewing{#1}}
\define@key{submit}{allow-assistant-grading}[]{\def\sbm@grading{#1}}
\define@key{submit}{url}[]{\def\sbm@url{#1}}
\define@key{submit}{lti}[]{\def\sbm@lti{#1}}
\define@key{submit}{ajax}[]{\def\sbm@ajax{true}}
\define@key{submit}{quiz}[]{\def\sbm@quiz{true}}
\makeatother

\newenvironment{submit}[2][]{%
\setkeys{submit}{#1}%
\par\noindent\begin{tcolorbox}[width=\textwidth,adjusted title=#2]%
}{%
\end{tcolorbox}}

% Quizzes

\newcommand{\wrong}{\item[\fbox{\phantom{\large x}}]}
\renewcommand{\right}{\item[\fbox{\large x}]}
\newcommand{\undet}{\item[\fbox{\large *}]}

\newcounter{question}\stepcounter{question}
\newenvironment{answers}{\begin{enumerate}}{\end{enumerate}}

\makeatletter
\define@key{quiz}{submissions}[]{\def\qz@submissions{#1}}
\define@key{quiz}{points-to-pass}[]{\def\qz@points{#1}}
\define@key{quiz}{title}[]{\def\qz@title{#1}}
\define@key{quiz}{pick-randomly}[]{\def\qz@randomly{#1}}
\define@key{quiz}{category}[]{\def\qz@category{#1}}
\define@key{quiz}{status}[]{\def\qz@status{#1}}
\define@key{quiz}{reveal-model-at-max-submissions}[]{\def\qz@reveal{#1}}
\define@key{quiz}{show-model}[]{\def\qz@show{#1}}
\define@key{quiz}{allow-assistant-viewing}[]{\def\qz@viewing{#1}}
\define@key{quiz}{allow-assistant-grading}[]{\def\qz@grading{#1}}
\define@key{quiz}{feedback}[]{\def\qz@feedback{true}}
\define@key{quiz}{no-override}[]{\def\qz@noover{true}}
\define@key{quiz}{preserve-questions-between-attempts}[]{\def\qz@preserve{true}}
\setkeys{quiz}{}%
\newenvironment{quiz}[3][]{%
\setkeys{quiz}{#1}{}%
\section*{Quiz #2}}{\setcounter{question}{1}}
\makeatother

% Pick

\makeatletter
\define@key{pick}{class}[]{\def\pick@class{#1}}
\define@key{pick}{key}[]{\def\pick@key{#1}}
\define@key{pick}{randomized}[]{\def\pick@randomized{#1}}
\define@key{pick}{correct-count}[]{\def\pick@correct{#1}}
\define@key{pick}{required}[]{\def\pick@required{true}}
\define@key{pick}{partial-points}[]{\def\pick@partial{true}}
\setkeys{pick}{}%
\newenvironment{pick}[3][]{%
\setkeys{pick}{#1}{}%
\par\textbf{Q\thequestion:}~}{\stepcounter{question}}
\makeatother

% Freetext

\makeatletter
\newlength{\ft@height}
\newlength{\ft@length}
\define@key{freetext}{required}[]{\def\ft@required{true}}
\define@key{freetext}{length}[]{\def\ft@length{#1}}
\define@key{freetext}{height}[]{\def\ft@height{#1}}
\define@key{freetext}{class}[]{\def\ft@class{#1}}
\define@key{freetext}{key}[]{\def\ft@key{#1}}
\setkeys{freetext}{length=100em,height=5\baselineskip,class=left}%
\newenvironment{freetext}[4][]{%
\setkeys{freetext}{#1}{}
\par\textbf{Q\thequestion:}~}{\stepcounter{question}}
\makeatother

% LaTeX environments (assumed by default, some used in limited ways)

% \begin{document} ... \end{document}
% \begin{itemize} ... \item ... \end{itemize}
% \begin{enumerate} ... \item ... \end{enumerate}
% \begin{tabular}[...] ... & ... & ... \\ ... \end{tabular}
% \begin{thebibliography}{...} ... \end{thebibligraphy}
% \begin{equation} ... \end{equation}
% \begin{center} ... \end{center}

% LaTeX commands (assumed by default)

% \documentclass{}
% \bibliographystyle{...}
% \tableofcontents
% \contentsline{...}{...}{...}
% \chapter{...}
% \section{...}
% \subsection{...}
% \emph{...} or {\em ...}
% \textit{...}
% \textbf{...} or {\bf ...}
% \texttt{...}
% \captionof{...}{...}
% \newcounter{...}
% \the...
% \stepcounter{...}
% \refstepcounter{...}
% \addtocounter{...}{...}
% \setcounter{...}{...}
% \numberwithin{...}{...}
% \include{...}
% \input{...}
% \includegraphics[...]{...}
% \cite{...}
% \ref{...}
% \label{...}
% \url{...}
% \href{...}{...}
% \hyperref[...]{...}
% \hypertarget{...}{...}
% \hyperlink{...}{...}
% \textbackslash
% \textasciicircum
% \textunderscore
% \textasciitilde
% \nbspc
% \aa
% \AA
% \hrulefill

"#
}


/// rst_length_to_string
///
/// Converts a given reStructuredText length reference into a string.
fn rst_length_to_string (length: &Length) -> String {
  match length {
    Length::Em(val) => val.to_string() + "em",
    Length::Ex(val) => val.to_string() + "ex",
    Length::Mm(val) => val.to_string() + "mm",
    Length::Cm(val) => val.to_string() + "cm",
    Length::In(val) => val.to_string() + "in",
    Length::Px(val) => val.to_string() + "px",
    Length::Pt(val) => val.to_string() + "pt",
    Length::Pc(val) => val.to_string() + "pc",
  }
}


/// ### html_alignment_to_string
///
/// Converts a HTMLAlignment variant to the corresponding string.
fn html_alignment_to_string (alignment: &HTMLAlignment) -> String {
  match alignment {
    HTMLAlignment::Top => String::from("top"),
    HTMLAlignment::Middle => String::from("middle"),
    HTMLAlignment::Bottom => String::from("bottom"),
    HTMLAlignment::Left => String::from("left"),
    HTMLAlignment::Center => String::from("center"),
    HTMLAlignment::Right => String::from("right"),
  }
}


/// ### horizontal_alignment_to_string
///
/// Converts a HorizontalAlignment variant to the corresponding string.
fn horizontal_alignment_to_string (alignment: &HorizontalAlignment) -> String {
  match alignment {
    HorizontalAlignment::Left => String::from("left"),
    HorizontalAlignment::Center => String::from("center"),
    HorizontalAlignment::Right => String::from("right"),
  }
}
