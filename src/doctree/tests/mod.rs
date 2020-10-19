#![allow(unused_imports)]

/// ## tests
/// This is the test module for the DocTree struct.
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use std::path::PathBuf;

use crate::doctree::DocTree;
use crate::doctree::tree_node_types::TreeNodeType;
use crate::parser::Parser;
use crate::common::TraversalType;


mod test_constructor;
mod test_walkers;
