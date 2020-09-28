/// ## tree_node
/// A submodule for the TreeNode type.
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;

/// ### TreeNode
/// A tree node that contains a struct of `TreeNodeType`
/// plus the information needed to traverse the tree.
#[derive(Debug)]
pub struct TreeNode {
  pub id: NodeId,
  target_label: Option<Vec<String>>,
  pub data : TreeNodeType,
  pub children: Option<Children>,

}

impl TreeNode {

  /// ### new
  /// A `TreeNode` constructor.
  pub fn new(data: TreeNodeType, id: NodeId, target_label: Option<Vec<String>>) -> Self {

    TreeNode {
      id: id,
      target_label: target_label,
      children: Self::children_or_none(&data),
      data: data
    }
  }


  /// ### new_from_id_ref
  /// Works similarly to `TreeNode::new`, except also increments the id
  /// behind the given address in addition to assignning the previous value
  /// to the node being constructred.
  pub fn new_from_id_ref (data: TreeNodeType, id_ref: &mut NodeId, target_label: Option<Vec<String>>) -> Self {

    let node = Self {
      id: *id_ref, // assign current id value to node
      target_label: target_label,
      children: Self::children_or_none(&data),
      data: data,
    };

    *id_ref += 1; // increment the id before returning with new node
    node
  }


  /// ### children_or_none
  /// Set the children of a `TreeNode` to `Some(Children)` or `None`,
  /// depending on the given node data variant.
  fn children_or_none (data_variant: &TreeNodeType) -> Option<Children> {

    use crate::doctree::node_categories::NodeCategory;

    let mut node_type_properties = data_variant.node_categories();

    if node_type_properties.any( |cat|
      match cat {
        NodeCategory::CompoundStructural | NodeCategory::CompoundBody | NodeCategory::CompoundSubBody => true,
        _ => false
      }
    ) {
      Some(Vec::new())
    } else if let TreeNodeType::Paragraph { .. } | TreeNodeType::AplusPickChoice { .. } | TreeNodeType::AplusQuestionnaireHint { .. } = data_variant {
      Some(Vec::new())
    } else {
      None
    }
  }


  /// ### shared_target_label
  /// Returns a shared (immutable) reference to the optional target label.
  /// If the label is `None`, as is hasn't been set, returns an
  /// empty string slice instead.
  pub fn shared_target_label (&self) -> &Option<Vec<String>> {
    &self.target_label
  }
  
  
  /// ### push_child
  /// Pushes a given child node the the end of `self.children`.
  pub fn push_child (&mut self, node : TreeNode) {

    if self.child_is_allowed(&node.data) {
      if let Some(children) = &mut self.children {
        children.push(node);
      } else {
        panic!("Node of type {:#?} is not allowed to have children. Computer says no...", self.data)
      }
    } else {
      panic!("Child of type {:#?} not allowed inside a {:#?}. Computer says no...", node.data, self.data);
    }
  }


  /// ### append_children
  /// Appends multiple children to `self.children`.
  pub fn append_children(&mut self, children: &mut Vec<TreeNode>) {

    // Check whether all children are valid
    for child in children.iter() {
      if self.child_is_allowed(&child.data) {
        continue
      } else {
        eprintln!("Found incompatible child {:#?} when appending children to {:#?}.\nComputer says no...\n", child.data, self.data);
        panic!();
      }
    }

    if let Some(child_vec) = &mut self.children {
      child_vec.append(children);
    } else {

    }
  }


  pub fn shared_target_labels (&self) -> &Option<Vec<String>> {
    &self.target_label
  }


  /// ### set_target_label
  /// Sets the target label of the node to given `Option<Vec<String>>`.
  pub fn set_target_label (&mut self, label: Option<Vec<String>>) {

    self.target_label = label;
  }


  /// ### shared_children
  /// 
  /// Optionally returns a shared reference to vector containing child nodes.
  pub fn shared_children (&self) -> &Option<Children> {

    &self.children
  }


