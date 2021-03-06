/*!
A submodule for the `TreeNode` type.

Copyright © 2020 Santtu Söderholm
*/

use super::*;

/// A tree node that contains a struct of `TreeNodeType`
/// plus the information needed to traverse the tree.
#[derive(Debug)]
pub struct TreeNode {
    id: NodeId,
    refnames: Option<Vec<String>>,
    classes: Option<Vec<String>>,
    data: TreeNodeType,
    children: Option<Children>,
}

impl TreeNode {

    /// A `TreeNode` constructor.
    pub fn new(
        data: TreeNodeType,
        id: NodeId,
        target_label: Option<Vec<String>>,
        classes: Option<Vec<String>>,
    ) -> Self {
        TreeNode {
            id: id,
            refnames: target_label,
            classes: classes,
            children: Self::children_or_none(&data),
            data: data,
        }
    }

    /// Set the children of a `TreeNode` to `Some(Children)` or `None`,
    /// depending on the given node data variant.
    fn children_or_none(data_variant: &TreeNodeType) -> Option<Children> {
        use crate::doctree::node_categories::NodeCategory;

        let mut node_type_properties = data_variant.node_categories();

        if node_type_properties.any(|cat| match cat {
            NodeCategory::CompoundStructural
            | NodeCategory::CompoundBody
            | NodeCategory::CompoundSubBody => true,
            _ => false,
        }) {
            Some(Vec::new())
        } else if let TreeNodeType::Paragraph { .. }
        | TreeNodeType::AplusPickChoice { .. }
        | TreeNodeType::AplusQuestionnaireHint { .. } = data_variant
        {
            Some(Vec::new())
        } else {
            None
        }
    }

    /// Returns a copy of own id.
    pub fn id(&self) -> NodeId {
        self.id
    }

    /// Returns a shared (immutable) reference to the optional target label.
    /// If the label is `None`, as is hasn't been set, returns an
    /// empty string slice instead.
    pub fn shared_target_label(&self) -> &Option<Vec<String>> {
        &self.refnames
    }

    /// Pushes a given child node the the end of `self.children`.
    pub fn push_child(&mut self, node: TreeNode) -> Result<(), TreeNode> {
        if self.child_is_allowed(&node.data) {
            if let Some(children) = &mut self.children {
                children.push(node);
                Ok(())
            } else {
                eprintln!(
                    "Node of type {} is not allowed to have children.",
                    self.data
                );
                Err(node)
            }
        } else {
            eprintln!(
                "Child of type {} not allowed inside {}.",
                node.data, self.data
            );
            Err(node)
        }
    }

