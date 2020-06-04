/// This module contains the inline parts
/// of the lexer state transitions

mod test;

use crate::lexer::Lexer;
use crate::lexer::token::{Token, TokenType};
use crate::lexer::state::State;
use crate::lexer::Action;

use regex;

pub const INLINE_TRANSITIONS: &[(TokenType, &'static str, Action)] = &[
  (TokenType::Escape, r"^\\(.)", tokenize_escape),
  (TokenType::Code, r"^``([^`]+)``", tokenize_code),
  (TokenType::TargetReference, r"^`(.+?) <(.+?)>`(__?)", tokenize_inline_target_ref),
  (TokenType::InlineReference, r"^`(.+?)`(__?)", tokenize_inline_ref),
  (TokenType::RoleContent, r"^:([a-zA-Z0-9:-]+?):`(.+?)`", tokenize_role_content),
  (TokenType::ContentRole, r"^`(.+?)`:([a-zA-Z0-9:-]+?):", tokenize_content_role),
  (TokenType::StrongEmphasis, r"^\*\*(.+?)\*\*", tokenize_strong_emphasis),
  (TokenType::Emphasis, r"^\*(.+?)\*", tokenize_emphasis),
  (TokenType::FootnoteOrCitation, r"^\[(.+?)\]_", tokenize_footnote_or_citation),
  (TokenType::URI, r"^<(([^:/?#]+):)?(//([^/?#]*))?([^?#]*)(\?([^#]*))?(#(.*))?>", tokenize_uri), // Regex taken from https://tools.ietf.org/html/rfc3986#appendix-B
  (TokenType::BlankLines, r"^(\r?\n[ \t]*\r?\n)+", tokenize_blankline),
  (TokenType::Newline, r"^\n", tokenize_newline),
  (TokenType::Text, r"^[^\\\n\[*`:<>]+", tokenize_text_no_ldelim),
  (TokenType::Text, r"^(.)", tokenize_text),
  (TokenType::InlineWhitespace, r"[ \t]+", tokenize_inline_whitespace),
];

fn tokenize_escape (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let s = cs.get(0).unwrap().start();
  let e = cs.get(0).unwrap().end();
  let c = cs.get(1).unwrap();

  lexer.tokens.push(
    Token::new(
      tt,
      String::from(c.as_str()),
      s + lexer.pos.pos,
      e + lexer.pos.pos,
    )
  );
}

fn tokenize_code (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  let code = cs.get(1).unwrap();

  lexer.set_lexeme_limits(&m);

  lexer.tokens.push(
    Token::new(
      tt,
      String::from(code.as_str()),
      m.start() + lexer.pos.pos,
      m.end() + lexer.pos.pos,
    )
  );

  lexer.update_pos();

}


fn tokenize_inline_target_ref (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  let link_alias = cs.get(1).unwrap();
  let link = cs.get(2).unwrap();
  let ref_type = cs.get(3).unwrap();

  lexer.set_lexeme_limits(&m);

  lexer.tokens.push(
    Token::new(
      tt,
      String::from(""),
      m.start() + lexer.pos.pos,
      m.end() + lexer.pos.pos
    )
  );

  lexer.set_lexeme_limits(&link_alias);

  lexer.tokens.push(
    Token::new(
      TokenType::LinkAlias,
      String::from(link_alias.as_str()),
      link_alias.start() + lexer.pos.pos,
      link_alias.end() + lexer.pos.pos
    )
  );

  lexer.set_lexeme_limits(&link);

  lexer.tokens.push(
    Token::new(
      TokenType::Hyperlink,
      String::from(link.as_str()),
      link.start() + lexer.pos.pos,
      link.end() + lexer.pos.pos
    )
  );

  lexer.set_lexeme_limits(&ref_type);

  lexer.tokens.push(
    Token::new(
      TokenType::RefAnonOrNot,
      String::from(ref_type.as_str()),
      ref_type.start() + lexer.pos.pos,
      ref_type.end() + lexer.pos.pos
    )
  );

  lexer.set_lexeme_limits(&m);

  lexer.update_pos();

}


fn tokenize_inline_ref (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  let target = cs.get(1).unwrap();
  let ref_type = cs.get(2).unwrap();

  lexer.set_lexeme_limits(&m);

  lexer.tokens.push(
    Token::new(
      tt,
      String::from(""),
      m.start() + lexer.pos.pos,
      m.end() + lexer.pos.pos
    )
  );

  lexer.set_lexeme_limits(&target);

  lexer.tokens.push(
    Token::new(
      TokenType::Target,
      String::from(target.as_str()),
      target.start() + lexer.pos.pos,
      target.end() + lexer.pos.pos
    )
  );

  lexer.set_lexeme_limits(&ref_type);

  lexer.tokens.push(
    Token::new(
      TokenType::RefAnonOrNot,
      String::from(ref_type.as_str()),
      ref_type.start() + lexer.pos.pos,
      ref_type.end() + lexer.pos.pos
    )
  );

  lexer.set_lexeme_limits(&m);

  lexer.update_pos();

}

fn tokenize_role_content (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  let role = cs.get(1).unwrap();
  let content = cs.get(2).unwrap();

  lexer.set_lexeme_limits(&m);

  lexer.tokens.push(
    Token::new(
      tt,
      String::from(""),
      m.start() + lexer.pos.pos,
      m.end() + lexer.pos.pos,
    )
  );

  println!("\nTokenizing Role...");

  lexer.set_lexeme_limits(&role);

  lexer.tokens.push(
    Token::new(
      TokenType::Role,
      String::from(role.as_str()),
      role.start() + lexer.pos.pos,
      role.end() + lexer.pos.pos,
    )
  );

  lexer.update_pos();

  println!("Tokenizing Content...");

  lexer.set_lexeme_limits(&content);

  lexer.tokens.push(
    Token::new(
      TokenType::Content,
      String::from(content.as_str()),
      content.start() + lexer.pos.pos,
      content.end() + lexer.pos.pos,
    )
  );

  lexer.set_lexeme_limits(&m);

  lexer.update_pos();

}

fn tokenize_content_role (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  let content = cs.get(1).unwrap();
  let role = cs.get(2).unwrap();

  lexer.set_lexeme_limits(&m);

  lexer.tokens.push(
    Token::new(
      tt,
      String::from(""),
      m.start() + lexer.pos.pos,
      m.end() + lexer.pos.pos,
    )
  );

  println!("\nTokenizing Content...");

  lexer.set_lexeme_limits(&content);

  lexer.tokens.push(
    Token::new(
      TokenType::Content,
      String::from(role.as_str()),
      content.start() + lexer.pos.pos,
      content.end() + lexer.pos.pos,
    )
  );

  lexer.update_pos();

  println!("Tokenizing Role...");

  lexer.set_lexeme_limits(&role);

  lexer.tokens.push(
    Token::new(
      TokenType::Role,
      String::from(content.as_str()),
      role.start() + lexer.pos.pos,
      role.end() + lexer.pos.pos,
    )
  );

  lexer.set_lexeme_limits(&m);

  lexer.update_pos();


}


fn tokenize_strong_emphasis (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  let text = cs.get(1).unwrap();

  lexer.set_lexeme_limits(&m);

  lexer.tokens.push(
    Token::new(
      tt,
      String::from(text.as_str()),
      m.start() + lexer.pos.pos,
      m.end() + lexer.pos.pos,
    )
  );

  lexer.update_pos();

}


fn tokenize_emphasis (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  let text = cs.get(1).unwrap();

  lexer.set_lexeme_limits(&m);

  lexer.tokens.push(
    Token::new(
      tt,
      String::from(text.as_str()),
      m.start() + lexer.pos.pos,
      m.end() + lexer.pos.pos,
    )
  );

  lexer.update_pos();

}

fn tokenize_footnote_or_citation (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();
  let target = cs.get(1).unwrap();

  lexer.set_lexeme_limits(&m);

  lexer.tokens.push(
    Token::new(
      tt,
      String::from(target.as_str()),
      m.start() + lexer.pos.pos,
      m.end() + lexer.pos.pos,
    )
  );

  lexer.update_pos();

}


fn tokenize_uri (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();

  lexer.set_lexeme_limits(&m);

  lexer.tokens.push(
    Token::new(
      tt,
      String::from(m.as_str()),
      m.start() + lexer.pos.pos,
      m.end() + lexer.pos.pos,
    )
  );

  if let Some(scheme) = cs.get(1) {
    
    lexer.set_lexeme_limits(&scheme);

    lexer.tokens.push(
      Token::new(
        TokenType::Scheme,
        String::from(scheme.as_str()),
        scheme.start() + lexer.pos.pos,
        scheme.end() + lexer.pos.pos
      )
    );

  };

  if let Some(authority) = cs.get(3) {
    
    lexer.set_lexeme_limits(&authority);

    lexer.tokens.push(
      Token::new(
        TokenType::Authority,
        String::from(authority.as_str()),
        authority.start() + lexer.pos.pos,
        authority.end() + lexer.pos.pos
      )
    );

  };

  if let Some(path) = cs.get(4) {
    
    lexer.set_lexeme_limits(&path);

    lexer.tokens.push(
      Token::new(
        TokenType::Path,
        String::from(path.as_str()),
        path.start() + lexer.pos.pos,
        path.end() + lexer.pos.pos
      )
    );

  };

  if let Some(query) = cs.get(6) {
    
    lexer.set_lexeme_limits(&query);

    lexer.tokens.push(
      Token::new(
        TokenType::Query,
        String::from(query.as_str()),
        query.start() + lexer.pos.pos,
        query.end() + lexer.pos.pos
      )
    );

  };

  if let Some(fragment) = cs.get(8) {
    
    lexer.set_lexeme_limits(&fragment);

    lexer.tokens.push(
      Token::new(
        TokenType::Fragment,
        String::from(fragment.as_str()),
        fragment.start() + lexer.pos.pos,
        fragment.end() + lexer.pos.pos
      )
    );

  };

  lexer.set_lexeme_limits(&m);

  lexer.update_pos();

}

fn tokenize_inline_whitespace (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();

  lexer.set_lexeme_limits(&m);
  
  lexer.tokens.push(
    Token::new(
      tt,
      String::from(" "),
      m.start() + lexer.pos.pos,
      m.end() + lexer.pos.pos,
    )
  );

  lexer.update_pos();

}


fn tokenize_text_no_ldelim (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);

  let m = cs.get(0).unwrap();

  lexer.set_lexeme_limits(&m);
  
  lexer.tokens.push(
    Token::new(
      tt,
      String::from(m.as_str()),
      m.start() + lexer.pos.pos,
      m.end() + lexer.pos.pos,
    )
  );

  lexer.update_pos();

}

fn tokenize_text (lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("\nTokenizing {:?}...", tt);
  
  let m = cs.get(0).unwrap();
  
  lexer.set_lexeme_limits(&m);

  lexer.tokens.push(
    Token::new(
      tt,
      String::from(m.as_str()),
      m.start() + lexer.pos.pos,
      m.end() + lexer.pos.pos,
    )
  );

  lexer.update_pos();

}


fn tokenize_newline(lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}...\n", tt);

  let m = cs.get(0).unwrap();

  lexer.set_lexeme_limits(&m);

  lexer.tokens.push(
    Token::new(
      tt,
      String::from(m.as_str()),
      m.start() + lexer.pos.pos,
      m.end() + lexer.pos.pos,
    )
  );

}

fn tokenize_blankline(lexer: &mut Lexer, tt: TokenType, cs: &regex::Captures) {

  println!("Tokenizing {:?}...\n", tt);

  let m = cs.get(0).unwrap();

  lexer.tokens.push(
    Token::new(
      tt,
      String::from("\n\n"),
      m.start() + lexer.pos.pos,
      m.end() + lexer.pos.pos,
    )
  );

  lexer.update_pos();

  lexer.state = State::Body
}

