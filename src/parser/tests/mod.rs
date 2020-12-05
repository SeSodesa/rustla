#![allow(unused_imports)] // Tests complain otehrwise...

/// ## tests
///
/// A submodule for Parser-related tests.
///
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi
use std::path::PathBuf;

use crate::common::EnumKind;
use crate::doctree::directives::AdmonitionType;
use crate::doctree::tree_node_types::TreeNodeType;
use crate::doctree::DocTree;
use crate::parser::line_cursor::LineCursor;
use crate::parser::types_and_aliases::InlineParsingResult;
use crate::parser::Parser;

mod test_admonitions;
mod test_aplus_point_of_interest;
mod test_aplus_questionnaire;
mod test_block_quotes;
mod test_block_reading;
mod test_bullet_lists;
mod test_class;
mod test_comments;
mod test_converters;
mod test_definition_lists;
mod test_enumerated_lists;
mod test_field_lists;
mod test_hyperlink_targets;
mod test_images;
mod test_inline_parsing;
mod test_list_tables;
mod test_literal_blocks;
mod test_math_blocks;
mod test_mixed_structures;
mod test_regexes;
mod test_sections_and_transitions;
mod test_sphinx_only;
mod test_unknown_directives;
