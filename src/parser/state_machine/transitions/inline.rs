
use super::*;


pub fn tokenize_escape (parser: &mut Parser, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let s = cs.get(0).unwrap().start();
  let e = cs.get(0).unwrap().end();
  let c = cs.get(1).unwrap();

  parser.tokens.push(
    Token::new(
      tt,
      String::from(c.as_str()),
      s + parser.pos.pos,
      e + parser.pos.pos,
    )
  );
}

pub fn tokenize_code (parser: &mut Parser, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  let code = cs.get(1).unwrap();

  parser.set_lexeme_limits(&m);

  parser.tokens.push(
    Token::new(
      tt,
      String::from(code.as_str()),
      m.start() + parser.pos.pos,
      m.end() + parser.pos.pos,
    )
  );

  // parser.update_pos();

}


pub fn tokenize_inline_target_ref (parser: &mut Parser, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  let link_alias = cs.get(1).unwrap();
  let link = cs.get(2).unwrap();
  let ref_type = cs.get(3).unwrap();

  parser.set_lexeme_limits(&m);

  parser.tokens.push(
    Token::new(
      tt,
      String::from(""),
      m.start() + parser.pos.pos,
      m.end() + parser.pos.pos
    )
  );

  parser.set_lexeme_limits(&link_alias);

  parser.tokens.push(
    Token::new(
      TokenType::LinkAlias,
      String::from(link_alias.as_str()),
      link_alias.start() + parser.pos.pos,
      link_alias.end() + parser.pos.pos
    )
  );

  parser.set_lexeme_limits(&link);

  parser.tokens.push(
    Token::new(
      TokenType::Hyperlink,
      String::from(link.as_str()),
      link.start() + parser.pos.pos,
      link.end() + parser.pos.pos
    )
  );

  parser.set_lexeme_limits(&ref_type);

  parser.tokens.push(
    Token::new(
      TokenType::RefAnonOrNot,
      String::from(ref_type.as_str()),
      ref_type.start() + parser.pos.pos,
      ref_type.end() + parser.pos.pos
    )
  );

  parser.set_lexeme_limits(&m);

  // parser.update_pos();

}


pub fn tokenize_inline_ref (parser: &mut Parser, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  let target = cs.get(1).unwrap();
  let ref_type = cs.get(2).unwrap();

  parser.set_lexeme_limits(&m);

  parser.tokens.push(
    Token::new(
      tt,
      String::from(""),
      m.start() + parser.pos.pos,
      m.end() + parser.pos.pos
    )
  );

  parser.set_lexeme_limits(&target);

  parser.tokens.push(
    Token::new(
      TokenType::Target,
      String::from(target.as_str()),
      target.start() + parser.pos.pos,
      target.end() + parser.pos.pos
    )
  );

  parser.set_lexeme_limits(&ref_type);

  parser.tokens.push(
    Token::new(
      TokenType::RefAnonOrNot,
      String::from(ref_type.as_str()),
      ref_type.start() + parser.pos.pos,
      ref_type.end() + parser.pos.pos
    )
  );

  parser.set_lexeme_limits(&m);

  // parser.update_pos();

}

pub fn tokenize_role_content (parser: &mut Parser, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  let role = cs.get(1).unwrap();
  let content = cs.get(2).unwrap();

  parser.set_lexeme_limits(&m);

  parser.tokens.push(
    Token::new(
      tt,
      String::from(""),
      m.start() + parser.pos.pos,
      m.end() + parser.pos.pos,
    )
  );

  println!("\nTokenizing Role...");

  parser.set_lexeme_limits(&role);

  parser.tokens.push(
    Token::new(
      TokenType::Role,
      String::from(role.as_str()),
      role.start() + parser.pos.pos,
      role.end() + parser.pos.pos,
    )
  );

  // parser.update_pos();

  println!("Tokenizing Content...");

  parser.set_lexeme_limits(&content);

  parser.tokens.push(
    Token::new(
      TokenType::Content,
      String::from(content.as_str()),
      content.start() + parser.pos.pos,
      content.end() + parser.pos.pos,
    )
  );

  parser.set_lexeme_limits(&m);

  // parser.update_pos();

}

pub fn tokenize_content_role (parser: &mut Parser, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  let content = cs.get(1).unwrap();
  let role = cs.get(2).unwrap();

  parser.set_lexeme_limits(&m);

  parser.tokens.push(
    Token::new(
      tt,
      String::from(""),
      m.start() + parser.pos.pos,
      m.end() + parser.pos.pos,
    )
  );

  println!("\nTokenizing Content...");

  parser.set_lexeme_limits(&content);

  parser.tokens.push(
    Token::new(
      TokenType::Content,
      String::from(role.as_str()),
      content.start() + parser.pos.pos,
      content.end() + parser.pos.pos,
    )
  );

  // parser.update_pos();

  println!("Tokenizing Role...");

  parser.set_lexeme_limits(&role);

  parser.tokens.push(
    Token::new(
      TokenType::Role,
      String::from(content.as_str()),
      role.start() + parser.pos.pos,
      role.end() + parser.pos.pos,
    )
  );

  parser.set_lexeme_limits(&m);

  // parser.update_pos();


}


