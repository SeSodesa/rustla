


use super::*;
use crate::common;

#[cfg(test)]


#[test]
fn read_text_block_01 () {

  let src = "

asdsafasfga  sffwsdaf
asfsdafasdfffasfsdfsaf
asfdfasdfasdfafasdfasdf
asdfsdafasdfsdafadsfsdf

asdfdsfsdafsadfaf
asfsffdsfasfasdf
asdfsdafasdfasdfa

";

  let lines = common::str_to_lines(src);

  eprintln!("{:#?}", lines);

  let (block, offset) = match Parser::read_text_block(&lines, 2, false, false, None) {
    Ok(block) => block,
    Err(e) => {
      eprintln!("{}", e);
      panic!();
    }
  };
  
  eprintln!("{:#?}", block);

  let block_str = block.join("\n");

  assert_eq!("asdsafasfga  sffwsdaf
asfsdafasdfffasfsdfsaf
asfdfasdfasdfafasdfasdf
asdfsdafasdfsdafadsfsdf", block_str);

}


#[test]
fn read_text_block_02 () {

  let src = "
  
asdsafasfgasf  fwsdaf
asfsdafasdfffasfsdfsaf
  asfdfasdfasdfafasdfasdf
asdfsdafasdfsdafadsfsdf

asdfdsfsdafsadfaf
asfsffdsfasfasdf
asdfsdafasdfasdfa
    
";

  let lines = common::str_to_lines(src);

  eprintln!("{:#?}", lines);

  match Parser::read_text_block(&lines, 2, false, false, None) {
    Ok(_) => panic!("There was indent where one was not allowed..."),
    Err(e) => {
      eprintln!("{:#?}", e);
      assert_eq!(
        "No indent allowed but indent found on line 4!\nComputer says no...\n",
        e
      )
    }
  };

}


#[test]
fn read_text_block_03 () {

  let src = "
  
  asdsafasfgasf  fwsdaf
  asfsdafasdfffasfsdfsaf
  asfdfasdfasdfafasdfasdf
  asdfsdafasdfsdafadsfsdf

asdfdsfsdafsadfaf
asfsffdsfasfasdf
asdfsdafasdfasdfa
    
";

  let lines = common::str_to_lines(src);

  eprintln!("{:#?}", lines);

  match Parser::read_text_block(&lines, 2, true, false, None) {
    Ok((block, offset)) => {

      eprintln!("{:#?}", block);

      assert_eq!(
        block.join("\n"),
"  asdsafasfgasf  fwsdaf
  asfsdafasdfffasfsdfsaf
  asfdfasdfasdfafasdfasdf
  asdfsdafasdfsdafadsfsdf"        
      );
    },
    Err(e) => {
      eprintln!("{}", e);
      panic!();
    }
  };

}



#[test]
fn read_indented_block_01 () {

  let src = "
  
  asdsafasfgasf  fwsdaf
      asfsdafasdfffas  fsdfsaf
    asfdfasd  fasdfafasdfasdf
  asdfsdafasdfsda  fadsfsdf

asdfdsfsdafsadfaf
asfsffdsfasfasdf
asdfsdafasdfasdfa
    
";

  let lines = common::str_to_lines(src);

  match Parser::read_indented_block(&lines, Some(2), None, Some(true), None, None) {
    Ok((lines, _indent, line_diff, _empty_finish)) => {

      eprintln!("{:#?}", lines);

      assert_eq!(
        lines.join("\n"),
"asdsafasfgasf  fwsdaf
    asfsdafasdfffas  fsdfsaf
  asfdfasd  fasdfafasdfasdf
asdfsdafasdfsda  fadsfsdf
");
    },
    Err(e) => {
      eprintln!("{}", e);
      panic!();
    }
  };

}


#[test]
fn read_indented_block_02 () {

  let src = "

    asdsafasfgasf  fwsdaf
      asfsdafasdfffas  fsdfsaf
    asfdfasd  fasdfafasdfasdf
  asdfsdafasdfsda  fadsfsdf

asdfdsfsdafsadfaf
asfsffdsfasfasdf
asdfsdafasdfasdfa

";

  let lines = common::str_to_lines(src);

  match Parser::read_indented_block(&lines, Some(2), None, None, Some(2), None) {
    Ok((lines, _indent, line_diff, _empty_finish)) => {

      eprintln!("{:#?}", lines);

      assert_eq!(
        lines.join("\n"),
"  asdsafasfgasf  fwsdaf
    asfsdafasdfffas  fsdfsaf
  asfdfasd  fasdfafasdfasdf
asdfsdafasdfsda  fadsfsdf
"
      );
    },
    Err(e) => {
      eprintln!("{}", e);
      panic!();
    }
  };

}


#[test]
fn read_indented_block_03 () {

  let src = "    

 asdsafasfgasf  fwsdaf
      asfsdafasdfffas  fsdfsaf
    asfdfasd  fasdfafasdfasdf
  asdfsdafasdfsda  fadsfsdf
  asdfdsfsdafsadfaf
asfsffdsfasfasdf
asdfsdafasdfasdfa

";

  let lines = common::str_to_lines(src);

  match Parser::read_indented_block(&lines, Some(2), None, None, None, None) {
    Ok((lines, _indent, line_diff, _empty_finish)) => {

      eprintln!("{:#?}", lines);

      assert_eq!(
        lines.join("\n"),
"asdsafasfgasf  fwsdaf
     asfsdafasdfffas  fsdfsaf
   asfdfasd  fasdfafasdfasdf
 asdfsdafasdfsda  fadsfsdf
 asdfdsfsdafsadfaf"
      );
    },
    Err(e) => {
      eprintln!("{}", e);
      panic!();
    }
  };

}

#[test]
fn read_indented_block_04 () {

  let src = "    

* asdsafasfgasf  fwsdaf
  asfsdafasdfffas  fsdfsaf
  asfdfasd  fasdfafasdfasdf
  asdfsdafasdfsda  fadsfsdf
  asdfdsfsdafsadfaf
  asfsffdsfasfasdf

asdfsdafasdfasdfa

";

  let lines = common::str_to_lines(src);

  match Parser::read_indented_block(&lines, Some(2), None, None, Some(2), Some(2)) {
    Ok((lines, _indent, line_diff, _empty_finish)) => {

      eprintln!("{:#?}", lines);

      assert_eq!(line_diff, 7);

      assert_eq!(
        lines.join("\n"),
"asdsafasfgasf  fwsdaf
asfsdafasdfffas  fsdfsaf
asfdfasd  fasdfafasdfasdf
asdfsdafasdfsda  fadsfsdf
asdfdsfsdafsadfaf
asfsffdsfasfasdf
"
      );
    },
    Err(e) => {
      eprintln!("{}", e);
      panic!();
    }
  };

}


#[test]
fn read_indented_block_05 () {

  let src = "    

* asdsafasfgasf  fwsdaf
  asfsdafasdfffas  fsdfsaf
  asfdfasd  fasdfafasdfasdf
  asdfsdafasdfsda  fadsfsdf
  asdfdsfsdafsadfaf
  asfsffdsfasfasdf


  adasdasdasdasdfasd
  <sdfasdfadsffafs
  sadfdfdsasfasff

asfsadfasdfsad
";

  let lines = common::str_to_lines(src);

  match Parser::read_indented_block(&lines, Some(2), None, None, Some(2), Some(2)) {
    Ok((lines, _indent, line_diff, _empty_finish)) => {

      eprintln!("{:#?}", lines);

      assert_eq!(line_diff, 12);

      assert_eq!(
        lines.join("\n"),
"asdsafasfgasf  fwsdaf
asfsdafasdfffas  fsdfsaf
asfdfasd  fasdfafasdfasdf
asdfsdafasdfsda  fadsfsdf
asdfdsfsdafsadfaf
asfsffdsfasfasdf


adasdasdasdasdfasd
<sdfasdfadsffafs
sadfdfdsasfasff
"
      );
    },
    Err(e) => {
      eprintln!("{}", e);
      panic!();
    }
  };
}


#[test]
fn read_indented_block_06 () {

  let src = "

  sdasdasdasdasd
  adsadadasdasd

  adasdasdasdasdfasd
  <sdfasdfadsffafs
  sadfdfdsasfasff

asfsadfasdfsad
";

  let lines = common::str_to_lines(src);

  match Parser::read_indented_block(&lines, Some(2), Some(true), None, Some(2), None) {
    Ok((lines, _indent, line_diff, _empty_finish)) => {

      eprintln!("{:#?}", lines);

      assert_eq!(line_diff, 2);

      assert_eq!(
        lines.join("\n"),
"sdasdasdasdasd
adsadadasdasd"
      );
    },
    Err(e) => {
      eprintln!("{}", e);
      panic!();
    }
  };
}


#[test]
fn bullet_list_01 () {

  let src = String::from("
- This is the first bullet list item.");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree.tree = doctree.tree.walk_to_root();

  //eprintln!("{:#?}", doctree.tree.walk_to_root());

  match doctree.tree.node.children[1].data {
    TreeNodeType::BulletList{..}=> (),
    _ => panic!("No bullet list node where one was expected!\n")
  }

}


#[test]
fn bullet_list_02 () {

  let src = String::from("
  - List item 1

    Second paragraph of the list item.
    
    Third paragraph of this list item...
    
  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree.tree = doctree.tree.walk_to_root();

  eprintln!("{:#?}", doctree.tree);

  match doctree.tree.node.children[1].children[0].children[0].data {
    TreeNodeType::Paragraph => (),
    _ => panic!("Second non-whitespace child of ListItem wasn't a paragraph!\n")
  }

  match doctree.tree.node.children[1].children[0].children[2].data {
    TreeNodeType::Paragraph => (),
    _ => panic!("Third non-whitespace child of ListItem wasn't a paragraph!\n")
  }
}


#[test]
fn bullet_list_03 () {

  let src = String::from("
  - List item 1

    Second paragraph of the list item.

  - List item 2
    
  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree.tree = doctree.tree.walk_to_root();

  eprintln!("{:#?}", doctree.tree);

  match doctree.tree.node.children[1].children[0].data {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("First child of BulletList wasn't a ListItem!\n")
  }

  match doctree.tree.node.children[1].children[1].data {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("Second child of BulletList wasn't a ListItem!\n")
  }

  match doctree.tree.node.children[1].children[0].children[0].data {
    TreeNodeType::Paragraph => (),
    _ => panic!("First non-whitespace child of ListItem wasn't a paragraph!\n")
  }

  match doctree.tree.node.children[1].children[0].children[2].data {
    TreeNodeType::Paragraph => (),
    _ => panic!("Third non-whitespace child of ListItem wasn't a paragraph!\n")
  }

}



#[test]
fn bullet_list_04 () {

  let src = String::from("
  - List item 1

    Second paragraph of the list item.

  - List item 2

  asfasdfdsfasfasdfasfd
    
  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree.tree = doctree.tree.walk_to_root();

  eprintln!("{:#?}", doctree.tree);

  match doctree.tree.node.children[1].children[0].data {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("First child of BulletList wasn't a ListItem!\n")
  }

  match doctree.tree.node.children[1].children[1].data {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("Second child of BulletList wasn't a ListItem!\n")
  }

  match doctree.tree.node.children[1].children[0].children[0].data {
    TreeNodeType::Paragraph => (),
    _ => panic!("First non-whitespace child of ListItem wasn't a paragraph!\n")
  }

  match doctree.tree.node.children[1].children[0].children[2].data {
    TreeNodeType::Paragraph => (),
    _ => panic!("Third non-whitespace child of ListItem wasn't a paragraph!\n")
  }

  match doctree.tree.node.children[2].data {
    TreeNodeType::Paragraph => (),
    _ => panic!("No empty line after bullet list!\n")
  }

}



#[test]
fn bullet_list_05 () {

  let src = String::from("
  - List item 1

    Second paragraph of the list item.

    - Sublist item 1

    - Sublist item 2

  - List item 2

  asfasdfdsfasfasdfasfd
    
  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();

  eprintln!("{:#?}", doctree.tree);

  match doctree.tree.node.children[1].children[0].data {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("First child of BulletList wasn't a ListItem!\n")
  }

  match doctree.tree.node.children[1].children[0].children[4].data {
    TreeNodeType::BulletList{..} => (),
    _ => panic!("Second child of BulletList wasn't a sublist!\n")
  }

  match doctree.tree.node.children[1].children[1].data {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("Third child of BulletList wasn't a ListItem!\n")
  }

}



#[test]
fn bullet_list_06 () {

  let src = String::from("
  + List item 1

    Second paragraph of the list item.

    - Sublist item 1

    - Sublist item 2

      * Subsublist item 1

      * Subsublist item 2

    - Sublist item 3

  + List item 2

  asfasdfdsfasfasdfasfd
    
  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree.tree = doctree.tree.walk_to_root();

  eprintln!("{:#?}", doctree.tree);

  match doctree.tree.node.children[1].children[0].data {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("First non-whitespace child of BulletList wasn't a ListItem!\n")
  }

  match doctree.tree.node.children[1].children[0].children[0].data {
    TreeNodeType::Paragraph{..} => (),
    _ => panic!("No Paragraph!\n")
  }

  match doctree.tree.node.children[1].children[0].children[4].data {
    TreeNodeType::BulletList{..} => (),
    _ => panic!("No BulletList!\n")
  }

  match doctree.tree.node.children[1].children[0].children[4].children[0].data {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("No BulletListItem!\n")
  }

  match doctree.tree.node.children[1].children[0].children[4].children[1].data {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("No BulletListItem!\n")
  }

  match doctree.tree.node.children[1].children[0].children[4].children[1].children[2].data {
    TreeNodeType::BulletList{..} => (),
    _ => panic!("No BulletListItem!\n")
  }
  
  match doctree.tree.node.children[1].children[1].data {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("Second non-whitespace child of BulletList wasn't a BulletList!\n")
  }

}


#[test]
fn enumerated_list_01 () {

  let src = String::from("
  (i) List item 1
      with a valid second line

  Some unindented text.
    
  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree.tree = doctree.tree.walk_to_root();

  eprintln!("{:#?}", doctree.tree);

  match doctree.tree.node.children[1].data {
    TreeNodeType::EnumeratedList{..} => (),
    _ => panic!("No EnumeratedList detected!\n")
  }

  match doctree.tree.node.children[1].children[0].data {
    TreeNodeType::EnumeratedListItem{..} => (),
    _ => panic!("No EnumeratedListItem as child of EnumeratedList!\n")
  }

  match doctree.tree.node.children[1].children[0].children[0].data {
    TreeNodeType::Paragraph => (),
    _ => panic!("No Paragraph as child of EnumeratdListItem!\n")
  }

}


#[test]
fn enumerated_list_02 () {

  let src = String::from("
  (i) List item 1
      with a valid second line

      Second paragraph of this list item.

  (i) List item 1
      of a second list
    
  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree.tree = doctree.tree.walk_to_root();

  eprintln!("{:#?}", doctree.tree);

  match doctree.tree.node.children[1].data {
    TreeNodeType::EnumeratedList{..} => (),
    _ => panic!("No EnumeratedList detected!\n")
  }

  match doctree.tree.node.children[2].data {
    TreeNodeType::EnumeratedList{..} => (),
    _ => panic!("No EnumeratedList detected!\n")
  }

  match doctree.tree.node.children[1].children[0].children[0].data {
    TreeNodeType::Paragraph{..} => (),
    _ => panic!("No Paragraph detected!\n")
  }

  match doctree.tree.node.children[1].children[0].children[2].data {
    TreeNodeType::Paragraph{..} => (),
    _ => panic!("No second Paragraph detected!\n")
  }

}


#[test]
fn enumerated_list_03 () {

  let src = String::from("
  (i) a) List item ia
         with a valid second line

      Second paragraph of list item i.

  (i) List item 1
      of a second list
    
  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree.tree = doctree.tree.walk_to_root();

  eprintln!("{:#?}", doctree.tree);

  match doctree.tree.node.children[1].data {
    TreeNodeType::EnumeratedList { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].data {
    TreeNodeType::EnumeratedListItem { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].children[0].data {
    TreeNodeType::EnumeratedList { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].children[0].children[0].data {
    TreeNodeType::EnumeratedListItem { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].children[0].children[0].children[0].data {
    TreeNodeType::Paragraph => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].children[1].data {
    TreeNodeType::Paragraph => (),
    _ => panic!()
  }

  match doctree.tree.node.children[2].data {
    TreeNodeType::EnumeratedList { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[2].children[0].data {
    TreeNodeType::EnumeratedListItem { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[2].children[0].children[0].data {
    TreeNodeType::Paragraph => (),
    _ => panic!()
  }

}


#[test]
fn enumerated_list_04 () {

  let src = String::from("
  (#) First item of automatically numbered list

  (#) Second item of automatically numbered list

  (3) Third item that has to match with the internal counter of the list

  (#) Fourth item of the same list, with automatic numbering, again.
    
  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree.tree = doctree.tree.walk_to_root();

  eprintln!("{:#?}", doctree.tree);

  match doctree.tree.node.children[1].data {
    TreeNodeType::EnumeratedList { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].data {
    TreeNodeType::EnumeratedListItem { index_in_list, .. } => {
      if index_in_list != 1 { panic!() }
    },
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[1].data {
    TreeNodeType::EnumeratedListItem { index_in_list, .. } => {
      if index_in_list != 2 { panic!() }
    },
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[2].data {
    TreeNodeType::EnumeratedListItem { index_in_list, .. } => {
      if index_in_list != 3 { panic!() }
    },
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[3].data {
    TreeNodeType::EnumeratedListItem { index_in_list, .. } => {
      if index_in_list != 4 { panic!() }
    },
    _ => panic!()
  }

}


#[test]
fn enumerated_list_05 () {

  let src = String::from("
  (i) #) List item i1
         with a valid second line

      ii) List item i2

      #) List item i3

      First paragraph of list item i.

  (#) List item ii

  (iii) List item iii
    
  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree.tree = doctree.tree.walk_to_root();

  eprintln!("{:#?}", doctree.tree);

  match doctree.tree.node.children[1].data {
    TreeNodeType::EnumeratedList { n_of_items, .. } => {
      if n_of_items != 3 { panic!() }
    }
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].data {
    TreeNodeType::EnumeratedListItem { .. } => {}
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].children[0].data {
    TreeNodeType::EnumeratedList{ .. } => {}
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].children[0].children[0].data {
    TreeNodeType::EnumeratedListItem { kind, index_in_list, .. } => {
      if kind != EnumKind::Arabic || index_in_list != 1 { panic!() }
    }
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].children[1].data {
    TreeNodeType::EnumeratedList { kind, start_index, .. } => {
      if kind != EnumKind::LowerRoman || start_index != 2 { panic!() }
    }
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].children[1].children[0].data {
    TreeNodeType::EnumeratedListItem { kind, index_in_list, .. } => {
      if kind != EnumKind::LowerRoman || index_in_list != 2 { panic!() }
    }
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].children[1].children[1].data {
    TreeNodeType::EnumeratedListItem { kind, index_in_list, .. } => {
      if kind != EnumKind::LowerRoman || index_in_list != 3 { panic!() }
    }
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].children[2].data {
    TreeNodeType::Paragraph => {}
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[1].data {
    TreeNodeType::EnumeratedListItem { kind, index_in_list, .. } => {
      if kind != EnumKind::LowerRoman || index_in_list != 2 { panic!() }
    }
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[2].data {
    TreeNodeType::EnumeratedListItem { kind, index_in_list, .. } => {
      if kind != EnumKind::LowerRoman || index_in_list != 3 { panic!() }
    }
    _ => panic!()
  }

}


#[test]
fn mixed_nested_lists_01 () {

  let src = String::from("
  (i) * List item 1
        of a nested bullet list within
        an enumerated list...

      * Nested list item 2

        b) Nested enuemrated list in a nested bullet list

      Second paragraph of list item i.

  (ii) List item 2 of the parent list.

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree.tree = doctree.tree.walk_to_root();

  eprintln!("{:#?}", doctree.tree);

  match doctree.tree.node.children[1].data {
    TreeNodeType::EnumeratedList { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].data {
    TreeNodeType::EnumeratedListItem { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].children[0].data {
    TreeNodeType::BulletList { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].children[0].children[0].data {
    TreeNodeType::BulletListItem { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].children[0].children[1].data {
    TreeNodeType::BulletListItem { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].children[0].children[1].children[2].data {
    TreeNodeType::EnumeratedList { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].children[1].data {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[1].data {
    TreeNodeType::EnumeratedListItem { .. } => (),
    _ => panic!()
  }
 
}


#[test]
fn field_list_01 () {

  let src = String::from("
:field marker 1: Marker body
  with a line indented relative to it

:field marker 2: Body with
    some more indentation
    and a third line as well

    * and
    * why
    * not
    * a list
    * as well

An ending paragraph...

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree.tree = doctree.tree.walk_to_root();

  eprintln!("{:#?}", doctree.tree);

  match doctree.tree.node.children[1].data {
    TreeNodeType::FieldList { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].data {
    TreeNodeType::FieldListItem { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].children[0].data {
    TreeNodeType::Paragraph => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[1].data {
    TreeNodeType::FieldListItem { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[1].children[0].data {
    TreeNodeType::Paragraph => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[1].children[2].data {
    TreeNodeType::BulletList { .. } => (),
    _ => panic!()
  }

}


#[test]
fn field_list_02 () {

  let src = String::from("
    :field marker 1: Marker body
  and a line with too little indentation


An ending paragraph...

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree.tree = doctree.tree.walk_to_root();

  eprintln!("{:#?}", doctree.tree);

  match doctree.tree.node.children[1].data {
    TreeNodeType::FieldList { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[2].data {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[3].data {
    TreeNodeType::EmptyLine => (),
    _ => panic!()
  }

  match doctree.tree.node.children[4].data {
    TreeNodeType::EmptyLine => (),
    _ => panic!()
  }

  match doctree.tree.node.children[5].data {
    TreeNodeType::Paragraph => (),
    _ => panic!()
  }
}


#[test]
fn field_list_03 () {

  let src = String::from("
  :Date: 2001-08-16
  :Version: 1
  :Authors: - Me
            - Myself
            - I
  :Indentation: Since the field marker may be quite long, the second
     and subsequent lines of the field body do not have to line up
     with the first line, but they must be indented relative to the
     field name marker, and they must line up with each other.
  :Parameter i: integer

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree.tree = doctree.tree.walk_to_root();

  eprintln!("{:#?}", doctree.tree);


  match doctree.tree.node.children[1].data {
    TreeNodeType::FieldList { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].data {
    TreeNodeType::FieldListItem { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].children[0].data {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[1].data {
    TreeNodeType::FieldListItem { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[1].children[0].data {
    TreeNodeType::Paragraph { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[2].data {
    TreeNodeType::FieldListItem { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[2].children[0].data {
    TreeNodeType::BulletList { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[3].data {
    TreeNodeType::FieldListItem { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[3].children[0].data {
    TreeNodeType::Paragraph => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[4].data {
    TreeNodeType::FieldListItem { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[4].children[0].data {
    TreeNodeType::Paragraph => (),
    _ => panic!()
  }
}


#[test]
fn footnote_01 () {

  let src = String::from("
  .. [1] Here is a paragraph
     with body indent.

     * Bullet list inside foonote

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree.tree = doctree.tree.walk_to_root();

  eprintln!("{:#?}", doctree.tree);

  match doctree.tree.node.children[1].data {
    TreeNodeType::Footnote { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].data {
    TreeNodeType::Paragraph => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[1].data {
    TreeNodeType::EmptyLine => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[2].data {
    TreeNodeType::BulletList { .. } => (),
    _ => panic!()
  }
}


#[test]
fn footnote_02 () {

  let src = String::from("
  .. [1] Here is a paragraph

  .. [1] Another footnote with the same label (and target).
         The duplicate label should generate a warning.

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree.tree = doctree.tree.walk_to_root();

  eprintln!("{:#?}", doctree.tree);

  match doctree.tree.node.children[1].data {
    TreeNodeType::Footnote { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[1].children[0].data {
    TreeNodeType::Paragraph => (),
    _ => panic!()
  }

  match doctree.tree.node.children[2].data {
    TreeNodeType::Footnote { .. } => (),
    _ => panic!()
  }

  match doctree.tree.node.children[2].children[0].data {
    TreeNodeType::Paragraph => (),
    _ => panic!()
  }
}


#[test]
fn footnote_03 () {

  let src = String::from("
  .. [*] 1
  .. [*] 2
  .. [*] 3
  .. [*] 4
  .. [*] 5
  .. [*] 6
  .. [*] 7
  .. [*] 8
  .. [*] 9
  .. [*] 10
  .. [*] 11
  .. [*] 12
  .. [*] 13
  .. [*] 14
  .. [*] 15
  .. [*] 16
  .. [*] 17
  .. [*] 18
  .. [*] 19
  .. [*] 20
  .. [*] 21
  .. [*] 22
  .. [*] 23
  .. [*] 24
  .. [*] 25
  .. [*] 26
  .. [*] 27
  .. [*] 28
  .. [*] 29

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree.tree = doctree.tree.walk_to_root();

  eprintln!("{:#?}", doctree.tree);

  match &doctree.tree.node.children[1].data {
    TreeNodeType::Footnote {label, ..} => {
      if label == "*" {} else {panic!()}
    }
    _ => panic!()
  }

  match &doctree.tree.node.children[3].data {
    TreeNodeType::Footnote {label, ..} => {
      if label == "‡" {} else {panic!()}
    }
    _ => panic!()
  }

  match &doctree.tree.node.children[11].data {
    TreeNodeType::Footnote {label, ..} => {
      if label == "**" {} else {panic!()}
    }
    _ => panic!()
  }

  match &doctree.tree.node.children[13].data {
    TreeNodeType::Footnote {label, ..} => {
      if label == "‡‡" {} else {panic!()}
    }
    _ => panic!()
  }

  match &doctree.tree.node.children[21].data {
    TreeNodeType::Footnote {label, ..} => {
      if label == "***" {} else {panic!()}
    }
    _ => panic!()
  }

  match &doctree.tree.node.children[29].data {
    TreeNodeType::Footnote {label, ..} => {
      if label == "♦♦♦" {} else {panic!()}
    }
    _ => panic!()
  }
}


#[test]
fn footnote_04 () {

  let src = String::from("
  .. [2] 1
  .. [#test-with-mixed] 2
  .. [#] 3
  .. [#second] 4
  .. [#] 5

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree.tree = doctree.tree.walk_to_root();

  eprintln!("{:#?}", doctree.tree);

  match &doctree.tree.node.children[1].data {
    TreeNodeType::Footnote { label, target, ..} => {
      if label == "2" && target == "2" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.tree.node.children[2].data {
    TreeNodeType::Footnote { label, target, ..} => {
      if label == "1" && target == "test-with-mixed" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.tree.node.children[3].data {
    TreeNodeType::Footnote { label, target, ..} => {
      if label == "3" && target == "3" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.tree.node.children[4].data {
    TreeNodeType::Footnote { label, target, ..} => {
      if label == "4" && target == "second" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.tree.node.children[5].data {
    TreeNodeType::Footnote { label, target, ..} => {
      if label == "5" && target == "5" {} else { panic!() }
    }
    _ => panic!()
  }
}


#[test]
fn footnote_05 () {

  let src = String::from("
  .. [2] 1
  .. [#test-with-mixed] 2
  .. [*] .. [#nested] 4
  .. [*] 5
  .. [2] 5

  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree, None, 0, None);

  doctree = parser.parse().unwrap_tree();
  doctree.tree = doctree.tree.walk_to_root();

  eprintln!("{:#?}", doctree.tree);

  match &doctree.tree.node.children[1].data {
    TreeNodeType::Footnote { label, target, .. } => {
      if label == "2" && target == "2" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.tree.node.children[2].data {
    TreeNodeType::Footnote { label, target, .. } => {
      if label == "1" && target == "test-with-mixed" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.tree.node.children[3].data {
    TreeNodeType::Footnote { label, target, .. } => {
      if label == "*" && target == "*" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.tree.node.children[3].children[0].data {
    TreeNodeType::Footnote { label, target, .. } => {
      if label == "3" && target == "nested" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.tree.node.children[4].data {
    TreeNodeType::Footnote { label, target, .. } => {
      if label == "†" && target == "†" {} else { panic!() }
    }
    _ => panic!()
  }

  match &doctree.tree.node.children[5].data {
    TreeNodeType::Footnote { label, target, .. } => {
      if label == "2" && target == "2" {} else { panic!() }
    }
    _ => panic!()
  }
}


#[test]
fn upper_roman_to_usize_01 () {

  let iii = "III";
  let iv = "IV";
  let mmmmcmxcix = "MMMMCMXCIX";
  let over_max = "MMMMCMXCX";

  let iii_as_u32 = match Parser::upper_roman_to_usize(iii) {
    Some(num) => num,
    None => panic!("Couldn't convert the Roman numeral III to u32\n")
  };
  let iv_as_u32 = match Parser::upper_roman_to_usize(iv) {
    Some(num) => num,
    None => panic!("Couldn't convert the Roman numeral iv to u32\n")
  };
  let mmmmcmxcix_as_u32 = match Parser::upper_roman_to_usize(mmmmcmxcix) {
    Some(num) => num,
    None => panic!("Couldn't convert the Roman numeral MMMMCMXCIX to u32\n")
  };
  let over_max_as_u32 = Parser::upper_roman_to_usize(over_max);

  assert_eq!(3, iii_as_u32);
  assert_eq!(4, iv_as_u32);
  assert_eq!(4999, mmmmcmxcix_as_u32);
  assert_eq!(None, over_max_as_u32);

}


#[test]
fn lower_roman_to_usize_01 () {

  let iii = "iii";
  let iv = "iv";
  let mmmmcmxcix = "mmmmcmxcix";
  let over_max = "mmmmcmxcx";

  let iii_as_u32 = match Parser::lower_roman_to_usize(iii) {
    Some(num) => num,
    None => panic!("Couldn't convert the Roman numeral iii to u32\n")
  };
  let iv_as_u32 = match Parser::lower_roman_to_usize(iv) {
    Some(num) => num,
    None => panic!("Couldn't convert the Roman numeral iv to u32\n")
  };
  let mmmmcmxcix_as_u32 = match Parser::lower_roman_to_usize(mmmmcmxcix) {
    Some(num) => num,
    None => panic!("Couldn't convert the Roman numeral mmmmcmxcix to u32\n")
  };
  let over_max_as_u32 = Parser::lower_roman_to_usize(over_max);

  assert_eq!(3, iii_as_u32);
  assert_eq!(4, iv_as_u32);
  assert_eq!(4999, mmmmcmxcix_as_u32);
  assert_eq!(None, over_max_as_u32);

}
