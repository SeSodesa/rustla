/*!
This module contains the `State` type and the different transition functions corresponding to each state
in its submodules.

Copyright © 2020 Santtu Söderholm
*/

// ===============================================
// Submodules for namespacing transition functions
// ===============================================
pub mod aplus;
pub mod aplus_questionnaire;
pub mod block_quote;
pub mod body;
pub mod bullet_list;
pub mod common;
pub mod definition_list;
pub mod enumerated_list;
pub mod field_list;
pub mod inline;
pub mod literal_block;
pub mod transitions;
pub mod unknown_transitions;

use lazy_static::lazy_static;
use regex;

use super::*;

/// An enum of states.
/// The variants are used as keys to the static `TRANSITION_MAP`, which stores vectors of
/// transitions as values.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum State {

    /// A state for parsing body nodes inside admonitions.
    Admonition,

    /// A state for detecting reStructuredText & Sphinx body elements,
    /// in addition to column breaks in the form of `::newcol` for A+ nodes that support them.
    /// These include the Point of Interest directive.
    AplusMultiCol,

    /// A state for recognizing the sub-directives:
    /// 1. `pick-one`,
    /// 2. `pick-any` and
    /// 3. `freetext`
    AplusQuestionnaire,

    /// A state for detecting choices and assignments inside a A+ questionnaire
    /// subdirective `pick-one`.
    AplusPickOne,

    /// A state for detecting choices and assignments inside a A+ questionnaire
    /// subdirective `pick-any`.
    AplusPickAny,

    /// A state for recognizing body elements such as lists or footnotes when focused on document root.
    Body,

    /// A state for detecting body elements inside a section.
    Section,

    /// A state for recognizing body elements inside a block quote.
    /// In addition to normal body elements, attributions are also
    /// recognized as such in this state.
    BlockQuote,

    /// In this state, the parser only recognizes empty lines and bullet list items.
    BulletList,

    /// Citation nodes may contain arbitrary body elements.
    /// This state is therefore reserved for recognizing them when focused on a citation node.
    Citation,

    /// Definition lists may only contain empty lines and definition list items.
    DefinitionList,

    /// When in this state, the parser only recognizes empty lines and enumerated list items.
    EnumeratedList,

    HyperlinkTarget,

    /// List items of any type, such as enumerated or field list items can contain arbitrary body elements.
    /// This state is reserved for recognizing them when focused on one of the list item type nodes.
    ListItem,

    /// When focused on a field list node, the parser only recognizes empty lines and field list items.
    FieldList,

    Figure,

    /// Footnotes can contain arbitrary body elements.
    /// This state is reserved for recognizing them when focused on a footnote node.
    Footnote,

    /// There are 3 different types of hyperlink targets:
    ///
    /// 1. *internal*, which link to body elements that directly follow them,
    /// 2. *external*, that reference external URIs and
    /// 3. *indirect*, which reference other hyperlink targets inside the same document.
    ///
    /// ??? Normally, an external or indirect hyperlink target would simply be a node on its own, that simply contains a reference label
    /// of some kind. However, chained *internal* hyperlinks all reference the same target node,
    /// so a state of its own (this one) is reserved for parsing them until a node of a different kind (including other types
    /// of hyperlink targets) is encountered. Once this happens, all of the internal hyperlinks are set to point
    /// to this same target node. ???
    InternalHyperlinkTarget,

    /// When focused on an option list, only empty lines and option list items are recognized.
    /// This state is reserved for that purpose.
    OptionList,

    /// Empty and line block lines (lines beginning with '`|`') are recognized in this state.
    LineBlock,

    /// A state for recognizing bullet list items inside a ListTable
    ListTable,

    /// A state for parsing field lists inside diretives. Field lists located inside directive nodes
    /// work as directive parameters or settings.
    ExtensionOptions,

    /// A state for parsing section titles and document transitions (a.k.a. `\hrulefill` commands in LaTeX terms).
    Line,

    /// A state for parsing empty lines and literal blocks of text.
    /// Literal blocks are (non-contiguous) indented or "quoted" blocks of text that
    /// are  preceded by a paragraph ending in a `::`.
    LiteralBlock,

    /// An explicit failure state. Allows explicit signalling of transition failures.
    Failure,

    /// An End of File state. Could have also been named EOI, as in end of input,
    /// as this state is transitioned to when a parser reaches the end of its source input:
    /// This does not neecssarily correspond to the end of the given file during nested parsing sessions,
    /// as nested parsers are usually limited to a parsijng single block of text behind a node indentifier.
    EOF,
}

