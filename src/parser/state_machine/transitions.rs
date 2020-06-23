/// Module contains a list of transition tuples

use std::collections::HashMap;

use regex;
use lazy_static::lazy_static;

use super::*;


#[derive(Copy, Clone)]
pub enum PatternName {

  // Body elements, possibly nested
  Bullet,
  Enumerator,
  FieldMarker,
  OptionMarker,
  DocTest,
  LineBlock,
  ExplicitMarkup,
  AnonymousTarget,
  Line,
  Text,

  // Inline Elements for parsing Strings
  StrongEmphasis,
  Emphasis,
  Interpreted,
  PhraseRef,
  SimpleRef,
  Literal,
  InlineTarget,
  SubstitutionRef,
  ImplicitURL,
}


/// ### UncompiledTransition
/// A type alias for a transition `(PatternName, regex_pattern, TransitionMethod)`, whose regex pattern has not
/// been compiled into a DFA yet.
type UncompiledTransition  = (PatternName, &'static str, TransitionMethod);



pub const BODY_TRANSITIONS: &[UncompiledTransition] = &[
  (PatternName::Bullet, r"^([+-*\u{2022}])( +|$)", Body::bullet),
];


pub const BULLET_LIST_TRANSITIONS: &[UncompiledTransition] = &[
  (PatternName::Bullet, r"^([+-*\u{2022}])( +|$)", BulletList::bullet)
];

pub const DEFINITION_LIST_TRANSITIONS: &[UncompiledTransition] = &[

];

pub const ENUMERATED_LIST_TRANSITIONS: &[UncompiledTransition] = &[

];

pub const FIELD_LIST_TRANSITIONS: &[UncompiledTransition] = &[

];

pub const OPTION_LIST_TRANSITIONS: &[UncompiledTransition] = &[

];


pub const LINE_BLOCK_TRANSITIONS: &[UncompiledTransition] = &[

];


pub const EXTENSION_OPTION_TRANSITIONS: &[UncompiledTransition] = &[

];


pub const EXPLICIT_MARKUP_TRANSITIONS: &[UncompiledTransition] = &[

];

pub const TEXT_TRANSITIONS: &[UncompiledTransition] = &[

];


pub const DEFINITION_TRANSITIONS: &[UncompiledTransition] = &[

];


pub const LINE_TRANSITIONS: &[UncompiledTransition] = &[

];

pub const SUBSTITUTION_DEF_TRANSITIONS: &[UncompiledTransition] = &[

];


pub const INLINE_TRANSITIONS: &[UncompiledTransition] = &[

];


lazy_static! {

  /// ### TRANSITION_MAP
  /// A static map of transititions for each state of
  /// the `Parser` `StateMachine`.
  /// 
  /// With this regexes are only compiled into automata once.
  pub static ref TRANSITION_MAP: HashMap<&'static str, Vec<(PatternName, regex::Regex, TransitionMethod)>> = {

    let mut action_map = collections::HashMap::new();

    
    let mut body_actions = Vec::with_capacity(BODY_TRANSITIONS.len());

    for (pat_name, expr, fun) in BODY_TRANSITIONS.iter() {
      let r = regex::Regex::new(expr).unwrap();
      body_actions.push((*pat_name, r, *fun));
    }

    action_map.insert("Body", body_actions);


    let mut bullet_actions = Vec::with_capacity(BULLET_LIST_TRANSITIONS.len());

    for (pat_name, expr, fun) in BULLET_LIST_TRANSITIONS.iter() {
      let r = regex::Regex::new(expr).unwrap();
      bullet_actions.push((*pat_name, r, *fun));
    }

    action_map.insert("Bullet", bullet_actions);


    let mut definition_actions = Vec::with_capacity(DEFINITION_LIST_TRANSITIONS.len());
    
    for (pat_name, expr, fun) in DEFINITION_LIST_TRANSITIONS.iter() {
      let r = regex::Regex::new(expr).unwrap();
      definition_actions.push((*pat_name, r, *fun));
    }

    action_map.insert("Definition", definition_actions);


    let mut enumerated_actions = Vec::with_capacity(ENUMERATED_LIST_TRANSITIONS.len());
    
    for (pat_name, expr, fun) in ENUMERATED_LIST_TRANSITIONS.iter() {
      let r = regex::Regex::new(expr).unwrap();
      enumerated_actions.push((*pat_name, r, *fun));
    }

    action_map.insert("Enumerated", enumerated_actions);


    let mut field_actions = Vec::with_capacity(FIELD_LIST_TRANSITIONS.len());
    
    for (pat_name, expr, fun) in FIELD_LIST_TRANSITIONS.iter() {
      let r = regex::Regex::new(expr).unwrap();
      field_actions.push((*pat_name, r, *fun));
    }

    action_map.insert("FieldList", field_actions);


    let mut option_actions = Vec::with_capacity(OPTION_LIST_TRANSITIONS.len());
    
    for (pat_name, expr, fun) in FIELD_LIST_TRANSITIONS.iter() {
      let r = regex::Regex::new(expr).unwrap();
      option_actions.push((*pat_name, r, *fun));
    }

    action_map.insert("OptionList", option_actions);


    let mut line_block_actions = Vec::with_capacity(LINE_BLOCK_TRANSITIONS.len());
    
    for (pat_name, expr, fun) in LINE_BLOCK_TRANSITIONS.iter() {
      let r = regex::Regex::new(expr).unwrap();
      line_block_actions.push((*pat_name, r, *fun));
    }

    action_map.insert("LineBlock", line_block_actions);


    let mut extension_option_actions = Vec::with_capacity(EXTENSION_OPTION_TRANSITIONS.len());
    
    for (pat_name, expr, fun) in FIELD_LIST_TRANSITIONS.iter() {
      let r = regex::Regex::new(expr).unwrap();
      extension_option_actions.push((*pat_name, r, *fun));
    }

    action_map.insert("ExtensionOption", extension_option_actions);

  
    let mut explicit_markup_actions = Vec::with_capacity(EXPLICIT_MARKUP_TRANSITIONS.len());
    
    for (pat_name, expr, fun) in FIELD_LIST_TRANSITIONS.iter() {
      let r = regex::Regex::new(expr).unwrap();
      explicit_markup_actions.push((*pat_name, r, *fun));
    }

    action_map.insert("ExplicitMarkup", explicit_markup_actions);


    let mut text_actions = Vec::with_capacity(TEXT_TRANSITIONS.len());
    
    for (pat_name, expr, fun) in TEXT_TRANSITIONS.iter() {
      let r = regex::Regex::new(expr).unwrap();
      text_actions.push((*pat_name, r, *fun));
    }

    action_map.insert("Text", text_actions);


    let mut definition_actions = Vec::with_capacity(DEFINITION_TRANSITIONS.len());

    for (pat_name, expr, fun) in DEFINITION_TRANSITIONS.iter() {
      let r = regex::Regex::new(expr).unwrap();
      definition_actions.push((*pat_name, r, *fun));
    }

    action_map.insert("Definition", definition_actions);


    let mut line_actions = Vec::with_capacity(LINE_TRANSITIONS.len());

    for (pat_name, expr, fun) in LINE_TRANSITIONS.iter() {
      let r = regex::Regex::new(expr).unwrap();
      line_actions.push((*pat_name, r, *fun));
    }

    action_map.insert("Line", line_actions);    


    let mut subst_def_actions = Vec::with_capacity(SUBSTITUTION_DEF_TRANSITIONS.len());

    for (pat_name, expr, fun) in SUBSTITUTION_DEF_TRANSITIONS.iter() {
      let r = regex::Regex::new(expr).unwrap();
      subst_def_actions.push((*pat_name, r, *fun));
    }

    action_map.insert("Line", subst_def_actions);    


    let mut inline_actions = Vec::with_capacity(INLINE_TRANSITIONS.len());

    for (pat_name, expr, fun) in INLINE_TRANSITIONS.iter() {
      let r = regex::Regex::new(expr).unwrap();
      inline_actions.push((*pat_name, r, *fun));
    }

    action_map.insert("Inline", inline_actions);

    action_map

  };

}