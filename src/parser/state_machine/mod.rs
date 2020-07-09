/// This module contains specifications
/// of state machines used by the parser.

pub mod states;
mod body;
mod bullet_list;
mod common;
mod inline;
mod list_item;
mod transitions;
mod tests;

use std::cmp;

use super::*;
use crate::utils;
use states::*;
use transitions::{TRANSITION_MAP, COMPILED_INLINE_TRANSITIONS, UncompiledTransition,  *};
use crate::doctree::{self, TreeNode, EnumeratorType};


/// ### TransitionMethod
/// A function pointer type alias for a State transition method.
/// `TransitionMethod`s take in the document tree and regex captures
/// for doctree modifications. Unless errors occur,
/// they return an `Ok`-wrapped tuple of optional doctree (because the parser contains an `Option`al doctree and not just a doctree),
/// a possible next state for the parser, information about manipulating the machine stack and whether to advance the parser line cursor.
/// If the optional next state is *not* `None`, the current state is either replaced with the new state or
/// the new state is pushed on top of the machine stack of the parser and parsing proceeds
/// in that state from the current line.
type TransitionMethod = fn(src_lines: &Vec<String>, current_line: &mut usize, doctree: Option<DocTree>, captures: regex::Captures, next_state: &PatternName) -> Result<(Option<DocTree>, Option<StateMachine>, PushOrPop, LineAdvance), &'static str>;

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
#[derive(Debug, Hash, PartialEq, Eq)]
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
  /// Retrieves the list of transitions from a `StateMachine` variant
  /// using a `match` statement. This seems like a lot of repetition,
  /// but this is the only way of doing this when wrapping each
  /// different state machine type in an enum.
  pub fn get_transitions (&self) -> Result<&Vec<Transition>, &'static str> {

    match self {
      StateMachine::Body              => {
        Ok(TRANSITION_MAP.get(self).unwrap())
      },
      StateMachine::BulletList        => {
        Ok(TRANSITION_MAP.get(self).unwrap())
      },
      StateMachine::ListItem          => {
        Ok(TRANSITION_MAP.get(self).unwrap())
      },
      StateMachine::DefinitionList    => {
        Ok(TRANSITION_MAP.get(self).unwrap())
      },
      StateMachine::EnumeratedList    => {
        Ok(TRANSITION_MAP.get(self).unwrap())
      },
      StateMachine::FieldList         => {
        Ok(TRANSITION_MAP.get(self).unwrap())
      },
      StateMachine::OptionList        => {
        Ok(TRANSITION_MAP.get(self).unwrap())
      },
      StateMachine::LineBlock         => {
        Ok(TRANSITION_MAP.get(self).unwrap())
      },
      StateMachine::ExtensionOptions  => {
        Ok(TRANSITION_MAP.get(self).unwrap())
      },
      StateMachine::ExplicitMarkup    => {
        Ok(TRANSITION_MAP.get(self).unwrap())
      },
      StateMachine::Text              => {
        Ok(TRANSITION_MAP.get(self).unwrap())
      },
      StateMachine::Definition        => {
        Ok(TRANSITION_MAP.get(self).unwrap())
      },
      StateMachine::Line              => {
        Ok(TRANSITION_MAP.get(self).unwrap())
      },
      StateMachine::SubstitutionDef   => {
        Ok(TRANSITION_MAP.get(self).unwrap())
      },
      StateMachine::EOF               => Err("Already moved past EOF. No transitions to perform.\n"),
      StateMachine::Failure           => Err("Failure state has no transitions\n")
    }
  }
}



/// ### MachineWithState
/// A state machine in a state `S`,
/// which is its own type. This allows different
/// state machines to hold common fields,
/// while the embedded state types can hold their
/// own specific fields like transition tables.
#[derive(Debug)]
pub struct MachineWithState <S> {
  state: S,
}


impl MachineWithState<Body> {

  /// ### new
  /// A state machine constructor. This is only implemented for
  /// the `Body` state, as it is the starting state when it
  /// comes to rST parsing. Transitions to and creation of
  /// other states is handled by implementing the `From`
  /// trait (the `from` function) for those states.
  pub fn new() -> Self {

    Self {
      state: Body::new(),
    }
  }

}


impl From<MachineWithState<Body>> for MachineWithState<Inline> {

  fn from (machine: MachineWithState<Body>) -> Self {
    Self {
      state: Inline::new()
    }
  }

}

impl MachineWithState<Inline> {

