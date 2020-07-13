


use super::*;
use crate::utils;

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

  let lines = utils::str_to_lines(src);

  eprintln!("{:#?}", lines);

  let block = match Parser::read_text_block(&lines, 2, None) {
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

  let lines = utils::str_to_lines(src);

  eprintln!("{:#?}", lines);

  match Parser::read_text_block(&lines, 2, None) {
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

  let lines = utils::str_to_lines(src);

  eprintln!("{:#?}", lines);

  match Parser::read_text_block(&lines, 2, Some(true)) {
    Ok(block) => {

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

  let lines = utils::str_to_lines(src);

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

  let lines = utils::str_to_lines(src);

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

  let lines = utils::str_to_lines(src);

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

  let lines = utils::str_to_lines(src);

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

  let lines = utils::str_to_lines(src);

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

  let lines = utils::str_to_lines(src);

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

  let mut parser = Parser::new(src, doctree);

  doctree = match parser.parse() {
    Ok(doctree) => doctree.unwrap(),
    Err(e) => {
      eprintln!("{}", e);
      panic!();
    }
  };

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

  let mut parser = Parser::new(src, doctree);

  doctree = match parser.parse() {
    Ok(doctree) => doctree.unwrap(),
    Err(e) => {
      eprintln!("{}", e);
      panic!();
    }
  };

  // eprintln!("{:#?}", doctree.tree);

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

  let mut parser = Parser::new(src, doctree);

  doctree = match parser.parse() {
    Ok(doctree) => doctree.unwrap(),
    Err(e) => {
      eprintln!("{}", e);
      panic!();
    }
  };

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

  let mut parser = Parser::new(src, doctree);

  doctree = match parser.parse() {
    Ok(doctree) => doctree.unwrap(),
    Err(e) => {
      eprintln!("{}", e);
      panic!();
    }
  };

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

  let mut parser = Parser::new(src, doctree);

  doctree = match parser.parse() {
    Ok(doctree) => doctree.unwrap(),
    Err(e) => {
      eprintln!("{}", e);
      panic!();
    }
  };

  eprintln!("{:#?}", doctree.tree);

  match doctree.tree.node.children[1].children[0].data {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("First child of BulletList wasn't a ListItem!\n")
  }

  match doctree.tree.node.children[1].children[1].data {
    TreeNodeType::BulletList{..} => (),
    _ => panic!("Second child of BulletList wasn't a sublist!\n")
  }

  match doctree.tree.node.children[1].children[2].data {
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

  let mut parser = Parser::new(src, doctree);

  doctree = match parser.parse() {
    Ok(doctree) => doctree.unwrap(),
    Err(e) => {
      eprintln!("{}", e);
      panic!();
    }
  };

  eprintln!("{:#?}", doctree.tree);

  match doctree.tree.node.children[1].children[0].data {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("First non-whitespace child of BulletList wasn't a ListItem!\n")
  }

  match doctree.tree.node.children[1].children[1].data {
    TreeNodeType::BulletList{..} => (),
    _ => panic!("Second non-whitespace child of BulletList wasn't a BulletList!\n")
  }

  match doctree.tree.node.children[1].children[1].children[0].data {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("First non-whitespace child of sublist wasn't a ListItem!\n")
  }

  match doctree.tree.node.children[1].children[1].children[2].data {
    TreeNodeType::BulletList{..} => (),
    _ => panic!("Third non-whitespace child of sublist wasn't a BulletList!\n")
  }

  match doctree.tree.node.children[1].children[1].children[2].children[1].data {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("Second non-whitespace child of subsublist wasn't a ListItem!\n")
  }

  match doctree.tree.node.children[1].children[1].children[3].data {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("Fourth non-whitespace child of sublist wasn't a ListItem!\n")
  }

  match doctree.tree.node.children[1].children[2].data {
    TreeNodeType::BulletListItem{..} => (),
    _ => panic!("Third non-whitespace child of BulletList wasn't a ListItem!\n")
  }

  match doctree.tree.node.children[2].data {
    TreeNodeType::Paragraph{..} => (),
    _ => panic!("Item after BulletList at document root level wasn't a Paragraph!\n")
  }

}


#[test]
fn enumerated_list_01 () {

  let src = String::from("
  (i) List item 1
    
  ");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree);

  doctree = match parser.parse() {
    Ok(doctree) => doctree.unwrap(),
    Err(e) => {
      eprintln!("{}", e);
      panic!();
    }
  };

  // eprintln!("{:#?}", doctree.tree);

  match doctree.tree.node.children[1].data {
    TreeNodeType::EnumeratedList{..} => (),
    _ => panic!("No EnumeratedList detected!\n")
  }

  todo!()

}



#[test]
fn upper_roman_to_usize_01 () {

  let iii = "III";
  let iv = "IV";
  let mmmmcmxcix = "MMMMCMXCIX";
  let over_max = "MMMMCMXCX";

  let iii_as_u32 = match Parser::upper_roman_to_u32(iii) {
    Some(num) => num,
    None => panic!("Couldn't convert the Roman numeral III to u32\n")
  };
  let iv_as_u32 = match Parser::upper_roman_to_u32(iv) {
    Some(num) => num,
    None => panic!("Couldn't convert the Roman numeral iv to u32\n")
  };
  let mmmmcmxcix_as_u32 = match Parser::upper_roman_to_u32(mmmmcmxcix) {
    Some(num) => num,
    None => panic!("Couldn't convert the Roman numeral MMMMCMXCIX to u32\n")
  };
  let over_max_as_u32 = Parser::upper_roman_to_u32(over_max);

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
