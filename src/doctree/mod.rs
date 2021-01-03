/*!
This module defines the document tree and its nodes.
The implementation is in the form of a zipper:
the `DocTree` is a container for the metadata (such as hyperlinks) related to the parsing of the tree
and store the tree in the form a a `TreeZipper` in its field `DocTree.tree`.
The `TreeZipper` type is specified in the submodule `crate::parser::tree_zipper`.

Copyright © 2020 Santtu Söderholm
*/
use std::collections::HashMap;

mod larst_writer;
mod node_categories;
mod restructuredtext_transforms;
mod tree_zipper;
use tree_zipper::TreeZipper;
pub mod tree_node;
use tree_node::TreeNode;
pub mod tree_node_types;
use tree_node_types::TreeNodeType;
pub mod directives;
mod hyperref_data;
use hyperref_data::{HyperrefData, ANON_REF_LABEL_PREFIX, ANON_REF_LABEL_SUFFIX};
mod class_data;
use class_data::ClassData;
mod section_data;
use section_data::SectionData;
mod walkers;

use crate::common::{
    EnumDelims, EnumKind, FootnoteKind, HTMLAlignment, HorizontalAlignment, Length, MetricType,
    NodeId, SectionLineStyle, TableColWidths, ToCBacklinks,
};

// --------------
//  Test module
// --------------
mod tests;

/// A container for the document tree.
/// In addition to holding ownership of the
/// tree (stored in a zipper), also contains
/// metadata about the tree.
pub struct DocTree {

    /// The canonicalized file path without the file suffix.
    filename_stem: String,

    /// The path to the folder the source file is stored in.
    /// The object file will be stored in the same folder with a different suffix.
    file_folder: String,

    /// Holds the tree focused on a specific node.
    tree: TreeZipper,

    /// Keeps track of how many nodes have been added to the tree thus far
    /// besides the root node, that gets an ID of `0`. Some nodes might differ
    /// in their behaviour depending on their insertion order into the tree.
    /// For example, a field list will be transformed into bibliographic data,
    /// if it is the first non-(whitespace|comment) node in the tree.
    node_count: NodeId,

    /// The container for hyperref data related to the doctree.
    hyperref_data: HyperrefData,

    /// A container that holds on to the possibly generated HTML classes.
    class_data: ClassData,

    /// A container that keeps track of known section styles and section levels corresponding to them.
    section_data: SectionData,
}

use std::path::PathBuf;

/// Document tree container methods
impl DocTree {

    /// A `DocTree` constructor.
    pub fn new(doc_name: PathBuf) -> Self {
        let root_id: NodeId = 0;
        let root_data = TreeNodeType::Document;
        let root_node = TreeNode::new(root_data, root_id, None, None);

        let file_stem: String = if let Some(path_os_str) = doc_name.file_stem() {
            if let Some(path_str) = path_os_str.to_str() {
                path_str.to_string()
            } else {
                panic!("Invalid unicode in file path. Computer says no...")
            }
        } else {
            // eprintln!("No recognizable source file name to be found. Computer says no...");
            String::new()
        };

        let file_folder = if let Some(parent) = doc_name.parent() {
            if let Some(path_str) = parent.to_str() {
                path_str.to_string()
            } else {
                panic!("Source folder path could not be converted to a string. Computer says no...")
            }
        } else {
            // eprintln!("Source is not in any folder (even root). Computer says no...");
            String::new()
        };

        DocTree {
            filename_stem: file_stem,
            file_folder: file_folder,
            tree: TreeZipper::new(root_node, None, None),
            node_count: root_id + 1,
            hyperref_data: HyperrefData::new(),
            class_data: ClassData::new(),
            section_data: SectionData::new(),
        }
    }

    /// Returns the value of the contnained node counter.
    pub fn n_of_nodes(&self) -> NodeId {
        self.node_count
    }

    /// Mainly for debugging purposes.
    /// Prints the contaiend tree, focused on the current node.
    pub fn print_tree(&self) {
        eprintln!("The Document Tree\n=================");
        eprintln!("{:#?}", self.tree)
    }

    /// Prints the currently focused on node.
    fn print_node(&self) {
        eprintln!("{:#?}", self.tree.shared_node())
    }

    /// Prints the id of the currently focused on node.
    fn print_node_id(&self) {
        eprintln!("{:#?}", self.tree.node_id())
    }

    /// Returns a copy of the current node count in the DocTree.
    pub fn node_count(&self) -> NodeId {
        self.node_count
    }

