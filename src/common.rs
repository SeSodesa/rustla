/*!
This submodule contains useful functions and other constructs that don't
sensibly belong to any specific entity in the program.

Copyright © 2020 Santtu Söderholm
*/
use std::io::BufRead;
use std::{fs, io, path, str};

// =======================
// Text handling utilities
// =======================

/// Returns a `Vec<String>` from a given `&str`,
/// split at new lines `\n` or `\r\n`.
pub fn str_to_lines(string: &str) -> Vec<String> {
    let line_vec = string
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    line_vec
}

/// Read the lines of a given file into a buffer.
pub fn read_path_lines<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<path::Path>,
{
    let file: fs::File = fs::File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

/// Normalizes the given `&str` according to the reStructuredText specification.
/// In this context, normalization means converting any contiguous whitespace into
/// a single ASCII space character and converting letters to their lower-case variants.
///
/// ### Note
/// This might return something nonsensical, as converting a single multi-scalar grapheme
/// into lower-case will return the multiple constituent "characters" as their lower-case variants.
pub fn normalize_refname(name: &str) -> String {
    name.split_whitespace()
        .collect::<Vec<&str>>() // Collects the SplitWhiteSpace iterator into a vector of &strs
        .join(" ") // Joins the vector of &strs into an allocated String
        .to_lowercase() // Performs a UTF8-compliant transformation of unicode scalars in the String
                        // into their lower-case counterparts
}

/// A whitespace-aware function for stripping indentation
/// from `String`s. Returns `Ok(String)` if successful.
/// If non-whitespace characters are encountered before
/// the notified `amount` has been stripped, an `Err(message)`
/// is returned instead.
pub fn strip_indent(line: String, amount: usize) -> Result<String, &'static str> {
    if line.is_empty() {
        return Ok(line);
    }

    let mut chars = line.chars();

    for i in 0..amount {
        let c = chars.next().unwrap();

        if !c.is_whitespace() && i < amount {
            return Err("\nNon-whitespace character encountered before supposed indentation level reached.\n");
        }
    }

    Ok(chars.as_str().to_string())
}

// ============
// Type aliases
// ============

/// A type alias for an integer used as a node identifier.
pub type NodeId = u32;

/// A type alias for different kinds of enumerators such as list or foonote ordinals in integer format.
pub type EnumAsInt = u32;

/// A type alias for question points.
pub type QuizPoints = u32;


/// A type alias for the number type used in the `Length` enum.
pub type LengthNum = f64;

// ==========================
// Enumerators and converters
// ==========================

#[derive(Copy, Clone, Debug)]

/// An enum of transition regex pattern names, both for body and inline level elements.
pub enum PatternName {
    // Body elements, possibly nested
    Attribution,
    EmptyLine,
    Bullet,
    Citation,
    Comment,
    Enumerator,
    ExplicitMarkup,
    Directive,
    DocTest,
    FieldMarker,
    Footnote(FootnoteKind),
    HyperlinkTarget,
    IndentedLiteralBlock,
    Line,
    LineBlock,
    OptionMarker,
    Paragraph,
    QuotedLiteralBlock,
    Text,

    // Inline Elements for parsing Strings
    Escape,
    StrongEmphasis, // **strongly emphasised text**
    Emphasis,       // *emphasized text*
    Interpreted,    // Plain interpreted text with the default role set by transpiler.
    PhraseRef,      // A reference in the form `text with spaces`__?
    SimpleRef,      // A reference that doesn't need backticks: reference__?
    Literal,        // Code
    FootNoteRef,
    InlineTarget,    // Reference target in inline text: _`target label`
    SubstitutionRef, // Reference to substitution definition. Is replaced by the definition
    ImplicitURL,
    StandaloneHyperlink,
    WhiteSpace,

    // A+ specific
    AplusColBreak,
    AplusQuestionnaireDirective,
}

/// An enumeration fo the different A+ questionnaire types. This is used the differentiate
/// between questionnaire hint output formats, among other things.
#[derive(Debug)]
pub enum AplusQuestionnaireType {
    PickOne,
    PickAny,
    FreeText,
}

/// A section can be underlined, or over- and underlined with a certain character.
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum SectionLineStyle {
    OverAndUnder(char),
    Under(char),
}

/// Enumerated list item labels can either end with a period `.` or a right parenthesis `)`.
/// A third option is to enclose them in matching parentheses `(` and `)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnumDelims {
    Period,
    Parens,
    RParen,
}

/// List enumerator labels can be Arabic numerals, lower- or upper-case alphet `a--z` or `A--Z`,
/// or lower- or upper-case Roman numerals between `1--4999`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnumKind {
    Arabic,
    LowerAlpha,
    UpperAlpha,
    LowerRoman,
    UpperRoman,
    Automatic,
}

/// There are 4 different kinds of footnote markers:
/// 1. Manually numbered: `.. [1]` , `.. [2]`, ...
/// 2. automatically numbered: .. [#]
/// 3. automatically nubered with a simple reference name: .. [#simple_ref-name]
/// 4. Automatically generated symbolic markers: .. [*]
#[derive(Debug, Clone, Copy)]
pub enum FootnoteKind {
    Manual,
    AutoNumbered,
    SimpleRefName,
    AutoSymbol,
}

