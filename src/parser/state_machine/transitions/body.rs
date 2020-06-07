use super::*;



pub fn tokenize_blank_lines (lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}", tt);

  let m = cs.get(0).unwrap();

  lex.set_lexeme_limits(&m);

  lex.tokens.push(
    Token::new(
      tt,
      String::from("\n\n"),
      m.start() + lex.pos.pos,
      m.end() + lex.pos.pos,
    )
  );

  // lex.update_pos();

}


/// ### tokenize_section_title
/// Creates the tokens related to overlined titles
pub fn tokenize_section_title (lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

  let ws = cs.get(1).unwrap();

  lex.set_lexeme_limits(&ws);

  lex.tokens.push(
    Token::new(
      TokenType::BlankLines,
      String::from("\n\n"),
      ws.start() + lex.pos.pos,
      ws.end() + lex.pos.pos,
    )
  );

  //lex.update_pos();

  let text = cs.get(3).unwrap();
  let title = cs.get(2).unwrap();

  lex.set_lexeme_limits(&title);

  lex.tokens.push(
    Token::new(
      tt,
      String::from(text.as_str()),
      title.start() + lex.pos.pos,
      title.end() + lex.pos.pos,
    )
  );

  // lex.update_pos();

}


/// ### Tokenize_unnumbered_list
/// Tokenizes an unnumbered list
pub fn tokenize_unnumbered_list(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

  let list_item = cs.get(0).unwrap();

  println!("Tokenizing preceding whitespace...\n");

  let ws = cs.get(1).unwrap();

  lex.set_lexeme_limits(&ws);

  // lex.update_pos();

  lex.set_lexeme_limits(&list_item);

  lex.tokens.push(
    Token::new(
      tt,
      String::from(""),
      list_item.start() + lex.pos.pos,
      list_item.end() + lex.pos.pos
    )
  );

  let bullet = cs.get(2).unwrap();

  lex.set_lexeme_limits(&bullet);

  lex.tokens.push(
    Token::new(
      TokenType::Bullet,
      bullet.as_str().to_string(),
      bullet.start() + lex.pos.pos,
      bullet.end() + lex.pos.pos
    )
  );

  // lex.update_pos();

  lex.state = State::Inline;

}

/// ### Tokenize_numbered_list
/// Tokenizes an unnumbered list
pub fn tokenize_numbered_list(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### Tokenize_alpha_list
/// Tokenizes an unnumbered list
pub fn tokenize_alpha_list(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}

/// ### Tokenize_definition_list
/// Tokenizes an unnumbered list
pub fn tokenize_definition_list(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}

/// ### Tokenize_field_list
/// Tokenizes an unnumbered list
pub fn tokenize_field_list(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_literal_block
/// Tokenizes a literal block
pub fn tokenize_literal_block(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_per_line_literal_block
/// Tokenizes a per-line literal block
pub fn tokenize_per_line_literal_block(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_line_block
/// Tokenizes a line block
pub fn tokenize_line_block(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_paragraph
/// Tokenizes a paragraph
pub fn tokenize_paragraph(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

  let m = cs.get(0).unwrap();
  let ws = cs.get(1).unwrap();
  let par = cs.get(2).unwrap();

  lex.set_lexeme_limits(&ws);

  // lex.tokens.push(
  //   Token::new(
  //     TokenType::BlankLines,
  //     String::from("\n\n"),
  //     ws.start() + lex.pos.pos,
  //     ws.end() + lex.pos.pos,
  //   )
  // );

  // lex.update_pos();

  lex.set_lexeme_limits(&par);

  lex.tokens.push(
    Token::new(
      tt,
      String::from(""),
      par.start() + lex.pos.pos,
      par.end() + lex.pos.pos,
    )
  );

  lex.state = State::Inline;

}


/// ### tokenize_general_directive
/// Tokenizes a paragraph
pub fn tokenize_general_directive(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}



/// ### tokenize_reference_target
/// Tokenizes a reference target
pub fn tokenize_reference_target(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_footnote_or_citation_target
/// Tokenizes both footnote and citation targets
pub fn tokenize_footnote_or_citation_target(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_substitution_definition
/// Tokenizes a subsititution definition target
pub fn tokenize_substitution_definition(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}


/// ### tokenize_comment
/// Tokenizes a comment target
pub fn tokenize_comment(lex: &mut Parser, tt:TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}\n", tt);

}

