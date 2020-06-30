/// Module contains a list of transition tuples

use std::collections::HashMap;

use regex;
use lazy_static::lazy_static;

use super::*;


#[derive(Copy, Clone, Debug)]
/// ### PatternName
/// An enum of transition regex pattern names, both for body and inline level elements.
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
  Escape,
  StrongEmphasis, // **strongly emphasised text**
  Emphasis, // *emphasized text*
  Interpreted, // Plain interpreted text with the default role set by transpiler.
  RoleThenContent, // Interpreted text with role before content, :role_label:`text`
  ContentThenRole, // Interpreted text with content before role, `text`:role_label:
  PhraseRef, // A reference in the form `text with spaces`__?
  SimpleRef, // A reference that doesn't need backticks: reference__?
  Literal, // Code
  FootNoteRef,
  InlineTarget, // Reference target in inline text: _`target label`
  SubstitutionRef, // Reference to substitution definition. Is replaced by the definition
  ImplicitURL,
  StandaloneHyperlink,
  WhiteSpace,
}


/// ### UncompiledTransition
/// A type alias for a transition `(PatternName, regex_pattern, TransitionMethod)`, whose regex pattern has not
/// been compiled into a DFA yet.
type UncompiledTransition  = (PatternName, &'static str, TransitionMethod);



pub const BODY_TRANSITIONS: &[UncompiledTransition] = &[
  (PatternName::Bullet, r"^([+\-*\u{2022}])( +|$)", Body::bullet),
];


pub const BULLET_LIST_TRANSITIONS: &[UncompiledTransition] = &[
  (PatternName::Bullet, r"^([+\-*\u{2022}])( +|$)", BulletList::bullet)
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

const URI_SCHEME: &'static str = "(https?|mailto|ftp)";


pub const INLINE_TRANSITIONS: &[InlineTransition] = &[
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
      (?P<scheme> # 😵
        about|acap|addbook|afp|afs|aim|callto|castanet|chttp|cid|crid|data|dav|dict|dns|eid|fax|feed|file|finger|freenet|ftp|go|gopher|
        gsm-sms|h323|h324|hdl|hnews|http|https|hydra|iioploc|ilu|im|imap|info|ior|ipp|irc|iris.beep|iseek|jar|javascript|jdbc|ldap|lifn|
        livescript|lrq|mailbox|mailserver|mailto|md5|mid|mocha|modem|mtqp|mupdate|news|nfs|nntp|opaquelocktoken|phone|pop|pop3|pres|printer|
        prospero|rdar|res|rtsp|rvp|rwhois|rx|sdp|service|shttp|sip|sips|smb|snews|snmp|soap.beep|soap.beeps|ssh|t120|tag|tcp|tel|telephone|
        telnet|tftp|tip|tn3270|tv|urn|uuid|vemmi|videotex|view-source|wais|whodp|whois++|x-man-page|xmlrpc.beep|xmlrpc.beeps|z39.50r|z39.50s
      )
      :
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
    (?P<email>
      [-_a-zA-Z0-9]+
      (?:\.[-_!~*'{|}/\#?\^`&=+$%a-zA-Z0-9]+)*
      @
      [-_a-zA-Z0-9]+
      (?:a-zA-Z0-9]+)*
    )
    ", Inline::reference),
  (PatternName::Text, r"^([^\\\n\[*`:_\s]+)(?:[^_][a-zA-Z0-9]+_)?", Inline::text),
  (PatternName::Text, r"^([\s\S]+)", Inline::text)
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


    // let mut inline_actions = Vec::with_capacity(INLINE_TRANSITIONS.len());

    // for (pat_name, expr, fun) in INLINE_TRANSITIONS.iter() {
    //   let r = regex::Regex::new(expr).unwrap();
    //   inline_actions.push((*pat_name, r, *fun));
    // }

    // action_map.insert("Inline", inline_actions);

    action_map

  };

  /// ### COMPILED_INLINE_TRANSITIONS
  /// A vector of transitions specific to MachineWithState<Inline>.
  /// Inline text has different parsing requirements than (nested)
  /// `Body` elements as they do not form blocks of text,
  /// making detecting by source line impractical.
  pub static ref COMPILED_INLINE_TRANSITIONS: Vec<(PatternName, regex::Regex, InlineParsingMethod)> = {

    let mut inline_transitions = Vec::with_capacity(INLINE_TRANSITIONS.len());

    for (pat_name, expr, fun) in INLINE_TRANSITIONS.iter() {
      let r = regex::Regex::new(expr).unwrap();
      inline_transitions.push((*pat_name, r, *fun));
    }

    inline_transitions

  };


}