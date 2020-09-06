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

    self.tree.write_to_larst()
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
  fn write_to_larst (mut self) {

    self = self.walk_to_root(); // Start out by walking to root.

    self.node.larst_pre_order_write();

    if let Some(children) = self.node.shared_children() {
      for child in children {
        child.write_to_larst();
      }
    }

    self.node.larst_pre_order_write();
  }

}


impl TreeNode {

  /// ### write_to_larst
  /// Recursively writes a node and its children (and the children of those, etc.) to LarST.
  fn write_to_larst (&self) {

    self.larst_pre_order_write();

    if let Some(children) = self.shared_children() {
      for child in children {
        child.write_to_larst();
      }
    }

    self.larst_pre_order_write();
  }  

  /// ### write
  fn larst_pre_order_write (&self) {

    self.data.larst_pre_order_write()
  }


  fn larst_post_order_write (&self) {

    self.data.larst_post_order_write()
  }
}


// Pre- and post-order operations for the

impl TreeNodeType {

  /// ### larst_pre_order_write
  /// 
  /// Defines the text pattern each `TreeNodeType` variant
  /// starts with.
  fn larst_pre_order_write (&self) {

    match self {
      Self::Abbreviation { .. } => todo!(),
      Self::AbsoluteURI { .. } => todo!(),
      Self::Acronym { .. } => todo!(),
      Self::Address => todo!(),
      Self::Admonition { .. } => todo!(),
      Self::Attribution { .. } => todo!(),
      Self::Author { .. } => todo!(),
      Self::Authors {..} => todo!(),
      Self::AutomaticSectionNumbering {..} => todo!(),
      Self::BlockQuote { .. } => todo!(),
      Self::BulletList { .. } => todo!(),
      Self::BulletListItem{ .. } => todo!(),
      Self::Caption { .. } => todo!(),
      Self::Citation { .. } => todo!(),
      Self::CitationReference { .. } => todo!(),
      Self::Classifier { .. } => todo!(),
      Self::Code { .. } => todo!(),
      Self::ColSpec { .. } => todo!(),
      Self::Comment { .. } => todo!(),
      Self::CompoundParagraph { .. } => todo!(),
      Self::Contact { .. } => todo!(),
      Self::Container { .. } => todo!(),
      Self::Copyright { .. } => todo!(),
      Self::CSVTable { .. } => todo!(),
      Self::Date => todo!(),
      Self::Decoration => todo!(),
      Self::Definition => todo!(),
      Self::DefinitionList { .. } => todo!(),
      Self::DefinitionListItem { .. } => todo!(),
      Self::Description => todo!(),
      Self::DocInfo => todo!(),
      Self::DoctestBlock{ .. } => todo!(),
      Self::Document { .. }   => todo!(),
      Self::Emphasis { .. } => todo!(),
      Self::EmptyLine => todo!(),
      Self::Entry => todo!(),
      Self::EnumeratedList { .. } => todo!(),
      Self::EnumeratedListItem { .. } => todo!(),
      Self::ExternalHyperlinkTarget { .. } => todo!(),
      Self::Field => todo!(),
      Self::FieldBody { .. } => todo!(),
      Self::FieldList { .. } => todo!(),
      Self::FieldListItem { .. } => todo!(),
      Self::Figure { .. } => todo!(),
      Self::Footer { .. } => todo!(),
      Self::Footnote { .. } => todo!(),
      Self::FootnoteReference { .. } => todo!(),
      Self::Header { .. } => todo!(),
      Self::Generated => todo!(),
      Self::Image { .. } => todo!(),
      Self::IndirectHyperlinkTarget { .. } => todo!(),
      Self::Inline { .. } => todo!(),
      Self::InlineTarget { .. } => todo!(),
      Self::InterpretedText { .. } => todo!(),
      Self::Label { .. } => todo!(),
      Self::Legend { .. } => todo!(),
      Self::Line { .. } => todo!(),
      Self::LineBlock { .. } => todo!(),
      Self::ListTable { .. } => todo!(),
      Self::Literal { .. } => todo!(),
      Self::LiteralBlock { .. } => todo!(),
      Self::Math { .. } => todo!(),
      Self::MathBlock { .. } => todo!(),
      Self::OptionList { .. } => todo!(),
      Self::OptionListItem { .. } => todo!(),
      Self::OptionString { .. } => todo!(),
      Self::Organization { .. } => todo!(),
      Self::Paragraph { .. } => todo!(),
      Self::ParsedLiteralBlock { .. } => todo!(),
      Self::Pending { .. } => todo!(),
      Self::Problematic { .. } => todo!(),
      Self::Raw { .. } => todo!(),
      Self::Reference { .. } => todo!(),
      Self::Revision { .. } => todo!(),
      Self::Row { .. } => todo!(),
      Self::Rubric { .. } => todo!(),
      Self::Section { .. }    => todo!(),
      Self::Sidebar { .. } => todo!(),
      Self::Status { .. } => todo!(),
      Self::StandaloneEmail { .. } => todo!(),
      Self::StrongEmphasis { .. } => todo!(),
      Self::Subscript { .. } => todo!(),
      Self::SubstitutionDefinition { .. } => todo!(),
      Self::SubstitutionReference { .. } => todo!(),
      Self::Subtitle { .. } => todo!(),
      Self::Superscript { .. } => todo!(),
      Self::SystemMessage { .. } => todo!(),
      Self::Table { .. } => todo!(),
      Self::Target { .. } => todo!(),
      Self::TBody { .. } => todo!(),
      Self::Term { .. } => todo!(),
      Self::Text { .. } => todo!(),
      Self::TGroup { .. } => todo!(),
      Self::THead { .. } => todo!(),
      Self::Title { .. } => todo!(),
      Self::TitleReference { .. } => todo!(),
      Self::Topic { .. } => todo!(),
      Self::Transition {}     => todo!(),
      Self::Version { .. } => todo!(),
      Self::WhiteSpace { .. } => todo!()
    }
  }

