/*!
A submodule for testing te parsing of unknown directives.

Copyright © 2020 Santtu Söderholm
*/

use super::*;

#[cfg(test)]
#[test]
fn unknown_directive_01() {
    let src = String::from(
        "
.. some-unknown-dirctive:: some argument here...
  :option1: something
  :option2: something else

A paragraph.


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

    match doctree
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::UnknownDirective {
            directive_name,
            argument,
            options,
            body_indent,
        } => {
            assert_eq!(directive_name, "some-unknown-dirctive");
            assert_eq!(argument, "some argument here...");
            assert_eq!(options.get("option1").unwrap(), "something");
            assert_eq!(options.get("option2").unwrap(), "something else");
        }
        _ => panic!(),
    }

    match doctree
        .shared_child(1).unwrap().shared_data() {
        TreeNodeType::Paragraph { .. } => {}
        _ => panic!(),
    }
}

#[test]
fn unknown_directive_02() {
    let src = String::from(
        "
Below is an unknown directive. It will be parsed as an unknown directive.

.. unknown:: argument with
  multiple lines
  :option1: a
  :option2: bunch
  :option3: the next option will override this one
  :option3: here
  :option5: something else entirely

  Paragraph inside unknown directive

  - And a bullet list with just one item

This is no longer a part of the above literal block inside a block quote.
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

    match doctree
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::Paragraph { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(1).unwrap().shared_data() {
        TreeNodeType::UnknownDirective {
            directive_name,
            argument,
            options,
            ..
        } => {
            assert_eq!(directive_name, "unknown");
            assert_eq!(argument, "argument with multiple lines");
            assert_eq!(options.get("option1").unwrap(), "a");
            assert_eq!(options.get("option2").unwrap(), "bunch");
            assert_eq!(options.get("option3").unwrap(), "here");
            assert!(options.get("option4").is_none(), true);
            assert_eq!(options.get("option5").unwrap(), "something else entirely");
        }
        _ => panic!(),
    }

    match doctree
        .shared_child(1).unwrap()
        .shared_child(0).unwrap().shared_data() {
        TreeNodeType::Paragraph { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(1).unwrap()
        .shared_child(1).unwrap().shared_data() {
        TreeNodeType::BulletList { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(2).unwrap().shared_data() {
        TreeNodeType::Paragraph { .. } => {}
        _ => panic!(),
    }
}

#[test]
fn unknown_directive_no_argument_nor_options() {
    let src = String::from(
        "
.. unknown::

  A paragraph.

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

    if let TreeNodeType::UnknownDirective {
        directive_name,
        argument,
        options,
        ..
    } = doctree
        .shared_child(0).unwrap().shared_data()
    {
        assert_eq!(directive_name, "unknown");
        assert_eq!(argument, "");
        assert!(options.is_empty());
    } else {
        panic!()
    }

    if let TreeNodeType::Paragraph { .. } = doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap().shared_data() {
    } else {
        panic!()
    }
}

#[test]
fn unknown_directive_no_argument_but_options() {
    let src = String::from(
        "
.. unknown::
  :option: 1
  :or: 2
  :three: 3

  A paragraph.

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

    if let TreeNodeType::UnknownDirective {
        directive_name,
        argument,
        options,
        ..
    } = doctree
        .shared_child(0).unwrap().shared_data()
    {
        assert_eq!(directive_name, "unknown");
        assert_eq!(argument, "");
        assert_eq!(options.get("option").unwrap(), "1");
        assert_eq!(options.get("or").unwrap(), "2");
        assert_eq!(options.get("three").unwrap(), "3");
    } else {
        panic!()
    }

    if let TreeNodeType::Paragraph { .. } = doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap().shared_data() {
    } else {
        panic!()
    }
}

#[test]
fn unknown_directive_no_options_but_argument() {
    let src = String::from(
        "
.. unknown:: argument
  on multiple lines

  Content paragraph.

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

    if let TreeNodeType::UnknownDirective {
        directive_name,
        argument,
        options,
        ..
    } = doctree
        .shared_child(0).unwrap().shared_data()
    {
        assert_eq!(directive_name, "unknown");
        assert_eq!(argument, "argument on multiple lines");
        assert!(options.is_empty());
    } else {
        panic!()
    }

    if let TreeNodeType::Paragraph { .. } = doctree
        .shared_child(0).unwrap()
        .shared_child(0).unwrap().shared_data() {
    } else {
        panic!()
    }
}