// ====================
// Statemachine methods
// ====================
impl State {

    /// Transitions a `StateMachine` into a `Failure` state using the From trait,
    /// the implementation of which automatically implements the Into trait.
    pub fn to_failure(self) -> Self {
        match self {
            _ => State::Failure,
        }
    }

    /// Retrieves the list of transitions based on a given `StateMachine` variant
    /// using a `match` statement. First checks for end states that don't contain transitions,
    /// such as `EOF` or `Failure` and if these are not matched,
    /// retrieves a list of transitions from the `TRANSITION_MAP`.
    pub fn get_transitions(
        &self,
        line_cursor: &LineCursor,
    ) -> Result<&Vec<Transition>, &'static str> {
        match self {
            State::EOF => Err("Already moved past EOF. No transitions to perform.\n"),
            State::Failure => Err("Failure state has no transitions\n"),
            State::Section
            | State::ListItem
            | State::Footnote
            | State::Citation
            | Self::Admonition
            | Self::Figure => Ok(TRANSITION_MAP.get(&State::Body).unwrap()),
            _ => {
                if let Some(transition_table) = TRANSITION_MAP.get(self) {
                    Ok(transition_table)
                } else {
                    panic!(
                        "Found no transition table for state {:#?} on line {}",
                        self,
                        line_cursor.sum_total()
                    )
                }
            }
        }
    }
}

/// =================================
/// StateMachine associated functions
/// =================================
impl State {

