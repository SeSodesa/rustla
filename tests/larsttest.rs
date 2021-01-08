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
    let src = common::str_to_lines(
"
Something before

#. First :math:`X=1`
    with wrapping
#. Next :math:`X=2` and something
    that continues to the next line!

    * Recursive items
    * Another recursive
        item continues
        to the next line
        how deep
        this could go
        after
        all

#. An this list just
    goes on!

Something after

#. .. image:: fig/clique.*

#. .. image:: fig/clique.*

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
fn itemize () {
    let src = common::str_to_lines(
"
Something before

* First :math:`X=1`
  with wrapping
* Next :math:`X=2` and something
  that continues to the next line!

  #. Recursive items
  #. Another recursive
     item continues
     to the next line
     how deep
     this could go
     after
     all

* An this list just
  goes on!

Something after

* .. image:: fig/clique.*

* .. image:: fig/clique.*


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
fn tabular () {
    let src = common::str_to_lines(
"
Tähän vähän tekstiä

ja sitten vähän lisää.

.. list-table:: ABCD

  * - A
    - B
  * - C
    - D

Taulukko 1: Elementit A, B, C ja D

.. list-table::

  * - .. image:: fig/clique.*
        :align: center

    - .. image:: fig/clique.*

.. list-table::

  * - The variables are as follows:

      * A
      * B
      * C

    - .. image:: fig/clique.*

Taulukon jälkeenkin jotain.

Jatkoa seuraa.

.. list-table::
  :widths: 67 33

  * - The variables are as follows:

      * A
      * B

        .. math::

          B=C

      * C

    - .. image:: fig/clique.*

One
Two
Three

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
fn thebibliography () {
    let src = common::str_to_lines(
r#"
.. [DG84]

  W.\ Dowling and J.\ Gallier.
  Linear time algorithms for testing the satisfiability of
  propositional Horn formulae.
  *Journal of Logic Programming*, 1(3):267--284, 1984.

.. [Lloyd87]

  J.\ Lloyd.
  *Foundations of Logic Programming*.
  Springer, 1987.

.. [MMZZM01]

  M.\ Moskewicz, C.\ Madigan, Y.\ Zhao, L.\ Zhang, and S.\ Malik.
  Chaff: Engineering an efficient SAT solver.
  In *Proceedings of DAC 2001*, pages 530--5350, 2001.
  URL:
  https://www.princeton.edu/~chaff/publication/DAC2001v56.pdf.
"#
    );
    let mut doctree = DocTree::new(PathBuf::new());
    doctree = Parser::new(src, doctree, Some(0), 0, Some(State::Body), 0)
        .parse()
        .unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();
    todo!()
}