    /// mainly for debugging purposes
    /// Prints out the internal targe labels stored in `self.hyperref_data` currently being worked on.
    pub fn print_internal_labels(&self) {
        eprintln!(
            "{:#?}",
            self.hyperref_data
                .shared_accumulated_internal_target_label()
        );
    }

    /// Focuses `self.tree` on its parent node if there is one.
    pub fn focus_on_parent(mut self) -> Self {
        self.tree = match self.tree.focus_on_parent() {
            Ok(tree) => tree,
            Err(tree) => {
                eprintln!("INFO: Tried focusing on node parent but no parent found.\n");
                tree
            }
        };

        self
    }

    /// Creates a new node from given data, pushes it to the children of currently focused on node and focuses on the new node.
    /// If this succeeds, also increments `self.node_count`.
    /// Returns `Result::{Ok(self), Err(self)}`, depending on the success of this operation.
    pub fn push_data_and_focus(mut self, mut node_data: TreeNodeType) -> Result<Self, Self> {
        let target_labels = self.hyperref_actions(&mut node_data);
        let classes = self.classes();
        match self
            .tree
            .push_data_and_focus(node_data, self.node_count, target_labels, classes)
        {
            Ok(tree) => {
                self.node_count += 1;
                self.tree = tree;
                Ok(self)
            }
            Err(tree) => {
                self.tree = tree;
                Err(self)
            }
        }
    }

    /// Creates a new node from given data and pushes it to the children of currently focused on node.
    /// If this succeeds, also increments `self.node_count`.
    /// Returns self in either `Ok` or an `Err`.
    pub fn push_data(mut self, mut node_data: TreeNodeType) -> Result<Self, Self> {
        let target_labels = self.hyperref_actions(&mut node_data);
        let classes = self.classes();
        match self
            .tree
            .push_data(node_data, self.node_count, target_labels, classes)
        {
            Ok(tree) => {
                self.tree = tree;
                self.node_count += 1;
                Ok(self)
            }
            Err(tree) => {
                self.tree = tree;
                Err(self)
            }
        }
    }

    /// Pushes a new node to the children of the node currently focused on.
    /// If the addition was successful, returns `Ok(())`, else returns the given node wrapped in an `Err`.
    pub fn push_child(&mut self, mut node: TreeNode) -> Result<(), TreeNode> {

        let incoming_target_labels = self.hyperref_actions(node.mut_data());
        node.set_target_label(incoming_target_labels);
        match self.tree.push_child(node) {
            Ok(()) => {
                self.node_count += 1;
                Ok(())
            }
            Err(node) => Err(node),
        }
    }
    /// Removes the last child of the  current node and returns in an `Option`.
    pub fn pop_child(&mut self) -> Option<TreeNode> {
        match self.tree.pop_child() {
            Some(node) => Some(node),
            None => None,
        }
    }