    /// Removes the last child from own children and returns it in a `Result`, if successful.
    /// If no whilc is found, returns `Err(())`.
    pub fn pop_child(&mut self) -> Option<Self> {
        if let Some(children) = self.mut_children() {
            if let Some(child) = children.pop() {
                Some(child)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Appends multiple children to `self.children`.
    pub fn append_children(&mut self, children: &mut Vec<TreeNode>) {
        // Check whether all children are valid
        for child in children.iter() {
            if self.child_is_allowed(&child.data) {
                continue;
            } else {
                panic!("Found incompatible child {} when appending children to {}. Computer says no...", child.data, self.data)
            }
        }

        if let Some(child_vec) = &mut self.children {
            child_vec.append(children);
        } else {
            panic!(
                "Node of type {} is not allowed to have children. Computer says no...",
                self.data
            )
        }
    }

    /// Retuns a shared reference to own target labels.
    pub fn shared_target_labels(&self) -> &Option<Vec<String>> {
        &self.refnames
    }

    /// Sets the target label of the node to given `Option<Vec<String>>`.
    pub fn set_target_label(&mut self, label: Option<Vec<String>>) {
        self.refnames = label;
    }

    /// Optionally returns a shared reference to vector containing child nodes.
    pub fn shared_children(&self) -> &Option<Children> {
        &self.children
    }

    /// Optionally returns a mutable reference to vector containing child nodes.
    pub fn mut_children(&mut self) -> &mut Option<Children> {
        &mut self.children
    }

    /// Checks whether a node is allowed to be inserted into another node.
    pub fn child_is_allowed(&self, node_data: &TreeNodeType) -> bool {
        use crate::doctree::node_categories::NodeCategory;

        match self.data {
            TreeNodeType::Abbreviation { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::AbsoluteURI { .. } => false,
            TreeNodeType::Acronym { .. } => false, // No documentation on docutils!
            TreeNodeType::Address => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Admonition { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Attribution { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Author { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Authors { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::AutomaticSectionNumbering { .. } => false, // Not really a node in rST
            TreeNodeType::BlockQuote { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else if let TreeNodeType::Attribution { .. } = node_data {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::BulletList { .. } => match node_data {
                TreeNodeType::EmptyLine | TreeNodeType::BulletListItem { .. } => true,
                _ => false,
            },
            TreeNodeType::BulletListItem { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Caption { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Citation { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::CitationReference { .. } => false,
            TreeNodeType::Class { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Classifier { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Code { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::ColSpec { .. } => false,
            TreeNodeType::Comment { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::CompoundParagraph { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Contact { .. } => false,
            TreeNodeType::Container { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Copyright { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::CSVTable { .. } => todo!(),
            TreeNodeType::Date => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Decoration => match node_data {
                TreeNodeType::Footer { .. } | TreeNodeType::Header { .. } => true,
                _ => false,
            },
            TreeNodeType::Definition => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::DefinitionList { .. } => match node_data {
                TreeNodeType::EmptyLine | TreeNodeType::DefinitionListItem { .. } => true,
                _ => false,
            },
            TreeNodeType::DefinitionListItem { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Description => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::DocInfo => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Bibliographic = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::DoctestBlock { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Document { .. } => {
                if node_data.node_categories().any(|cat| match cat {
                    NodeCategory::Structural | NodeCategory::SubStructural | NodeCategory::Body => {
                        true
                    }
                    _ => false,
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Emphasis { .. } => false,
            TreeNodeType::EmptyLine => false,
            TreeNodeType::Entry { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::EnumeratedList { .. } => match node_data {
                TreeNodeType::EmptyLine | TreeNodeType::EnumeratedListItem { .. } => true,
                _ => false,
            },
            TreeNodeType::EnumeratedListItem { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::ExternalHyperlinkTarget { .. } => false,
            TreeNodeType::Field => todo!(),
            TreeNodeType::FieldBody { .. } => todo!(),
            TreeNodeType::FieldList { .. } => match node_data {
                TreeNodeType::EmptyLine | TreeNodeType::FieldListItem { .. } => true,
                _ => false,
            },
            TreeNodeType::FieldListItem { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Figure { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) || match node_data {
                    TreeNodeType::Caption { .. } | TreeNodeType::Legend { .. } => true,
                    _ => false,
                } {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Footer { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Footnote { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::FootnoteReference { .. } => todo!(),
            TreeNodeType::Header { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Generated => todo!(),
            TreeNodeType::Image { .. } => false,
            TreeNodeType::Include { .. } => false,
            TreeNodeType::IndirectHyperlinkTarget { .. } => false,
            TreeNodeType::Inline { .. } => false,
            TreeNodeType::InlineTarget { .. } => false,
            TreeNodeType::InterpretedText { .. } => false,
            TreeNodeType::Label { .. } => todo!(),
            TreeNodeType::Legend { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Line { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::LineBlock { .. } => match node_data {
                TreeNodeType::EmptyLine | TreeNodeType::Line { .. } => true,
                _ => false,
            },
            TreeNodeType::ListTable { .. } => match node_data {
                TreeNodeType::BulletList { .. } => true,
                _ => false,
            },
            TreeNodeType::Literal { .. } => false,
            TreeNodeType::LiteralBlock { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Math { .. } => false,
            TreeNodeType::MathBlock { .. } => false,
            TreeNodeType::OptionList { .. } => match node_data {
                TreeNodeType::EmptyLine | TreeNodeType::OptionListItem { .. } => true,
                _ => false,
            },
            TreeNodeType::OptionListItem { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::OptionString { .. } => todo!(),
            TreeNodeType::Organization { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Paragraph { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::ParsedLiteralBlock { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Pending { .. } => {
                todo!("No information on \"Pending\" node children in docutils documentation...")
            }
            TreeNodeType::Problematic { .. } => todo!(
                "No information on \"Problematic\" node children in docutils documentation..."
            ),
            TreeNodeType::Raw { .. } => {
                todo!("What is a \"Raw\" element supposed to be, exactly...?")
            }
            TreeNodeType::Reference { .. } => false, // inline ref
            TreeNodeType::Revision { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Row { .. } => todo!("No documentation on table rows in docutils..."),
            TreeNodeType::Rubric { .. } => false,
            TreeNodeType::Section { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else if match node_data {
                    TreeNodeType::Transition
                    | TreeNodeType::Section { .. }
                    | TreeNodeType::Topic { .. }
                    | TreeNodeType::Sidebar { .. } => true,
                    _ => false,
                } {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Sidebar { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else if let TreeNodeType::Topic { .. } = node_data {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Status { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::StrongEmphasis { .. } => false,
            TreeNodeType::Subscript { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::SubstitutionDefinition { .. } => false,
            TreeNodeType::SubstitutionReference { .. } => false,
            TreeNodeType::Subtitle { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Superscript { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::SystemMessage { .. } => todo!(),
            TreeNodeType::Table { .. } => todo!(),
            TreeNodeType::Target { .. } => false,
            TreeNodeType::TBody { .. } => todo!(),
            TreeNodeType::Term { .. } => todo!(),
            TreeNodeType::Text { .. } => false,
            TreeNodeType::TGroup { .. } => todo!(),
            TreeNodeType::THead { .. } => todo!(),
            TreeNodeType::TRow { .. } => {
                if let TreeNodeType::Entry { .. } = node_data {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Title { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::TitleReference { .. } => false,
            TreeNodeType::Topic { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Transition {} => todo!(),
            TreeNodeType::UnknownDirective { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::Version { .. } => false,
            TreeNodeType::WhiteSpace { .. } => false,

            // ============================
            //  Sphinx specific directives
            // ============================
            TreeNodeType::SphinxOnly { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::SphinxCodeBlock { .. } => false,

            // ========================
            //  A+ specific directives
            // ========================
            TreeNodeType::AplusPOI { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else if let TreeNodeType::AplusColBreak = node_data {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::AplusColBreak => false,
            TreeNodeType::AplusQuestionnaire { .. } => match node_data {
                TreeNodeType::EmptyLine
                | TreeNodeType::Paragraph { .. }
                | TreeNodeType::AplusPickOne { .. }
                | TreeNodeType::AplusPickAny { .. }
                | TreeNodeType::AplusFreeText { .. } => true,
                _ => false,
            },
            TreeNodeType::AplusPickOne { .. } => match node_data {
                TreeNodeType::Paragraph { .. }
                | TreeNodeType::AplusPickChoices { .. }
                | TreeNodeType::AplusQuestionnaireHints { .. } => true,
                _ => false,
            },
            TreeNodeType::AplusPickAny { .. } => match node_data {
                TreeNodeType::Paragraph { .. }
                | TreeNodeType::AplusPickChoices { .. }
                | TreeNodeType::AplusQuestionnaireHints { .. } => true,
                _ => false,
            },
            TreeNodeType::AplusFreeText { .. } => match node_data {
                TreeNodeType::Paragraph { .. }
                | TreeNodeType::AplusPickChoices { .. }
                | TreeNodeType::AplusQuestionnaireHints { .. } => true,
                _ => false,
            },
            TreeNodeType::AplusPickChoices { .. } => match node_data {
                TreeNodeType::AplusPickChoice { .. } => true,
                _ => false,
            },
            TreeNodeType::AplusPickChoice { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::AplusQuestionnaireHints { .. } => match node_data {
                TreeNodeType::AplusQuestionnaireHint { .. } => true,
                _ => false,
            },
            TreeNodeType::AplusQuestionnaireHint { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Inline = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::AplusSubmit { .. } => {
                if node_data.node_categories().any(|cat| {
                    if let NodeCategory::Body = cat {
                        true
                    } else {
                        false
                    }
                }) {
                    true
                } else {
                    false
                }
            }
            TreeNodeType::AplusActiveElementInput { .. } => false,
            TreeNodeType::AplusActiveElementOutput { .. } => false,
        }
    }

    /// Returns a shared reference to a child node of a given index.
    /// Panics, if the child does not exist.
    pub fn shared_child(&self, index: usize) -> Option<&Self> {
        if let Some(children) = &self.children {
            match children.get(index) {
                Some(node) => Some(node),
                None => None,
            }
        } else {
            None
        }
    }

    /// Returns a mutable reference to a child node of a given index.
    /// Panics, if the child does not exist.
    pub fn mut_child(&mut self, index: usize) -> Option<&mut Self> {
        if let Some(children) = &mut self.children {
            match children.get_mut(index) {
                Some(node) => Some(node),
                None => None,
            }
        } else {
            None
        }
    }

    /// For retrieving an immutable reference to the data type of a node.
    pub fn shared_data(&self) -> &TreeNodeType {
        &self.data
    }

    /// For retrieving a mutable reference to the data type of a node.
    pub fn mut_data(&mut self) -> &mut TreeNodeType {
        &mut self.data
    }
}
