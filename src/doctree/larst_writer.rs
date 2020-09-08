/// ## larst_writer
/// 
/// A submodule that contains the larst writer method of the doctree,
/// and the patterns related to it.
/// 
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;

impl DocTree {

  /// ### write_to_larst
  /// 
  /// A function that writes a rusTLa doctree into a LarST file.
  /// 
  /// #### TODO
  /// 
  /// Add a return type such as a `Result<String, ()>` that contains the generated object code in a single string.
  /// Alternatively, pass a file pointer around and write (append) to it, returning it at the end if successful.
  pub fn write_to_larst (self) {

    let filename = self.filename_stem + ".tex";

    eprintln!("filename: {}\n", filename);

    use std::fs::{File, OpenOptions};
    // TODO: Add check for file existence...
    let mut file: File = match OpenOptions::new().append(true).create(true).open(filename.as_str()) {
      Ok(file) => file,
      Err(e) => panic!("Could not open LarST file for writing purposes: {}", e)
    };

    self.tree.write_to_larst(&mut file)
  }

}


impl TreeZipper {

  /// ### write_to_larst
  /// 
  /// This is the actual recursive function that goes over the tree zipper and writes each node
  /// into its LarST string representation based on its `TreeNodeType`.
  /// Starts out by calling `TreeNodeType`-specific pre-order action,
  /// then recursively calls itself for the children of the node and
  /// finishes by calling a post-order action on `self`.
  fn write_to_larst (mut self, file_ptr: &mut std::fs::File) {

    self = self.walk_to_root(); // Start out by walking to root.

    self.node.larst_pre_order_write(file_ptr);

    if let Some(children) = self.node.shared_children() {
      for child in children {
        child.write_to_larst(file_ptr);
      }
    }

    self.node.larst_post_order_write(file_ptr);
  }
}


impl TreeNode {

  /// ### write_to_larst
  /// Recursively writes a node and its children (and the children of those, etc.) to LarST.
  fn write_to_larst (&self, file_ptr: &mut std::fs::File) {

    self.larst_pre_order_write(file_ptr);

    if let Some(children) = self.shared_children() {
      for child in children {
        child.write_to_larst(file_ptr);
      }
    }

    self.larst_post_order_write(file_ptr);
  }  

  /// ### larst_pre_order_write
  /// 
  /// Calls the pre-order LarST writer method of the contained `TreeNodeType` variant.
  fn larst_pre_order_write (&self, file_ptr: &mut std::fs::File) {

    let refnames = if let Some(refnames) = self.shared_target_label() {

      let mut targets = String::new();
      for refname in refnames.iter() {
        targets += &format!("\\label{{{}}}\n", refname);
      }
      targets
    } else {
      String::new()
    };

    self.data.larst_pre_order_write(file_ptr, refnames)
  }


  /// ### larst_post_order_write
  /// 
  /// Calls the post-order LarST writer method of the contained `TreeNodeType` variant.
  fn larst_post_order_write (&self, file_ptr: &mut std::fs::File) {

    let refnames = if let Some(refnames) = self.shared_target_label() {

      let mut targets = String::new();
      for refname in refnames.iter() {
        targets += &format!("\\label{{{}}}\n", refname);
      }
      targets
    } else {
      String::new()
    };

    self.data.larst_post_order_write(file_ptr, refnames)
  }
}


impl TreeNodeType {

