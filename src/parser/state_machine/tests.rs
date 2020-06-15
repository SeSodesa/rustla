


use super::*;
use crate::utils;

#[cfg(test)]


#[test]
fn read_text_block_01 () {

  let src = "\n\
  \n\
  asdsafasfgasffwsdaf\n\
  asfsdafasdfffasfsdfsaf\n\
  asfdfasdfasdfafasdfasdf\n\
  asdfsdafasdfsdafadsfsdf\n\
  \n\
  asdfdsfsdafsadfaf\n\
  asfsffdsfasfasdf\n\
  asdfsdafasdfasdfa\n\
    
  ";

  let lines = utils::str_to_lines(src);

  println!("{:#?}", lines);

  let block = match StateMachine::read_text_block(&lines, 2, None) {
    Ok(block) => block,
    Err(e) => {
      eprintln!("{}", e);
      panic!();
    }
  };
  println!("{:#?}", block);

  let block_str = block.join("\n");

  assert_eq!("asdsafasfgasffwsdaf\n\
  asfsdafasdfffasfsdfsaf\n\
  asfdfasdfasdfafasdfasdf\n\
  asdfsdafasdfsdafadsfsdf", block_str);

}