/// ## test_admonitions
/// A submodule for admonition unit tests

use super::*;

#[cfg(test)]


#[test]
fn standard_admonition_01 () {
  let src = String::from("
.. note:: This is a note admonition.
  This is the second line of the first paragraph.

.. warning::
    This is another admonition.
    This is the second line of the first paragraph.

.. tip::
  :name: test
  :class: class
  :extra: extra (this should be ignored)

  This paragraph forms the contents of the tip admonition above.
  If content is not found, the parser will panic.

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();

  match doctree.shared_child(1).shared_data() {
    TreeNodeType::Admonition { content_indent, classes, name, variant } => {
      match (classes, name, variant) {
        (None, None, AdmonitionDirective::Note) => {}
        _ => panic!()
      }
    },
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(0).shared_data() {
    TreeNodeType::Paragraph {..} => {},
    _ => panic!()
  }

  match doctree.shared_child(2).shared_data() {
    TreeNodeType::Admonition { content_indent, classes, name, variant } => {
      match (classes, name, variant) {
        (None, None, AdmonitionDirective::Warning) => {}
        _ => panic!()
      }
    },
    _ => panic!()
  }

  match doctree.shared_child(2).shared_child(0).shared_data() {
    TreeNodeType::Paragraph {..} => {},
    _ => panic!()
  }

  match doctree.shared_child(3).shared_data() {
    TreeNodeType::Admonition { content_indent, classes, name, variant } => {
      match (classes, name, variant) {
        (Some(class), Some(name), AdmonitionDirective::Tip) if class.as_str() == "class" && name.as_str() == "test" => {}
        _ => panic!()
      }
    },
    _ => panic!()
  }

  match doctree.shared_child(3).shared_child(0).shared_data() {
    TreeNodeType::Paragraph {..} => {},
    _ => panic!()
  }
}


#[test]
fn generic_admonition_01 () {
  let src = String::from("
.. admonition:: This is a generic admonition with the argument on the first
    line after the directive marker and extending on the following line as well.
    :option1: This is not recognized as an admonition option, only \"class\" and \"name\" are valid.
    :name: hyperref target name

    This paragraph starts the admonition contents.
    Here is a second line.

This paragraph no longer belongs to the above admonition.

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();


  match doctree.shared_child(1).shared_data() {
    TreeNodeType::Admonition { content_indent, classes, name, variant } => {
      match (classes, name, variant) {
        (classes, name, AdmonitionDirective::Admonition {title}) if title.as_str() == "This is a generic admonition with the argument on the first line after the directive marker and extending on the following line as well." && classes.is_none() && name.as_deref().unwrap() == "hyperref target name" => {}
        _ => panic!()
      }
    },
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(2).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
    _ => panic!()
  }
}


#[test]
fn generic_admonition_02 () {
  let src = String::from("
.. admonition::
  This is a generic admonition, the argument of which starts on
  the line following the directive marker.
  :class: options start here
  :name: here is a reference name
  :unrecognized: This option is discarded by the parsing function.

  The admonition contents start here,
  with a single paragraph.

  - followed by
  - a bullet list

  ").lines().map(|s| s.to_string()).collect::<Vec<String>>();

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None, 0);

  doctree = parser.parse().unwrap_tree();
  doctree = doctree.walk_to_root();
  doctree.print_tree();


  match doctree.shared_child(1).shared_data() {
    TreeNodeType::Admonition { content_indent, classes, name, variant } => {
      match (classes, name, variant) {
        (classes, name, AdmonitionDirective::Admonition { title }) if title.as_str() == "This is a generic admonition, the argument of which starts on the line following the directive marker." && classes.as_deref().unwrap() == "options start here" && name.as_deref().unwrap() == "here is a reference name" => {}
        _ => panic!()
      }
    },
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(0).shared_data() {
    TreeNodeType::Paragraph { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(2).shared_data() {
    TreeNodeType::BulletList { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(2).shared_child(0).shared_data() {
    TreeNodeType::BulletListItem { .. } => {}
    _ => panic!()
  }

  match doctree.shared_child(1).shared_child(2).shared_child(1).shared_data() {
    TreeNodeType::BulletListItem { .. } => {}
    _ => panic!()
  }
}