    /// Performs any node specific hyperref label additions to the doctree based on given node data.
    /// Returns an optional internal target label.
    fn hyperref_actions(&mut self, node_data: &mut TreeNodeType) -> Option<Vec<String>> {

        use crate::common::normalize_refname;

        // Check if there is an incoming internal target label
        let accumulated_target_label = self.hyperref_data.mut_accumulated_internal_target_label();
        let mut target_labels: Vec<String> = if accumulated_target_label.is_empty() {
            Vec::new()
        } else {
            match node_data {
                TreeNodeType::EmptyLine | TreeNodeType::WhiteSpace { .. } => Vec::new(),
                _ => {
                    let labels = accumulated_target_label.drain(..).collect();
                    accumulated_target_label.clear();
                    labels
                }
            }
        };

        // Check for targetable or referential nodes. If one is encountered,
        // add it to the known targets or references.
        match node_data {
            TreeNodeType::Footnote { target, label, kind, .. } => {
                match kind {
                    FootnoteKind::Manual => {
                        let normalized_refname = normalize_refname(label);
                        target_labels.push(normalized_refname);
                    }
                    FootnoteKind::AutoNumbered => {
                        match self.new_autonumber_footnote_label() {
                            Some(number) => {
                                *target = number.clone();
                                *label = number.clone();
                                target_labels.push(number)
                            }
                            None => ()
                        }
                    }
                    FootnoteKind::SimpleRefName => {
                        match self.new_autonumber_footnote_label() {
                            Some(number) => {
                                *target = number.clone();
                                *label = number.clone();
                                target_labels.push(number)
                            }
                            None => ()
                        }
                    }
                    FootnoteKind::AutoSymbol => {
                        match self.new_symbolic_footnote_label() {
                            Some(symbol) => {
                                *target = symbol.clone();
                                *label = symbol.clone();
                                target_labels.push(symbol)
                            }
                            None => ()
                        }
                    }
                };
                for label in target_labels.iter() {
                    self.add_target(label, self.node_count)
                }
                if let FootnoteKind::AutoSymbol = kind {
                    self.increment_symbolic_footnotes();
                }
            }
            TreeNodeType::FootnoteReference { displayed_text, target_label, kind } => {
                match kind {
                    FootnoteKind::Manual => target_labels.push(
                        crate::common::normalize_refname(&displayed_text)
                    ),
                    FootnoteKind::AutoNumbered => match self.new_autonumber_footnote_ref_label() {
                        Some(label) => {
                            *displayed_text = label.clone();
                            *target_label = label.clone();
                            target_labels.push(label)
                        },
                        None => return None
                    },
                    FootnoteKind::SimpleRefName => {
                        match self.new_autonumber_footnote_ref_label() {
                            Some(label) => {
                                *target_label = label.clone();
                                target_labels.push(label)
                            },
                            None => return None
                        }
                        target_labels.push(crate::common::normalize_refname(&displayed_text));
                    },
                    FootnoteKind::AutoSymbol => match self.new_symbolic_footnote_label() {
                        Some(label) => target_labels.push(label),
                        None => return None
                    }
                };
                for label in target_labels.iter() {
                    self.add_reference(&label, self.node_count)
                }
            },
            TreeNodeType::ExternalHyperlinkTarget { uri, target, .. } => {
                let normalized_refname = normalize_refname(target);
                target_labels.push(normalized_refname);
                for label in target_labels.iter() {
                    self.add_target(label,self.node_count);
                }
            }
            TreeNodeType::IndirectHyperlinkTarget {
                target,
                indirect_target,
                ..
            } => {
                let normalized_target_refname = normalize_refname(target);
                let normalized_indirect_refname = normalize_refname(target);
                for label in target_labels.iter() {
                    self.add_target(
                        label,
                        self.node_count,
                    );
                }
                self.add_reference(
                    &normalize_refname(normalized_indirect_refname.as_str()),
                    self.node_count,
                );
            }
            TreeNodeType::Section {
                title_text,
                level,
                line_style,
            } => {
                target_labels.push(normalize_refname(title_text));
                for label in target_labels.iter() {
                    self.add_target(label,self.node_count);
                }
                self.section_data.add_section_level(*line_style);
                if *level > self.section_data.highest_encountered_section_level() {
                    self.section_data.increment_encountered_section_number();
                }
            }
            _ => for label in target_labels.iter() {
                self.add_target(label,self.node_count);
            },
        };
        if target_labels.is_empty() { None } else { Some(target_labels) }
    }


    /// Returns the stack of incoming classes, if there are any.
    fn classes(&mut self) -> Option<Vec<String>> {
        let classes = self.class_data.mut_classes();
        if classes.is_empty() {
            None
        } else {
            Some(classes.drain(..).collect())
        }
    }

    /// Returns a shared reference to the current node .
    pub fn shared_node(&self) -> &TreeNode {
        self.tree.shared_node()
    }

    /// Returns a shared reference to the current node .
    pub fn mut_node(&mut self) -> &mut TreeNode {
        self.tree.mut_node()
    }

    /// Returns an optional shared reference to the current node's children, if the exist.
    pub fn shared_children(&self) -> Option<&Vec<TreeNode>> {
        if let Some(children) = self.tree.shared_children() {
            Some(children)
        } else {
            None
        }
    }

    /// Returns an optional mutable reference to the current node's children, if the exist.
    pub fn mut_children(&mut self) -> Option<&mut Vec<TreeNode>> {
        if let Some(children) = self.tree.mut_children() {
            Some(children)
        } else {
            None
        }
    }

    /// Retrieves a shared reference to the data of the
    /// currently focused on node.
    pub fn shared_node_data(&self) -> &TreeNodeType {
        self.tree.shared_node().shared_data()
    }

    /// Retrieves a shared reference to the data of the
    /// currently focused on node.
    pub fn mut_node_data(&mut self) -> &mut TreeNodeType {
        self.tree.mut_node().mut_data()
    }

