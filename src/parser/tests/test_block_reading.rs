/*!
A submodule for testing block reading functions.

Copyright © 2020 Santtu Söderholm
*/

use super::*;
use crate::parser::types_and_aliases::IndentedBlockResult;

#[cfg(test)]
#[test]
fn read_text_block_01() {
    let src = "

asdsafasfga  sffwsdaf
asfsdafasdfffasfsdfsaf
asfdfasdfasdfafasdfasdf
asdfsdafasdfsdafadsfsdf

asdfdsfsdafsadfaf
asfsffdsfasfasdf
asdfsdafasdfasdfa

";

    let lines = crate::common::str_to_lines(src);

    eprintln!("{:#?}", lines);

    let (block, offset) = match Parser::read_text_block(&lines, 2, false, false, None, true) {
        Ok(block) => block,
        Err(e) => {
            eprintln!("{}", e);
            panic!();
        }
    };

    eprintln!("{:#?}", block);

    let block_str = block.join("\n");

    assert_eq!(
        "asdsafasfga  sffwsdaf
asfsdafasdfffasfsdfsaf
asfdfasdfasdfafasdfasdf
asdfsdafasdfsdafadsfsdf",
        block_str
    );
}

#[test]
fn read_text_block_02() {
    let src = "

asdsafasfgasf  fwsdaf
asfsdafasdfffasfsdfsaf
  asfdfasdfasdfafasdfasdf
asdfsdafasdfsdafadsfsdf

asdfdsfsdafsadfaf
asfsffdsfasfasdf
asdfsdafasdfasdfa

";

    let lines = crate::common::str_to_lines(src);

    eprintln!("{:#?}", lines);

    match Parser::read_text_block(&lines, 2, false, false, None, true) {
        Ok((lines, offset)) => {
            assert_eq!(
                vec!["asdsafasfgasf  fwsdaf", "asfsdafasdfffasfsdfsaf"],
                lines
            )
        }
        Err(e) => {
            eprintln!("{:#?}", e);
            panic!()
        }
    };
}

#[test]
fn read_text_block_03() {
    let src = "

  asdsafasfgasf  fwsdaf
  asfsdafasdfffasfsdfsaf
  asfdfasdfasdfafasdfasdf
  asdfsdafasdfsdafadsfsdf

asdfdsfsdafsadfaf
asfsffdsfasfasdf
asdfsdafasdfasdfa

";

    let lines = crate::common::str_to_lines(src);

    eprintln!("{:#?}", lines);

    match Parser::read_text_block(&lines, 2, true, false, None, true) {
        Ok((block, offset)) => {
            eprintln!("{:#?}", block);

            assert_eq!(
                block.join("\n"),
                "  asdsafasfgasf  fwsdaf
  asfsdafasdfffasfsdfsaf
  asfdfasdfasdfafasdfasdf
  asdfsdafasdfsdafadsfsdf"
            );
        }
        Err(e) => {
            eprintln!("{}", e);
            panic!();
        }
    };
}

#[test]
fn read_indented_block_01() {
    let src = "

  asdsafasfgasf  fwsdaf
      asfsdafasdfffas  fsdfsaf
    asfdfasd  fasdfafasdfasdf
  asdfsdafasdfsda  fadsfsdf

asdfdsfsdafsadfaf
asfsffdsfasfasdf
asdfsdafasdfasdfa

";

    let lines = crate::common::str_to_lines(src);

    match Parser::read_indented_block(&lines, 2, false, true, None, None, false) {
        IndentedBlockResult::Ok {lines, minimum_indent, offset, blank_finish } => {
            eprintln!("{:#?}", lines);

            assert_eq!(
                lines.join("\n"),
                "asdsafasfgasf  fwsdaf
    asfsdafasdfffas  fsdfsaf
  asfdfasd  fasdfafasdfasdf
asdfsdafasdfsda  fadsfsdf
"
            );
        }
        _ => {
            panic!();
        }
    };
}