  /// ### parse
  /// A function that parses inline text. Returns the nodes generated,
  /// if there are any.
  fn parse (&self, inline_src_block: String, current_line: &mut usize) -> Option<Vec<TreeNode>> {

    let mut nodes: Vec<TreeNode> = Vec::new();

    let mut col: usize = 0;

    // Remove backslashes
    let src_without_escapes = inline_src_block.replace("\\", "");

    let src_chars = &mut src_without_escapes.chars();

    match self.match_iter(&src_chars) {
      Some((node, offset)) => {

        nodes.push(node);

        // Move iterator to start of next possible match
        for _ in 0..offset - 1 {
          let c = src_chars.next().unwrap();
          eprintln!("Consuming {:#?}...", c);

          col += 1;

          if c == '\n' {
            eprintln!("Detected newline...\n");
            *current_line += 1;
            col = 0;
          }
        }
      },

      None => {
        eprintln!("No match on line {}, col {}.\nProceeding to consume next character...\n", current_line, col);
      }
    }

    while let Some(c) = src_chars.next() {

      eprintln!("Consuming {:#?}...\n", c);

      col += 1;

      if c == '\n' {
        eprintln!("Detected newline...\n");
        *current_line += 1;
        col = 0;
      }

      match self.match_iter(&src_chars) {
        Some((node, offset)) => {

          nodes.push(node);

          // Move iterator to start of next possible match
          for _ in 0..offset - 1 {
            let c = src_chars.next().unwrap();
            eprintln!("Consuming {:#?}", c);

            col += 1;

            if c == '\n' {
              eprintln!("Detected newline...\n");
              *current_line += 1;
              col = 0;
            }
          }
        },

        None => {
          eprintln!("No match on line {}, col {}.\n", current_line, col);
        }
      }
    }

    if nodes.is_empty() {
      return None
    }

    Some(nodes)

  }

  /// ### match_iter
  /// A function for checking the string representation of
  /// a given `Chars` iterator for a regex match and executing
  /// the corresponding parsing method. Returns the `Option`al
  /// generated node if successful, otherwise returns with `None`.
  fn match_iter <'machine, 'chars> (&'machine self, chars_iter: &'chars str::Chars) -> Option<(TreeNode, usize)> {

    let src_str = chars_iter.as_str();

    if src_str.is_empty() {
      eprintln!("Source has been drained of characters.\n");
      return None
    }

    eprintln!("Matching against {:#?}\n", src_str);

    for (pattern_name, regexp, parsing_function) in self.state.transitions.iter() {

      match regexp.captures(src_str) {

        Some(capts) => {

          eprintln!("Match found for {:#?}\n", pattern_name);

          let (node, offset) = parsing_function(*pattern_name, &capts);

          //eprintln!("{:#?}", node);

          return Some((node, offset));

        },

        None => {
          //eprintln!("No match for {:#?}", pattern_name);
          continue // no match, do nothing
        }

      };
    }

    None

  }
}



/// ====================
/// MachineWithState methods
/// ====================
impl <S> MachineWithState <S> {

  /// ### run
  /// Starts the processing of the given source.
  /// Returns a modified `DocTree`.
  /// This function is initially called by the parser,
  /// but subsequent calls can be made by the state
  /// machines on the top of the parser stack.
  pub fn run (&mut self) -> Option<DocTree>{

    unimplemented!();

  }

}

/// =================================
/// StateMachine associated functions
/// =================================
impl StateMachine {
  fn compile_state_transitions (transitions: &[UncompiledTransition]) -> Vec<Transition> {

    eprintln!("Compiling transitions for a state...\n");

    let mut transitions = Vec::with_capacity(StateMachine::BODY_TRANSITIONS.len());

    for (pat_name, expr, fun) in StateMachine::BODY_TRANSITIONS.iter() {
      let r = regex::Regex::new(expr).unwrap();
      transitions.push((*pat_name, r, *fun));
    }

    transitions
  }
}

/// =================================
/// StateMachine associated constants
/// =================================
impl StateMachine {

  /// ### BODY_TRANSITIONS
  /// An array of transitions related to `StateMachine::Body`.
  pub const BODY_TRANSITIONS: [UncompiledTransition; 4] = [
    (PatternName::EmptyLine, Self::BLANK_LINE_PATTERN, common::empty_line),
    (PatternName::Bullet, Self::BULLET_PATTERN, Body::bullet),
    (PatternName::Enumerator, Self::ENUMERATOR_PATTERN, Body::enumerator),
    (PatternName::Text, Self::PARAGRAPH_PATTERN, Body::paragraph)
  ];


