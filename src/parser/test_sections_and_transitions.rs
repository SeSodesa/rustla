/// ## test_sections_and_transitions
/// A submodule for testing document transitions and section titles.
/// 
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;

#[cfg(test)]


#[test]
fn transition_01 () {

  let src = String::from("
  
Below is a transition.

=======

The line is at least 4 symbols long.

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.child(4).shared_data() {
    TreeNodeType::Transition => (),
    _ => panic!()
  }
}


#[test]
fn over_under_section_01 () {

  let src = String::from("
  
Below is an over- and underlined section title.

=======================================
 This is Spart.. I mean a section title
=======================================

This paragraph belongs to the section started by the above title,
not to the document root.

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  match doctree.child(4).shared_data() {
    TreeNodeType::Section { level, .. } => {
      if *level != 1 { panic!() }
    }
    _ => panic!()
  }
}


#[test]
fn mixed_sections_01 () {

  let src = String::from("
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

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();

  doctree.print_tree();

  todo!()
}
