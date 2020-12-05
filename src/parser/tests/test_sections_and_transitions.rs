/// ## test_sections_and_transitions
/// A submodule for testing document transitions and section titles.
///
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi
use super::*;

#[cfg(test)]
#[test]
fn transition_01() {
    let src = String::from(
        "

Below is a transition.

=======

The line is at least 4 symbols long.

  ",
    )
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(src, doctree, None, 0, None, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();

    doctree.print_tree();

    match doctree.shared_child(1).shared_data() {
        TreeNodeType::Transition => (),
        _ => panic!(),
    }
}

#[test]
fn over_under_section_01() {
    let src = String::from(
        "

Below is an over- and underlined section title.

=======================================
 This is Spart.. I mean a section title
=======================================

This paragraph belongs to the section started by the above title,
not to the document root.

  ",
    )
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(src, doctree, None, 0, None, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();

    doctree.print_tree();

    match doctree.shared_child(1).shared_data() {
        TreeNodeType::Section { level, .. } => {
            if *level != 1 {
                panic!()
            }
        }
        _ => panic!(),
    }
}

#[test]
fn mixed_sections_01() {
    let src = String::from(
        "
=======================
 Level 1 Section Title
=======================

-----------------------
 Level 2 Section Title
-----------------------

Level 3 Section Title
=====================

Level 4 Section Title
---------------------

Level 5 Section Title
`````````````````````

---------------------
Level 2 Section Title
---------------------

Level 6 Section Title
'''''''''''''''''''''

Level 7 Section Title
.....................

Level 8 Section Title
~~~~~~~~~~~~~~~~~~~~~

Level 9 Section Title
*********************

Level 10 Section Title
++++++++++++++++++++++

Level 11 Section Title
^^^^^^^^^^^^^^^^^^^^^^

======================
Level 1 Section Title
======================

  ",
    )
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(src, doctree, None, 0, None, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();

    doctree.print_tree();

    match doctree.shared_child(0).shared_data() {
        TreeNodeType::Section { level, .. } => {
            if *level != 1 {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree.shared_child(0).shared_child(0).shared_data() {
        TreeNodeType::Section { level, .. } => {
            if *level != 2 {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_data()
    {
        TreeNodeType::Section { level, .. } => {
            if *level != 3 {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_data()
    {
        TreeNodeType::Section { level, .. } => {
            if *level != 4 {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_data()
    {
        TreeNodeType::Section { level, .. } => {
            if *level != 5 {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree.shared_child(0).shared_child(1).shared_data() {
        TreeNodeType::Section { level, .. } => {
            if *level != 2 {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree
        .shared_child(0)
        .shared_child(1)
        .shared_child(0)
        .shared_data()
    {
        TreeNodeType::Section { level, .. } => {
            if *level != 6 {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree
        .shared_child(0)
        .shared_child(1)
        .shared_child(0)
        .shared_child(0)
        .shared_data()
    {
        TreeNodeType::Section { level, .. } => {
            if *level != 7 {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree
        .shared_child(0)
        .shared_child(1)
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_data()
    {
        TreeNodeType::Section { level, .. } => {
            if *level != 8 {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree
        .shared_child(0)
        .shared_child(1)
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_data()
    {
        TreeNodeType::Section { level, .. } => {
            if *level != 9 {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree
        .shared_child(0)
        .shared_child(1)
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_data()
    {
        TreeNodeType::Section { level, .. } => {
            if *level != 10 {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree
        .shared_child(0)
        .shared_child(1)
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_data()
    {
        TreeNodeType::Section { level, .. } => {
            if *level != 11 {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree.shared_child(1).shared_data() {
        TreeNodeType::Section { level, .. } => {
            if *level != 1 {
                panic!()
            }
        }
        _ => panic!(),
    }
}

#[test]
fn transition_after_admonition() {
    let src = String::from(
        "
.. ATTENTION::

  Seuraavan tehtävän tekeminen (eli ``.gitignore``-tiedoston luominen)
  on erittäinen suositeltavaa, koska se helpottaa työskentelyäsi
  tällä kurssilla.
  Aivan välttämätöntä se ei ole, eikä sitä tarvitse tehdä juuri nyt.
  Voit palata tähän tehtävään myöhemmin, kun huomaat ``.gitignore``-tiedoston
  tarpeelliseksi.

  <Content>

=======

A paragraph after transition.

  ",
    )
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(src, doctree, None, 0, None, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    if let TreeNodeType::Admonition { variant, .. } = doctree.shared_child(0).shared_data() {
        match variant {
            AdmonitionType::Attention => {}
            _ => panic!(),
        }
    } else {
        panic!()
    }

    if let TreeNodeType::Paragraph { .. } = doctree.shared_child(0).shared_child(0).shared_data() {
    } else {
        panic!()
    }

    match doctree.shared_child(1).shared_data() {
        TreeNodeType::Transition => (),
        _ => panic!(),
    }

    match doctree.shared_child(2).shared_data() {
        TreeNodeType::Paragraph { .. } => (),
        _ => panic!(),
    }
}
