


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
fn bullet_list_01 () {

  let src = String::from("
- This is the first bullet list item.  The blank line above the
  first list item is required; blank lines between list items
  (such as below this paragraph) are optional.");

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

  match doctree.tree.node.children[1].data {
    TreeNodeType::BulletList{..}=> (),
    _ => panic!("No bullet list node where one was expected!\n")
  }

}


#[test]
fn bullet_list_02 () {

  let src = String::from("
  - List item 1

    Second paragraph of the list item.");

  let mut doctree = DocTree::new(String::from("test"));

  let mut parser = Parser::new(src, doctree);

  doctree = match parser.parse() {
    Ok(doctree) => doctree.unwrap(),
    Err(e) => {
      eprintln!("{}", e);
      panic!();
    }
  };

  eprintln!("{:#?}", doctree.tree.walk_to_root());

  todo!()

}
