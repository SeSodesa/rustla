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

    use std::fs::{File, OpenOptions};
    use std::path::Path;

    const TEX_FILE_SUFFIX: &str = ".tex";
    const APLUS_CLASS_FILE_NAME: &str = "aplus.cls";

    let folder = &self.file_folder;
    let mut object_file_path = PathBuf::from(folder);
    let mut aplus_class_file_path = PathBuf::from(folder);
    object_file_path.push(self.filename_stem + TEX_FILE_SUFFIX);
    aplus_class_file_path.push(APLUS_CLASS_FILE_NAME);



    // TODO: Add check for file existence...
    let mut object_file: File = match OpenOptions::new().write(true).truncate(true).create(true).open(object_file_path) {
      Ok(file) => file,
      Err(e) => panic!("Could not open LarST file for writing purposes: {}", e)
    };

    // If object file generation was successful, generate A+ class file
    let mut aplus_class_file: File = match OpenOptions::new().write(true).truncate(true).create(true).open(aplus_class_file_path) {
      Ok(file) => file,
      Err(e) => panic!("Could not open LarST file for writing purposes: {}", e)
    };

    use std::io::Write;
    match aplus_class_file.write(aplus_cls_contents().as_bytes()){
      Ok(_) => {},
      Err(_) => panic!("Could not write to A+ class file after generating object code. Computer says no...")
    };

    self.tree.write_to_larst(&mut object_file)
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
        format!("\\begin{{itemize}}\n")
      },
      Self::BulletListItem{ bullet, bullet_indent, text_indent } => {
        format!("\\item ")
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

        let classifiers = if classifiers.is_empty() { "".to_string() } else { format!(": {}", classifiers.join(", ")) };

        format!("\\item \\textbf{{{}}}{}\n\n", term, classifiers)
      },
      Self::Description => todo!(),
      Self::DocInfo => todo!(),
      Self::DoctestBlock{ .. } => todo!(),
      Self::Document { .. }   => {
        format!(
"\
\\documentclass[12pt]{{aplus}}

\\begin{{document}}\n\n")
      },
      Self::Emphasis { text } => {
        format!("\\textit{{{}}}", text)
      },
      Self::EmptyLine => {
        format!("") // Notice, not "\n\n" but "\n"
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
      Self::Reference { displayed_text, target_label } => {
        format!("\\hyperref[{}]{{{}}}", target_label, displayed_text)
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
      Self::StandaloneEmail { text } => {
        format!("\\href{{mailto:{}}}{{{}}}", text, text)
      },
      Self::StrongEmphasis { text }  => {
        format!("\\textbf{{{}}}", text)
      },
      Self::Subscript { text }                => {
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

      // ========================
      //  A+ specific directives
      // ========================

      Self::AplusPOI {

        title,
    
        // Options
        id,
        previous,
        next,
        hidden,
        class,
        height,
        columns,
        bgimg,
        not_in_slides,
        not_in_book,
        no_poi_box,
        ..
      } => {

        let mut options = String::from("[");
        let delim = ", ";

        if let Some(option) = id { options = options + option + delim};
        if let Some(option) = previous { options = options + option + delim};
        if let Some(option) = next { options = options + option + delim};
        if let Some(option) = hidden { options = options + option + delim};
        if let Some(option) = class { options = options + option + delim};
        if let Some(option) = height { options = options + option + delim};
        if let Some(option) = columns { options = options + option + delim};
        if let Some(option) = bgimg { options = options + option + delim};
        if let Some(option) = not_in_slides { options = options + option + delim};
        if let Some(option) = not_in_book { options = options + option + delim};
        if let Some(option) = no_poi_box { options = options + option + delim};

        options.push(']');

        let option_string =  if options.as_mut_str() == "[]" { "" } else { options.as_str() };
        format!("\\begin{{poi}}{}{{{}}}\n\n", option_string, title)
      },

      Self::AplusColBreak => "\\newcol\n\n".to_string()
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
      Self::BulletList { .. }               => format!("\\end{{itemize}}\n\n"),
      Self::BulletListItem{ .. }            => "".to_string(),
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
      Self::DefinitionList { .. }           => "\\end{itemize}\n".to_string(),
      Self::DefinitionListItem { .. }       => "\n".to_string(),
      Self::Description                     => todo!(),
      Self::DocInfo                         => todo!(),
      Self::DoctestBlock{ .. }              => todo!(),
      Self::Document { .. }                 => "\\end{document}\n".to_string(),
      Self::Emphasis { .. }                 => "".to_string(),
      Self::EmptyLine                       => "".to_string(),
      Self::Entry                           => todo!(),
      Self::EnumeratedList { .. }           => "\\end{enumerate}\n\n".to_string(),
      Self::EnumeratedListItem { .. }       => "\n".to_string(),
      Self::ExternalHyperlinkTarget { .. }  => "\n".to_string(),
      Self::Field                           => todo!(),
      Self::FieldBody { .. }                => todo!(),
      Self::FieldList { .. }                => "\\end{itemize}\n".to_string(),
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
      Self::Legend { .. }                   => "\n".to_string(),
      Self::Line { .. }                     => "\n".to_string(),
      Self::LineBlock { .. }                => "\n".to_string(),
      Self::ListTable { .. }                => "\n".to_string(),
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
      Self::TBody { .. }                    => "\n".to_string(),
      Self::Term { .. }                     => todo!(),
      Self::Text { .. }                     => "".to_string(),
      Self::TGroup { .. }                   => todo!(),
      Self::THead { .. }                    => "\n".to_string(),
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

      // ========================
      //  A+ specific directives
      // ========================

      Self::AplusPOI {

        title,
    
        // Options
        id,
        previous,
        next,
        hidden,
        class,
        height,
        columns,
        bgimg,
        not_in_slides,
        not_in_book,
        no_poi_box,
        ..
      } => {
        "\\end{poi}\n\n".to_string()
      },

      Self::AplusColBreak => "".to_string(),
    };

    use std::io::Write;
    match file_ptr.write(post_string.as_bytes()){
      Ok(_) => {},
      Err(_) => panic!("Could not write the postfix string \"{}\" to file. Computer says no...", post_string)
    };
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
% (c) 2019 Tomi Janhunen

\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{aplus}

\LoadClass{article}
\RequirePackage{url}
\RequirePackage{hyperref}
\RequirePackage{graphicx}
\RequirePackage{amsmath}
\RequirePackage{amssymb}
\RequirePackage{keyval}
\RequirePackage{ifthen}
\RequirePackage{xstring}
\RequirePackage{comment}
\RequirePackage{listings}
  \lstset{
    basicstyle=\ttfamily\scriptsize,
    showspaces=false,
    showstringspaces=false,
    tabsize=4,
    numbers=left,
    numberstyle=\tiny,
    numberblanklines=true,
  }
  \lstnewenvironment{codeblock}{\renewcommand\lstlistingname{Listing}%
  }{}

% Font issues
\RequirePackage[T1]{fontenc}

% Reset page dimensions
\usepackage[nohead,nofoot,top=1in,margin=1in]{geometry}
\pagestyle{empty}

\newcommand{\chapter}[1]{\begin{center}\Huge\textbf{#1}\end{center}}

% Set fonts toward ``Read the Docs''
\usepackage[scaled]{helvet}
\renewcommand\familydefault{\sfdefault}

% No indentation
\setlength{\parindent}{0pt}
\setlength{\parskip}{0.5\baselineskip}

% Remove (sub)section numbering
\makeatletter
\renewcommand{\@seccntformat}[1]{}
\makeatother

% Remove the section title for references
\renewcommand{\refname}{\vspace{-2\baselineskip}}

% Typical environments in mathematical text

\newenvironment{definition}{\par\textbf{Definition}\\}{}
\newenvironment{proposition}{\par\textbf{Proposition}\\}{}
\newenvironment{theorem}{\par\textbf{Theorem}\\}{}
\newenvironment{corollary}{\par\textbf{Corollary}\\}{}
\newenvironment{example}{\par\textbf{Example}\\}{}

% RST Simulations in LaTeX

\excludecomment{only}

\newcommand{\code}[1][]{\texttt{#1}}
\newenvironment{codeblock}{\begin{texttt}}{\end{texttt}}

% Points of interest (slide-type objects within material)

\newcommand{\newcol}{\newpage}
\newenvironment{poi}{}{}

% A+ Simulations in LaTeX

\newcommand{\Header}[1]{\LARGE\textbf{#1}}

\newcommand{\wrong}{\item[\fbox{\phantom{\large x}}]}
\renewcommand{\right}{\item[\fbox{\large x}]}

\newcounter{question}\stepcounter{question}
\newenvironment{quiz}[1]{\par\Header{Quiz #1}\par}{\setcounter{question}{1}}
\newenvironment{pick}[2]{\Header{Q\thequestion~}}{\stepcounter{question}}
\newenvironment{answers}{\begin{enumerate}}{\end{enumerate}}
"#
}
