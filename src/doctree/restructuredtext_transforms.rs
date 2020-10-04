/// ## reStructuredText transforms
/// 
/// A submodule that defines the transforms performed for each node type,
/// before the tree is printed. These include things like transforming the
/// second child of a `Figure` node to a `Caption`, if it is a `Paragraph`.
/// 
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;

impl DocTree {

  /// Modifies `self.tree` with the known reStrucuturedText transforms.
  pub fn perform_restructuredtext_transforms (mut self) -> Self {

    self = self.walk_to_root();
    self.tree = self.tree.perform_restructuredtext_transforms();
    self = self.walk_to_root();
    self
  }
}

impl TreeZipper {

  /// Recursively modifies the data of `self.node` and its children,
  /// based on the node type `self.node.data`.
  pub fn perform_restructuredtext_transforms (mut self) -> Self {

    self.node.perform_restructuredtext_transforms();
    self
  }
}

impl TreeNode {

  /// Transforms `self.data` into a different type based on its current value.
  /// This is where the recursion really kicks in.
  pub fn perform_restructuredtext_transforms (&mut self) {

    use crate::doctree::tree_node_types::TreeNodeType;

    match self.mut_data() {
      TreeNodeType::Abbreviation { .. } => {},
      TreeNodeType::AbsoluteURI { .. } =>  {},
      TreeNodeType::Acronym { .. } =>  {},
      TreeNodeType::Address =>  {},
      TreeNodeType::Admonition { .. } =>  {},
      TreeNodeType::Attribution { .. } =>  {},
      TreeNodeType::Author { .. } =>  {},
      TreeNodeType::Authors {..} =>  {},
      TreeNodeType::AutomaticSectionNumbering {..} =>  {},
      TreeNodeType::BlockQuote { .. } =>  {},
      TreeNodeType::BulletList { .. } =>  {},
      TreeNodeType::BulletListItem{ .. } =>  {},
      TreeNodeType::Caption { .. } =>  {},
      TreeNodeType::Citation { .. } =>  {},
      TreeNodeType::CitationReference { .. } =>  {},
      TreeNodeType::Classifier { .. } =>  {},
      TreeNodeType::Code { .. } =>  {},
      TreeNodeType::ColSpec { .. } =>  {},
      TreeNodeType::Comment { .. } =>  {},
      TreeNodeType::CompoundParagraph { .. } =>  {},
      TreeNodeType::Contact { .. } =>  {},
      TreeNodeType::Container { .. } =>  {},
      TreeNodeType::Copyright { .. } =>  {},
      TreeNodeType::CSVTable { .. } =>  {},
      TreeNodeType::Date =>  {},
      TreeNodeType::Decoration =>  {},
      TreeNodeType::Definition =>  {},
      TreeNodeType::DefinitionList { .. } =>  {},
      TreeNodeType::DefinitionListItem { .. } =>  {},
      TreeNodeType::Description =>  {},
      TreeNodeType::DocInfo =>  {},
      TreeNodeType::DoctestBlock{ .. } =>  {},
      TreeNodeType::Document { .. }   =>  {},
      TreeNodeType::Emphasis { .. } =>  {},
      TreeNodeType::EmptyLine =>  {},
      TreeNodeType::Entry =>  {},
      TreeNodeType::EnumeratedList { .. } =>  {},
      TreeNodeType::EnumeratedListItem { .. } =>  {},
      TreeNodeType::ExternalHyperlinkTarget { .. } =>  {},
      TreeNodeType::Field =>  {},
      TreeNodeType::FieldBody { .. } =>  {},
      TreeNodeType::FieldList { .. } =>  {},
      TreeNodeType::FieldListItem { .. } =>  {},
      TreeNodeType::Figure { .. } =>  {
        if let Some(children) = self.mut_children() {
          if let Some(child) = children.get_mut(1) {
            if let TreeNodeType::Paragraph {  indent} = child.mut_data() {
              // Transform paragraph data into a caption
              child.data = TreeNodeType::Caption {
                indent: *indent
              };
            }
          } else {
            // Do nothing
          }
        }
      },
      TreeNodeType::Footer { .. } =>  {},
      TreeNodeType::Footnote { .. } =>  {},
      TreeNodeType::FootnoteReference { .. } =>  {},
      TreeNodeType::Header { .. } =>  {},
      TreeNodeType::Generated =>  {},
      TreeNodeType::Image { .. } =>  {},
      TreeNodeType::IndirectHyperlinkTarget { .. } =>  {},
      TreeNodeType::Inline { .. } =>  {},
      TreeNodeType::InlineTarget { .. } =>  {},
      TreeNodeType::InterpretedText { .. } =>  {},
      TreeNodeType::Label { .. } =>  {},
      TreeNodeType::Legend { .. } =>  {},
      TreeNodeType::Line { .. } =>  {},
      TreeNodeType::LineBlock { .. } =>  {},
      TreeNodeType::ListTable { .. } =>  {
        if let Some(children) = self.mut_children() { // The table itself
          if let Some(child) = children.get_mut(0) {
            if let TreeNodeType::BulletList { bullet, bullet_indent, text_indent } = child.mut_data() {
              child.data = TreeNodeType::TBody;
              if let Some(rows) = child.mut_children() {
                for row in children {
                  if let TreeNodeType::BulletListItem { .. } = row.mut_data() {
                    row.data = TreeNodeType::TRow;
                    if let Some(columns) = row.mut_children() {
                      for column in columns {
                        // We must go deeper...
                      }
                    }
                  } else if let TreeNodeType::EmptyLine = row.mut_data() {
                    // Keep as is
                  } else {
                    eprintln!("Cannot transform anything other than bullet list items or empty lines inside a list table...")
                  }
                }
              }
            }
          }
        }
      },
      TreeNodeType::Literal { .. } =>  {},
      TreeNodeType::LiteralBlock { .. } =>  {},
      TreeNodeType::Math { .. } =>  {},
      TreeNodeType::MathBlock { .. } =>  {},
      TreeNodeType::OptionList { .. } =>  {},
      TreeNodeType::OptionListItem { .. } =>  {},
      TreeNodeType::OptionString { .. } =>  {},
      TreeNodeType::Organization { .. } =>  {},
      TreeNodeType::Paragraph { .. } =>  {},
      TreeNodeType::ParsedLiteralBlock { .. } =>  {},
      TreeNodeType::Pending { .. } =>  {},
      TreeNodeType::Problematic { .. } =>  {},
      TreeNodeType::Raw { .. } =>  {},
      TreeNodeType::Reference { .. } =>  {},
      TreeNodeType::Revision { .. } =>  {},
      TreeNodeType::Row { .. } =>  {},
      TreeNodeType::Rubric { .. } =>  {},
      TreeNodeType::Section { .. }    =>  {},
      TreeNodeType::Sidebar { .. } =>  {},
      TreeNodeType::Status { .. } =>  {},
      TreeNodeType::StandaloneEmail { .. } =>  {},
      TreeNodeType::StrongEmphasis { .. } =>  {},
      TreeNodeType::Subscript { .. } =>  {},
      TreeNodeType::SubstitutionDefinition { .. } =>  {},
      TreeNodeType::SubstitutionReference { .. } =>  {},
      TreeNodeType::Subtitle { .. } =>  {},
      TreeNodeType::Superscript { .. } =>  {},
      TreeNodeType::SystemMessage { .. } =>  {},
      TreeNodeType::Table { .. } =>  {},
      TreeNodeType::Target { .. } =>  {},
      TreeNodeType::TBody { .. } =>  {

      },
      TreeNodeType::Term { .. } =>  {},
      TreeNodeType::Text { .. } =>  {},
      TreeNodeType::TGroup { .. } =>  {},
      TreeNodeType::THead { .. } =>  {},
      TreeNodeType::TRow { .. } =>  {},
      TreeNodeType::Title { .. } =>  {},
      TreeNodeType::TitleReference { .. } =>  {},
      TreeNodeType::Topic { .. } =>  {},
      TreeNodeType::Transition {}     =>  {},
      TreeNodeType::Version { .. } =>  {},
      TreeNodeType::WhiteSpace { .. } =>  {},


      // ============================
      //  Sphinx specific directives
      // ============================

      TreeNodeType::SphinxOnly { .. } =>  {},

      // ========================
      //  A+ specific directives
      // ========================

      TreeNodeType::AplusPOI { .. } =>  {},
      TreeNodeType::AplusColBreak =>  {},
      TreeNodeType::AplusQuestionnaire { .. } =>  {},
      TreeNodeType::AplusPickOne { .. } =>  {},
      TreeNodeType::AplusPickAny { .. } =>  {},
      TreeNodeType::AplusFreeText { .. } =>  {},
      TreeNodeType::AplusPickChoices { .. } =>  {},
      TreeNodeType::AplusPickChoice { .. } =>  {},
      TreeNodeType::AplusQuestionnaireHints { .. } =>  {},
      TreeNodeType::AplusQuestionnaireHint { .. } =>  {},
      TreeNodeType::AplusSubmit { .. } =>  {},
      TreeNodeType::AplusActiveElementInput { .. } =>  {},
      TreeNodeType::AplusActiveElementOutput { .. } =>  {},
    };

    if let Some(children) = self.mut_children() {
      for child in children {
        child.perform_restructuredtext_transforms()
      }
    }
  }
}