/// A hyperlink target may be one of 3 types:
///
/// 1. internal,
/// 2. external or
/// 3. indirect
///
/// **Internal** hyperlink targets have empty link blocks. They provide an end point allowing
/// a hyperlink to connect one place to another within a document.
/// An internal hyperlink target points to the element following the target.
///
/// **External** hyperlink targets have an absolute or relative URI or email address in their link blocks.
/// An external hyperlink's URI may begin on the same line as the explicit markup start and target name, or it may begin in an indented text block immediately following, with no intervening blank lines.
/// If there are multiple lines in the link block, they are concatenated.
/// Any unescaped whitespace is removed.
///
/// **Indirect** hyperlink targets have a hyperlink reference in their link blocks.
/// Just as with hyperlink references anywhere else in a document,
/// if a phrase-reference is used in the link block it must be enclosed in backquotes.
/// As with external hyperlink targets, the link block of an indirect hyperlink target may
/// begin on the same line as the explicit markup start or the next line.
/// It may also be split over multiple lines, in which case the lines are
/// joined with whitespace before being normalized.
#[derive(Debug, Clone)]
pub enum LinkTarget {
    Internal(String),
    External(String),
    Indirect(String),
}

/// An enumeration of the different types of references that a reference node might contain.
#[derive(Debug)]
pub enum Reference {
    Internal(String),
    URI(String),
    EMail(String),
}

/// There are 3 types of interpreted inline text, such as math:
/// 1. where the given role precedes the interpreted content and
/// 2. where the interpreted content precedes the given role.
/// 3. where  the type is not specified and the default role is used.
#[derive(Debug, Clone, Copy)]
pub enum InterpretedTextKind {
    Default,
    RoleThenContent,
    ContentThenRole,
}

/// An enumeration of how lengths can be interpreted.
/// This includes precentages of current context and absolute length
#[derive(Debug)]
pub enum MetricType {
    Percentage(f64),
    Lenght(Length),
}

/// Units of length recognized by reStructuredText.
#[derive(Debug)]
pub enum Length {

    /// em unit, the element's font size
    Em(LengthNum),


    /// ex unit, x-height of the element's font size
    Ex(LengthNum),


    /// Millimeters
    Mm(LengthNum),


    /// Centimeters.
    Cm(LengthNum),


    /// Inches. 1in == 2.54 cm == 96 px.
    In(LengthNum),


    /// Pixels. 1px == 1/96 in
    ///
    /// ### Note!
    /// In LaTeX, 1 px == 1/72 in.
    Px(LengthNum),


    /// Points. 1pt == 1/72 in
    Pt(LengthNum),


    /// Picas. 1 pc == 1/6 in == 12 pt
    Pc(LengthNum),
}

impl std::fmt::Display for Length {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let fmt_str = match self {
            Self::Em(num) => format!("{}em", num),
            Self::Ex(num) => format!("{}ex", num),
            Self::Mm(num) => format!("{}mm", num),
            Self::Cm(num) => format!("{}cm", num),
            Self::In(num) => format!("{}in", num),
            Self::Px(num) => format!("{}px", num),
            Self::Pt(num) => format!("{}pt", num),
            Self::Pc(num) => format!("{}pc", num),
        };
        write!(f, "{}", fmt_str)
    }
}

/// An enumeration of different horizontal alignment options.
#[derive(Debug)]
pub enum TableColWidths {
    Columns(Vec<f64>),
    Auto, // Determined by writer
}

/// An enumeration of different horizontal alignment options:
/// `Left`, `Middle` or `Right`.
#[derive(Debug)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

impl std::fmt::Display for HorizontalAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let fmt_str = match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
        };
        write!(f, "align={}", fmt_str)
    }
}

/// An enumeration of different backlinking alternatives for a table of contents node.
/// Instructs the doctree to generate links from section headers back to the table of
/// contents entries, the table of contents itself, or generate no backlinks.
#[derive(Debug)]
pub enum ToCBacklinks {
    Entry,
    Top,
    None,
}

/// An enumeration of the (deprecated) "align" attribute alternatives
/// recognized by the HTML `<img>` tag.
#[derive(Debug)]
pub enum HTMLAlignment {
    Top,
    Middle,
    Bottom,
    Left,
    Center,
    Right,
}

impl std::fmt::Display for HTMLAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let fmt_str = match self {
            Self::Top => "top",
            Self::Middle => "middle",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
        };
        write!(f, "{}", fmt_str)
    }
}

///
/// Enumerated the types of tree traversals that one of the `DocTree` walk methods might perform.
pub enum TraversalType {

    /// Traversal based on node ID. Causes the walker method to look for a specific node
    /// with the given ID.
    ID(NodeId),
}

use crate::doctree::DocTree;
use crate::parser::state_machine::State;

/// An enumeration of the different ways a (nested) parsing session might terminate.
/// The return type of the `Parser::parse` method. Generally, finishing conditions
/// that are not outright failures will enclose the document tree fed to the parser
/// when it was initialized.
pub enum ParsingResult {