  /// ### larst_pre_order_write
  /// 
  /// Defines the text pattern each `TreeNodeType` variant
  /// starts with.
  fn larst_pre_order_write (&self, file_ptr: &mut std::fs::File, ref_names: String) {

    let pre_string = match self {
      Self::Abbreviation { .. }           => todo!(),
      Self::AbsoluteURI { text } => {
        format!(r"\url{{{}}}", text)
      },
      Self::Acronym { .. }      => todo!(),
      Self::Address             => todo!(),
      Self::Admonition { .. }   => todo!(),
      Self::Attribution { .. }  => todo!(),
      Self::Author { .. }       => todo!(),
      Self::Authors {..}        => todo!(),
      Self::AutomaticSectionNumbering {..} => todo!(),
      Self::BlockQuote { body_indent } => {
        format!(r"\begin{{quotation}}")
      },
      Self::BulletList { bullet, bullet_indent, text_indent } => {
        format!(r"\begin{{itemize}}")
      },
      Self::BulletListItem{ bullet, bullet_indent, text_indent } => {
        format!(r"{}\item ", " ".repeat(*bullet_indent))
      },
      Self::Caption { indent } => {
        format!(r"\caption{{")
      },
      Self::Citation { body_indent, label } => {
        todo!()
      },
      Self::CitationReference { displayed_text, target_label } => {
        format!(r"\hyperref[{}]{{{}}}", target_label, displayed_text)
      },
      Self::Classifier { .. }         => todo!(),
      Self::Code { text, language, name, class, number_lines } => {

        let lang = if let Some(lang) = language { format!("[{}]", lang) } else {"".to_string()};
        format!("\\begin{{codeblock}}{}\n", lang)
      },
      Self::ColSpec { .. }            => todo!(),
      Self::Comment { .. }            => todo!(),
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
        format!("\\item[{}] ({})]\n", term, classifiers.join(", "))
      },
      Self::Description => todo!(),
      Self::DocInfo => todo!(),
      Self::DoctestBlock{ .. } => todo!(),
      Self::Document { .. }   => {
        format!("\\begin{{document}}\n")
      },
      Self::Emphasis { text } => {
        format!("\\textit{{{}}}", text)
      },
      Self::EmptyLine => {
        format!("\n") // Notice, not "\n\n" but "\n"
      },
      Self::Entry => todo!(),
      Self::EnumeratedList { delims, kind, start_index, n_of_items, enumerator_indent } => {
        format!("\\begin{{enumerate}}\n")
      },
      Self::EnumeratedListItem { delims, kind, index_in_list, enumerator_indent, text_indent } => {
        format!("{}\\item ", " ".repeat(*enumerator_indent))
      },
      Self::ExternalHyperlinkTarget { .. }      => todo!(),
      Self::Field                               => todo!(),
      Self::FieldBody { .. }                    => todo!(),
      Self::FieldList { marker_indent } => {
        format!("\\begin{{itemize}}\n")
      },
      Self::FieldListItem { raw_marker_name, marker_name_as_inline_nodes, .. } => {
        format!("\\item[{}] ", raw_marker_name)
      },
      Self::Figure { .. }                   => format!("\\begin{{center}}\n"),
      Self::Footer { .. }                   => todo!(),
      Self::Footnote { .. }                 => todo!(),
      Self::FootnoteReference { .. }        => todo!(),
      Self::Header { .. }                   => todo!(),
      Self::Generated                       => todo!(),
      Self::Image { uri, alt, height, width, scale, align, target, name, class } => {

        let mut options = String::new();
        let connector = ", ";

        options = if let Some(val) = alt    { if options.is_empty() { options + val } else { options + connector + val } } else { options };
        options = if let Some(val) = height { if options.is_empty() { options + val } else { options + connector + val } } else { options };
        options = if let Some(val) = width  { if options.is_empty() { options + val } else { options + connector + val } } else { options };
        options = if let Some(val) = scale  { if options.is_empty() { options + val } else { options + connector + val } } else { options };
        options = if let Some(val) = align  { if options.is_empty() { options + val } else { options + connector + val } } else { options };

        format!("\\includegraphics[{}]{{{}}}\n", options, uri)
      },
      Self::IndirectHyperlinkTarget { .. }  => todo!(),
      Self::Inline { .. }                   => todo!(),
      Self::InlineTarget { .. }             => todo!(),
      Self::InterpretedText { .. }          => todo!(),
      Self::Label { .. }                    => todo!(),
      Self::Legend { .. }                   => todo!(),
      Self::Line { .. }                     => todo!(),
      Self::LineBlock { .. }                => todo!(),
      Self::ListTable { .. }                => todo!(),
      Self::Literal { text }       => format!("\\code{{{}}}", text),
      Self::LiteralBlock { text }  => format!("\\begin{{codeblock}}\n{}", text),
      Self::Math { .. }                     => "".to_string(),
      Self::MathBlock { block_text, name, class } => {

        let ref_labels = if let Some(name) = name {
          let mut labels = String::new();
          let names = name.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
          for name in names.iter() {
            labels += &format!("\\label{}\n", name);
          }
          labels
        } else { String::new() };
        format!("\\begin{{equation}}\n{}\n{}", ref_labels, block_text)
      },
      Self::OptionList { .. }               => todo!(),
      Self::OptionListItem { .. }           => todo!(),
      Self::OptionString { .. }             => todo!(),
      Self::Organization { .. }             => todo!(),
      Self::Paragraph { .. }                => "\n\n".to_string(),
      Self::ParsedLiteralBlock { .. }       => todo!(),
      Self::Pending { .. }                  => todo!(),
      Self::Problematic { .. }              => todo!(),
      Self::Raw { .. }                      => "\\begin{codeblock}\n".to_string(),
      Self::Reference { displayed_text, target_label } => {
        format!("\\hyperref[{}]{{{}}}", target_label, displayed_text)
      },
      Self::Revision { .. }                 => todo!(),
      Self::Row { .. }                      => todo!(),
      Self::Rubric { .. }                   => todo!(),
      Self::Section { .. }                  => todo!(),
      Self::Sidebar { .. }                  => todo!(),
      Self::Status { .. }                   => todo!(),
      Self::StandaloneEmail { .. }          => todo!(),
      Self::StrongEmphasis { text }           => {
        format!("\\textbf{{{}}}", text)
      },
      Self::Subscript { .. }                => todo!(),
      Self::SubstitutionDefinition { .. }   => todo!(),
      Self::SubstitutionReference { displayed_text, target_label } => {
        todo!()
      },
      Self::Subtitle { .. }                 => todo!(),
      Self::Superscript { .. }              => todo!(),
      Self::SystemMessage { .. }            => todo!(),
      Self::Table { .. }                    => todo!(),
      Self::Target { .. }                   => todo!(),
      Self::TBody { .. }                    => todo!(),
      Self::Term { .. }                     => todo!(),
      Self::Text { text }          => {
        format!("{}", text)
      },
      Self::TGroup { .. }                   => todo!(),
      Self::THead { .. }                    => todo!(),
      Self::Title { .. }                    => todo!(),
      Self::TitleReference { displayed_text, target_label } =>  {
        format!("\\hyperref[{}]{{{}}}", target_label, displayed_text)
      },
      Self::Topic { .. }                    => todo!(),
      Self::Transition {}                   =>  {
        format!("\n\\hrulefill\n")
      },
      Self::Version { .. }                  => todo!(),
      Self::WhiteSpace { text } => {
        format!("{}", text)
      },
    };

