/*!
A submodule that defines the transforms performed for each node type,
before the tree is printed. These include things like transforming the
second child of a `Figure` node to a `Caption`, if it is a `Paragraph`.

Copyright © 2020 Santtu Söderholm <santtu.soderholm@tuni.fi>
*/
use crate::doctree::tree_node::TreeNode;
use crate::doctree::tree_zipper::TreeZipper;
use crate::doctree::DocTree;

impl DocTree {
    /// Modifies `self.tree` with the known reStrucuturedText transforms.
    pub fn perform_restructuredtext_transforms(mut self) -> Self {
        self = self.walk_to_root();
        self.tree = self.tree.perform_restructuredtext_transforms();
        self = self.walk_to_root();
        self
    }
}

impl TreeZipper {
    /// Recursively modifies the data of `self.node` and its children,
    /// based on the node type `self.node.data`.
    pub fn perform_restructuredtext_transforms(mut self) -> Self {
        self.mut_node().perform_restructuredtext_transforms();
        self
    }
}

impl TreeNode {
    /// Transforms `self.data` into a different type based on its current value.
    /// This is where the recursion really kicks in.
    pub fn perform_restructuredtext_transforms(&mut self) {
        use crate::doctree::tree_node_types::TreeNodeType;

        match self.mut_data() {
            TreeNodeType::Abbreviation { .. } => {}
            TreeNodeType::AbsoluteURI { .. } => {}
            TreeNodeType::Acronym { .. } => {}
            TreeNodeType::Address => {}
            TreeNodeType::Admonition { .. } => {}
            TreeNodeType::Attribution { .. } => {}
            TreeNodeType::Author { .. } => {}
            TreeNodeType::Authors { .. } => {}
            TreeNodeType::AutomaticSectionNumbering { .. } => {}
            TreeNodeType::BlockQuote { .. } => {}
            TreeNodeType::BulletList { .. } => {}
            TreeNodeType::BulletListItem { .. } => {}
            TreeNodeType::Caption { .. } => {}
            TreeNodeType::Citation { .. } => {}
            TreeNodeType::CitationReference { .. } => {}
            TreeNodeType::Class { .. } => {}
            TreeNodeType::Classifier { .. } => {}
            TreeNodeType::Code { .. } => {}
            TreeNodeType::ColSpec { .. } => {}
            TreeNodeType::Comment { .. } => {}
            TreeNodeType::CompoundParagraph { .. } => {}
            TreeNodeType::Contact { .. } => {}
            TreeNodeType::Container { .. } => {}
            TreeNodeType::Copyright { .. } => {}
            TreeNodeType::CSVTable { .. } => {}
            TreeNodeType::Date => {}
            TreeNodeType::Decoration => {}
            TreeNodeType::Definition => {}
            TreeNodeType::DefinitionList { .. } => {}
            TreeNodeType::DefinitionListItem { .. } => {}
            TreeNodeType::Description => {}
            TreeNodeType::DocInfo => {}
            TreeNodeType::DoctestBlock { .. } => {}
            TreeNodeType::Document { .. } => {}
            TreeNodeType::Emphasis { .. } => {}
            TreeNodeType::EmptyLine => {}
            TreeNodeType::Entry { .. } => {}
            TreeNodeType::EnumeratedList { .. } => {}
            TreeNodeType::EnumeratedListItem { .. } => {}
            TreeNodeType::ExternalHyperlinkTarget { .. } => {}
            TreeNodeType::Field => {}
            TreeNodeType::FieldBody { .. } => {}
            TreeNodeType::FieldList { .. } => {}
            TreeNodeType::FieldListItem { .. } => {}
            TreeNodeType::Figure { .. } => {
                if let Some(children) = self.mut_children() {
                    if let Some(child) = children.get_mut(1) {
                        if let TreeNodeType::Paragraph { indent } = child.mut_data() {
                            // Transform paragraph data into a caption
                            *child.mut_data() = TreeNodeType::Caption { indent: *indent };
                        }
                    } else {
                        // Do nothing
                    }
                }
            }
            TreeNodeType::Footer { .. } => {}
            TreeNodeType::Footnote { .. } => {}
            TreeNodeType::FootnoteReference { .. } => {}
            TreeNodeType::Header { .. } => {}
            TreeNodeType::Generated => {}
            TreeNodeType::Image { .. } => {}
            TreeNodeType::Include { .. } => {}
            TreeNodeType::IndirectHyperlinkTarget { .. } => {}
            TreeNodeType::Inline { .. } => {}
            TreeNodeType::InlineTarget { .. } => {}
            TreeNodeType::InterpretedText { .. } => {}
            TreeNodeType::Label { .. } => {}
            TreeNodeType::Legend { .. } => {}
            TreeNodeType::Line { .. } => {}
            TreeNodeType::LineBlock { .. } => {}
            TreeNodeType::ListTable { .. } => {
                // Reconstruct the contained bullet list into a table
                let rows: Vec<TreeNodeType> = Vec::new();

                if let Some(children) = self.mut_children() {
                    if let Some(bullet_list) = children.get_mut(0) {
                        if let TreeNodeType::BulletList {
                            bullet,
                            bullet_indent,
                            text_indent,
                        } = bullet_list.mut_data()
                        {
                            // Transform the contained bullet list into a table body...
                            *bullet_list.mut_data() = TreeNodeType::TBody;

                            // Retrieve the list items from the bullet list...
                            if let Some(list_items) = bullet_list.mut_children() {
                                // Iterate over the list items and transform them into table rows
                                for list_item in list_items {
                                    if let TreeNodeType::BulletListItem { .. } =
                                        list_item.mut_data()
                                    {
                                        *list_item.mut_data() = TreeNodeType::TRow;

                                        if let Some(list_item_children) = list_item.mut_children() {
                                            if let Some(nested_child) =
                                                list_item_children.get_mut(0)
                                            {
                                                if let TreeNodeType::BulletList { .. } =
                                                    nested_child.mut_data()
                                                {
                                                    // Remove the list items from nested bullet list and turn them into table cells or entries
                                                    let mut table_row_cells: Vec<TreeNode> =
                                                        if let Some(cells) =
                                                            nested_child.mut_children()
                                                        {
                                                            cells.drain(..).collect()
                                                        } else {
                                                            panic!("List table row has no cells. Computer says no...")
                                                        };

                                                    let n_of_entries = table_row_cells.len();
                                                    for cell in table_row_cells
                                                        .iter_mut()
                                                        .take(n_of_entries - 1)
                                                    {
                                                        *cell.mut_data() =
                                                            TreeNodeType::Entry { is_last: false };
                                                    }

                                                    if let Some(entry) = table_row_cells.last_mut()
                                                    {
                                                        *entry.mut_data() =
                                                            TreeNodeType::Entry { is_last: true };
                                                    }

                                                    // Remove the bullet list from between table row and table cells...
                                                    list_item
                                                        .mut_children()
                                                        .as_mut()
                                                        .unwrap()
                                                        .drain(..);
                                                    // Insert entries into table row...
                                                    list_item.append_children(&mut table_row_cells);
                                                }
                                            }
                                        }
                                    } else if let TreeNodeType::EmptyLine = list_item.mut_data() {
                                        // Keep as is
                                    } else {
                                        eprintln!("Cannot transform anything other than bullet list items or empty lines inside a list table...")
                                    }
                                }
                            }
                        }
                    }
                }
            }
            TreeNodeType::Literal { .. } => {}
            TreeNodeType::LiteralBlock { .. } => {}
            TreeNodeType::Math { .. } => {}
            TreeNodeType::MathBlock { .. } => {}
            TreeNodeType::OptionList { .. } => {}
            TreeNodeType::OptionListItem { .. } => {}
            TreeNodeType::OptionString { .. } => {}
            TreeNodeType::Organization { .. } => {}
            TreeNodeType::Paragraph { .. } => {}
            TreeNodeType::ParsedLiteralBlock { .. } => {}
            TreeNodeType::Pending { .. } => {}
            TreeNodeType::Problematic { .. } => {}
            TreeNodeType::Raw { .. } => {}
            TreeNodeType::Reference { .. } => {}
            TreeNodeType::Revision { .. } => {}
            TreeNodeType::Row { .. } => {}
            TreeNodeType::Rubric { .. } => {}
            TreeNodeType::Section { .. } => {}
            TreeNodeType::Sidebar { .. } => {}
            TreeNodeType::Status { .. } => {}
            TreeNodeType::StrongEmphasis { .. } => {}
            TreeNodeType::Subscript { .. } => {}
            TreeNodeType::SubstitutionDefinition { .. } => {}
            TreeNodeType::SubstitutionReference { .. } => {}
            TreeNodeType::Subtitle { .. } => {}
            TreeNodeType::Superscript { .. } => {}
            TreeNodeType::SystemMessage { .. } => {}
            TreeNodeType::Table { .. } => {}
            TreeNodeType::Target { .. } => {}
            TreeNodeType::TBody { .. } => {}
            TreeNodeType::Term { .. } => {}
            TreeNodeType::Text { .. } => {}
            TreeNodeType::TGroup { .. } => {}
            TreeNodeType::THead { .. } => {}
            TreeNodeType::TRow { .. } => {}
            TreeNodeType::Title { .. } => {}
            TreeNodeType::TitleReference { .. } => {}
            TreeNodeType::Topic { .. } => {}
            TreeNodeType::Transition {} => {}
            TreeNodeType::UnknownDirective { .. } => {}
            TreeNodeType::Version { .. } => {}
            TreeNodeType::WhiteSpace { .. } => {}

            // ============================
            //  Sphinx specific directives
            // ============================
            TreeNodeType::SphinxOnly { .. } => {}
            TreeNodeType::SphinxCodeBlock { .. } => {}

            // ========================
            //  A+ specific directives
            // ========================
            TreeNodeType::AplusPOI { .. } => {}
            TreeNodeType::AplusColBreak => {}
            TreeNodeType::AplusQuestionnaire { .. } => {}
            TreeNodeType::AplusPickOne { .. } => {}
            TreeNodeType::AplusPickAny { .. } => {}
            TreeNodeType::AplusFreeText { .. } => {}
            TreeNodeType::AplusPickChoices { .. } => {}
            TreeNodeType::AplusPickChoice { .. } => {}
            TreeNodeType::AplusQuestionnaireHints { .. } => {}
            TreeNodeType::AplusQuestionnaireHint { .. } => {}
            TreeNodeType::AplusSubmit { .. } => {}
            TreeNodeType::AplusActiveElementInput { .. } => {}
            TreeNodeType::AplusActiveElementOutput { .. } => {}
        };

        if let Some(children) = self.mut_children() {
            for child in children {
                child.perform_restructuredtext_transforms()
            }
        }
    }
}
