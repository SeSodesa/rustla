/// This module contains specifications
/// of state machines used by the parser.

// ===============================================
// Submodules for namespacing transition functions
// ===============================================
mod body;
mod bullet_list;
mod common;
mod enumerated_list;
mod inline;
mod list_item;
pub mod transitions;
mod tests;

use std::cmp;

use regex;

use super::*;
use crate::utils::{self, EnumDelims, EnumKind};
use transitions::{TRANSITION_MAP, COMPILED_INLINE_TRANSITIONS, UncompiledTransition,  *};
use crate::doctree::{self, TreeNode};


/// ### TransitionMethod
/// A function pointer type alias for a State transition method.
/// `TransitionMethod`s take in the document tree and regex captures
/// for doctree modifications.
/// They return a `TransitionResult::{Success, Failure}`, the success variant of which contains a doctree,
/// a possible next state for the parser, information about manipulating the machine stack and whether to advance the parser line cursor.
/// If the optional next state is *not* `None`, the current state is either replaced with the new state or
/// the new state is pushed on top of the machine stack of the parser and parsing proceeds
/// in that state from the current line.
type TransitionMethod = fn(src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, next_state: &PatternName) -> TransitionResult;

/// ### Transition
/// A type alias for a tuple `(PatternName, Regex, TransitionMethod)`
type Transition = (PatternName, regex::Regex, TransitionMethod);


/// ### InlineTransitionMethod
/// A type alias for a function describing an inline transition.
/// Returns a node a length of the match, so that the inline parser
/// could determine how many characters to eat off the start of the
/// source string.
type InlineParsingMethod = fn (pattern_name: PatternName, captures: &regex::Captures) -> (TreeNode, usize);


/// ### InlineTransition
/// A type alias for a tuple `(PatternName, regex pattern, InlineTransitionMethod)`.
type InlineTransition = (PatternName, &'static str, InlineParsingMethod);


/// ### PushOrPop
/// An enum for manipulating the machine stack. Transition methods should return this information
/// with a possible next state, so the parser knows how to proceed. The `Push` variant signifies
/// a state should be pushed on top of the stack, `Pop` tells of the need to pop from the stack
/// and `Neither` initiates a transition of the current state into another one.
#[derive(Debug)]
pub enum PushOrPop {
  Push, Pop, Neither
}

/// ### LineAdvance
/// An enum returned by the transition methods to tell the parser whether
/// it needs to advance its line cursor after the method execution or not.
pub enum LineAdvance {
  Some(usize),
  None
}


/// ### StateMachine
/// An enum of `MachineWithState`s.
/// Enclosing machine variants with different states in an enum allows us
/// to give ownership of a generic machine to an arbitrary structure,
/// as enums are only as large as their largest variant.
/// Inspired heavily by [this](https://hoverbear.org/blog/rust-state-machine-pattern/)
/// article.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum StateMachine {
  Body,
  BulletList,
  DefinitionList,
  EnumeratedList,
  ListItem,
  FieldList,
  OptionList,
  LineBlock,
  ExtensionOptions,
  ExplicitMarkup,
  Text,
  Definition,
  Line,
  SubstitutionDef,
  Failure,
  EOF
}


// ====================
// Statemachine methods
// ====================
impl StateMachine {

  /// ### to_failure
  /// Transitions a `StateMachine` into a `Failure` state using the From trait,
  /// the implementation of which automatically implements the Into trait.
  pub fn to_failure (self) -> Self {
    match self {
      StateMachine::Body              => StateMachine::Failure,
      StateMachine::BulletList        => StateMachine::Failure,
      StateMachine::DefinitionList    => StateMachine::Failure,
      StateMachine::EnumeratedList    => StateMachine::Failure,
      StateMachine::FieldList         => StateMachine::Failure,
      StateMachine::OptionList        => StateMachine::Failure,
      StateMachine::LineBlock         => StateMachine::Failure,
      StateMachine::ExtensionOptions  => StateMachine::Failure,
      StateMachine::ExplicitMarkup    => StateMachine::Failure,
      StateMachine::Text              => StateMachine::Failure,
      StateMachine::Definition        => StateMachine::Failure,
      StateMachine::Line              => StateMachine::Failure,
      StateMachine::SubstitutionDef   => StateMachine::Failure,
      _ => unreachable!()
    }
  }