    /// This will be returned, if the parser finished by passing over the last line of the source.
    /// This generally indicates that the source file was parsed successfully.
    EOF {
        doctree: DocTree,
        state_stack: Vec<State>,
    },

    /// This will be returned if the parser was unable to parse any elements on some line of the source,
    /// as patterns not matching will drain the parser state stack of states. This might be useful during
    /// nested parsing sessions, when an empty stack right at the start of the parsing process indicates
    /// that there were no expected nested structures on the same line.
    EmptyStateStack {
        doctree: DocTree,
        state_stack: Vec<State>,
    },

    /// A simple failure type. This will be returned when there was clearly no way to recover.
    Failure { message: String, doctree: DocTree },
}

impl ParsingResult {

    /// Unwraps the contained doctree in one of the non-failure variants.
    /// Simply panics if this is attempted for the `Failure` variant.
    pub fn unwrap_tree(self) -> DocTree {
        match self {
            Self::EOF {
                doctree,
                state_stack,
            } => doctree,
            Self::EmptyStateStack {
                doctree,
                state_stack,
            } => doctree,
            Self::Failure { doctree, .. } => doctree,
        }
    }
}

///
/// There are 6 possible statuses for A+ exercises:
///
/// * ready: Visible exercise listed in table of contents.
/// * unlisted (default): Unlisted in table of contents, otherwise same as ready.
/// * hidden: Hidden from non course staff.
/// * enrollment: Questions for students when they enroll to a course.
/// * enrollment_ext: Same as enrollment but for external students.
/// * maintenance: Hides the exercise description and prevents submissions.
#[derive(Debug)]
pub enum AplusExerciseStatus {
    Ready,
    Unlisted,
    Hidden,
    Enrollment,
    EnrollmentExt,
    Maintenance,
}

///
/// An enumeration of the different tokenizers offered by the A+ Radar tokenizer.
///
/// See [the docs](https://github.com/Aalto-LeTech/radar/tree/master/tokenizer#tokenizers)  for more details.
#[derive(Clone, Copy, Debug)]
pub enum AplusRadarTokenizer {
    Python3,
    Scala,
    JavaScript,
    CSS,
    HTML,
    None,
}

///
/// The variant "both" forces the element to a new line, "left" ("right") allows
/// no floating elements on the left (right)
#[derive(Clone, Copy, Debug)]
pub enum AplusActiveElementClear {
    /// Forces the element to a new line
    Both,

    /// Allows no floating elements on the left.
    Left,

    /// Allows no floating elements on the right.
    Right,
}

///
/// Use "file" for file inputs, "clickable" for clickable inputs, and
/// "dropdown" for dropdown. For dropdowns, the available options should
/// be listed after the type indicating "dropdown" in this
/// format: "dropdown:option1,option2,option3"
#[derive(Debug)]
pub enum AplusActiveElementInputType {
    /// Use for file inputs
    File,

    /// Use for clickable inputs
    Clickable,

    /// Use for dropdown menu. Comes with options in a String.
    Dropdown(String),
}

///
/// Default type is text; for image (png) outputs use "image"
#[derive(Clone, Copy, Debug)]
pub enum AplusActiveElementOutputType {
    Text,
    Image,
}

///
/// An enumeration of the different writer output formats.
/// Currently stdout and files are supported.
pub enum OutputStream {
    /// Directs the output to the stdout stream.
    StdOut,
    /// Directs the output to the stderr stream.
    StdErr,
    /// Directs to output to a file.
    File,
}

// ===========
//  Constants
// ===========

/// The standard Docutils system uses these symbols as footnote marks
/// when a FootnoteKind::AutoSymbol is detected.
/// They are, from first to last:
///
/// 1.  asterisk/star (`*`)
/// 2.  dagger (`†`|`U+02020`)
/// 3.  double dagger (`‡`|`U+02021`)
/// 4.  section mark (`§`|`U+000A7`)
/// 5.  pilcrow or paragraph mark (`¶`|`U+000B6`)
/// 6.  number sign (`#`)
/// 7.  spade suit (`♠`|`U+02660`)
/// 8.  heart suit (`♥`|`U+02665`)
/// 9.  diamond suit (`♦`|`U+02666`)
/// 10. club suit (`♣`|`U+02663`)
///
/// As the next autosymbol is detected the next unused item
/// from this list will be used as the footnote label character.
/// If `n` is the number of times this list has been iterated over
/// and `s` the current autosymbol, then the actual label
/// of the footnote is `s^(n+1)`. For example, if a document has
/// `12` automatically symboled footnotes and a new one is constructed,
/// then its label will be `‡‡ = ‡² = ‡¹⁺¹`.
pub const FOOTNOTE_SYMBOLS: [char; 10] = ['*', '†', '‡', '§', '¶', '#', '♠', '♥', '♦', '♣'];

///
/// These are the characters that can be used in underlining section titles,
///  marking the lines of literal text blocks and creating transitions.
pub const SECTION_AND_QUOTING_CHARS: [char; 32] = [
    '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=',
    '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~',
];
