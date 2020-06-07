/// This is the `parser` module of ruSTLa

mod state_machine;


mod token;
mod state;
mod position;
mod error;

#[cfg(test)]
mod tests;

use std::io::{self, BufReader, Lines};
use std::fs::File;

use std::fmt;
use std::str;
use regex;
use lazy_static::lazy_static;

use crate::parser::token::{Token, TokenType};
use crate::parser::state::{State};
use crate::parser::position::Pos;
use std::collections;
use crate::parser::error::{TokenizeError, ParseError};
use state_machine::transitions::{BODY_TRANSITIONS, INLINE_TRANSITIONS};


//#[derive(PartialEq)]
pub struct Parser {
  line_iter: Lines<BufReader<File>>,
  state: State,
  actions: &'static ActionMap,
  // body_actions: Vec<(TokenType, regex::Regex, Action)>,
  // inline_actions: Vec<(TokenType, regex::Regex, Action)>,
  tokens: Vec<Token>,
  pos: Pos,
  has_error: bool
}


/// Lexer type methods
impl Parser {

  /// ### new
  /// A Lexer constructor
  pub fn new(line_iter: Lines<BufReader<File>>, state: state::State) -> Self {

    Parser {
      line_iter: line_iter,
      state: state,
      actions: &ACTION_MAP,
      tokens: Vec::new(),
      pos: Pos::new(),
      has_error: false
    }
  }


  /// ### lex
  /// Loops over the source string
  /// until it has been consumed,
  /// calling `scan_token` to try and match
  /// lexemes at the current position.
  /// Consumes the Lexer itself as well.
  fn lex(&mut self) {

    println!("\nLexing in {:?} mode...\nstarting from row {:?}, col {:?}", self.state, self.pos.row, self.pos.col);

    while let Some(next_line_result) = self.line_iter.next() {

      if let Ok(line) = next_line_result {
        self.scan_line(line)
      } else {
        eprintln!("No line to read at (pos, row, col) = ({}, {}, {})", self.pos.pos, self.pos.row, self.pos.col);
        self.has_error = true; 
      };

    }

    self.tokens.push(
      Token::new(
        TokenType::EOF,
        String::from(""),
        self.pos.pos,
        self.pos.pos
      )
    );

  }


  /// ### scan_line
  /// Reads in the next line `from self.line_iter`
  /// for tokenizing.
  fn scan_line(&mut self, line: String) {

    let s = line.as_str();

    let mut char_iter = s.chars();

    while let Some(c) = char_iter.next() {

      self.scan_token(&mut char_iter);
    }

  }

  /// ### scan_token
  /// Reads the next lexeme and produces
  /// a token mathcing it. This is the
  /// core of the parser itself.
  fn scan_token<'t>(&mut self, char_iter: &'t mut str::Chars ) -> Option<regex::Captures<'t>>{

    let s = char_iter.as_str();

    let av: &ActionVector = &self.actions.get(&self.state).unwrap();

    for (tt, re, a) in av {

      if let Some(cs) = re.captures(s) {

        a(self, tt.clone(), &cs);

        self.pos.lexeme_start = self.pos.lookahead;

        return Some(cs);

      } else {
        continue
      }
    }

    None

  }

  /// ### perform_action
  /// Calls the callback function `a` corresponding to
  /// the detected lexeme.
  fn perform_action(&mut self, a: &Action, tt: &TokenType, cs: &regex::Captures) {

    a(self, tt.clone(), cs);

    self.pos.lexeme_start = self.pos.lookahead;

  }


  /// ### set_lexeme_limits
  /// Changes the `self.pos.lexeme_start` and `self.pos.lookahead`
  /// to match the beginning and end of the current lexeme.
  fn set_lexeme_limits(&mut self, m: &regex::Match) {
    self.pos.lexeme_start = m.start() + self.pos.pos;
    self.pos.lookahead = m.end() + self.pos.pos;
  }


  /// ### update_pos
  /// Pushes `pos` to the end
  /// position of the
  /// latest detected lexeme.
  /// If this doesn't succeed, simply
  /// makes sure `self.pos` doesn't
  /// lag behind `self.lexeme_start`.
  fn update_pos(&mut self, char_iter: &mut str::Chars) {
    
    //println!("Updating pos...\n");

    while self.pos.pos < self.pos.lookahead - 1 {

      if let Some(c) = char_iter.next() {

        self.increment_pos(&c);

      } else {
        break
      }

    }

  }


  /// ###increment_pos
  /// Increments the values in Lexer.pos
  /// based on the next incoming character.
  fn increment_pos(&mut self, c: &char) {
    println!("Consuming {:?}...", c);

    self.pos.pos += 1;
    self.pos.col += 1;
    if self.pos.lexeme_start < self.pos.pos {
      self.pos.lexeme_start += 1;
    }
    if *c == '\n' {
      self.pos.row += 1;
      self.pos.col = 0;
    }

    println!("Updated (pos, lexeme_start, lookahead, row, col) -> ({}, {}, {}, {}, {})\n",
      self.pos.pos, self.pos.lexeme_start, self.pos.lookahead, self.pos.row, self.pos.col);
  }

}


/// ### val_from_key
/// Goes through a given list of tuples
/// ```
/// (TokenType, str_pattern, Action)
/// ```
/// and looks for a matching tokentype.
/// If it finds one, returns and `Option<&'static str>`,
/// otherwise returns `None`.
fn val_from_key(search_key: &TokenType, map: &[(TokenType, &'static str, Action)]) -> Option<&'static str> {
  for (_, val, _) in map.iter().filter(|&(map_key, _, _)| map_key == search_key) { 
    return Some(val);
  }
  None
}



lazy_static! {

  /// ### ACTION_MAP
  /// A static map of actions specified for the `Lexer` type.
  /// This allows for the easy creation of sublexers,
  /// as with both the parent and child, the type of actions
  /// can simply be a reference to this map.
  /// 
  /// Plus, with this regexes are only compiled into automata once.
  static ref ACTION_MAP: ActionMap = {
    let mut action_map = collections::HashMap::new();

    let mut body_actions = Vec::with_capacity(BODY_TRANSITIONS.len());
    let mut inline_actions = Vec::with_capacity(INLINE_TRANSITIONS.len());

    for (tt, re, fun) in BODY_TRANSITIONS.iter() {
      let r = regex::Regex::new(re).unwrap();
      body_actions.push((tt.clone(), r, *fun));
    }

    action_map.insert(State::Body, body_actions);

    for (tt, re, fun) in INLINE_TRANSITIONS.iter() {
      let r = regex::Regex::new(re).unwrap();
      inline_actions.push((tt.clone(), r, *fun));
    }

    action_map.insert(State::Inline, inline_actions); 
    
    action_map

  };
}


/// ### Action
/// A function pointer type alias for a Lexer action
pub type Action = fn(&mut Parser, TokenType, &regex::Captures) -> ();

/// ### Actionvector
/// Contains tuples `(TokenType, Regex, Action)`
pub type ActionVector = Vec<(TokenType, regex::Regex, Action)>;

/// ### ActionMap
/// Maps Lexer states to suitable `ActionVector`s.
pub type ActionMap = collections::HashMap<state::State, ActionVector>;



impl fmt::Debug for Parser {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Lexer")
        .field("lexeme_start", &self.pos.pos)
        .field("lookahead", &self.pos.lookahead)
        .finish()
  }
}

impl fmt::Display for Parser {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Lexer location: row = {}, col = {}", self.pos.row, self.pos.col)
  }
}