  /// ### get_transitions
  /// Retrieves the list of transitions based on a given `StateMachine` variant
  /// using a `match` statement. First checks for end states that don't contain transitions,
  /// such as `EOF` or `Failure` and if these are not matched,
  /// retrieves a list of transitions from the `TRANSITION_MAP`.
  pub fn get_transitions (&self) -> Result<&Vec<Transition>, &'static str> {

    match self {
      StateMachine::EOF               => Err("Already moved past EOF. No transitions to perform.\n"),
      StateMachine::Failure           => Err("Failure state has no transitions\n"),
      _ => {
        Ok(TRANSITION_MAP.get(self).unwrap())
      }
    }
  }
}


/// =================================
/// StateMachine associated functions
/// =================================
impl StateMachine {

  /// ### compile_state_transitions
  /// Takes in a reference/slice to an associated array of uncompiled transitions
  /// and compiles the regex patterns found. Returns a `Vec<Transition>` with compiled state machines
  /// in palce of the regex patterns.
  /// 
  /// Error handling needs to be added.
  fn compile_state_transitions (transitions: &[UncompiledTransition]) -> Vec<Transition> {

    let mut compiled_transitions = Vec::with_capacity(transitions.len());

    for (pat_name, expr, fun) in transitions.iter() {
      let r = regex::Regex::new(expr).unwrap();
      compiled_transitions.push((*pat_name, r, *fun));
    }

    compiled_transitions
  }

}

/// =================================
/// StateMachine associated constants
/// =================================
impl StateMachine {

