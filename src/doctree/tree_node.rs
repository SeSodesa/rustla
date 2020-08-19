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
  target_label: Option<String>,
  pub data : TreeNodeType,
  pub children: Option<Children>,

}

impl TreeNode {

  /// ### new
  /// A `TreeNode` constructor.
  pub fn new(data: TreeNodeType, id: NodeId, target_label: Option<String>) -> Self {

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
  pub fn new_from_id_ref (data: TreeNodeType, id_ref: &mut NodeId, target_label: Option<String>) -> Self {

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
    match data_variant {
      TreeNodeType::Emphasis { .. }                   | TreeNodeType::StrongEmphasis { .. }
      | TreeNodeType::InterpretedText { .. }          | TreeNodeType::Literal { .. }
      | TreeNodeType::InlineTarget { .. }             | TreeNodeType::Reference { .. }
      | TreeNodeType::FootnoteReference { .. }        | TreeNodeType::CitationReference { .. }
      | TreeNodeType::SubstitutionReference { .. }    | TreeNodeType::TitleReference { .. }
      | TreeNodeType::AbsoluteURI { .. }              | TreeNodeType::StandaloneEmail { .. }
      | TreeNodeType::WhiteSpace { .. }               | TreeNodeType::ExternalHyperlinkTarget { .. }
      | TreeNodeType::IndirectHyperlinkTarget { .. }  | TreeNodeType::Text { .. }
      | TreeNodeType::EmptyLine
        => None,
      _ => Some(Vec::<TreeNode>::new())
    }
  }


  /// ### shared_target_label
  /// Returns a shared (immutable) reference to the optional target label.
  /// If the label is `None`, as is hasn't been set, returns an
  /// empty string slice instead.
  pub fn shared_target_label (&self) -> &str {
    if let Some(label) = self.target_label.as_ref() { label } else { "" }
  }
  
  
  /// ### push_child
  /// Pushes a given child node the the end of `self.children`.
  pub fn push_child (&mut self, node : TreeNode) {

    if self.child_is_allowed(&node.data) {
      if let Some(children) = &mut self.children {
        children.push(node);
      } else {
        panic!("This type of node is not allowed to have children.\nComputer says no...\n")
      }
      
    } else {
      eprintln!("Child of type {:#?} not allowed inside a {:#?}.\nComputer says no...\n", node.data, self.data);
      panic!();
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


  /// ### set_target_label
  /// Sets the target label of the node to given `Option<String>`.
  pub fn set_target_label (&mut self, label: Option<String>) {
    self.target_label = label;
  }


  /// ### child_is_allowed
  /// Checks whether a node is allowed to be inserted into another node.
  pub fn child_is_allowed (&self, node_data: &TreeNodeType) -> bool {

    match self.data {

      // Structural nodes can other (sub)srutural elements
      TreeNodeType::Document { .. } | TreeNodeType::Section { .. } | TreeNodeType::Topic
      | TreeNodeType::Sidebar => {
        match node_data {
          TreeNodeType::Section { .. }                    | TreeNodeType::Transition                      | TreeNodeType::Topic
          | TreeNodeType:: Sidebar
          | TreeNodeType::Paragraph { .. }                | TreeNodeType::BulletList { .. }               | TreeNodeType::EnumeratedList { .. }
          | TreeNodeType::DefinitionList { .. }           | TreeNodeType::FieldList { .. }                | TreeNodeType::OptionList
          | TreeNodeType::LiteralBlock { .. }             | TreeNodeType::LineBlock                       | TreeNodeType::BlockQuote
          | TreeNodeType::DoctestBlock                    | TreeNodeType::Footnote  { .. }                | TreeNodeType::Citation { .. }
          | TreeNodeType::ExternalHyperlinkTarget { .. }  | TreeNodeType::IndirectHyperlinkTarget { .. }  | TreeNodeType::SubstitutionDefinition
          | TreeNodeType::Comment                         | TreeNodeType::EmptyLine                       | TreeNodeType::Directive( .. )
            => true,
          _ => false
        }
      }

      // These (sub)body elements are allowed to contain body level nodes
      TreeNodeType::BulletListItem { .. }       | TreeNodeType::EnumeratedListItem { .. }
      | TreeNodeType::DefinitionListItem { .. } | TreeNodeType::FieldListItem { .. }      | TreeNodeType::OptionListItem
      | TreeNodeType::BlockQuote                | TreeNodeType::Footnote { .. }           | TreeNodeType::Citation { .. }
      | TreeNodeType::Directive( .. ) => {
        match node_data {
          TreeNodeType::Paragraph { .. }                  | TreeNodeType::BulletList { .. }               | TreeNodeType::EnumeratedList { .. }
          | TreeNodeType::DefinitionList { .. }           | TreeNodeType::FieldList { .. }                | TreeNodeType::OptionList
          | TreeNodeType::LiteralBlock { .. }             | TreeNodeType::LineBlock                       | TreeNodeType::BlockQuote
          | TreeNodeType::DoctestBlock                    | TreeNodeType::Footnote  { .. }                | TreeNodeType::Citation { .. }
          | TreeNodeType::ExternalHyperlinkTarget { .. }  | TreeNodeType::IndirectHyperlinkTarget { .. }  | TreeNodeType::SubstitutionDefinition
          | TreeNodeType::Comment                         | TreeNodeType::EmptyLine                       | TreeNodeType::Transition
          | TreeNodeType::Section { .. }                  | TreeNodeType::Directive( .. )
            => true,
          _ => false
        }
      },

      // Bullet lists may only contain empty lines or bullet list items
      TreeNodeType::BulletList { .. } => {
        match node_data {
          TreeNodeType::EmptyLine | TreeNodeType::BulletListItem { .. } => true,
          _ => false
        }
      }

      // Enumerated lists may only contain empty lines or enumerated list items
      TreeNodeType::EnumeratedList { .. } => {
        match node_data {
          TreeNodeType::EmptyLine | TreeNodeType::EnumeratedListItem { .. } => true,
          _ => false
        }
      }

      // Field lists may only contain empty lines or field list items
      TreeNodeType::FieldList { .. } => {
        match node_data {
          TreeNodeType::EmptyLine | TreeNodeType::FieldListItem { .. } => true,
          _ => false
        }
      }

      // Definition lists may only contain empty lines or definition list items
      TreeNodeType::DefinitionList { .. } => {
        match node_data {
          TreeNodeType::EmptyLine | TreeNodeType::DefinitionListItem { .. } => true,
          _ => false
        }
      }

      // Option lists may only contain empty lines or option list items
      TreeNodeType::OptionList { .. } => {
        match node_data {
          TreeNodeType::EmptyLine | TreeNodeType::OptionListItem { .. } => true,
          _ => false
        }
      }

      // Only paragraphs may contain inline nodes
      TreeNodeType::Paragraph { .. } => {
        match node_data {
          TreeNodeType::Emphasis { .. }             | TreeNodeType::StrongEmphasis { .. }         | TreeNodeType::InterpretedText
          | TreeNodeType::Literal { .. }            | TreeNodeType::InlineTarget { .. }           | TreeNodeType::FootnoteReference { .. }
          | TreeNodeType::CitationReference { .. }  | TreeNodeType::SubstitutionReference { .. }  | TreeNodeType::AbsoluteURI { .. }
          | TreeNodeType::StandaloneEmail { .. }    | TreeNodeType::Text { .. }                   | TreeNodeType::WhiteSpace { .. }
            => true,
          _ => false
        }
      },
      _ => false
    }
  }


  /// ### child
  /// Returns a reference to a child node of a given index.
  /// Panics, if the child does not exist.
  pub fn child (&self, index: usize) -> &Self {
    if let Some(children) = &self.children {
      match children.get(index) {
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