  /// ### larst_post_order_write
  /// 
  /// Defines the text pattern each `TreeNodeType` variant
  /// ends with.
  fn larst_post_order_write (&self) {

    match self {
      Self::Abbreviation { .. } => todo!(),
      Self::AbsoluteURI { .. } => todo!(),
      Self::Acronym { .. } => todo!(),
      Self::Address => todo!(),
      Self::Admonition { .. } => todo!(),
      Self::Attribution { .. } => todo!(),
      Self::Author { .. } => todo!(),
      Self::Authors {..} => todo!(),
      Self::AutomaticSectionNumbering {..} => todo!(),
      Self::BlockQuote { .. } => todo!(),
      Self::BulletList { .. } => todo!(),
      Self::BulletListItem{ .. } => todo!(),
      Self::Caption { .. } => todo!(),
      Self::Citation { .. } => todo!(),
      Self::CitationReference { .. } => todo!(),
      Self::Classifier { .. } => todo!(),
      Self::Code { .. } => todo!(),
      Self::ColSpec { .. } => todo!(),
      Self::Comment { .. } => todo!(),
      Self::CompoundParagraph { .. } => todo!(),
      Self::Contact { .. } => todo!(),
      Self::Container { .. } => todo!(),
      Self::Copyright { .. } => todo!(),
      Self::CSVTable { .. } => todo!(),
      Self::Date => todo!(),
      Self::Decoration => todo!(),
      Self::Definition => todo!(),
      Self::DefinitionList { .. } => todo!(),
      Self::DefinitionListItem { .. } => todo!(),
      Self::Description => todo!(),
      Self::DocInfo => todo!(),
      Self::DoctestBlock{ .. } => todo!(),
      Self::Document { .. }   => todo!(),
      Self::Emphasis { .. } => todo!(),
      Self::EmptyLine => todo!(),
      Self::Entry => todo!(),
      Self::EnumeratedList { .. } => todo!(),
      Self::EnumeratedListItem { .. } => todo!(),
      Self::ExternalHyperlinkTarget { .. } => todo!(),
      Self::Field => todo!(),
      Self::FieldBody { .. } => todo!(),
      Self::FieldList { .. } => todo!(),
      Self::FieldListItem { .. } => todo!(),
      Self::Figure { .. } => todo!(),
      Self::Footer { .. } => todo!(),
      Self::Footnote { .. } => todo!(),
      Self::FootnoteReference { .. } => todo!(),
      Self::Header { .. } => todo!(),
      Self::Generated => todo!(),
      Self::Image { .. } => todo!(),
      Self::IndirectHyperlinkTarget { .. } => todo!(),
      Self::Inline { .. } => todo!(),
      Self::InlineTarget { .. } => todo!(),
      Self::InterpretedText { .. } => todo!(),
      Self::Label { .. } => todo!(),
      Self::Legend { .. } => todo!(),
      Self::Line { .. } => todo!(),
      Self::LineBlock { .. } => todo!(),
      Self::ListTable { .. } => todo!(),
      Self::Literal { .. } => todo!(),
      Self::LiteralBlock { .. } => todo!(),
      Self::Math { .. } => todo!(),
      Self::MathBlock { .. } => todo!(),
      Self::OptionList { .. } => todo!(),
      Self::OptionListItem { .. } => todo!(),
      Self::OptionString { .. } => todo!(),
      Self::Organization { .. } => todo!(),
      Self::Paragraph { .. } => todo!(),
      Self::ParsedLiteralBlock { .. } => todo!(),
      Self::Pending { .. } => todo!(),
      Self::Problematic { .. } => todo!(),
      Self::Raw { .. } => todo!(),
      Self::Reference { .. } => todo!(),
      Self::Revision { .. } => todo!(),
      Self::Row { .. } => todo!(),
      Self::Rubric { .. } => todo!(),
      Self::Section { .. }    => todo!(),
      Self::Sidebar { .. } => todo!(),
      Self::Status { .. } => todo!(),
      Self::StandaloneEmail { .. } => todo!(),
      Self::StrongEmphasis { .. } => todo!(),
      Self::Subscript { .. } => todo!(),
      Self::SubstitutionDefinition { .. } => todo!(),
      Self::SubstitutionReference { .. } => todo!(),
      Self::Subtitle { .. } => todo!(),
      Self::Superscript { .. } => todo!(),
      Self::SystemMessage { .. } => todo!(),
      Self::Table { .. } => todo!(),
      Self::Target { .. } => todo!(),
      Self::TBody { .. } => todo!(),
      Self::Term { .. } => todo!(),
      Self::Text { .. } => todo!(),
      Self::TGroup { .. } => todo!(),
      Self::THead { .. } => todo!(),
      Self::Title { .. } => todo!(),
      Self::TitleReference { .. } => todo!(),
      Self::Topic { .. } => todo!(),
      Self::Transition {}     => todo!(),
      Self::Version { .. } => todo!(),
      Self::WhiteSpace { .. } => todo!()
    }
  }
}