#[test]
fn read_indented_block_02() {
    let src = "

    asdsafasfgasf  fwsdaf
      asfsdafasdfffas  fsdfsaf
    asfdfasd  fasdfafasdfasdf
  asdfsdafasdfsda  fadsfsdf

asdfdsfsdafsadfaf
asfsffdsfasfasdf
asdfsdafasdfasdfa

";

    let lines = crate::common::str_to_lines(src);

    match Parser::read_indented_block(&lines, 2, false, true, Some(2), None, false) {
        IndentedBlockResult::Ok {lines, minimum_indent, offset, blank_finish } => {
            eprintln!("{:#?}", lines);

            assert_eq!(
                lines.join("\n"),
                "  asdsafasfgasf  fwsdaf
    asfsdafasdfffas  fsdfsaf
  asfdfasd  fasdfafasdfasdf
asdfsdafasdfsda  fadsfsdf
"
            );
        }
        _ => {
            panic!();
        }
    };
}

#[test]
fn read_indented_block_03() {
    let src = "

 asdsafasfgasf  fwsdaf
      asfsdafasdfffas  fsdfsaf
    asfdfasd  fasdfafasdfasdf
  asdfsdafasdfsda  fadsfsdf
  asdfdsfsdafsadfaf
asfsffdsfasfasdf
asdfsdafasdfasdfa

";

    let lines = crate::common::str_to_lines(src);

    match Parser::read_indented_block(&lines, 2, false, true, None, None, false) {
        IndentedBlockResult::Ok {lines, minimum_indent, offset, blank_finish } => {
            eprintln!("{:#?}", lines);

            assert_eq!(
                lines.join("\n"),
                "asdsafasfgasf  fwsdaf
     asfsdafasdfffas  fsdfsaf
   asfdfasd  fasdfafasdfasdf
 asdfsdafasdfsda  fadsfsdf
 asdfdsfsdafsadfaf"
            );
        }
        _ => {
            panic!();
        }
    };
}

#[test]
fn read_indented_block_04() {
    let src = "

* asdsafasfgasf  fwsdaf
  asfsdafasdfffas  fsdfsaf
  asfdfasd  fasdfafasdfasdf
  asdfsdafasdfsda  fadsfsdf
  asdfdsfsdafsadfaf
  asfsffdsfasfasdf

asdfsdafasdfasdfa

";

    let lines = crate::common::str_to_lines(src);

    match Parser::read_indented_block(&lines, 2, false, true, Some(2), Some(2), false) {
        IndentedBlockResult::Ok {lines, minimum_indent, offset, blank_finish } => {
            eprintln!("{:#?}", lines);

            assert_eq!(offset, 7);

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
        }
        _ => {
            panic!();
        }
    };
}

#[test]
fn read_indented_block_05() {
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

    let lines = crate::common::str_to_lines(src);

    match Parser::read_indented_block(&lines, 2, false, true, Some(2), Some(2), false) {
        IndentedBlockResult::Ok {lines, minimum_indent, offset, blank_finish } => {
            eprintln!("{:#?}", lines);

            assert_eq!(offset, 12);

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
        }
        _ => {
            panic!();
        }
    };
}

#[test]
fn read_indented_block_06() {
    let src = "

  sdasdasdasdasd
  adsadadasdasd

  adasdasdasdasdfasd
  <sdfasdfadsffafs
  sadfdfdsasfasff

asfsadfasdfsad
";

    let lines = crate::common::str_to_lines(src);

    match Parser::read_indented_block(&lines, 2, true, true, Some(2), None, false) {
        IndentedBlockResult::Ok {lines, minimum_indent, offset, blank_finish } => {
            eprintln!("{:#?}", lines);

            assert_eq!(offset, 2);

            assert_eq!(
                lines.join("\n"),
                "sdasdasdasdasd
adsadadasdasd"
            );
        }
        _ => {
            panic!();
        }
    };
}