    /// Retrieves a shared reference to the data of the given child of the current node.
    pub fn get_child_data(&self, index: usize) -> &TreeNodeType {
        if let Some(children) = self.tree.shared_node().shared_children() {
            match children.get(index) {
                Some(node) => node.shared_data(),
                None => {
                    eprintln!("Focused on node does not have as many children as is implied.\nComputer says no...\n");
                    panic!()
                }
            }
        } else {
            panic!("Cannot retrieve shared child data from a node that cannot have children. Computer says no...")
        }
    }

    pub fn n_of_children(&self) -> usize {
        self.tree.n_of_children()
    }

    /// Retrieves a mutable reference to the data of the given child of the current node.
    pub fn get_mut_child_data(&mut self, index: usize) -> &mut TreeNodeType {
        if let Some(children) = self.tree.mut_node().mut_children() {
            match children.get_mut(index) {
                Some(node) => node.mut_data(),
                None => {
                    eprintln!("Focused on node does not have as many children as is implied.\nComputer says no...\n");
                    panic!()
                }
            }
        } else {
            panic!("Cannot retrieve mutable child data from a node that cannot have children. Computer says no...")
        }
    }

    /// Retrieves a shared reference to a given child.
    pub fn shared_child(&self, index: usize) -> &TreeNode {
        if let Some(children) = self.tree.shared_node().shared_children() {
            match children.get(index) {
                Some(node) => node,
                None => {
                    panic!("Focused on node does not have as many children as is implied. Computer says no...")
                }
            }
        } else {
            panic!(
                "Cannot retrieve child from a node that cannot have children. Computer says no..."
            )
        }
    }

    /// Retrieves a shared reference to a given child.
    pub fn mut_child(&mut self, index: usize) -> &mut TreeNode {
        if let Some(children) = self.tree.mut_node().mut_children() {
            match children.get_mut(index) {
                Some(node) => node,
                None => {
                    panic!("Focused on node does not have as many children as is implied. Computer says no...")
                }
            }
        } else {
            panic!(
                "Cannot retrieve child from a node that cannot have children. Computer says no..."
            )
        }
    }

    /// Retrieves the node data of a sibling of the currently focused-on node with the given index.
    pub fn shared_sibling_data(&self, sibling_index: usize) -> Option<&TreeNodeType> {
        if let Some(sibling_data) = self.tree.shared_sibling_data(sibling_index) {
            Some(sibling_data)
        } else {
            eprintln!("Warning: No sibling with index {}...\n", sibling_index);
            None
        }
    }

    /// Retrieves the index of the current node with respect to its parent.
    pub fn index_in_parent(&self) -> Option<usize> {
        self.tree.index_in_parent()
    }

    /// Appends the nodes given in a given vector of nodes to the currently
    /// focused on node in `self.tree`.
    pub fn append_children(&mut self, nodes: &mut Children) {
        let children = nodes.len() as NodeId; // No overflow checks...
        self.tree.append_children(nodes);
        self.node_count += children;
    }

    /// Checks whether the doctree already contains a hyperlink target with the given label.
    pub fn has_target_label(&self, label_to_be_inspected_for: &str) -> bool {
        self.hyperref_data
            .shared_targets()
            .contains_key(label_to_be_inspected_for)
    }

    /// Checks whether the doctree already contains a hyperlink reference with the given label.
    pub fn has_reference_label(&self, label_to_be_inspected_for: &str) -> bool {
        self.hyperref_data
            .shared_references()
            .contains_key(label_to_be_inspected_for)
    }

    /// Retrieves a copy of the node id currently focused on.
    pub fn current_node_id(&self) -> NodeId {
        self.tree.node_id()
    }

    /// Adds a given label to the known hyperref targets or updates the actual targe node id
    /// if a label is already in the known labels.
    fn add_target(&mut self, label: &String, id: NodeId) {
        match self.hyperref_data.mut_targets().insert(label.clone(), id) {
            Some(node_id) => {
                eprintln!("Found an existing node with the target label \"{}\".\nReplacing duplicate node id value {} with {}...\n", label, node_id, id);
            }
            None => {}
        };
    }

    /// Adds a given label to the known hyperref targets or updates the actual targe node id
    /// if a label is already in the known labels.
    fn add_reference(&mut self, label: &String, id: NodeId) {
        match self
            .hyperref_data
            .mut_references()
            .insert(label.clone(), id)
        {
            Some(node_id) => {
                eprintln!("Found an existing node with the reference label \"{}\".\nReplacing duplicate node id value {} with {}...\n", label, node_id, id);
            }
            None => {}
        };
    }