    /// Takes in a reference/slice to an associated array of uncompiled transitions
    /// and compiles the regex patterns found. Returns a `Vec<Transition>` with compiled state machines
    /// in place of the regex patterns.
    fn compile_state_transitions(transitions: &[UncompiledTransition]) -> Vec<Transition> {
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
impl State {}

lazy_static! {

  /// A static map of transititions for each `State` of the `Parser`.
  ///
  /// With this regexes are only compiled into automata once.
  pub static ref TRANSITION_MAP: std::collections::HashMap<State, Vec<(Pattern, regex::Regex, TransitionMethod)>> = {

    let mut action_map = std::collections::HashMap::with_capacity(10);

    let body_actions = State::compile_state_transitions(&State::BODY_TRANSITIONS);
    action_map.insert(State::Body, body_actions);

    let block_quote_actions = State::compile_state_transitions(&State::BLOCK_QUOTE_TRANSITIONS);
    action_map.insert(State::BlockQuote, block_quote_actions);

    let bullet_actions = State::compile_state_transitions(&State::BULLET_LIST_TRANSITIONS);
    action_map.insert(State::BulletList, bullet_actions);

    let definition_actions = State::compile_state_transitions(&State::DEFINITION_LIST_TRANSITIONS);
    action_map.insert(State::DefinitionList, definition_actions);

    let enumerated_actions = State::compile_state_transitions(&State::ENUMERATED_LIST_TRANSITIONS);
    action_map.insert(State::EnumeratedList, enumerated_actions);

    let field_actions = State::compile_state_transitions(&State::FIELD_LIST_TRANSITIONS);
    action_map.insert(State::FieldList, field_actions);

    let option_actions = State::compile_state_transitions(&State::OPTION_LIST_TRANSITIONS);
    action_map.insert(State::OptionList, option_actions);

    let line_block_actions = State::compile_state_transitions(&State::LINE_BLOCK_TRANSITIONS);
    action_map.insert(State::LineBlock, line_block_actions);

    let literal_block_actions = State::compile_state_transitions(&State::LITERAL_BLOCK_TRANSITIONS);
    action_map.insert(State::LiteralBlock, literal_block_actions);

    let extension_option_actions = State::compile_state_transitions(&State::EXTENSION_OPTION_TRANSITIONS);
    action_map.insert(State::ExtensionOptions, extension_option_actions);

    let line_actions = State::compile_state_transitions(&State::LINE_TRANSITIONS);
    action_map.insert(State::Line, line_actions);

    let list_table_actions = State::compile_state_transitions(&State::LIST_TABLE_TRANSITIONS);
    action_map.insert(State::ListTable, list_table_actions);

    // A+
    let aplus_multicol_actions = State::compile_state_transitions(&State::APLUS_MULTICOL_TRANSITIONS);
    action_map.insert(State::AplusMultiCol, aplus_multicol_actions);

    let aplus_questionnaire_actions = State::compile_state_transitions(&State::APLUS_QUESTIONNAIRE_TRANSITIONS);
    action_map.insert(State::AplusQuestionnaire, aplus_questionnaire_actions);

    action_map

  };

  /// Inline text has different parsing requirements than (nested)
  /// `Body` elements as they do not form blocks of text,
  /// making detecting by source line impractical.
  ///
  /// Instead, a block of source text is given to `Parser::parse_inline_nodes`
  /// which is then scanned with regular expressions.
  pub static ref COMPILED_INLINE_TRANSITIONS: Vec<(Pattern, regex::Regex, InlineParsingMethod)> = {

    let mut inline_transitions = Vec::with_capacity(State::INLINE_TRANSITIONS.len());

    for (pat_name, expr, fun) in State::INLINE_TRANSITIONS.iter() {
      let r = regex::Regex::new(expr).unwrap();
      inline_transitions.push((*pat_name, r, *fun));
    }

    inline_transitions
  };
}

impl <'source> Parser <'source> {
    /// Checks whether the line following the current one allows for the construction of an enumerate list item.
    /// Either the following line has to be blank, indented or the next enumerator in
    /// the current list has to be constructable from it.
    fn is_enumerated_list_item(
        src_lines: &[String],
        line_cursor: &mut LineCursor,
        captures: &regex::Captures,
        section_level: &mut usize,
        base_indent: usize,
        detected_enumerator_indent: usize,
        detected_number: usize,
        detected_kind: EnumKind,
        detected_delims: EnumDelims,
        pattern_name: &Pattern,
        list_kind: &EnumKind,
        in_list_item: bool,
        list_item_number: usize,
        list_start_index: usize,
    ) -> bool {

        use crate::parser::automata::ENUMERATOR_AUTOMATON;

        if let Some(next_line) = src_lines.get(line_cursor.relative_offset() + 1) {
            let next_line_indent = next_line
                .chars()
                .take_while(|c| c.is_whitespace())
                .count() + base_indent;

            if next_line.trim().is_empty() || next_line_indent > detected_enumerator_indent {
                return true
            } else if next_line_indent == detected_enumerator_indent {
                if let Some(next_captures) = ENUMERATOR_AUTOMATON.captures(next_line) {
                    let (next_number, next_kind, next_delims) = match converters::enum_captures_to_int_kind_and_delims(
                        &next_captures,
                        Some(list_kind),
                        in_list_item,
                        true,
                        list_item_number,
                        list_start_index
                    ) {
                        Some((number, kind, delims)) => (number, kind, delims),
                        None => return false
                    };
                    if ! (
                        next_number == detected_number + 1
                        && next_kind == detected_kind
                        && next_delims == detected_delims
                    ) {
                        eprintln!("Non-matching enumerator on next line...");
                        return false
                    } else {
                        true
                    }
                } else {
                    return false
                }
            } else {
                true
            }
        } else {
            true
        }
    }
}