  /// ### BODY_TRANSITIONS
  /// An array of transitions related to `StateMachine::Body`.
  pub const BODY_TRANSITIONS: [UncompiledTransition; 18] = [
    (PatternName::EmptyLine, Self::BLANK_LINE_PATTERN, common::empty_line),
    (PatternName::Bullet, Self::BULLET_PATTERN, body::bullet),
    (PatternName::Enumerator{delims: EnumDelims::Parens, kind: EnumKind::Arabic}, StateMachine::ARABIC_PARENS_PATTERN, body::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::RParen, kind: EnumKind::Arabic}, StateMachine::ARABIC_RPAREN_PATTERN, body::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::Period, kind: EnumKind::Arabic}, StateMachine::ARABIC_PERIOD_PATTERN, body::enumerator),

    (PatternName::Enumerator{delims: EnumDelims::Parens, kind: EnumKind::LowerAlpha}, StateMachine::LOWER_ALPHA_PARENS_PATTERN, body::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::RParen, kind: EnumKind::LowerAlpha}, StateMachine::LOWER_ALPHA_RPAREN_PATTERN, body::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::Period, kind: EnumKind::LowerAlpha}, StateMachine::LOWER_ALPHA_PERIOD_PATTERN, body::enumerator),

    (PatternName::Enumerator{delims: EnumDelims::Parens, kind: EnumKind::UpperAlpha}, StateMachine::UPPER_ALPHA_PARENS_PATTERN, body::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::RParen, kind: EnumKind::UpperAlpha}, StateMachine::UPPER_ALPHA_RPAREN_PATTERN, body::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::Period, kind: EnumKind::UpperAlpha}, StateMachine::UPPER_ALPHA_PERIOD_PATTERN, body::enumerator),

    (PatternName::Enumerator{delims: EnumDelims::Parens, kind: EnumKind::LowerRoman}, StateMachine::LOWER_ROMAN_PARENS_PATTERN, body::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::RParen, kind: EnumKind::LowerRoman}, StateMachine::LOWER_ALPHA_RPAREN_PATTERN, body::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::Period, kind: EnumKind::LowerRoman}, StateMachine::LOWER_ROMAN_PERIOD_PATTERN, body::enumerator),

    (PatternName::Enumerator{delims: EnumDelims::Parens, kind: EnumKind::UpperRoman}, StateMachine::UPPER_ROMAN_PARENS_PATTERN, body::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::RParen, kind: EnumKind::UpperRoman}, StateMachine::UPPER_ROMAN_RPAREN_PATTERN, body::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::Period, kind: EnumKind::UpperRoman}, StateMachine::UPPER_ROMAN_PERIOD_PATTERN, body::enumerator),

    (PatternName::Text, Self::PARAGRAPH_PATTERN, body::paragraph)
  ];


  /// ### BULLET_LIST_TRANSITIONS_TRANSITIONS
  /// An array of transitions related to `StateMachine::BulletList`.
  pub const BULLET_LIST_TRANSITIONS: [UncompiledTransition; 2] = [
    (PatternName::EmptyLine, Self::BLANK_LINE_PATTERN, common::empty_line),
    (PatternName::Bullet, Self::BULLET_PATTERN, bullet_list::bullet)
  ];


  /// ### BULLET_LIST_ITEM_TRANSITIONS
  /// An array of transitions related to `StateMachine::BulletListItem`.
  pub const LIST_ITEM_TRANSITIONS: [UncompiledTransition; 18] = [
    (PatternName::EmptyLine, Self::BLANK_LINE_PATTERN, common::empty_line),
    (PatternName::Bullet, Self::BULLET_PATTERN, list_item::bullet),

    (PatternName::Enumerator{delims: EnumDelims::Parens, kind: EnumKind::Arabic}, StateMachine::ARABIC_PARENS_PATTERN, list_item::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::RParen, kind: EnumKind::Arabic}, StateMachine::ARABIC_RPAREN_PATTERN, list_item::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::Period, kind: EnumKind::Arabic}, StateMachine::ARABIC_PERIOD_PATTERN, list_item::enumerator),

    (PatternName::Enumerator{delims: EnumDelims::Parens, kind: EnumKind::LowerAlpha}, StateMachine::LOWER_ALPHA_PARENS_PATTERN, list_item::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::RParen, kind: EnumKind::LowerAlpha}, StateMachine::LOWER_ALPHA_RPAREN_PATTERN, list_item::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::Period, kind: EnumKind::LowerAlpha}, StateMachine::LOWER_ALPHA_PERIOD_PATTERN, list_item::enumerator),

    (PatternName::Enumerator{delims: EnumDelims::Parens, kind: EnumKind::UpperAlpha}, StateMachine::UPPER_ALPHA_PARENS_PATTERN, list_item::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::RParen, kind: EnumKind::UpperAlpha}, StateMachine::UPPER_ALPHA_RPAREN_PATTERN, list_item::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::Period, kind: EnumKind::UpperAlpha}, StateMachine::UPPER_ALPHA_PERIOD_PATTERN, list_item::enumerator),

    (PatternName::Enumerator{delims: EnumDelims::Parens, kind: EnumKind::LowerRoman}, StateMachine::LOWER_ROMAN_PARENS_PATTERN, list_item::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::RParen, kind: EnumKind::LowerRoman}, StateMachine::LOWER_ALPHA_RPAREN_PATTERN, list_item::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::Period, kind: EnumKind::LowerRoman}, StateMachine::LOWER_ROMAN_PERIOD_PATTERN, list_item::enumerator),

    (PatternName::Enumerator{delims: EnumDelims::Parens, kind: EnumKind::UpperRoman}, StateMachine::UPPER_ROMAN_PARENS_PATTERN, list_item::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::RParen, kind: EnumKind::UpperRoman}, StateMachine::UPPER_ROMAN_RPAREN_PATTERN, list_item::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::Period, kind: EnumKind::UpperRoman}, StateMachine::UPPER_ROMAN_PERIOD_PATTERN, list_item::enumerator),

    (PatternName::Paragraph, Self::PARAGRAPH_PATTERN, list_item::paragraph),
  ];


  /// ### DEFINITION_LIST_TRANSITIONS
  /// An array of transitions related to `StateMachine::DefinitionList`.
  pub const DEFINITION_LIST_TRANSITIONS: [UncompiledTransition; 0] = [

  ];

  /// ### ENUMERATED_LIST_TRANSITIONS
  /// An array of transitions related to `StateMachine::EnumeratedList`.
  pub const ENUMERATED_LIST_TRANSITIONS: [UncompiledTransition; 16] = [
    (PatternName::EmptyLine, StateMachine::BLANK_LINE_PATTERN, common::empty_line),

    (PatternName::Enumerator{delims: EnumDelims::Parens, kind: EnumKind::Arabic}, StateMachine::ARABIC_PARENS_PATTERN, enumerated_list::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::RParen, kind: EnumKind::Arabic}, StateMachine::ARABIC_RPAREN_PATTERN, enumerated_list::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::Period, kind: EnumKind::Arabic}, StateMachine::ARABIC_PERIOD_PATTERN, enumerated_list::enumerator),

    (PatternName::Enumerator{delims: EnumDelims::Parens, kind: EnumKind::LowerAlpha}, StateMachine::LOWER_ALPHA_PARENS_PATTERN, enumerated_list::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::RParen, kind: EnumKind::LowerAlpha}, StateMachine::LOWER_ALPHA_RPAREN_PATTERN, enumerated_list::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::Period, kind: EnumKind::LowerAlpha}, StateMachine::LOWER_ALPHA_PERIOD_PATTERN, enumerated_list::enumerator),

    (PatternName::Enumerator{delims: EnumDelims::Parens, kind: EnumKind::UpperAlpha}, StateMachine::UPPER_ALPHA_PARENS_PATTERN, enumerated_list::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::RParen, kind: EnumKind::UpperAlpha}, StateMachine::UPPER_ALPHA_RPAREN_PATTERN, enumerated_list::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::Period, kind: EnumKind::UpperAlpha}, StateMachine::UPPER_ALPHA_PERIOD_PATTERN, enumerated_list::enumerator),

    (PatternName::Enumerator{delims: EnumDelims::Parens, kind: EnumKind::LowerRoman}, StateMachine::LOWER_ROMAN_PARENS_PATTERN, enumerated_list::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::RParen, kind: EnumKind::LowerRoman}, StateMachine::LOWER_ALPHA_RPAREN_PATTERN, enumerated_list::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::Period, kind: EnumKind::LowerRoman}, StateMachine::LOWER_ROMAN_PERIOD_PATTERN, enumerated_list::enumerator),

    (PatternName::Enumerator{delims: EnumDelims::Parens, kind: EnumKind::UpperRoman}, StateMachine::UPPER_ROMAN_PARENS_PATTERN, enumerated_list::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::RParen, kind: EnumKind::UpperRoman}, StateMachine::UPPER_ROMAN_RPAREN_PATTERN, enumerated_list::enumerator),
    (PatternName::Enumerator{delims: EnumDelims::Period, kind: EnumKind::UpperRoman}, StateMachine::UPPER_ROMAN_PERIOD_PATTERN, enumerated_list::enumerator),
  ];


  /// ### FIELD_LIST_TRANSITIONS
  /// An array of transitions related to `StateMachine::FieldList`.
  pub const FIELD_LIST_TRANSITIONS: [UncompiledTransition; 0] = [

  ];

  /// ### OPTION_LIST_TRANSITIONS
  /// An array of transitions related to `StateMachine::OptionList`.
  pub const OPTION_LIST_TRANSITIONS: [UncompiledTransition; 0] = [

  ];


  /// ### LINE_BLOCK_TRANSITIONS
  /// An array of transitions related to `StateMachine::LineBlock`.
  pub const LINE_BLOCK_TRANSITIONS: [UncompiledTransition; 0] = [

  ];


  /// ### EXTENSION_OPTIONS_TRANSITIONS
  /// An array of transitions related to `StateMachine::ExtensionOptions`.
  pub const EXTENSION_OPTION_TRANSITIONS: [UncompiledTransition; 0] = [

  ];


  /// ### EXPLICIT_MARKUP_TRANSITIONS
  /// An array of transitions related to `StateMachine::ExplicitMarkup`.
  pub const EXPLICIT_MARKUP_TRANSITIONS: [UncompiledTransition; 0] = [

  ];


  /// ### TEXT_TRANSITIONS
  /// An array of transitions related to `StateMachine::Text`.
  pub const TEXT_TRANSITIONS: [UncompiledTransition; 0] = [

  ];


  /// ### DEFINITION_TRANSITIONS
  /// An array of transitions related to `StateMachine::Definition`.
  pub const DEFINITION_TRANSITIONS: [UncompiledTransition; 0] = [

  ];


  /// ### LINE_TRANSITIONS
  /// An array of transitions related to `StateMachine::Line`.
  pub const LINE_TRANSITIONS: [UncompiledTransition; 0] = [

  ];


  /// ### SUBSTITUTION_DEF_TRANSITIONS
  /// An array of transitions related to `StateMachine::SubstitutionDef`.
  pub const SUBSTITUTION_DEF_TRANSITIONS: [UncompiledTransition; 0] = [

  ];


  /// ### INLINE_TRANSITIONS
  /// An array of transitions related to `StateMachine::DefinitionList`.
  pub const INLINE_TRANSITIONS: [InlineTransition; 12] = [
    (PatternName::WhiteSpace, r"^\s+", inline::whitespace),
    (PatternName::StrongEmphasis, r"^\*\*(\S|\S.*\S)\*\*", inline::paired_delimiter),
    (PatternName::Emphasis, r"^\*(\S|\S.*\S)\*", inline::paired_delimiter),
    (PatternName::Literal, r"^``(\S|\S.*\S)``", inline::paired_delimiter),
    (PatternName::InlineTarget, r"^_`([\w .]+)`", inline::paired_delimiter),
    (PatternName::PhraseRef, r"^`(\S|\S.*\S)`__?", inline::reference),
    (PatternName::Interpreted, r"^`(\S|\S.*\S)`", inline::paired_delimiter),
    (PatternName::FootNoteRef, r"^\[(\S|\S.*\S)\]__?", inline::reference),
    (PatternName::SimpleRef, r"^([\p{L}0-9]+(?:[-+._:][\p{L}0-9]+)*)__?", inline::reference),
    (PatternName::SubstitutionRef, r"^\|(\S|\S.*\S)\|(?:_|__)?", inline::reference),

    // ### StandaloneHyperlink
    //
    // source: https://www.rfc-editor.org/rfc/rfc2396.txt, appendix B
    //
    // The capturing groups correspond to the following constructs:
    //   $1 = http:
    //   $2 = http
    //   $3 = //www.ics.uci.edu
    //   $4 = www.ics.uci.edu
    //   $5 = /pub/ietf/uri/
    //   $6 = <undefined>
    //   $7 = <undefined>
    //   $8 = #Related
    //   $9 = Related
    //
    // where <undefined> indicates that the component is not present, as is
    // the case for the query component in the above example.  Therefore, we
    // can determine the value of the four components and fragment as
    //
    //   scheme    = $2
    //   authority = $4
    //   path      = $5
    //   query     = $7
    //   fragment  = $9
    //(PatternName::StandaloneHyperlink, r"^(([^:/?#]+):)?(//([^/?#]*))?([^?#]*)(\?([^#]*))?(#(.*))?", Inline::reference),
    (PatternName::StandaloneHyperlink, r"(?x)^
      (?P<absolute>
        (?:
          (?P<scheme> # 😵
            about|acap|addbook|afp|afs|aim|callto|castanet|chttp|cid|crid|data|dav|dict|dns|eid|fax|feed|file|finger|freenet|ftp|go|gopher|
            gsm-sms|h323|h324|hdl|hnews|http|https|hydra|iioploc|ilu|im|imap|info|ior|ipp|irc|iris.beep|iseek|jar|javascript|jdbc|ldap|lifn|
            livescript|lrq|mailbox|mailserver|mailto|md5|mid|mocha|modem|mtqp|mupdate|news|nfs|nntp|opaquelocktoken|phone|pop|pop3|pres|printer|
            prospero|rdar|res|rtsp|rvp|rwhois|rx|sdp|service|shttp|sip|sips|smb|snews|snmp|soap.beep|soap.beeps|ssh|t120|tag|tcp|tel|telephone|
            telnet|tftp|tip|tn3270|tv|urn|uuid|vemmi|videotex|view-source|wais|whodp|whois++|x-man-page|xmlrpc.beep|xmlrpc.beeps|z39.50r|z39.50s
          )
          :
        )
        (?://
          (?P<authority>
            (?:(?P<userinfo>[A-Za-z0-9]+(?:.[A-Za-z0-9]+)*)@)?
            (?P<host>[a-zA-Z0-9]+(?:[-.][a-zA-Z0-9]+)*)
            (?::(?P<port>[0-9]+))?
          )
        )?
        (?P<path>
          /?[a-zA-Z0-9]+(?:/[A-Za-z0-9]+)*/?
        )
        (?:\?
          (?P<query>
            [=&a-zA-Z0-9]+
          )
        )?
        (?:\#
          (?P<fragment>
            [a-zA-Z0-9]+
          )
        )?
      )
      | # if not absolute uri, then email
      ^(?P<email>
        [-_a-zA-Z0-9]+
        (?:\.[-_!~*'{|}/\#?\^`&=+$%a-zA-Z0-9]+)*
        @
        [-_a-zA-Z0-9]+
        (?:[.-][a-zA-Z0-9]+)*
      )
      ", inline::reference),
    //(PatternName::Text, r"^([^\\\n\[*`:_]+)(?:[^_][a-zA-Z0-9]+_)?", Inline::text),
    (PatternName::Text, r"^([\S]+)", inline::text)
  ];


  // ==================================
  // Patterns common to multiple states
  // ==================================


  /// #### BLANK_LINE_PATTERN
  /// A pattern for matching blank lines, as in lines that contain nothing but whitespace.
  const BLANK_LINE_PATTERN: &'static str = r"^\s*$";


  /// #### BULLET_PATERN
  /// A pattern for matching bullet list bullets.
  const BULLET_PATTERN: &'static str = r"^(\s*)([+\-*\u{2022}])(?: +|$)";


  /// A pattern for Arabic numerals with closing parentheses
  const ARABIC_PARENS_PATTERN: &'static str = r"^(\s*)\(([0-9]+)\)(?: +|$)";
  /// A pattern for Arabic numerals with a closing right parenthesis
  const ARABIC_RPAREN_PATTERN: &'static str = r"^(\s*)([0-9]+)\)(?: +|$)";
  /// A pattern for Arabic numerals with a closing period
  const ARABIC_PERIOD_PATTERN: &'static str = r"^(\s*)([0-9]+)\.(?: +|$)";

  /// A pattern for Arabic numerals with closing parentheses
  const LOWER_ALPHA_PARENS_PATTERN: &'static str = r"^(\s*)\(([a-z])\)(?: +|$)";
  /// A pattern for Arabic numerals with a closing right parenthesis
  const LOWER_ALPHA_RPAREN_PATTERN: &'static str = r"^(\s*)([a-z])\)(?: +|$)";
  /// A pattern for Arabic numerals with a closing period
  const LOWER_ALPHA_PERIOD_PATTERN: &'static str = r"^(\s*)([a-z])\.(?: +|$)";

  /// A pattern for Arabic numerals with closing parentheses
  const UPPER_ALPHA_PARENS_PATTERN: &'static str = r"^(\s*)\(([A-Z])\)(?: +|$)";
  /// A pattern for Arabic numerals with a closing right parenthesis
  const UPPER_ALPHA_RPAREN_PATTERN: &'static str = r"^(\s*)([A-Z])\)(?: +|$)";
  /// A pattern for Arabic numerals with a closing period
  const UPPER_ALPHA_PERIOD_PATTERN: &'static str = r"^(\s*)([A-Z])\.(?: +|$)";

  /// A pattern for Arabic numerals with closing parentheses
  const LOWER_ROMAN_PARENS_PATTERN: &'static str = r"^(\s*)\(([ivxlcdm]+)\)(?: +|$)";
  /// A pattern for Arabic numerals with a closing right parenthesis
  const LOWER_ROMAN_RPAREN_PATTERN: &'static str = r"^(\s*)([ivxlcdm]+)\)(?: +|$)";
  /// A pattern for Arabic numerals with a closing period
  const LOWER_ROMAN_PERIOD_PATTERN: &'static str = r"^(\s*)([ivxlcdm]+)\.(?: +|$)";

  /// A pattern for Arabic numerals with closing parentheses
  const UPPER_ROMAN_PARENS_PATTERN: &'static str = r"^(\s*)\(([IVXLCDM]+)\)(?: +|$)";
  /// A pattern for Arabic numerals with a closing right parenthesis
  const UPPER_ROMAN_RPAREN_PATTERN: &'static str = r"^(\s*)([IVXLCDM]+)\)(?: +|$)";
  /// A pattern for Arabic numerals with a closing period
  const UPPER_ROMAN_PERIOD_PATTERN: &'static str = r"^(\s*)([IVXLCDM]+)\.(?: +|$)";

  /// #### ENUMERATOR_PATTERN
  /// A pattern for matching enumerated list items.
  const ENUMERATOR_PATTERN: &'static str = r"(?x) # Insignificant whitespace mode on
    ^(?P<indent>\s*)
    (?: # Enumerator types
      \(
        (?: # Both parentheses around enumerator
          (?P<arabic_parens>[0-9]+)
          | (?P<lower_alpha_parens>[a-z])
          | (?P<upper_alpha_parens>[A-Z])
          | (?P<lower_roman_parens>[ivxlcdm]+)
          | (?P<upper_roman_parens>[ICXLCDM]+)
        )
      \)
    | (?: # Only right parenthesis after enumerator
        (?P<arabic_rparen>[0-9]+)
        | (?P<lower_alpha_rparen>[a-z])
        | (?P<upper_alpha_rparen>[A-Z])
        | (?P<lower_roman_rparen>[ivxlcdm]+)
        | (?P<upper_roman_rparen>[ICXLCDM]+)
      ) \)

    | (?: # Period after enumerator
        (?P<arabic_period>[0-9]+)
        | (?P<lower_alpha_period>[a-z])
        | (?P<upper_alpha_period>[A-Z])
        | (?P<lower_roman_period>[ivxlcdm]+)
        | (?P<upper_roman_period>[ICXLCDM]+)
      ) \.
    )
    (?:\ +|$)";


    /// #### PARAGRAPH_PATTERN
    /// A pattern for detecting any text, possibly beginning with whitespace.
    /// This pattern should generally be tested against only after all other
    /// possibilities have been eliminated. 
    const PARAGRAPH_PATTERN: &'static str = r"^(\s*)\S";

}


/// ### TransitionResult
/// An enumeration fo the different results, including errors,
/// that a transition function might have.
pub enum TransitionResult {

  /// #### Success
  /// This is returned if nothing goes wrong with a transition method.
  /// It includes the modified document tree, plus information about
  /// how to manipulate the parser stack, whether the parser should advance
  /// its line cursor.
  Success {
    doctree: DocTree,
    next_state: Option<StateMachine>,
    push_or_pop: PushOrPop,
    line_advance: LineAdvance,
  },

  /// #### Failure
  /// A general failure result. This will be returned if a clear error, such as a completetely invalid enumerator was
  /// encountered in a transition method functions. Contains an error message and the doctree in its current state.
  Failure {
    message: String,
  }
}