    use std::io::Write;
    match file_ptr.write(pre_string.as_bytes()){
      Ok(_) => {},
      Err(_) => panic!("Could not write the prefix string \"{}\" to file. Computer says no...", pre_string)
    };
  }

  /// ### larst_post_order_write
  /// 
  /// Defines the text pattern each `TreeNodeType` variant
  /// ends with.
  fn larst_post_order_write (&self, file_ptr: &mut std::fs::File, ref_names: String) {

    let post_string = match self {
      Self::Abbreviation { .. }             => todo!(),
      Self::AbsoluteURI { .. }              => "".to_string(),
      Self::Acronym { .. }                  => todo!(),
      Self::Address                         => todo!(),
      Self::Admonition { .. }               => "\n".to_string(),
      Self::Attribution { .. }              => todo!(),
      Self::Author { .. }                   => todo!(),
      Self::Authors { .. }                  => todo!(),
      Self::AutomaticSectionNumbering {..}  => todo!(),
      Self::BlockQuote { .. }               => "\n".to_string(),
      Self::BulletList { .. }               => format!("\\end{{itemize}}\n"),
      Self::BulletListItem{ .. }            => "\n".to_string(),
      Self::Caption { .. }                  => "}\n".to_string(),
      Self::Citation { .. }                 => "\n".to_string(),
      Self::CitationReference { .. }        => "".to_string(),
      Self::Classifier { .. }               => todo!(),
      Self::Code { .. }                     => "\\end{codeblock}\n".to_string(),
      Self::ColSpec { .. }                  => todo!(),
      Self::Comment { .. }                  => "\n".to_string(),
      Self::CompoundParagraph { .. }        => "\n".to_string(),
      Self::Contact { .. }                  => todo!(),
      Self::Container { .. }                => todo!(),
      Self::Copyright { .. }                => todo!(),
      Self::CSVTable { .. }                 => todo!(),
      Self::Date                            => todo!(),
      Self::Decoration                      => todo!(),
      Self::Definition                      => todo!(),
      Self::DefinitionList { .. }           => "\\end{itemize}".to_string(),
      Self::DefinitionListItem { .. }       => "\n".to_string(),
      Self::Description                     => todo!(),
      Self::DocInfo                         => todo!(),
      Self::DoctestBlock{ .. }              => todo!(),
      Self::Document { .. }                 => "\\end{document}\n\n".to_string(),
      Self::Emphasis { .. }                 => "".to_string(),
      Self::EmptyLine                       => "".to_string(),
      Self::Entry                           => todo!(),
      Self::EnumeratedList { .. }           => "\\end{enumerated}\n".to_string(),
      Self::EnumeratedListItem { .. }       => "\n".to_string(),
      Self::ExternalHyperlinkTarget { .. }  => "\n".to_string(),
      Self::Field                           => todo!(),
      Self::FieldBody { .. }                => todo!(),
      Self::FieldList { .. }                => "\\end{itemize}".to_string(),
      Self::FieldListItem { .. }            => "\n".to_string(),
      Self::Figure { .. }                   => "\\end{center}\n".to_string(),
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
      Self::Legend { .. }                   => todo!(),
      Self::Line { .. }                     => todo!(),
      Self::LineBlock { .. }                => todo!(),
      Self::ListTable { .. }                => todo!(),
      Self::Literal { .. }                  => todo!(),
      Self::LiteralBlock { .. }             => "\\end{codeblock}".to_string(),
      Self::Math { .. }                     => "".to_string(),
      Self::MathBlock { .. }                => "\\end{equation}\n".to_string(),
      Self::OptionList { .. }               => "\n".to_string(),
      Self::OptionListItem { .. }           => "\n".to_string(),
      Self::OptionString { .. }             => todo!(),
      Self::Organization { .. }             => todo!(),
      Self::Paragraph { .. }                => "\n\n".to_string(),
      Self::ParsedLiteralBlock { .. }       => "\n".to_string(),
      Self::Pending { .. }                  => todo!(),
      Self::Problematic { .. }              => todo!(),
      Self::Raw { .. }                      => "\\end{raw}\n".to_string(),
      Self::Reference { .. }                => "".to_string(),
      Self::Revision { .. }                 => todo!(),
      Self::Row { .. }                      => todo!(),
      Self::Rubric { .. }                   => "\n".to_string(),
      Self::Section { .. }                  => "\n".to_string(),
      Self::Sidebar { .. }                  => "\n".to_string(),
      Self::Status { .. }                   => todo!(),
      Self::StandaloneEmail { .. }          => "".to_string(),
      Self::StrongEmphasis { .. }           => "".to_string(),
      Self::Subscript { .. }                => "".to_string(),
      Self::SubstitutionDefinition { .. }   => "\n".to_string(),
      Self::SubstitutionReference { .. }    => "".to_string(),
      Self::Subtitle { .. }                 => "".to_string(),
      Self::Superscript { .. }              => "".to_string(),
      Self::SystemMessage { .. }            => todo!(),
      Self::Table { .. }                    => "\n".to_string(),
      Self::Target { .. }                   => "\n".to_string(),
      Self::TBody { .. }                    => todo!(),
      Self::Term { .. }                     => todo!(),
      Self::Text { .. }                     => "".to_string(),
      Self::TGroup { .. }                   => todo!(),
      Self::THead { .. }                    => todo!(),
      Self::Title { .. }                    => todo!(),
      Self::TitleReference { .. }           => todo!(),
      Self::Topic { .. }                    => todo!(),
      Self::Transition { .. }               => "\n".to_string(),
      Self::Version { .. }                  => todo!(),
      Self::WhiteSpace { .. }               => "".to_string(),
    };

    use std::io::Write;
    match file_ptr.write(post_string.as_bytes()){
      Ok(_) => {},
      Err(_) => panic!("Could not write the postfix string \"{}\" to file. Computer says no...", post_string)
    };
  }
}
