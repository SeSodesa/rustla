


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

  let block = match StateMachine::read_text_block(&lines, 2, None) {
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

  match StateMachine::read_text_block(&lines, 2, None) {
    Ok(_) => panic!("There was indent where one was not allowed..."),
    Err(e) => {
      assert_eq!(
        "No indent allowed but indent found on line 4!\nComputer says no...\n",
        e
      )
    }
  };

}