  /// ### BULLET_LIST_TRANSITIONS_TRANSITIONS
  /// An array of transitions related to `StateMachine::BulletList`.
  pub const BULLET_LIST_TRANSITIONS: [UncompiledTransition; 2] = [
    (PatternName::EmptyLine, Self::BLANK_LINE_PATTERN, Body::empty_line),
    (PatternName::Bullet, Self::BULLET_PATTERN, BulletList::bullet)
  ];


  /// ### BULLET_LIST_ITEM_TRANSITIONS
  /// An array of transitions related to `StateMachine::BulletListItem`.
  pub const BULLET_LIST_ITEM_TRANSITIONS: [UncompiledTransition; 3] = [
    (PatternName::EmptyLine, Self::BLANK_LINE_PATTERN, Body::empty_line),
    (PatternName::Bullet, Self::BULLET_PATTERN, ListItem::bullet),
    (PatternName::Paragraph, Self::PARAGRAPH_PATTERN, ListItem::paragraph),
  ];


  /// ### DEFINITION_LIST_TRANSITIONS
  /// An array of transitions related to `StateMachine::DefinitionList`.
  pub const DEFINITION_LIST_TRANSITIONS: [UncompiledTransition; 0] = [

  ];

  /// ### ENUMERATED_LIST_TRANSITIONS
  /// An array of transitions related to `StateMachine::EnumeratedList`.
  pub const ENUMERATED_LIST_TRANSITIONS: [UncompiledTransition; 0] = [

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
    (PatternName::WhiteSpace, r"^\s+", Inline::whitespace),
    (PatternName::StrongEmphasis, r"^\*\*(\S|\S.*\S)\*\*", Inline::paired_delimiter),
    (PatternName::Emphasis, r"^\*(\S|\S.*\S)\*", Inline::paired_delimiter),
    (PatternName::Literal, r"^``(\S|\S.*\S)``", Inline::paired_delimiter),
    (PatternName::InlineTarget, r"^_`([\w .]+)`", Inline::paired_delimiter),
    (PatternName::PhraseRef, r"^`(\S|\S.*\S)`__?", Inline::reference),
    (PatternName::Interpreted, r"^`(\S|\S.*\S)`", Inline::paired_delimiter),
    (PatternName::FootNoteRef, r"^\[(\S|\S.*\S)\]__?", Inline::reference),
    (PatternName::SimpleRef, r"^([\p{L}0-9]+(?:[-+._:][\p{L}0-9]+)*)__?", Inline::reference),
    (PatternName::SubstitutionRef, r"^\|(\S|\S.*\S)\|(?:_|__)?", Inline::reference),

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
      ", Inline::reference),
    //(PatternName::Text, r"^([^\\\n\[*`:_]+)(?:[^_][a-zA-Z0-9]+_)?", Inline::text),
    (PatternName::Text, r"^([\S]+)", Inline::text)
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


  /// #### ENUMERATOR_PATTERN
  /// A pattern for matching enumerated list items.
  const ENUMERATOR_PATTERN: &'static str = r"(?x) # Insignificant whitespace mode on
    ^(?P<indent>\s*)
    (?:
      # Both left and right parentheses around enumerator
      \((?P<arabic_parens>[0-9]+)\)
      | \((?P<lower_alpha_parens>[a-z])\)
      | \((?P<upper_alpha_parens>[A-Z])\)
      | \((?P<lower_roman_parens>[ivxlcdm]+)\)
      | \((?P<upper_roman_parens>[ICXLCDM]+)\)

      # Only right parenthesis after enumerator
      | (?P<arabic_rparen>[0-9]+)\)
      | (?P<lower_alpha_rparen>[a-z])\)
      | (?P<upper_alpha_rparen>[A-Z])\)
      | (?P<lower_roman_rparen>[ivxlcdm]+)\)
      | (?P<upper_roman_rparen>[ICXLCDM]+)\)

      # Period after enumerator
      | (?P<arabic_period>[0-9]+)\.
      | (?P<lower_alpha_period>[a-z])\.
      | (?P<upper_alpha_period>[A-Z])\.
      | (?P<lower_roman_period>[ivxlcdm]+)\.
      | (?P<upper_roman_period>[ICXLCDM]+)\.
    )(?-x)
    (?: +|$)";


    /// #### PARAGRAPH_PATTERN
    /// A pattern for detecting any text, possibly beginning with whitespace.
    /// This pattern should generally be tested against only after all other
    /// possibilities have been eliminated. 
    const PARAGRAPH_PATTERN: &'static str = r"^(\s*)\S";

}
