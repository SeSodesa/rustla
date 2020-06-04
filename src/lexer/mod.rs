/// This is the `lexer` module of ruSTLa

mod token;
mod state;
mod position;
mod body_actions;
mod inline_actions;
mod error;

#[cfg(test)]
mod tests;

use std::fmt;
use std::str;
use regex;
use lazy_static::lazy_static;

use crate::lexer::token::{Token, TokenType};
use crate::lexer::state::{State};
use crate::lexer::position::Pos;
use std::collections;
use crate::lexer::error::{TokenizeError, LexError};


/// ### Action
/// A function pointer type alias for a Lexer action
type Action = fn(&mut Lexer, TokenType, &regex::Captures) -> ();

/// ### Actionvector
/// Contains tuples `(TokenType, Regex, Action)`
type ActionVector = Vec<(TokenType, regex::Regex, Action)>;

/// ### ActionMap
/// Maps Lexer states to suitable `ActionVector`s.
type ActionMap = collections::HashMap<state::State, ActionVector>;

//#[derive(PartialEq)]
pub struct Lexer <'t> {
  src_iter: &'t mut str::Chars<'t>,
  state: State,
  actions: &'static ActionMap,
  // body_actions: Vec<(TokenType, regex::Regex, Action)>,
  // inline_actions: Vec<(TokenType, regex::Regex, Action)>,
  tokens: Vec<Token>,
  pos: &'t mut Pos
}


/// Lexer type methods
impl <'t> Lexer <'t> {

  /// ### new
  /// A Lexer constructor
  pub fn new(src_iter: &'t mut str::Chars<'t>, pos: &'t mut Pos, state: state::State) -> Self {

    Lexer {
      src_iter: src_iter,
      state: state,
      actions: &ACTION_MAP,
      tokens: Vec::new(),
      pos: pos,
    }
  }

  /// ### new_from_lexer
  /// Allows constructing a Lexer from another lexer.
  /// Mainly useful for generating sub lexers
  /// for inline lexing.
  pub fn new_from_lexer (lexer: &'t mut Lexer<'t>, state: state::State) -> Lexer<'t> {

    Lexer {
      src_iter: lexer.src_iter,
      state: state,
      actions: &lexer.actions,
      tokens: Vec::new(),
      pos: lexer.pos,
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

    if let None = self.scan_token() {
      eprintln!("No lexeme found at (pos, row, col) = ({}, {}, {})", self.pos.pos, self.pos.row, self.pos.col);
    }

    while let Some(c) = self.src_iter.next() {

      println!("Lexing in {:?} mode...\n", self.state);

      self.increment_pos(&c);

      if let None = self.scan_token() {
        eprintln!("No lexeme found at (pos, row, col) = ({}, {}, {})", self.pos.pos, self.pos.row, self.pos.col);
      }

    }

  }

  /// ### scan_token
  /// Reads the next lexeme and produces
  /// a token mathcing it. This is the
  /// core of the lexer itself.
  fn scan_token(&mut self) -> Option<regex::Captures<'t>>{

    let s = self.src_iter.as_str();

    let av: &ActionVector = &self.actions.get(&self.state).unwrap();

    for (tt, re, a) in av {

      if let Some(cs) = re.captures(s) {

        self.perform_action(a, tt, &cs);

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
  fn update_pos(&mut self) {
    
    //println!("Updating pos...\n");

    while self.pos.pos < self.pos.lookahead - 1 {

      if let Some(c) = self.src_iter.next() {

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

    let mut body_actions = Vec::with_capacity(body_actions::BODY_TRANSITIONS.len());
    let mut inline_actions = Vec::with_capacity(inline_actions::INLINE_TRANSITIONS.len());

    for (tt, re, fun) in body_actions::BODY_TRANSITIONS.iter() {
      let r = regex::Regex::new(re).unwrap();
      body_actions.push((tt.clone(), r, *fun));
    }

    action_map.insert(State::Body, body_actions);

    for (tt, re, fun) in inline_actions::INLINE_TRANSITIONS.iter() {
      let r = regex::Regex::new(re).unwrap();
      inline_actions.push((tt.clone(), r, *fun));
    }

    action_map.insert(State::Inline, inline_actions); 
    
    action_map

  };
}


impl fmt::Debug for Lexer <'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      f.debug_struct("Lexer")
        .field("lexeme_start", &self.pos.pos)
        .field("lookahead", &self.pos.lookahead)
        .finish()
  }
}

impl fmt::Display for Lexer <'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Lexer location: row = {}, col = {}", self.pos.row, self.pos.col)
  }
}