    /// Pushes a given label to the chain of detected internal target labels.
    /// Once a non-internal target is encountered, this array of labels will be
    /// made to point to the newly detected node and cleared.
    pub fn push_to_internal_target_stack(&mut self, label: String) {
        self.hyperref_data.add_internal_target_label(label);
    }

    /// Returns the number of symbolic footnotes that have been entered into the doctree.
    pub fn n_of_symbolic_footnotes(&self) -> u32 {
        self.hyperref_data.n_of_symbolic_footnotes()
    }

    /// Returns the number of symbolic footnote references that have been entered into the doctree.
    pub fn n_of_symbolic_footnote_refs(&self) -> u32 {
        self.hyperref_data.n_of_symbolic_footnote_refs()
    }

    /// Increments symbolic footnote counter of the doctree by 1.
    pub fn increment_symbolic_footnotes(&mut self) {
        self.hyperref_data.increment_symbolic_footnote_counter_by(1);
    }

    /// Increments symbolic footnote reference counter of the doctree by 1.
    pub fn increment_symbolic_footnote_refs(&mut self) {
        self.hyperref_data
            .increment_symbolic_footnote_ref_counter_by(1);
    }

    /// Increases the counter for anonymous targets entered into the doctree thus far by one.
    pub fn increment_anon_targets(&mut self) {
        self.hyperref_data.increment_anonymous_target_counter_by(1);
    }

    /// Increases the counter for anonymous targets entered into the doctree thus far by one.
    pub fn increment_anon_references(&mut self) {
        self.hyperref_data
            .increment_anonymous_target_ref_counter_by(1);
    }

    /// Increments the anon target counter and returns a copy of the result.
    pub fn next_anon_target_n(&mut self) -> u32 {
        self.increment_anon_targets();
        self.hyperref_data.n_of_anon_targets()
    }

    /// Increments the anon reference counter and returns a copy of the result.
    pub fn next_anon_reference_n(&mut self) -> u32 {
        self.increment_anon_references();
        self.hyperref_data.n_of_anon_target_refs()
    }

    /// Returns an allocated String representation of the next anonymous target label.
    pub fn next_anon_target_label(&mut self) -> String {
        format!(
            "{}{}{}",
            ANON_REF_LABEL_PREFIX,
            self.next_anon_target_n(),
            ANON_REF_LABEL_SUFFIX
        )
    }

    /// Returns an allocated String representation of the next anonymous reference label.
    pub fn next_anon_reference_label(&mut self) -> String {
        format!(
            "{}{}{}",
            ANON_REF_LABEL_PREFIX,
            self.next_anon_reference_n(),
            ANON_REF_LABEL_SUFFIX
        )
    }

    /// Returns a shared reference to `self.targets`.
    pub fn shared_targets(&self) -> &HashMap<String, NodeId> {
        self.hyperref_data.shared_targets()
    }

    /// Returns a mutable reference to `self.targets`.
    pub fn mut_targets(&mut self) -> &mut HashMap<String, NodeId> {
        self.hyperref_data.mut_targets()
    }

    /// Returns a shared reference to `self.references`.
    pub fn shared_references(&self) -> &HashMap<String, NodeId> {
        self.hyperref_data.shared_references()
    }

    /// Returns a mutable reference to `self.references`.
    pub fn mut_references(&mut self) -> &mut HashMap<String, NodeId> {
        self.hyperref_data.mut_references()
    }

    /// Generates a new section node data container by comparing the given `section_style` to known styles
    /// and corresponding levels via `self.section_levels`. If a section of such style already exists, the level of the section
    /// is simply set to the level matching it. If not, the maximum known level is plus 1
    /// is assigned to the section data.

    /// Note that this function does not yet modify known section data or hyperref targets.
    /// This is donw only if pushing the node data to the tree succeeds, and is handled
    /// by the related methods.
    pub fn new_section_data(
        &self,
        title_text: &str,
        section_style: SectionLineStyle,
    ) -> TreeNodeType {
        let section_level = self.section_data.line_style_section_level(&section_style);
        TreeNodeType::Section {
            level: section_level,
            title_text: title_text.to_string(),
            line_style: section_style,
        }
    }

