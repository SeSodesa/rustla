/// Module contains a list of transition tuples

use std::collections::HashMap;

use lazy_static::lazy_static;

use super::*;


#[derive(Copy, Clone, Debug)]
/// ### PatternName
/// An enum of transition regex pattern names, both for body and inline level elements.
pub enum PatternName {

  // Body elements, possibly nested
  EmptyLine,
  Bullet,
  Enumerator {delims: EnumDelims, kind: EnumKind},

  FieldMarker,
  OptionMarker,
  DocTest,
  LineBlock,
  ExplicitMarkup,
  AnonymousTarget,
  Line,
  Paragraph,
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
pub type UncompiledTransition  = (PatternName, &'static str, TransitionMethod);


lazy_static! {

  /// ### TRANSITION_MAP
  /// A static map of transititions for each state of
  /// the `Parser` `StateMachine`.
  /// 
  /// With this regexes are only compiled into automata once.
  pub static ref TRANSITION_MAP: HashMap<StateMachine, Vec<(PatternName, regex::Regex, TransitionMethod)>> = {

    let mut action_map = collections::HashMap::new();

    let body_actions = StateMachine::compile_state_transitions(&StateMachine::BODY_TRANSITIONS);
    action_map.insert(StateMachine::Body, body_actions);

    let bullet_actions = StateMachine::compile_state_transitions(&StateMachine::BULLET_LIST_TRANSITIONS);
    action_map.insert(StateMachine::BulletList, bullet_actions);

    let bullet_list_item_actions = StateMachine::compile_state_transitions(&StateMachine::BULLET_LIST_ITEM_TRANSITIONS);
    action_map.insert(StateMachine::ListItem, bullet_list_item_actions);

    let definition_actions = StateMachine::compile_state_transitions(&StateMachine::DEFINITION_LIST_TRANSITIONS);
    action_map.insert(StateMachine::DefinitionList, definition_actions);

    let enumerated_actions = StateMachine::compile_state_transitions(&StateMachine::ENUMERATED_LIST_TRANSITIONS);
    action_map.insert(StateMachine::EnumeratedList, enumerated_actions);

    let field_actions = StateMachine::compile_state_transitions(&StateMachine::FIELD_LIST_TRANSITIONS);
    action_map.insert(StateMachine::FieldList, field_actions);

    let option_actions = StateMachine::compile_state_transitions(&StateMachine::OPTION_LIST_TRANSITIONS);
    action_map.insert(StateMachine::OptionList, option_actions);

    let line_block_actions = StateMachine::compile_state_transitions(&StateMachine::LINE_BLOCK_TRANSITIONS);
    action_map.insert(StateMachine::LineBlock, line_block_actions);

    let extension_option_actions = StateMachine::compile_state_transitions(&StateMachine::EXTENSION_OPTION_TRANSITIONS);
    action_map.insert(StateMachine::ExtensionOptions, extension_option_actions);

    let explicit_markup_actions = StateMachine::compile_state_transitions(&StateMachine::EXPLICIT_MARKUP_TRANSITIONS);
    action_map.insert(StateMachine::ExplicitMarkup, explicit_markup_actions);

    let text_actions = StateMachine::compile_state_transitions(&StateMachine::TEXT_TRANSITIONS);
    action_map.insert(StateMachine::Text, text_actions);

    let definition_actions = StateMachine::compile_state_transitions(&StateMachine::DEFINITION_TRANSITIONS);
    action_map.insert(StateMachine::Definition, definition_actions);

    let line_actions = StateMachine::compile_state_transitions(&StateMachine::LINE_TRANSITIONS);
    action_map.insert(StateMachine::Line, line_actions);    

    let subst_def_actions = StateMachine::compile_state_transitions(&StateMachine::SUBSTITUTION_DEF_TRANSITIONS);
    action_map.insert(StateMachine::SubstitutionDef, subst_def_actions);    

    action_map

  };

  /// ### COMPILED_INLINE_TRANSITIONS
  /// A vector of transitions specific to MachineWithState<Inline>.
  /// Inline text has different parsing requirements than (nested)
  /// `Body` elements as they do not form blocks of text,
  /// making detecting by source line impractical.
  pub static ref COMPILED_INLINE_TRANSITIONS: Vec<(PatternName, regex::Regex, InlineParsingMethod)> = {

    let mut inline_transitions = Vec::with_capacity(StateMachine::INLINE_TRANSITIONS.len());

    for (pat_name, expr, fun) in StateMachine::INLINE_TRANSITIONS.iter() {
      let r = regex::Regex::new(expr).unwrap();
      inline_transitions.push((*pat_name, r, *fun));
    }

    inline_transitions

  };


}