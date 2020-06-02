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
  source: &'t str,
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
  pub fn new(source: &'static str, pos: &'t mut Pos, state: state::State) -> Self {

    Lexer {
      source: source,
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
  pub fn new_from_lexer (lexer: &'t mut Lexer, src: &'t str, state: state::State) -> Lexer<'t> {

    Lexer {
      source: src,
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
  fn lex(mut self) -> Vec<Token> {

    println!("\nLexing in {:?} mode...\nstarting from row {:?}, col {:?}", self.state, self.pos.row, self.pos.col);

    let s = self.source;
    let mut chars = s.chars();

    if let None = self.scan_token(&mut chars) {
      eprintln!("No lexeme found at (pos, row, col) = ({}, {}, {})", self.pos.pos, self.pos.row, self.pos.col);
    }

    while let Some(c) = chars.next() {

      println!("Consuming {:?}...", c);

      self.pos.pos += 1;
      self.pos.col += 1;
      if c == '\n' {
        self.pos.row += 1;
        self.pos.col = 0;
      }

      if let None = self.scan_token(&mut chars) {
        eprintln!("No lexeme found at (pos, row, col) = ({}, {}, {})", self.pos.pos, self.pos.row, self.pos.col);
      }

      assert!(self.pos.lookahead >= self.pos.pos);

    }

    self.tokens

  }

  /// ### scan_token
  /// Reads the next lexeme and produces
  /// a token mathcing it. This is the
  /// core of the lexer itself.
  fn scan_token<'t0>(&mut self, chars: &'t0 mut str::Chars) -> Option<regex::Captures<'t0>>{

    let s = chars.as_str();

    let av: &ActionVector = &self.actions.get(&self.state).unwrap();

    for (tt, re, a) in av {

      if let Some(cs) = re.captures(s) {

        self.perform_action(a, tt, chars, &cs);

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
  fn perform_action(&mut self, a: &Action, tt: &TokenType, chars: &mut str::Chars, cs: &regex::Captures) {

    self.pos.lookahead = cs.get(0).unwrap().end();

    println!("Performing action...");

    a(self, tt.clone(), cs);

    self.pos.pos = self.pos.lookahead;

    self.update_pos(chars);

  }


  /// ### update_pos
  /// Pushes `pos` to the end
  /// position of the
  /// latest detected lexeme.
  /// If this doesn't succeed, simply
  /// makes sure `self.pos` doesn't
  /// lag behind `self.lexeme_start`.
  fn update_pos(&mut self, chars: &mut str::Chars) {
    
    println!("Updating pos...\n");

    while self.pos.pos < self.pos.lookahead - 1 {

      if let Some(c) = chars.next() {

        println!("Consuming {:?}...", c);

        self.pos.pos += 1;
        self.pos.col += 1;

        if c == '\n' {
          self.pos.row += 1;
          self.pos.col = 0;
        }

        println!("Updated (pos, begin, lookahead, row, col) -> ({}, {}, {}, {})\n",
          self.pos.pos, self.pos.lookahead, self.pos.row, self.pos.col);

      } else {
        break
      }

    }

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

