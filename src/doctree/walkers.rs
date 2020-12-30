/*!
A submodule for doctree walker functions.

Copyright © 2020 Santtu Söderholm <santtu.soderholm@tuni.fi>
*/

use super::*;

use crate::common::TraversalType;

/// ---------
///  Walkers
/// ---------
///
/// Functions for walking to differents parts of the contained `TreeZipper`.
/// These include ID-based searches, as well as criteria related to the
/// contained data variant.
impl DocTree {

    /// Walks to the root of the contained tree zipper.
    pub fn walk_to_root(mut self) -> Self {
        self.tree = self.tree.walk_to_root();
        self
    }

    /// The mother of all walkers. Performs a tree walk based on given `TraversalType`.
    /// These include walking to a specific node id, but a reference to a `TreeNodeType`
    /// might also be used in determining when to stop walking.
    pub fn walk(mut self, traversal_type: TraversalType) -> Self {
        // Always walk to tree root before starting the search/walk.
        self.tree = self.tree.walk_to_root();

        match traversal_type {
            TraversalType::ID(id) => self.walk_to_node_with_id(id),
        }
    }

    /// Walks to a `TreeNode` with a specific given ID.
    /// Naively walks to the tree root before beginning the actual search,
    /// in order to ensure that all nodes are traversed.
    ///
    /// Panic!s if a node with the given id is not found.
    fn walk_to_node_with_id(mut self, id: NodeId) -> Self {
        if id > self.node_count() {
            panic!("No node with given ID. Computer says no...")
        }

        self.tree = match self.tree.walk_to_node_with_id(id) {
            Ok(zipper) => zipper,
            Err(zipper) => zipper,
        };

        self
    }

    /// Walks to a node with a given ID and the back again.
    /// Panic!s, if the given target node id has not been entered into the tree.
    fn walk_to_and_fro(self, to_id: NodeId, current_id: NodeId) -> Self {
        if to_id > self.node_count() {
            panic!("No node with given ID. Computer says no...")
        }

        todo!()
    }
}

/// ---------
///  Walkers
/// ---------
impl TreeZipper {

    /// Walks to a specific node based on a given id,
    /// using the NLR (pre-order) strategy.
    pub fn walk_to_node_with_id(mut self, id: NodeId) -> Result<Self, Self> {
        if self.node_id() == id {
            return Ok(self);
        }

        let n_of_children = if let Some(children) = self.shared_node().shared_children() {
            self.n_of_children()
        } else {
            match self.focus_on_parent() {
                Ok(zipper) | Err(zipper) => return Err(zipper),
            }
        };

        for ind in 0..n_of_children {
            self = if let Ok(child) = self.focus_on_child(ind) {
                child
            } else {
                unreachable!("This should not happen with enumerated children...")
            };
            match self.walk_to_node_with_id(id) {
                Ok(zipper) => return Ok(zipper),
                Err(zipper) => {
                    self = zipper;
                    continue;
                }
            }
        }

        self = match self.focus_on_parent() {
            Ok(zipper) | Err(zipper) => zipper,
        };
        Err(self)
    }
}
