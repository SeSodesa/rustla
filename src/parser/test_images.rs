/// ## test_images
/// A submodule for unit tests related to images.
/// 
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;

#[cfg(test)]


#[test]
fn image_01 () {
  let src = String::from("
.. image:: this/is/an/image/uri.png
  :class: html class attributes
  :name: here is a reference name
  :unrecognized: This option is discarded by the parsing function.
  :alt: This is alternate text for the visually impaired
  :height: 200(px|ex|em|pt|...)
  :width: 100(px|ex|em|pt|...)
  :scale: 50%?
  :align: left
  :target: turns image into link

- This bullet list
- is not a part of the above image.

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  match doctree.shared_child(1).shared_data() {

    TreeNodeType::Image {
          uri, alt, height, width, scale, align, target, name, class
        } => match (uri, alt, height, width, scale, align, target, name, class) {
        (
          uri, Some(alt), Some(height), Some(width), Some(scale), Some(align), Some(target), Some(name), Some(class)
        ) if
          uri.as_str() == "this/is/an/image/uri.png"
          && alt.as_str() == "This is alternate text for the visually impaired"
          && height.as_str() == "200(px|ex|em|pt|...)"
          && width.as_str() == "100(px|ex|em|pt|...)"
          && scale.as_str() == "50%?"
          && align.as_str() == "left"
          && target.as_str() == "turns image into link"
          && name.as_str() == "here is a reference name"
          && class.as_str() == "html class attributes"
          => {}
        _ => panic!("One of the image options doesn't match...")
      },
    _ => panic!("Not a simple image...")
  }

  match doctree.shared_child(2).shared_data() {
    TreeNodeType::BulletList { .. } => {}
    _ => panic!()
  }
}


#[test]
fn figure_01 () {
  let src = String::from("
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

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  match doctree.shared_child(1).shared_data() {
    TreeNodeType::Figure { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(0).shared_data() {
    TreeNodeType::Image { .. } => {}
    _ => panic!()
  }

  // This node is transformed into a caption during the transdormation phase of the doctree.
  match doctree.shared_child(1).shared_child(1).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(2).shared_data() {
    TreeNodeType::EmptyLine { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(3).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(5).shared_data() {
    TreeNodeType::BulletList { .. } => {}
    _ => panic!()
  }
}


#[test]
fn figure_02 () {
  let src = String::from("
* This bullet list item contains a figure.

  .. figure:: this/is/an/image/uri.png

    ..
      This comment will prevent a caption from being formed.
      A comment can be empty.

    This paragraph is already a part of the figure legend.

    - As is
    - This bullet list

Back to no indentation.

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  match doctree.shared_child(1).shared_data() {
    TreeNodeType::BulletList { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(0).shared_data() {
    TreeNodeType::BulletListItem { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(0).shared_child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(0).shared_child(2).shared_data() {
    TreeNodeType::Figure { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(0).shared_child(2).shared_child(0).shared_data() {
    TreeNodeType::Image { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(0).shared_child(2).shared_child(2).shared_data() {
    TreeNodeType::Comment { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(0).shared_child(2).shared_child(4).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(0).shared_child(2).shared_child(6).shared_data() {
    TreeNodeType::BulletList { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(0).shared_child(2).shared_child(6).shared_child(0).shared_data() {
    TreeNodeType::BulletListItem { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(0).shared_child(2).shared_child(6).shared_child(1).shared_data() {
    TreeNodeType::BulletListItem { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(2).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
    _ => panic!()
  }
}