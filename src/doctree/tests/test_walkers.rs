/*!
A unit test module for testing document tree walker methods.

Copyright © 2020 Santtu Söderholm
*/
use super::*;
use crate::parser::state_machine::State;

#[cfg(test)]
#[test]
fn walk_to_id_01() {
    let src = String::from(
        "
A simple test paragraph.

And another one.

  ",
    )
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(std::path::PathBuf::from("test"));
    let mut parser = Parser::new(src, doctree, 0, 0, State::Body, 0);
    doctree = parser.parse().unwrap_tree();

    let n_of_nodes = doctree.n_of_nodes();

    for i in 0..n_of_nodes {
        doctree = doctree.walk(TraversalType::ID(i));
        assert_eq!(doctree.current_node_id(), i);
    }
}

#[test]
fn walk_to_id_02() {
    let src = String::from(
        "
- A slightly more complicated test...
- ... with more structure between inline nodes.

.. Admonition:: A title

  Here the parser produces more nodes than in the previous

    - Test (a block quote inside admonition)

  ",
    )
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(std::path::PathBuf::from("test"));

    let mut parser = Parser::new(src, doctree, 0, 0, State::Body, 0);

    doctree = parser.parse().unwrap_tree();
    // doctree.print_tree();

    let n_of_nodes = doctree.n_of_nodes();

    for i in 0..n_of_nodes {
        doctree = doctree.walk(TraversalType::ID(i));
        assert_eq!(doctree.current_node_id(), i);
    }
}

#[test]
fn walk_to_id_03() {
    use crate::parser::Parser;

    let src = String::from(
        "
* This is a list

  - With a sublist

* Back to top level again

  + Another sublist

    * With a subsublist

Paragraph at top level.
Now with a second row.

  ",
    )
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(std::path::PathBuf::from("test"));

    let mut parser = Parser::new(src, doctree, 0, 0, State::Body, 0);

    doctree = parser.parse().unwrap_tree();
    // doctree.print_tree();

    let n_of_nodes = doctree.n_of_nodes();

    for i in 0..n_of_nodes {
        doctree = doctree.walk(TraversalType::ID(i));
        assert_eq!(doctree.current_node_id(), i);
        doctree.print_node_id();
    }
}
