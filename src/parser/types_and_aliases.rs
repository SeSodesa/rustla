/*!
A submodule that contains Parser-related type aliases.

Copyright © 2020 Santtu Söderholm
*/

use super::*;

// =====================================
//   Type aliases needed by the parser
// =====================================

/// A function pointer type alias for a State transition method.
/// `TransitionMethod`s take in the document tree and regex captures
/// for doctree modifications.
/// They return a `TransitionResult::{Success, Failure}`, the success variant of which contains a doctree,
/// a possible next state for the parser, information about manipulating the machine stack and whether to advance the parser line cursor.
/// If the optional next state is *not* `None`, the current state is either replaced with the new state or
/// the new state is pushed on top of the machine stack of the parser and parsing proceeds
/// in that state from the current line.
pub type TransitionMethod = fn(
    src_lines: &Vec<String>,
    base_indent: usize,
    section_level: &mut usize,
    line_cursor: &mut LineCursor,
    doctree: DocTree,
    captures: &regex::Captures,
    pattern_name: &Pattern,
) -> TransitionResult;

/// A type alias for a tuple `(PatternName, Regex, TransitionMethod)`
pub type Transition = (Pattern, regex::Regex, TransitionMethod);

/// A type alias for a transition `(PatternName, regex_pattern, TransitionMethod)`, whose regex pattern has not
/// been compiled into a DFA yet.
pub type UncompiledTransition = (Pattern, &'static str, TransitionMethod);

/// A type alias for a function describing an inline transition.
/// Returns a node a length of the match, so that the inline parser
/// could determine how many characters to eat off the start of the
/// source string.
pub type InlineParsingMethod = fn(
    opt_doctree_ref: &mut Option<&mut DocTree>,
    pattern_name: Pattern,
    captures: &regex::Captures,
) -> (Vec<TreeNodeType>, usize);

/// A type alias for a tuple `(PatternName, regex pattern, InlineTransitionMethod)`.
pub type InlineTransition = (Pattern, &'static str, InlineParsingMethod);

// ====================================================
//   Types and enums used by submodules of the parser
// ====================================================

/// An enumeration of the different results, including errors,
/// that a transition function might have.
pub enum TransitionResult {

    /// This is returned if nothing goes wrong with a transition method.
    /// It includes the modified document tree, plus information about
    /// how to manipulate the parser stack, whether the parser should advance
    /// its line cursor.
    Success {
        doctree: DocTree,
        push_or_pop: PushOrPop,
        line_advance: LineAdvance,
    },

    /// A general failure result. This will be returned if a clear error, such as a completetely invalid enumerator was
    /// encountered in a transition method functions. Contains an error message and the doctree in its current state.
    Failure { message: String, doctree: DocTree },
}

/// An enum for manipulating the machine stack. Transition methods should return this information
/// with a possible next state, so the parser knows how to proceed. The `Push` variant signifies
/// a state should be pushed on top of the stack, `Pop` tells of the need to pop from the stack
/// and `Neither` initiates a transition of the current state into another one.
#[derive(Debug)]
pub enum PushOrPop {

    /// Causes `Parser::parse` to push the contained states on top of the parser stack.
    Push(Vec<State>),
    /// Causes `Parser::parse` to pop the topmost state from the parser state stack.
    Pop,
    /// Signifies to `Parser::parse` that nothing about the stack needs to change.
    Neither,
}

/// An enum returned by the transition methods to tell the parser whether
/// it needs to advance its line cursor after the method execution or not.
pub enum LineAdvance {
    Some(usize),
    None,
}

/// An enumeration of the different ways an inline parsing function might succeed or fail.
pub enum InlineParsingResult {

    /// If no doctree was given to the inline parsing function, so tree nodes might be appended to it directly,
    /// the data of the generated nodes is given to the caller stored in a vector.
    Nodes(Vec<TreeNodeType>),

    /// If no nodes were discovered and no doctree was given to be modified, this empty variant is returned.
    NoNodes,
}

/// A enumeration of the different ways a node's child indentation might
/// interact with the indentation of the parent.
pub enum IndentationMatch {

    /// If a (sub)?body node has less indentation than its parent would require,
    /// it is interpreted as not belonging to the currently focused on node.
    TooLittle,

    /// This node belongs to the parent node.
    JustRight,

    /// This node is most likely a block quote.
    TooMuch,
}

/// A enumeration of the different ways the function `Parser::read_indented_block` could succeed or fail.
pub enum IndentedBlockResult {
    /// The reading of the text block succeeded as intended
    Ok {
        lines: Vec<String>,
        minimum_indent: usize,
        offset: usize,
        blank_finish: bool,
    },
    /// The given line vector was empty
    EmptyLinesErr,
    ///
    UnfinishedErr {
        lines: Vec<String>,
        minimum_indent: usize,
        offset: usize,
        blank_finish: bool,
    }
}

/// A enumeration of the different ways the function `Parser::read_text_block` could succeed or fail.
pub enum TextBlockResult {
    Ok {
        lines: Vec<String>,
        offset: usize,
    },
    Err {
        lines: Vec<String>,
        offset: usize,
    }
}