  /// ### child_is_allowed
  /// Checks whether a node is allowed to be inserted into another node.
  pub fn child_is_allowed (&self, node_data: &TreeNodeType) -> bool {

    use crate::doctree::node_categories::NodeCategory;

    match self.data {
      TreeNodeType::Abbreviation { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::AbsoluteURI { .. } => false,
      TreeNodeType::Acronym { .. } => false, // No documentation on docutils!
      TreeNodeType::Address => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::Admonition { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else { false },
      TreeNodeType::Attribution { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::Author { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::Authors {..} => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::AutomaticSectionNumbering {..} => false, // Not really a node in rST
      TreeNodeType::BlockQuote { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else if let TreeNodeType::Attribution { .. } = node_data { true } else { false },
      TreeNodeType::BulletList { .. } => match node_data { TreeNodeType::EmptyLine | TreeNodeType::BulletListItem { .. } => true, _ => false },
      TreeNodeType::BulletListItem{ .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else { false },
      TreeNodeType::Caption { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::Citation { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else { false },
      TreeNodeType::CitationReference { .. } => false,
      TreeNodeType::Classifier { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::Code { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::ColSpec { .. } => false,
      TreeNodeType::Comment { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else { false },
      TreeNodeType::CompoundParagraph { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else { false },
      TreeNodeType::Contact { .. } => false,
      TreeNodeType::Container { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else { false },
      TreeNodeType::Copyright { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::CSVTable { .. } => todo!(),
      TreeNodeType::Date => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::Decoration => match node_data { TreeNodeType::Footer { .. } | TreeNodeType::Header { .. } => true, _ => false },
      TreeNodeType::Definition => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else { false },
      TreeNodeType::DefinitionList { .. } => match node_data { TreeNodeType::EmptyLine | TreeNodeType::DefinitionListItem { .. } => true, _ => false },
      TreeNodeType::DefinitionListItem { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else { false },
      TreeNodeType::Description => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else { false },
      TreeNodeType::DocInfo => if node_data.node_categories().any(|cat| if let NodeCategory::Bibliographic = cat { true } else { false }) { true } else { false },
      TreeNodeType::DoctestBlock{ .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::Document { .. }   => if node_data.node_categories().any(|cat| match cat { NodeCategory::Structural | NodeCategory::SubStructural | NodeCategory::Body => true, _ => false }) { true } else { false },
      TreeNodeType::Emphasis { .. } => false,
      TreeNodeType::EmptyLine => false,
      TreeNodeType::Entry => todo!(),
      TreeNodeType::EnumeratedList { .. } => match node_data { TreeNodeType::EmptyLine | TreeNodeType::EnumeratedListItem { .. } => true, _ => false },
      TreeNodeType::EnumeratedListItem { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else { false },
      TreeNodeType::ExternalHyperlinkTarget { .. } => false,
      TreeNodeType::Field => todo!(),
      TreeNodeType::FieldBody { .. } => todo!(),
      TreeNodeType::FieldList { .. } => match node_data { TreeNodeType::EmptyLine | TreeNodeType::FieldListItem { .. } => true, _ => false },
      TreeNodeType::FieldListItem { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else { false },
      TreeNodeType::Figure { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) || match node_data { TreeNodeType::Caption { .. } | TreeNodeType::Legend { .. } => true, _ => false } { true } else { false },
      TreeNodeType::Footer { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else { false },
      TreeNodeType::Footnote { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else { false },
      TreeNodeType::FootnoteReference { .. } => todo!(),
      TreeNodeType::Header { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else { false },
      TreeNodeType::Generated => todo!(),
      TreeNodeType::Image { .. } => false,
      TreeNodeType::IndirectHyperlinkTarget { .. } => false,
      TreeNodeType::Inline { .. } => false,
      TreeNodeType::InlineTarget { .. } => false,
      TreeNodeType::InterpretedText { .. } => false,
      TreeNodeType::Label { .. } => todo!(),
      TreeNodeType::Legend { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else { false },
      TreeNodeType::Line { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::LineBlock { .. } => match node_data { TreeNodeType::EmptyLine | TreeNodeType::Line { .. } => true, _ => false },
      TreeNodeType::ListTable { .. } => todo!(),
      TreeNodeType::Literal { .. } => false,
      TreeNodeType::LiteralBlock { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::Math { .. } => false,
      TreeNodeType::MathBlock { .. } => false,
      TreeNodeType::OptionList { .. } => match node_data { TreeNodeType::EmptyLine | TreeNodeType::OptionListItem{ .. } => true, _ => false },
      TreeNodeType::OptionListItem { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else { false },
      TreeNodeType::OptionString { .. } => todo!(),
      TreeNodeType::Organization { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::Paragraph { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::ParsedLiteralBlock { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::Pending { .. } => todo!("No information on \"Pending\" node children in docutils documentation..."),
      TreeNodeType::Problematic { .. } => todo!("No information on \"Problematic\" node children in docutils documentation..."),
      TreeNodeType::Raw { .. } => todo!("What is a \"Raw\" element supposed to be, exactly...?"),
      TreeNodeType::Reference { .. } => false, // inline ref
      TreeNodeType::Revision { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::Row { .. } => todo!("No documentation on table rows in docutils..."),
      TreeNodeType::Rubric { .. } => false,
      TreeNodeType::Section { .. }    => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else if match node_data { TreeNodeType:: Transition | TreeNodeType::Section { .. } | TreeNodeType::Topic { .. } | TreeNodeType::Sidebar { .. } => true, _ => false } { true } else { false },
      TreeNodeType::Sidebar { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else if let TreeNodeType::Topic { .. } = node_data { true } else { false },
      TreeNodeType::Status { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::StandaloneEmail { .. } => false,
      TreeNodeType::StrongEmphasis { .. } => false,
      TreeNodeType::Subscript { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::SubstitutionDefinition { .. } => false,
      TreeNodeType::SubstitutionReference { .. } => false,
      TreeNodeType::Subtitle { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::Superscript { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::SystemMessage { .. } => todo!(),
      TreeNodeType::Table { .. } => todo!(),
      TreeNodeType::Target { .. } => false,
      TreeNodeType::TBody { .. } => todo!(),
      TreeNodeType::Term { .. } => todo!(),
      TreeNodeType::Text { .. } => false,
      TreeNodeType::TGroup { .. } => todo!(),
      TreeNodeType::THead { .. } => todo!(),
      TreeNodeType::Title { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::TitleReference { .. } => false,
      TreeNodeType::Topic { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false },
      TreeNodeType::Transition {} => todo!(),
      TreeNodeType::Version { .. } => false,
      TreeNodeType::WhiteSpace { .. } => false,

      // ============================
      //  Sphinx specific directives
      // ============================

      TreeNodeType::SphinxOnly { .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else { false },

      // ========================
      //  A+ specific directives
      // ========================
      TreeNodeType::AplusPOI{ .. } => if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else if let TreeNodeType::AplusColBreak = node_data { true } else { false },
      TreeNodeType::AplusColBreak => false,
      TreeNodeType::AplusQuestionnaire { .. } => match node_data {
        TreeNodeType::EmptyLine | TreeNodeType::Paragraph { .. } | TreeNodeType::AplusPickOne { .. } | TreeNodeType::AplusPickAny { .. } | TreeNodeType::AplusFreeText { .. } => true,
        _ => false
      },
      TreeNodeType::AplusPickOne { .. } => match node_data {
        TreeNodeType::Paragraph { .. } | TreeNodeType::AplusPickChoices { .. } | TreeNodeType::AplusQuestionnaireHints { .. } => true,
        _ => false
      }
      TreeNodeType::AplusPickAny { .. } => match node_data {
        TreeNodeType::Paragraph { .. } | TreeNodeType::AplusPickChoices { .. } | TreeNodeType::AplusQuestionnaireHints { .. } => true,
        _ => false
      }
      TreeNodeType::AplusFreeText { .. } => match node_data {
        TreeNodeType::Paragraph { .. } | TreeNodeType::AplusPickChoices { .. } | TreeNodeType::AplusQuestionnaireHints { .. } => true,
        _ => false
      }
      TreeNodeType::AplusPickChoices { .. } => match node_data{
        TreeNodeType::AplusPickChoice { .. } => true,
        _ => false,
      },
      TreeNodeType::AplusPickChoice { .. } => {
        if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false }
      },
      TreeNodeType::AplusQuestionnaireHints { .. } => match node_data {
        TreeNodeType::AplusQuestionnaireHint { .. } => true,
        _ => false
      }
      TreeNodeType::AplusQuestionnaireHint { .. } => {
        if node_data.node_categories().any(|cat| if let NodeCategory::Inline = cat { true } else { false }) { true } else { false }
      }
      TreeNodeType::AplusSubmit { .. } => {
        if node_data.node_categories().any(|cat| if let NodeCategory::Body = cat { true } else { false }) { true } else { false }
      }
      TreeNodeType::AplusActiveElementInput { .. } => false,
      TreeNodeType::AplusActiveElementOutput { .. } => false,
    }
  }


  /// ### shared_child
  /// Returns a shared reference to a child node of a given index.
  /// Panics, if the child does not exist.
  pub fn shared_child (&self, index: usize) -> &Self {
    if let Some(children) = &self.children {
      match children.get(index) {
        Some(node) => node,
        None => panic!("No child at index {}. Computer says no...", index)
      }
    } else {
      panic!("Current node cannot have children. Computer says no...")
    }    
  }


  /// ### mut_child
  /// Returns a mutable reference to a child node of a given index.
  /// Panics, if the child does not exist.
  pub fn mut_child (&mut self, index: usize) -> &mut Self {
    if let Some(children) = &mut self.children {
      match children.get_mut(index) {
        Some(node) => node,
        None => panic!("No child at index {}.\nComputer says no...\n", index)
      }
    } else {
      panic!("Current node cannot have children. Computer says no...")
    }
  }


  /// ### get_data_type
  /// For retrieving an immutable reference to the data type of a node.
  /// Mainly for printing purposes.
  pub fn shared_data (&self) -> &TreeNodeType {
    &self.data
  }


  /// ### get_data_type
  /// For retrieving an immutable reference to the data type of a node.
  /// Mainly for printing purposes.
  pub fn mut_data (&mut self) -> &mut TreeNodeType {
    &mut self.data
  }
}