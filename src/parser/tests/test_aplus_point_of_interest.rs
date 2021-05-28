/*!
A submodule for unit testing A+ point of interest parsing.

Copyright © 2020 Santtu Söderholm
*/

use super::*;

#[cfg(test)]
#[test]
fn poi_01() {
    let src = "
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


".lines().map(|s| s.to_string()).collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(&src, doctree, 0, 0, State::Body, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    if let TreeNodeType::AplusPOI {
        title,
        id,
        previous,
        next,
        hidden,
        class,
        height,
        columns,
        bgimg,
        not_in_slides,
        not_in_book,
        no_poi_box,
        ..
    } = doctree
        .shared_child(0).unwrap().shared_data()
    {
        assert_eq!(title, r#"Title text"#);
        assert_eq!(
            id.as_ref().unwrap(),
            r#"unique id, if not supplied a random id will be generated"#
        );
        assert_eq!(
            previous.as_ref().unwrap(),
            r#"id of previous point-of-interest (optional)"#
        );
        assert_eq!(
            next.as_ref().unwrap(),
            r#"id of next point-of-interest (optional)"#
        );
        assert_eq!(
            hidden.as_ref().unwrap(),
            r#"(if this flag is present, the content of this poi is hidden by default)"#
        );
        assert_eq!(class.as_ref().unwrap(), r#"any additional css classes"#);
        assert_eq!(height.is_none(), true);
        assert_eq!(
            columns.as_ref().unwrap(),
            r#"relative width of each column (e.g. for three columns 2 2 3)"#
        );
        assert_eq!(bgimg.as_ref().unwrap(), r#"path to background image"#);
        assert_eq!(
            not_in_slides.as_ref().unwrap(),
            r#"a flag used with the presentation maker. This POI does not show in the slides if this is defined."#
        );
        assert_eq!(
            not_in_book.as_ref().unwrap(),
            r#"If this flag is given, this POI does not appear in the A+ content chapter."#
        );
        assert_eq!(
            no_poi_box.as_ref().unwrap(),
            r#"Removes surrounding box and navigation"#
        );
    } else {
        panic!()
    }

    if let TreeNodeType::Paragraph { .. } = doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap().shared_data() {
    } else {
        panic!()
    }
    if let TreeNodeType::Paragraph { .. } = doctree
        .shared_child(0).unwrap()
        .shared_child(1).unwrap().shared_data() {
    } else {
        panic!()
    }
    if let TreeNodeType::AplusColBreak { .. } =
        doctree
        .shared_child(0).unwrap()
        .shared_child(2).unwrap().shared_data()
    {
    } else {
        panic!()
    }
    if let TreeNodeType::Paragraph { .. } = doctree
        .shared_child(0).unwrap()
        .shared_child(3).unwrap().shared_data() {
    } else {
        panic!()
    }
}
