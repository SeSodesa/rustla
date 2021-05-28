/*!
A submodule for testing images and figures.

Copyright © 2020 Santtu Söderholm
*/

use super::*;

#[cfg(test)]
#[test]
fn image_01() {
    let src =
"
.. image:: this/is/an/image/uri.png
  :class: html class attributes
  :name: here is a reference name
  :unrecognized: This option is discarded by the parsing function.
  :alt: This is alternate text for the visually impaired
  :height: 200(px|ex|em|pt|...)
  :width: 100(px|ex|em|pt|...)
  :scale: 50%
  :align: left
  :target: turns image into link

- This bullet list
- is not a part of the above image.

"
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(&src, doctree, 0, 0, State::Body, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    use crate::common::HTMLAlignment;

    match doctree
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::Image {
            uri,
            alt,
            height,
            width,
            scale,
            align,
            target,
            name,
            class,
            inline
        } => {
            assert_eq!(uri.as_str(), "this/is/an/image/uri.png");
            assert_eq!(
                alt.as_ref().unwrap().as_str(),
"This is alternate text for the visually impaired"
            );
            assert_eq!(height.is_none(), true);
            assert_eq!(width.is_none(), true);
            assert_eq!(scale.as_ref().unwrap().to_string(), "50");
            if let HTMLAlignment::Left = align.as_ref().unwrap() {
            } else {
                panic!()
            }
            assert_eq!(target.as_ref().unwrap().as_str(), "turns image into link");
            assert_eq!(name.as_ref().unwrap().as_str(), "here is a reference name");
            assert_eq!(class.as_ref().unwrap().as_str(), "html class attributes");
        }
        _ => panic!("Not a simple image..."),
    }

    match doctree
        .shared_child(1).unwrap().shared_data() {
        TreeNodeType::BulletList { .. } => {}
        _ => panic!(),
    }
}

#[test]
fn figure_01() {
    let src =
"
.. figure:: this/is/an/image/uri.png
  :class: html class attributes
  :name: here is a reference name
  :unrecognized: This option is discarded by the parsing function.
  :alt: This is alternate text for the visually impaired
  :height: 200(px|ex|em|pt|...)
  :width: 100(px|ex|em|pt|...)
  :scale: 50%?
  :align: left
  :target: turns image into link

  This first paragraph should be transformed into
  a figure caption node inside the figure parser.

  This paragraph is already a part of the figure legend.

  - As is
  - This bullet list

"
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(&src, doctree, 0, 0, State::Body, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.perform_restructuredtext_transforms();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    match doctree
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::Figure { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::Image { .. } => {}
        _ => panic!(),
    }

    // This node is transformed into a caption during the transdormation phase of the doctree.
    match doctree
        .shared_child(0).unwrap()
        .shared_child(1).unwrap().shared_data() {
        TreeNodeType::Caption { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(2).unwrap().shared_data() {
        TreeNodeType::Paragraph { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(3).unwrap().shared_data() {
        TreeNodeType::BulletList { .. } => {}
        _ => panic!(),
    }
}

#[test]
fn figure_02() {
    let src =
"
* This bullet list item contains a figure.

  .. figure:: this/is/an/image/uri.png

    ..
      This comment will prevent a caption from being formed.
      A comment can be empty.

    This paragraph is already a part of the figure legend.

    - As is
    - This bullet list

Back to no indentation.

"
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(&src, doctree, 0, 0, State::Body, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    match doctree
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::BulletList { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::BulletListItem { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::Figure { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::Image { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::Comment { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_child(2).unwrap()
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_child(3).unwrap()
        .shared_data()
    {
        TreeNodeType::BulletList { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_child(3).unwrap()
        .shared_child(0).unwrap()
        .shared_data()
    {
        TreeNodeType::BulletListItem { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_child(3).unwrap()
        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::BulletListItem { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(1).unwrap().shared_data() {
        TreeNodeType::Paragraph { .. } => {}
        _ => panic!(),
    }
}

#[test]
fn figure_03() {
    let src =
"
.. figure:: this/is/an/image/uri.png

  .. figure:: this/is/another/image/uri.png

    Caption paragraph: A figure inside a figure legend.

Back to no indentation.

"
    .lines()
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

    let mut doctree = DocTree::new(PathBuf::from("test"));

    let mut parser = Parser::new(&src, doctree, 0, 0, State::Body, 0);

    doctree = parser.parse().unwrap_tree();
    doctree = doctree.walk_to_root();
    doctree.print_tree();

    match doctree
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::Figure { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(1).unwrap().shared_data() {
        TreeNodeType::Figure { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(0).unwrap()
        .shared_child(1).unwrap()
        .shared_child(1).unwrap()
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(1).unwrap().shared_data() {
        TreeNodeType::Paragraph { .. } => {}
        _ => panic!(),
    }
}