pub fn tokenize_strong_emphasis (parser: &mut Parser, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  let text = cs.get(1).unwrap();

  parser.set_lexeme_limits(&m);

  parser.tokens.push(
    Token::new(
      tt,
      String::from(text.as_str()),
      m.start() + parser.pos.pos,
      m.end() + parser.pos.pos,
    )
  );

  // parser.update_pos();

}


pub fn tokenize_emphasis (parser: &mut Parser, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  let text = cs.get(1).unwrap();

  parser.set_lexeme_limits(&m);

  parser.tokens.push(
    Token::new(
      tt,
      String::from(text.as_str()),
      m.start() + parser.pos.pos,
      m.end() + parser.pos.pos,
    )
  );

  // parser.update_pos();

}

pub fn tokenize_footnote_or_citation (parser: &mut Parser, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  let target = cs.get(1).unwrap();

  parser.set_lexeme_limits(&m);

  parser.tokens.push(
    Token::new(
      tt,
      String::from(target.as_str()),
      m.start() + parser.pos.pos,
      m.end() + parser.pos.pos,
    )
  );

  // parser.update_pos();

}


pub fn tokenize_uri (parser: &mut Parser, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();

  parser.set_lexeme_limits(&m);

  parser.tokens.push(
    Token::new(
      tt,
      String::from(m.as_str()),
      m.start() + parser.pos.pos,
      m.end() + parser.pos.pos,
    )
  );

  if let Some(scheme) = cs.get(1) {
    
    parser.set_lexeme_limits(&scheme);

    parser.tokens.push(
      Token::new(
        TokenType::Scheme,
        String::from(scheme.as_str()),
        scheme.start() + parser.pos.pos,
        scheme.end() + parser.pos.pos
      )
    );

  };

  if let Some(authority) = cs.get(3) {
    
    parser.set_lexeme_limits(&authority);

    parser.tokens.push(
      Token::new(
        TokenType::Authority,
        String::from(authority.as_str()),
        authority.start() + parser.pos.pos,
        authority.end() + parser.pos.pos
      )
    );

  };

  if let Some(path) = cs.get(4) {
    
    parser.set_lexeme_limits(&path);

    parser.tokens.push(
      Token::new(
        TokenType::Path,
        String::from(path.as_str()),
        path.start() + parser.pos.pos,
        path.end() + parser.pos.pos
      )
    );

  };

  if let Some(query) = cs.get(6) {
    
    parser.set_lexeme_limits(&query);

    parser.tokens.push(
      Token::new(
        TokenType::Query,
        String::from(query.as_str()),
        query.start() + parser.pos.pos,
        query.end() + parser.pos.pos
      )
    );

  };

  if let Some(fragment) = cs.get(8) {
    
    parser.set_lexeme_limits(&fragment);

    parser.tokens.push(
      Token::new(
        TokenType::Fragment,
        String::from(fragment.as_str()),
        fragment.start() + parser.pos.pos,
        fragment.end() + parser.pos.pos
      )
    );

  };

  parser.set_lexeme_limits(&m);

  // parser.update_pos();

}

pub fn tokenize_inline_whitespace (parser: &mut Parser, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();

  parser.set_lexeme_limits(&m);
  
  parser.tokens.push(
    Token::new(
      tt,
      String::from(" "),
      m.start() + parser.pos.pos,
      m.end() + parser.pos.pos,
    )
  );

  // parser.update_pos();

}


pub fn tokenize_text_no_ldelim (parser: &mut Parser, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();

  parser.set_lexeme_limits(&m);
  
  parser.tokens.push(
    Token::new(
      tt,
      String::from(m.as_str()),
      m.start() + parser.pos.pos,
      m.end() + parser.pos.pos,
    )
  );

  // parser.update_pos();

}

pub fn tokenize_text (parser: &mut Parser, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);
  
  let m = cs.get(0).unwrap();
  
  parser.set_lexeme_limits(&m);

  parser.tokens.push(
    Token::new(
      tt,
      String::from(m.as_str()),
      m.start() + parser.pos.pos,
      m.end() + parser.pos.pos,
    )
  );

  // parser.update_pos();

}


pub fn tokenize_newline(parser: &mut Parser, tt: TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}...\n", tt);

  let m = cs.get(0).unwrap();

  parser.set_lexeme_limits(&m);

  parser.tokens.push(
    Token::new(
      tt,
      String::from(m.as_str()),
      m.start() + parser.pos.pos,
      m.end() + parser.pos.pos,
    )
  );

}

pub fn tokenize_blankline(parser: &mut Parser, tt: TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}...\n", tt);

  let m = cs.get(0).unwrap();

  parser.tokens.push(
    Token::new(
      tt,
      String::from("\n\n"),
      m.start() + parser.pos.pos,
      m.end() + parser.pos.pos,
    )
  );

  // parser.update_pos();

  parser.state = State::Body
}

