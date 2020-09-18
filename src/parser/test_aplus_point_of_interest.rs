/// ## test_aplus_point_of_interest
///
/// A submodule for unit testing A+ point of interest parsing.

use super::*;
use std::path::PathBuf;

#[cfg(test)]


#[test]
fn poi_01 () {

  let src = String::from("
.. point-of-interest:: Title text
  :id: unique id, if not supplied a random id will be generated
  :previous: id of previous point-of-interest (optional)
  :next: id of next point-of-interest (optional)
  :hidden: (if this flag is present, the content of this poi is hidden by default)
  :class: any additional css classes
  :height: fixed height in pixels
  :columns: relative width of each column (e.g. for three columns 2 2 3)
  :bgimg: path to background image
  :not_in_slides: a flag used with the presentation maker. This POI does not show in the slides if this is defined.
  :not_in_book: If this flag is given, this POI does not appear in the A+ content chapter.
  :no_poi_box: Removes surrounding box and navigation

  Content of point-of-interest here.

  Use ::newcol to start a new column:

  ::newcol

  New column starts here. If :columns: option not present columns of equal width will be created.
  (The presentation maker slides do not support the newcol feature.)


  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(PathBuf::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  todo!()
}