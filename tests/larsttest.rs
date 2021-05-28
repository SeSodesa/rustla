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
use rustla::common::EnumDelims;
use rustla::common::EnumKind;
use rustla::common::TableColWidths;

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
    doctree = Parser::new(&src, doctree, 0, 0,State::Body, 0)
        .parse()
        .unwrap_tree();
    doctree = doctree.walk_to_root();

    match doctree
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => {},
        _ => panic!()
    }
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
    doctree = Parser::new(&src, doctree, 0, 0,State::Body, 0)
        .parse()
        .unwrap_tree();
    doctree = doctree.walk_to_root();

    match doctree
        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::EnumeratedList { delims, kind, start_index, n_of_items, .. } => {
            assert_eq!(*delims, EnumDelims::Period);
            assert_eq!(*kind, EnumKind::Arabic);
            assert_eq!(*start_index, 1);
            assert_eq!(*n_of_items, 3);
        }
        _ => panic!()
    }
    match doctree
        .shared_child(1).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::EnumeratedListItem { delims, kind, index_in_list, .. } => {
            assert_eq!(*delims, EnumDelims::Period);
            assert_eq!(*kind, EnumKind::Arabic);
            assert_eq!(*index_in_list, 1);
        }
        _ => panic!()
    }
    match doctree
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::EnumeratedListItem { delims, kind, index_in_list, .. } => {
            assert_eq!(*delims, EnumDelims::Period);
            assert_eq!(*kind, EnumKind::Arabic);
            assert_eq!(*index_in_list, 2);
        }
        _ => panic!()
    }
    match doctree
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::BulletList { bullet, .. } => {
            assert_eq!(*bullet, '*');
        }
        _ => panic!()
    }
    match doctree
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::BulletListItem { bullet, .. } => {
            assert_eq!(*bullet, '*');
        }
        _ => panic!()
    }
    match doctree
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::BulletListItem { bullet, .. } => {
            assert_eq!(*bullet, '*');
        }
        _ => panic!()
    }
    match doctree
        .shared_child(1).unwrap()
        .shared_child(2).unwrap()
        .shared_data()
    {
        TreeNodeType::EnumeratedListItem { delims, kind, index_in_list, .. } => {
            assert_eq!(*delims, EnumDelims::Period);
            assert_eq!(*kind, EnumKind::Arabic);
            assert_eq!(*index_in_list, 3);
        }
        _ => panic!()
    }
    match doctree
        .shared_child(2).unwrap()
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => {}
        _ => panic!()
    }
    match doctree
        .shared_child(3).unwrap()
        .shared_data()
    {
        TreeNodeType::EnumeratedList { delims, kind, start_index, n_of_items, .. } => {
            assert_eq!(*delims, EnumDelims::Period);
            assert_eq!(*kind, EnumKind::Arabic);
            assert_eq!(*start_index, 1);
            assert_eq!(*n_of_items, 2);
        }
        _ => panic!()
    }
    match doctree
        .shared_child(3).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::Image { uri,  .. } => {
            assert_eq!(uri, "fig/clique.*");
        }
        _ => panic!()
    }
    match doctree
        .shared_child(3).unwrap()
        .shared_child(1).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::Image { uri, .. } => {
            assert_eq!(uri, "fig/clique.*");
        }
        _ => panic!()
    }
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
    doctree = Parser::new(&src, doctree, 0, 0,State::Body, 0)
        .parse()
        .unwrap_tree();
    doctree = doctree.walk_to_root();

    match doctree
        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::BulletList { bullet, .. } => {
            assert_eq!(*bullet, '*');
        }
        _ => panic!()
    }
    match doctree
        .shared_child(1).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::BulletListItem { bullet, .. } => {
            assert_eq!(*bullet, '*');
        }
        _ => panic!()
    }
    match doctree
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::BulletListItem { bullet, .. } => {
            assert_eq!(*bullet, '*');
        }
        _ => panic!()
    }
    match doctree
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => {}
        _ => panic!()
    }
    match doctree
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::EnumeratedList { delims, kind, n_of_items, .. } => {
            if let EnumDelims::Period = delims {} else { panic!() }
            if let EnumKind::Arabic = kind {} else { panic!() }
            assert_eq!(*n_of_items, 2);

        }
        _ => panic!()
    }
    match doctree
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::EnumeratedListItem { delims, kind, index_in_list, .. } => {
            if let EnumDelims::Period = delims {} else { panic!() }
            if let EnumKind::Arabic = kind {} else { panic!() }
            assert_eq!(*index_in_list, 1);

        }
        _ => panic!()
    }
    match doctree
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::EnumeratedListItem { delims, kind, index_in_list, .. } => {
            if let EnumDelims::Period = delims {} else { panic!() }
            if let EnumKind::Arabic = kind {} else { panic!() }
            assert_eq!(*index_in_list, 2);

        }
        _ => panic!()
    }
    match doctree
        .shared_child(1).unwrap()
        .shared_child(2).unwrap()
        .shared_data()
    {
        TreeNodeType::BulletListItem { bullet, .. } => {
            assert_eq!(*bullet, '*');

        }
        _ => panic!()
    }
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
    doctree = Parser::new(&src, doctree, 0, 0,State::Body, 0)
        .parse()
        .unwrap_tree();
    doctree = doctree.perform_restructuredtext_transforms();
    doctree = doctree.walk_to_root();

    match doctree
        .shared_child(2).unwrap()
        .shared_data()
    {
        TreeNodeType::ListTable { title, .. } => {
            assert_eq!(title.as_ref().unwrap(), "ABCD");
        }
        _ => panic!()
    }
    match doctree
        .shared_child(4).unwrap()
        .shared_data()
    {
        TreeNodeType::ListTable { title, .. } => {
            assert!(title.is_none());
        }
        _ => panic!()
    }
    match doctree
        .shared_child(5).unwrap()
        .shared_data()
    {
        TreeNodeType::ListTable { title, .. } => {
            assert!(title.is_none());
        }
        _ => panic!()
    }
    match doctree
        .shared_child(8).unwrap()
        .shared_data()
    {
        TreeNodeType::ListTable { title, widths, .. } => {
            assert!(title.is_none());
            if let Some(TableColWidths::Columns(cols)) = widths {
                assert_eq!(cols[0], 67.0);
                assert_eq!(cols[1], 33.0);
            } else {
                panic!()
            }
        }
        _ => panic!()
    }
    match doctree
        .shared_child(8).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::TBody => {}
        _ => panic!()
    }
    match doctree
        .shared_child(8).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::TRow => {}
        _ => panic!()
    }
    match doctree
        .shared_child(8).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::Entry { is_last } => {
            assert!( ! *is_last );
        }
        _ => panic!()
    }
    match doctree
        .shared_child(8).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => {}
        _ => panic!()
    }
    match doctree
        .shared_child(8).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::BulletList { bullet, .. } => {
            assert_eq!( *bullet, '*' );
        }
        _ => panic!()
    }
    match doctree
        .shared_child(8).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::BulletListItem { bullet, .. } => {
            assert_eq!( *bullet, '*' );
        }
        _ => panic!()
    }
    match doctree
        .shared_child(8).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::BulletListItem { bullet, .. } => {
            assert_eq!( *bullet, '*' );
        }
        _ => panic!()
    }
    match doctree
        .shared_child(8).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => {}
        _ => panic!()
    }
    match doctree
        .shared_child(8).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::MathBlock { math_block, .. } => {
            assert_eq!(*math_block, "B=C");
        }
        _ => panic!()
    }
    match doctree
        .shared_child(8).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::Entry { is_last } => {
            assert!( *is_last );
        }
        _ => panic!()
    }
    match doctree
        .shared_child(8).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::Image { uri, .. } => {
            assert_eq!( uri, "fig/clique.*" );
        }
        _ => panic!()
    }
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
    doctree = Parser::new(&src, doctree, 0, 0,State::Body, 0)
        .parse()
        .unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    match doctree
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::Citation { label, .. } => {
            assert_eq!(label, "DG84");
        }
        _ => panic!()
    }
    match doctree
        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::Citation { label, .. } => {
            assert_eq!(label, "Lloyd87");
        }
        _ => panic!()
    }
    match doctree
        .shared_child(2).unwrap()
        .shared_data()
    {
        TreeNodeType::Citation { label, .. } => {
            assert_eq!(label, "MMZZM01");
        }
        _ => panic!()
    }
}
