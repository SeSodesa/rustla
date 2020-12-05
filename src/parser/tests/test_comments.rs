/// ## test_comments
///
/// A unit test module for reST comments.
///
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi
use super::*;

#[cfg(test)]
#[test]
fn comment_01() {
    let src = String::from(
        "
.. This is a comment on a single line
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
        TreeNodeType::Comment { text } => {
            if text.as_ref().unwrap().as_str() != "This is a comment on a single line" {
                eprintln!("Erraneous text: {:#?}\n", text);
                panic!()
            }
        }
        _ => panic!(),
    }
}

#[test]
fn comment_02() {
    let src = String::from(
        "
..
  This is a single-line comment on the line following the marker
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
        TreeNodeType::Comment { text } => {
            if text.as_ref().unwrap().as_str()
                != "This is a single-line comment on the line following the marker"
            {
                eprintln!("Erraneous text: {:#?}\n", text);
                panic!()
            }
        }
        _ => panic!(),
    }
}

#[test]
fn comment_03() {
    let src = String::from(
        "
..

The above comment is empty.
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
        TreeNodeType::Comment { text } => {
            if text.is_some() {
                eprintln!("Erraneous text: {:#?}\n", text);
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree.shared_child(1).shared_data() {
        TreeNodeType::Paragraph { .. } => {}
        _ => panic!(),
    }
}

#[test]
fn comment_04() {
    let src = String::from(
        "
* ..
    This is a comment inside a bullet list item

  This here is a paragraph inside the same item.

This paragraph ends the test...

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
        TreeNodeType::BulletList { .. } => {}
        _ => panic!(),
    }

    match doctree.shared_child(0).shared_child(0).shared_data() {
        TreeNodeType::BulletListItem { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_data()
    {
        TreeNodeType::Comment { text } => {
            if text.as_ref().unwrap().as_str() != "This is a comment inside a bullet list item" {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree
        .shared_child(0)
        .shared_child(0)
        .shared_child(1)
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => {}
        _ => panic!(),
    }

    match doctree.shared_child(1).shared_data() {
        TreeNodeType::Paragraph { .. } => {}
        _ => panic!(),
    }
}