    /// Adds a new section to the doctree, also taking care of adding the section title
    /// to the hyperref data of the tree, updating the section counter and mapping
    /// the section type to the appropriate section level.
    pub fn add_section(mut self, title_text: &str, section_style: SectionLineStyle) -> Self {
        let section_data = self.new_section_data(title_text, section_style);

        self = match self.push_data(section_data) {
            Ok(tree) | Err(tree) => tree,
        };
        self
    }

    /// Walks up the tree to a given section level.
    pub fn walk_to_parent_section_level(mut self, level: usize) -> Self {
        self.tree = self.tree.walk_to_parent_section_level(level);
        self
    }

    /// Returns an `Option`al shared reference to the parent node.
    pub fn shared_parent_ref(&self) -> Option<&TreeZipper> {
        self.tree.shared_parent_ref()
    }

    /// Returns a shared reference to the data of the current node.
    pub fn shared_data(&self) -> &TreeNodeType {
        self.tree.shared_data()
    }

    /// Returns an `Option`al shared reference to parent node data.
    pub fn shared_parent_data(&self) -> Option<&TreeNodeType> {
        if let Some(parent_ref) = self.shared_parent_ref() {
            Some(parent_ref.shared_data())
        } else {
            None
        }
    }

    /// Generates a new symbolic footnote label, wrapped in an `Option`.
    /// Does not increment the symbolic footnote counter,
    /// as at the point of calling this function we have no information about whether the
    /// insertion of the respective node into the doctree succeeded.
    pub fn new_symbolic_footnote_label (&self) -> Option<String> {
            // Generate a label from crate::common::FOONOTE_SYMBOLS based on the number of autosymbol footnotes
            // entered into the document thus far.

            let n = self.n_of_symbolic_footnotes() as usize; // No overflow checks with as...

            let n_of_symbols = crate::common::FOOTNOTE_SYMBOLS.len();

            let passes = n / n_of_symbols;
            let index = n % n_of_symbols;
            let symbol: char = match crate::common::FOOTNOTE_SYMBOLS.get(index) {
                Some(symb) => *symb,
                None => {
                    eprintln!("No footnote symbol with index {}!", index);
                    return None;
                }
            };
            let label: String = vec![symbol; passes + 1].iter().collect();
            return Some(label)
    }

    /// Generates a new symbolic footnote reference label, wrapped in an `Option`.
    /// Does not increment the symbolic footnote counter,
    /// as at the point of calling this function we have no information about whether the
    /// insertion of the respective node into the doctree succeeded.
    pub fn new_symbolic_footnote_ref_label (&self) -> Option<String> {
        // Generate a label from crate::common::FOONOTE_SYMBOLS based on the number of autosymbol footnotes
        // entered into the document thus far.

        let n = self.n_of_symbolic_footnote_refs() as usize; // No overflow checks with as...

        let n_of_symbols = crate::common::FOOTNOTE_SYMBOLS.len();

        let passes = n / n_of_symbols;
        let index = n % n_of_symbols;
        let symbol: char = match crate::common::FOOTNOTE_SYMBOLS.get(index) {
            Some(symb) => *symb,
            None => {
                eprintln!("No footnote symbol with index {}!", index);
                return None;
            }
        };
        let label: String = vec![symbol; passes + 1].iter().collect();
        return Some(label)
    }

    /// Generates a new footnote number based on existing footnote labels.
    /// Again, does not modify the doctree, as knowledge about
    /// node insertion success is nil at this point.
    pub fn new_autonumber_footnote_label (&self) -> Option<String>{
        for n in 1..=crate::common::EnumAsInt::MAX {
            let n_str = n.to_string();
            if self.has_target_label(n_str.as_str()) {
                continue;
            }
            return Some(n_str);
        }
        eprintln!("All possible footnote numbers in use. Computer says no...");
        return None;
    }

    /// Generates a new footnote reference number based on existing footnote reference labels.
    /// Again, does not modify the doctree, as knowledge about
    /// node insertion success is nil at this point.
    pub fn new_autonumber_footnote_ref_label (&self) -> Option<String>{
        for n in 1..=crate::common::EnumAsInt::MAX {
            let n_str = n.to_string();
            if self.has_reference_label(n_str.as_str()) {
                continue;
            }
            return Some(n_str);
        }
        eprintln!("All possible footnote numbers in use. Computer says no...");
        return None;
    }
}

/// Shorthand for a vector of owned child nodes.
/// Empty vector indicates no children.
type Children = Vec<TreeNode>;
