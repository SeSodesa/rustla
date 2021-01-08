/*!
A test module that tests the output of LarST when it is fed the test files in
the LarSTtest repository.
*/

extern crate rustla;

use std::path::PathBuf;
use rustla::parser::Parser;
use rustla::parser::state_machine::State;
use rustla::doctree::DocTree;
use rustla::doctree::tree_node_types::TreeNodeType;
use rustla::common;
use rustla::common::ParsingResult;

#[cfg(test)]

#[test]
fn document () {
    let src = common::str_to_lines(
"
Lorem ipsum dolor sit amet, consectetur adipisci elit, sed eiusmod
tempor incidunt ut labore et dolore magna aliqua. Ut enim ad minim
veniam, quis nostrud exercitation ullamco laboris nisi ut aliquid ex
ea commodi consequat. Quis aute iure reprehenderit in voluptate velit
esse cillum dolore eu fugiat nulla pariatur. Excepteur sint obcaecat
cupiditat non proident, sunt in culpa qui officia deserunt mollit anim
id est laborum.
"
    );
    let mut doctree = DocTree::new(PathBuf::new());
    doctree = Parser::new(src, doctree, Some(0), 0, Some(State::Body), 0)
        .parse()
        .unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    todo!()
}

#[test]
fn enumerate () {
    todo!()
}
#[test]
fn itemize () {
    todo!()
}

#[test]
fn tabular () {
    todo!()
}

#[test]
fn thebibliography () {
    todo!()
}
