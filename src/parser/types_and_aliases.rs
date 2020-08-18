/// ## type_aliases
/// 
/// A submodule that contains Parser-related type aliases.
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi


use super::{*};

// =====================================
//   Type aliases needed by the parser
// =====================================

/// ### TransitionMethod
/// A function pointer type alias for a State transition method.
/// `TransitionMethod`s take in the document tree and regex captures
/// for doctree modifications.
/// They return a `TransitionResult::{Success, Failure}`, the success variant of which contains a doctree,
/// a possible next state for the parser, information about manipulating the machine stack and whether to advance the parser line cursor.
/// If the optional next state is *not* `None`, the current state is either replaced with the new state or
/// the new state is pushed on top of the machine stack of the parser and parsing proceeds
/// in that state from the current line.
pub type TransitionMethod = fn(src_lines: &Vec<String>, base_indent: &usize, section_level: &mut usize, line_cursor: &mut LineCursor, doctree: Option<DocTree>, captures: regex::Captures, next_state: &PatternName) -> TransitionResult;

/// ### Transition
/// A type alias for a tuple `(PatternName, Regex, TransitionMethod)`
pub type Transition = (PatternName, regex::Regex, TransitionMethod);


/// ### UncompiledTransition
/// A type alias for a transition `(PatternName, regex_pattern, TransitionMethod)`, whose regex pattern has not
/// been compiled into a DFA yet.
pub type UncompiledTransition  = (PatternName, &'static str, TransitionMethod);

/// ### InlineTransitionMethod
/// A type alias for a function describing an inline transition.
/// Returns a node a length of the match, so that the inline parser
/// could determine how many characters to eat off the start of the
/// source string.
pub type InlineParsingMethod = fn (opt_doctree_ref: Option<&mut DocTree>, pattern_name: PatternName, captures: &regex::Captures) -> (TreeNodeType, usize);


/// ### InlineTransition
/// A type alias for a tuple `(PatternName, regex pattern, InlineTransitionMethod)`.
pub type InlineTransition = (PatternName, &'static str, InlineParsingMethod);


// ====================================================
//   Types and enums used by submodules of the parser
// ====================================================

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
    next_states: Option<Vec<StateMachine>>,
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


/// ### InlineParsingResult
/// An enumeration of the different ways an inline parsing function might succeed or fail.
pub enum InlineParsingResult {

  /// #### DoctreeAndNodes
  /// Returned when a document tree was handed over to the inline parsing function for modification purposes
  /// and no errors occurred.
  DoctreeAndNodes (DocTree, Vec<TreeNodeType>),
  
  /// #### Nodes
  /// If no doctree was given to the inline parsing function, so tree nodes might be appended to it directly,
  /// the data of the generated nodes is given to the caller stored in a vector.
  Nodes (Vec<TreeNodeType>),

  /// #### NoNodes
  /// If no nodes were discovered and no doctree was given to be modified, this empty variant is returned.
  NoNodes,

}
