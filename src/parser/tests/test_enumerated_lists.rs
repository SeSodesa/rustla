/// ## test_enumerated_lists
/// A submodule for tests related to enumerated lists.
///
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi
use super::*;

#[cfg(test)]
#[test]
fn enumerated_list_01() {
    let src = String::from(
        "
(i) List item 1
    with a valid second line

Some unindented text.

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
        TreeNodeType::EnumeratedList { .. } => (),
        _ => panic!("No EnumeratedList detected!\n"),
    }

    match doctree.shared_child(0).shared_child(0).shared_data() {
        TreeNodeType::EnumeratedListItem { .. } => (),
        _ => panic!("No EnumeratedListItem as child of EnumeratedList!\n"),
    }

    match doctree
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => (),
        _ => panic!("No Paragraph as child of EnumeratdListItem!\n"),
    }
}

#[test]
fn enumerated_list_02() {
    let src = String::from(
        "
(i) List item 1
    with a valid second line

    Second paragraph of this list item.

(i) List item 1
    of a second list

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
        TreeNodeType::EnumeratedList { .. } => (),
        _ => panic!("No EnumeratedList detected!\n"),
    }

    match doctree.shared_child(1).shared_data() {
        TreeNodeType::EnumeratedList { .. } => (),
        _ => panic!("No EnumeratedList detected!\n"),
    }

    match doctree
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => (),
        _ => panic!("No Paragraph detected!\n"),
    }

    match doctree
        .shared_child(0)
        .shared_child(0)
        .shared_child(1)
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => (),
        _ => panic!("No second Paragraph detected!\n"),
    }
}

#[test]
fn enumerated_list_03() {
    let src = String::from(
        "
(i) a) List item ia
       with a valid second line

    Second paragraph of list item i.

(i) List item 1
    of a second list

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
        TreeNodeType::EnumeratedList { .. } => (),
        _ => panic!(),
    }

    match doctree.shared_child(0).shared_child(0).shared_data() {
        TreeNodeType::EnumeratedListItem { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_data()
    {
        TreeNodeType::EnumeratedList { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_data()
    {
        TreeNodeType::EnumeratedListItem { .. } => (),
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
        TreeNodeType::Paragraph { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(0)
        .shared_child(0)
        .shared_child(1)
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => (),
        _ => panic!(),
    }

    match doctree.shared_child(1).shared_data() {
        TreeNodeType::EnumeratedList { .. } => (),
        _ => panic!(),
    }

    match doctree.shared_child(1).shared_child(0).shared_data() {
        TreeNodeType::EnumeratedListItem { .. } => (),
        _ => panic!(),
    }

    match doctree
        .shared_child(1)
        .shared_child(0)
        .shared_child(0)
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => (),
        _ => panic!(),
    }
}

#[test]
fn enumerated_list_04() {
    let src = String::from(
        "
(#) First item of automatically numbered list

(#) Second item of automatically numbered list

(3) Third item that has to match with the internal counter of the list

(#) Fourth item of the same list, with automatic numbering, again.

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
        TreeNodeType::EnumeratedList { .. } => (),
        _ => panic!(),
    }

    match doctree.shared_child(0).shared_child(0).shared_data() {
        TreeNodeType::EnumeratedListItem { index_in_list, .. } => {
            if *index_in_list != 1 {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree.shared_child(0).shared_child(1).shared_data() {
        TreeNodeType::EnumeratedListItem { index_in_list, .. } => {
            if *index_in_list != 2 {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree.shared_child(0).shared_child(2).shared_data() {
        TreeNodeType::EnumeratedListItem { index_in_list, .. } => {
            if *index_in_list != 3 {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree.shared_child(0).shared_child(3).shared_data() {
        TreeNodeType::EnumeratedListItem { index_in_list, .. } => {
            if *index_in_list != 4 {
                panic!()
            }
        }
        _ => panic!(),
    }
}

#[test]
fn enumerated_list_05() {
    let src = String::from(
        "
(i) #) List item i1
       with a valid second line

    ii) List item i2

    #) List item i3

    First paragraph of list item i.

(#) List item ii

(iii) List item iii

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
        TreeNodeType::EnumeratedList { n_of_items, .. } => {
            if *n_of_items != 3 {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree.shared_child(0).shared_child(0).shared_data() {
        TreeNodeType::EnumeratedListItem { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_data()
    {
        TreeNodeType::EnumeratedList { .. } => {}
        _ => panic!(),
    }

    match doctree
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_child(0)
        .shared_data()
    {
        TreeNodeType::EnumeratedListItem {
            kind,
            index_in_list,
            ..
        } => {
            if *kind != EnumKind::Arabic || *index_in_list != 1 {
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
        TreeNodeType::EnumeratedList {
            kind, start_index, ..
        } => {
            if *kind != EnumKind::LowerRoman || *start_index != 2 {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree
        .shared_child(0)
        .shared_child(0)
        .shared_child(1)
        .shared_child(0)
        .shared_data()
    {
        TreeNodeType::EnumeratedListItem {
            kind,
            index_in_list,
            ..
        } => {
            if *kind != EnumKind::LowerRoman || *index_in_list != 2 {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree
        .shared_child(0)
        .shared_child(0)
        .shared_child(1)
        .shared_child(1)
        .shared_data()
    {
        TreeNodeType::EnumeratedListItem {
            kind,
            index_in_list,
            ..
        } => {
            if *kind != EnumKind::LowerRoman || *index_in_list != 3 {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree
        .shared_child(0)
        .shared_child(0)
        .shared_child(2)
        .shared_data()
    {
        TreeNodeType::Paragraph { .. } => {}
        _ => panic!(),
    }

    match doctree.shared_child(0).shared_child(1).shared_data() {
        TreeNodeType::EnumeratedListItem {
            kind,
            index_in_list,
            ..
        } => {
            if *kind != EnumKind::LowerRoman || *index_in_list != 2 {
                panic!()
            }
        }
        _ => panic!(),
    }

    match doctree.shared_child(0).shared_child(2).shared_data() {
        TreeNodeType::EnumeratedListItem {
            kind,
            index_in_list,
            ..
        } => {
            if *kind != EnumKind::LowerRoman || *index_in_list != 3 {
                panic!()
            }
        }
        _ => panic!(),
    }
}
