/// ## test_block_reading
/// A submodule for tests related to reading blocks of text.
///
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi
use super::*;

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

    let (block, offset) = match Parser::read_text_block(&lines, 2, false, false, None) {
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

    match Parser::read_text_block(&lines, 2, false, false, None) {
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

    match Parser::read_indented_block(&lines, Some(2), None, Some(true), None, None, false) {
        Ok((lines, _indent, line_diff, _empty_finish)) => {
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
        Err(e) => {
            eprintln!("{}", e);
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

    match Parser::read_indented_block(&lines, Some(2), None, None, Some(2), None, false) {
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
        }
        Err(e) => {
            eprintln!("{}", e);
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

    match Parser::read_indented_block(&lines, Some(2), None, None, None, None, false) {
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
        }
        Err(e) => {
            eprintln!("{}", e);
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

    match Parser::read_indented_block(&lines, Some(2), None, None, Some(2), Some(2), false) {
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
        }
        Err(e) => {
            eprintln!("{}", e);
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

    match Parser::read_indented_block(&lines, Some(2), None, None, Some(2), Some(2), false) {
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
        }
        Err(e) => {
            eprintln!("{}", e);
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

    match Parser::read_indented_block(&lines, Some(2), Some(true), None, Some(2), None, false) {
        Ok((lines, _indent, line_diff, _empty_finish)) => {
            eprintln!("{:#?}", lines);

            assert_eq!(line_diff, 2);

            assert_eq!(
                lines.join("\n"),
                "sdasdasdasdasd
adsadadasdasd"
            );
        }
        Err(e) => {
            eprintln!("{}", e);
            panic!();
        }
    };
}
