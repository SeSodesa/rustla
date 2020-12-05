#![allow(unused_imports)]

/// ## tests
/// This is the test module for the DocTree struct.
///
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi
use std::path::PathBuf;

use crate::common::TraversalType;
use crate::doctree::tree_node_types::TreeNodeType;
use crate::doctree::DocTree;
use crate::parser::Parser;

mod test_constructor;
mod test_walkers;
