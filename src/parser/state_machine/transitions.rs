/*!
Module contains lists of transition tuples related to different states
and common patterns as &str constants.

(c) Santtu Söderholm <santtu.soderholm@tuni.fi>
*/

use super::*;

/// =================================
/// StateMachine associated constants
/// =================================
impl State {

    /// An array of transitions related to `State::Body`.
    pub const BODY_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            PatternName::EmptyLine,
            BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (PatternName::Bullet, BULLET_PATTERN, body::bullet),
        (
            PatternName::Enumerator,
            crate::parser::regex_patterns::ENUMERATOR_PATTERN,
            body::enumerator,
        ),
        (
            PatternName::FieldMarker,
            FIELD_MARKER_PATTERN,
            body::field_marker,
        ),
        (
            PatternName::Footnote(FootnoteKind::Manual),
            MANUAL_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Footnote(FootnoteKind::AutoNumbered),
            AUTO_NUM_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Footnote(FootnoteKind::SimpleRefName),
            SIMPLE_NAME_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Footnote(FootnoteKind::AutoSymbol),
            AUTO_SYM_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (PatternName::Citation, CITATION_PATTERN, body::citation),
        (
            PatternName::HyperlinkTarget,
            HYPERLINK_TARGET_PATTERN,
            body::hyperlink_target,
        ),
        (PatternName::Directive, DIRECTIVE_PATTERN, body::directive),
        (PatternName::Comment, COMMENT_PATTERN, body::comment),
        (PatternName::Line, LINE_PATTERN, body::line),
        (PatternName::Text, TEXT_PATTERN, body::text),
    ];

    /// An array of transitions related to `State::Body`.
    pub const BLOCK_QUOTE_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            PatternName::EmptyLine,
            BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            PatternName::Attribution,
            ATTRIBUTION_PATTERN,
            block_quote::attribution,
        ),
        (PatternName::Bullet, BULLET_PATTERN, body::bullet),
        (
            PatternName::Enumerator,
            crate::parser::regex_patterns::ENUMERATOR_PATTERN,
            body::enumerator,
        ),
        (
            PatternName::FieldMarker,
            FIELD_MARKER_PATTERN,
            body::field_marker,
        ),
        (
            PatternName::Footnote(FootnoteKind::Manual),
            MANUAL_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Footnote(FootnoteKind::AutoNumbered),
            AUTO_NUM_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Footnote(FootnoteKind::SimpleRefName),
            SIMPLE_NAME_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Footnote(FootnoteKind::AutoSymbol),
            AUTO_SYM_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (PatternName::Citation, CITATION_PATTERN, body::citation),
        (
            PatternName::HyperlinkTarget,
            HYPERLINK_TARGET_PATTERN,
            body::hyperlink_target,
        ),
        (PatternName::Directive, DIRECTIVE_PATTERN, body::directive),
        (PatternName::Comment, COMMENT_PATTERN, body::comment),
        (PatternName::Line, LINE_PATTERN, body::line),
        (PatternName::Text, TEXT_PATTERN, body::text),
    ];

    /// An array of transitions related to `State::BulletList`.
    pub const BULLET_LIST_TRANSITIONS: [UncompiledTransition; 2] = [
        (
            PatternName::EmptyLine,
            BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (PatternName::Bullet, BULLET_PATTERN, bullet_list::bullet),
    ];

    /// An array of transitions related to `State::DefinitionList`.
    pub const DEFINITION_LIST_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            PatternName::EmptyLine,
            BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            PatternName::Bullet,
            BULLET_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::Enumerator,
            crate::parser::regex_patterns::ENUMERATOR_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::FieldMarker,
            FIELD_MARKER_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::Footnote(FootnoteKind::Manual),
            MANUAL_FOOTNOTE_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::Footnote(FootnoteKind::AutoNumbered),
            AUTO_NUM_FOOTNOTE_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::Footnote(FootnoteKind::SimpleRefName),
            SIMPLE_NAME_FOOTNOTE_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::Footnote(FootnoteKind::AutoSymbol),
            AUTO_SYM_FOOTNOTE_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::Citation,
            CITATION_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::HyperlinkTarget,
            HYPERLINK_TARGET_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::Directive,
            DIRECTIVE_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::Comment,
            COMMENT_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::Line,
            LINE_PATTERN,
            unknown_transitions::back_up,
        ),
        (PatternName::Text, TEXT_PATTERN, definition_list::text),
    ];

    /// An array of transitions related to `State::EnumeratedList`.
    pub const ENUMERATED_LIST_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            PatternName::EmptyLine,
            BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            PatternName::Enumerator,
            crate::parser::regex_patterns::ENUMERATOR_PATTERN,
            enumerated_list::enumerator,
        ),
    ];

    /// An array of transitions related to `State::FieldList`.
    pub const FIELD_LIST_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            PatternName::EmptyLine,
            BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            PatternName::FieldMarker,
            FIELD_MARKER_PATTERN,
            field_list::field_marker,
        ),
    ];

    const HYPERLINK_TARGET_TRANSITIONS: [UncompiledTransition; 0] = [];

    /// An array of transitions related to `State::OptionList`.
    pub const OPTION_LIST_TRANSITIONS: [UncompiledTransition; 0] = [];

    /// An array of transitions related to `State::LineBlock`.
    pub const LINE_BLOCK_TRANSITIONS: [UncompiledTransition; 0] = [];

    /// An array of transitions related to `State::Line`.
    pub const LITERAL_BLOCK_TRANSITIONS: [UncompiledTransition; 3] = [
        (
            PatternName::EmptyLine,
            BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            PatternName::QuotedLiteralBlock,
            QUOTED_LITERAL_BLOCK_PATTERN,
            literal_block::literal_block,
        ),
        (
            PatternName::IndentedLiteralBlock,
            INDENTED_LITERAL_BLOCK_PATTERN,
            literal_block::literal_block,
        ),
    ];

    /// An array of transitions related to `State::ExtensionOptions`.
    pub const EXTENSION_OPTION_TRANSITIONS: [UncompiledTransition; 0] = [];

    /// An array of transitions related to `State::ExplicitMarkup`.
    pub const EXPLICIT_MARKUP_TRANSITIONS: [UncompiledTransition; 0] = [];

    /// An array of transitions related to `State::Text`.
    pub const TEXT_TRANSITIONS: [UncompiledTransition; 0] = [];

    /// An array of transitions related to `State::Definition`.
    pub const DEFINITION_LIST_ITEM_TRANSITIONS: [UncompiledTransition; 0] = [];

    /// An array of transitions related to `State::Line`.
    pub const LINE_TRANSITIONS: [UncompiledTransition; 0] = [];

    /// An array of transitions related to `State::SubstitutionDef`.
    pub const SUBSTITUTION_DEF_TRANSITIONS: [UncompiledTransition; 0] = [];

    pub const LIST_TABLE_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            PatternName::EmptyLine,
            BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (PatternName::Bullet, BULLET_PATTERN, body::bullet),
    ];

    /// An array of transitions allowed in multi-column A+ directives such as points of interest.
    /// These are indentical to those, except the state also recognizes
    pub const APLUS_MULTICOL_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            PatternName::EmptyLine,
            BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            PatternName::AplusColBreak,
            APLUS_COL_BREAK_PATTERN,
            aplus::aplus_col_break,
        ),
        (PatternName::Bullet, BULLET_PATTERN, body::bullet),
        (
            PatternName::Enumerator,
            crate::parser::regex_patterns::ENUMERATOR_PATTERN,
            body::enumerator,
        ),
        (
            PatternName::FieldMarker,
            FIELD_MARKER_PATTERN,
            body::field_marker,
        ),
        (
            PatternName::Footnote(FootnoteKind::Manual),
            MANUAL_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Footnote(FootnoteKind::AutoNumbered),
            AUTO_NUM_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Footnote(FootnoteKind::SimpleRefName),
            SIMPLE_NAME_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Footnote(FootnoteKind::AutoSymbol),
            AUTO_SYM_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (PatternName::Citation, CITATION_PATTERN, body::citation),
        (
            PatternName::HyperlinkTarget,
            HYPERLINK_TARGET_PATTERN,
            body::hyperlink_target,
        ),
        (PatternName::Directive, DIRECTIVE_PATTERN, body::directive),
        (PatternName::Comment, COMMENT_PATTERN, body::comment),
        (PatternName::Line, LINE_PATTERN, body::line),
        (PatternName::Text, TEXT_PATTERN, body::text),
    ];

    pub const APLUS_QUESTIONNAIRE_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            PatternName::EmptyLine,
            BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            PatternName::AplusQuestionnaireDirective,
            APLUS_QUESTIONNAIRE_DIRECTIVE_PATTERN,
            aplus_questionnaire::parse_aplus_questionnaire_directive,
        ),
        (
            PatternName::Text,
            TEXT_PATTERN,
            aplus_questionnaire::parse_aplus_questionnaire_text,
        ),
    ];

    /// An array of inline transitions.
    pub const INLINE_TRANSITIONS: [InlineTransition; 12] = [
        (PatternName::WhiteSpace, r"^\s+", inline::whitespace),
        (
            PatternName::StrongEmphasis,
            STRONG_EMPH_PATTERN,
            inline::paired_delimiter,
        ),
        (
            PatternName::Emphasis,
            EMPH_PATTERN,
            inline::paired_delimiter,
        ),
        (
            PatternName::Literal,
            LITERAL_PATTERN,
            inline::paired_delimiter,
        ),
        (
            PatternName::InlineTarget,
            INLINE_TARGET_PATTERN,
            inline::inline_target,
        ),
        (
            PatternName::PhraseRef,
            PHRASE_REF_PATTERN,
            inline::phrase_ref,
        ),
        (
            PatternName::Interpreted,
            INTERPRETED_TEXT_PATTERN,
            inline::interpreted_text,
        ),
        (
            PatternName::FootNoteRef,
            FOOTNOTE_REF_PATTERN,
            inline::footnote_ref,
        ),
        (
            PatternName::SimpleRef,
            SIMPLE_REF_PATTERN,
            inline::simple_ref,
        ),
        (
            PatternName::SubstitutionRef,
            SUBSTITUTION_REF_PATTERN,
            inline::substitution_ref,
        ),

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
        (PatternName::StandaloneHyperlink, URI_PATTERN, inline::uri),
        //(PatternName::Text, r"^([^\\\n\[*`:_]+)(?:[^_][a-zA-Z0-9]+_)?", Inline::text),
        (PatternName::Text, r"^(\S+)", inline::text),
    ];
}

// ==================================
// Patterns common to multiple states
// ==================================

/// A pattern for matching attributions inside block quotes.
pub const ATTRIBUTION_PATTERN: &'static str = r"^(\s*)(?:--|---|—) *";

/// A pattern for matching blank lines, as in lines that contain nothing but whitespace.
const BLANK_LINE_PATTERN: &'static str = r"^\s*$";

/// A pattern for matching bullet list bullets.
const BULLET_PATTERN: &'static str = r"^(\s*)([+\-*\u{2022}\u{2023}\u{2043}])(?: +|$)";

/// A pattern for Arabic numerals with closing parentheses
const ARABIC_PARENS_PATTERN: &'static str = r"^(\s*)\(([0-9]+)\)(?: +|$)";
/// A pattern for Arabic numerals with a closing right parenthesis
const ARABIC_RPAREN_PATTERN: &'static str = r"^(\s*)([0-9]+)\)(?: +|$)";
/// A pattern for Arabic numerals with a closing period
const ARABIC_PERIOD_PATTERN: &'static str = r"^(\s*)([0-9]+)\.(?: +|$)";

/// A pattern for lower case alphabetic numerals with closing parentheses
const LOWER_ALPHA_PARENS_PATTERN: &'static str = r"^(\s*)\(([a-z])\)(?: +|$)";
/// A pattern for lower case alphabetic numerals with a closing right parenthesis
const LOWER_ALPHA_RPAREN_PATTERN: &'static str = r"^(\s*)([a-z])\)(?: +|$)";
/// A pattern for lower case alphabetic numerals with a closing period
const LOWER_ALPHA_PERIOD_PATTERN: &'static str = r"^(\s*)([a-z])\.(?: +|$)";

/// A pattern for upper case alphabetic numerals with closing parentheses
const UPPER_ALPHA_PARENS_PATTERN: &'static str = r"^(\s*)\(([A-Z])\)(?: +|$)";
/// A pattern for upper case alphabetic numerals with a closing right parenthesis
const UPPER_ALPHA_RPAREN_PATTERN: &'static str = r"^(\s*)([A-Z])\)(?: +|$)";
/// A pattern for upper case alphabetic numerals with a closing period
const UPPER_ALPHA_PERIOD_PATTERN: &'static str = r"^(\s*)([A-Z])\.(?: +|$)";

/// A pattern for lower Roman numerals with closing parentheses
const LOWER_ROMAN_PARENS_PATTERN: &'static str = r"^(\s*)\(([ivxlcdm]+)\)(?: +|$)";
/// A pattern for lower Roman numerals with a closing right parenthesis
const LOWER_ROMAN_RPAREN_PATTERN: &'static str = r"^(\s*)([ivxlcdm]+)\)(?: +|$)";
/// A pattern for lower Roman numerals with a closing period
const LOWER_ROMAN_PERIOD_PATTERN: &'static str = r"^(\s*)([ivxlcdm]+)\.(?: +|$)";

/// A pattern for upper Roman numerals with closing parentheses
const UPPER_ROMAN_PARENS_PATTERN: &'static str = r"^(\s*)\(([IVXLCDM]+)\)(?: +|$)";
/// A pattern for upper Roman numerals with a closing right parenthesis
const UPPER_ROMAN_RPAREN_PATTERN: &'static str = r"^(\s*)([IVXLCDM]+)\)(?: +|$)";
/// A pattern for upper Roman numerals with a closing period
const UPPER_ROMAN_PERIOD_PATTERN: &'static str = r"^(\s*)([IVXLCDM]+)\.(?: +|$)";

/// A pattern for upper Roman numerals with closing parentheses
const AUTO_ENUM_PARENS_PATTERN: &'static str = r"^(\s*)\((\#)\)(?: +|$)";
/// A pattern for upper Roman numerals with a closing right parenthesis
const AUTO_ENUM_RPAREN_PATTERN: &'static str = r"^(\s*)(\#)\)(?: +|$)";
/// A pattern for upper Roman numerals with a closing period
const AUTO_ENUM_PERIOD_PATTERN: &'static str = r"^(\s*)(\#)\.(?: +|$)";

/// A pattern that signifies the start of a field list, such as a bibliography.
/// Colons inside field names `:field name:` must be escaped if followed by whitespace,
/// as ": " signifies the end of a list marker.
pub const FIELD_MARKER_PATTERN: &'static str = r"^(\s*):([\S&&[^\\]]|\S.*?[\S&&[^\\]]):(?: +|$)";

/// A pattern for matching against an indented block of text when in `State::LiteralBlock`.
const INDENTED_LITERAL_BLOCK_PATTERN: &'static str = r"^(\s+)\S";

/// A pattern for matching against an "quoted" block of text when in `State::LiteralBlock`.
const QUOTED_LITERAL_BLOCK_PATTERN: &'static str =
    r#"^(\s*)(!|"|#|\$|%|&|'|\(|\)|\*|\+|,|-|\.|/|:|;|<|=|>|\?|@|\[|\\|\]|\^|_|`|\{|\||\}|~)"#;

// ========================
// Explicit markup patterns
// ========================

/// A pattern for matching against manually numbered footnotes.
const MANUAL_FOOTNOTE_PATTERN: &'static str = r"^(\s*)\.\.[ ]+\[(\d+)\](?:[ ]+|$)";

/// A footnote pattern with the symbol '#' for a label.
/// This triggers automatic numbering for the footnote to be generated.
const AUTO_NUM_FOOTNOTE_PATTERN: &'static str = r"^(\s*)\.\.[ ]+\[(\#)\](?:[ ]+|$)";

/// Similar to `AUTO_NUM_FOONOTE_PATTERN`, except allows referencing the same footnote
/// multiple times, as there is a simple reference name pointing to the footnote.
const SIMPLE_NAME_FOOTNOTE_PATTERN: &'static str =
    r"^(\s*)\.\.[ ]+\[\#([a-zA-Z][a-zA-Z0-9]+(?:[-+._:][a-zA-Z0-9]+)*)\](?:[ ]+|$)";

/// Prompts the generation of symbolic footnotes, with automatic reference mark generation.
const AUTO_SYM_FOOTNOTE_PATTERN: &'static str = r"^(\s*)\.\.[ ]+\[(\*)\](?:[ ]+|$)";

/// A pattern for matching against citations.
/// Similar to `FOOTNOTE_PATTERN`, but only
/// recognizes simple reference names in labels.
const CITATION_PATTERN: &'static str =
    r"^(\s*)\.\.[ ]+\[([a-zA-Z][a-zA-Z0-9]*(?:[-+._:][a-zA-Z0-9]+)*)\](?:[ ]+|$)";

/// A pattern for matching hyperlink targets. A hyperlink target may either be labeled with a simple reference name or
/// with and underscore `_`, the latter of which signifies an anonymous link.
const HYPERLINK_TARGET_PATTERN: &'static str =
    r"^(\s*)\.\.[ ]+_([a-zA-Z0-9][a-zA-Z0-9 ]*(?:[-+._:][a-zA-Z0-9 ]+)*[a-zA-Z0-9]+|_):(?:[ ]+|$)";

/// A pattern for matching substitution definitions, a.k.a. macros.
const SUBSTITUTION_DEF_PATTERN: &'static str = r"^(\s*)\.\.[ ]+\|(\S|\S.*\S)\| ::(?:[ ]+|$)";

/// A pattern for matching directives. The directive label is used to determine the type of directive
/// inside a transition function. The label itself is a simple reference name (an identifier).
const DIRECTIVE_PATTERN: &'static str =
    r"^(\s*)\.\.[ ]+([a-zA-Z][a-zA-Z0-9]+(?:[-+._:][a-zA-Z0-9]+)*)[ ]?::(?:[ ]+|$)";

/// A pattern for recognizing comments, after no other explicit markup pattern has matched.
const COMMENT_PATTERN: &'static str = r"^(\s*)\.\.(?: +|$)";

/// A pattern for recognizing lines related to section titles and transitions.
pub const LINE_PATTERN: &'static str = r#"^(!+|"+|#+|\$+|%+|&+|'+|\(+|\)+|\*+|\++|,+|-+|\.+|/+|:+|;+|<+|=+|>+|\?+|@+|\[+|\\+|\]+|\^+|_+|`+|\{+|\|+|\}+|~+) *$"#;

/// A pattern for detecting any text, possibly beginning with whitespace.
/// This pattern should generally be tested against only after all other
/// possibilities have been eliminated.
pub const TEXT_PATTERN: &'static str = r"^(\s*)\S";

// =================
//  Inline patterns
// =================

const STRONG_EMPH_PATTERN: &str = r#"(?x)^
  (?P<lookbehind>
    [-:/'"<(\[{\p{Ps}\p{Pi}\p{Pf}\p{Pd}\p{Po}\s&&[^\\*]]
  )?
  (?P<markup_start>
    \*\*
  )
  (?P<content>
    [\S&&[^\\]]|\S[\S\s]*?[\S&&[^\\]]
  )
  (?P<markup_end>
    \*\*
  )
  (?P<lookahead>
    \s|[-.,:;!?\\/'")\]}>\p{Pe}\p{Pi}\p{Pf}\p{Pd}\p{Po}&&[^*]]|$
  )
"#;

const EMPH_PATTERN: &str = r#"(?x)^
  (?P<lookbehind>
    [-:/'"<(\[{\p{Ps}\p{Pi}\p{Pf}\p{Pd}\p{Po}\s&&[^\\*]]
  )?
  (?P<markup_start>
    \*
  )
  (?P<content>
    [\S&&[^\\]]|\S[\S\s]*?[\S&&[^\\]]
  )
  (?P<markup_end>
    \*
  )
  (?P<lookahead>
    \s|[-.,:;!?\\/'")\]}>\p{Pe}\p{Pi}\p{Pf}\p{Pd}\p{Po}&&[^*]]|$
  )
"#;

const LITERAL_PATTERN: &str = r#"(?x)^
  (?P<lookbehind>
    [-:/'"<(\[{\p{Ps}\p{Pi}\p{Pf}\p{Pd}\p{Po}\s&&[^\\*]]
  )?
  (?P<markup_start>
    ``
  )
  (?P<content>
    [\S&&[^\\]]|\S[\S\s]*?[\S&&[^\\]]
  )
  (?P<markup_end>
    ``
  )
  (?P<lookahead>
    \s|[-.,:;!?\\/'")\]}>\p{Pe}\p{Pi}\p{Pf}\p{Pd}\p{Po}&&[^*]]|$
  )
"#;

const INLINE_TARGET_PATTERN: &str = r#"(?x)^
  (?P<lookbehind>
    [-:/'"<(\[{\p{Ps}\p{Pi}\p{Pf}\p{Pd}\p{Po}\s&&[^\\*]]
  )?
  (?P<markup_start>
    _`
  )
  (?P<content>
    [\S&&[^\\]]|\S[\S\s]*?[\S&&[^\\]]
  )
  (?P<markup_end>
    `
  )
  (?P<lookahead>
    \s|[-.,:;!?\\/'")\]}>\p{Pe}\p{Pi}\p{Pf}\p{Pd}\p{Po}&&[^*]]|$
  )
"#;

const INTERPRETED_TEXT_PATTERN: &str = r#"(?x)^
  (?P<lookbehind>
    [-:/'"<(\[{\p{Ps}\p{Pi}\p{Pf}\p{Pd}\p{Po}\s&&[^\\*]]
  )?

  (?P<front_role_marker>
    :(?P<front_role>
      \S|\S.*?\S
    ):
  )?
  (?P<markup_start>
    `
  )
  (?P<content>
    [\S&&[^\\]]|\S[\S\s]*?[\S&&[^\\]]
  )
  (?P<markup_end>
    `
  )
  (?P<back_role_marker>
    :(?P<back_role>
      \S|\S.*?\S
    ):
  )?
  (?P<lookahead>
    \s|[-.,:;!?\\/'")\]}>\p{Pe}\p{Pi}\p{Pf}\p{Pd}\p{Po}&&[^*]]|$
  )
"#;

const PHRASE_REF_PATTERN: &str = r#"(?x)^
  (?P<lookbehind>
    [-:/'"<(\[{\p{Ps}\p{Pi}\p{Pf}\p{Pd}\p{Po}\s&&[^\\*]]
  )?
  (?P<markup_start>
    `
  )
  (?P<content>
    [\S&&[^\\]]|\S[\S\s]*?[\S&&[^\\]]
  )
  (?P<embeded_uri_container>
    \s+#space character one or more times
    <
    (?P<embedded_uri>
      [\S&&[^\\]]|\S[\S\s]*?[\S&&[^\\]]
    )
    >
  )?
  (?P<markup_end>
    `
  )
  (?P<ref_type>
    __?
  )
  (?P<lookahead>
    \s|[-.,:;!?\\/'")\]}>\p{Pe}\p{Pi}\p{Pf}\p{Pd}\p{Po}&&[^*]]|$
  )
"#;

const SIMPLE_REF_PATTERN: &str = r#"(?x)^
  (?P<lookbehind>
    [-:/'"<(\[{\p{Ps}\p{Pi}\p{Pf}\p{Pd}\p{Po}\s&&[^\\*]]
  )?
  (?P<content>
    [a-zA-Z0-9]+(?:[-_.:+][a-zA-Z0-9]+)*
  )
  (?P<ref_type>
    __?
  )
  (?P<lookahead>
    \s|[-.,:;!?\\/'")\]}>\p{Pe}\p{Pi}\p{Pf}\p{Pd}\p{Po}&&[^*]]|$
  )
"#;

const FOOTNOTE_REF_PATTERN: &str = r#"(?x)^
  (?P<lookbehind>
    [-:/'"<(\[{\p{Ps}\p{Pi}\p{Pf}\p{Pd}\p{Po}\s&&[^\\*]]
  )?
  (?P<markup_start>
    \[
  )
  (?P<content>
    (?P<number>
      [0-9]+
    )
    |
    (?P<auto_number>
      #
    )
    |
    (?P<auto_number_label>
      #[a-z](-?[a-z0-9]+)*
    )
    |
    (?P<symbol>
      \*
    )

  )
  (?P<markup_end>
    \]
  )
  (?P<ref_type>
    __?
  )
  (?P<lookahead>
    \s|[-.,:;!?\\/'")\]}>\p{Pe}\p{Pi}\p{Pf}\p{Pd}\p{Po}&&[^*]]|$
  )
"#;
const CITATION_REF_PATTERN: &str = r#"(?x)^"
  (?P<lookbehind>
    [-:/'"<(\[{\p{Ps}\p{Pi}\p{Pf}\p{Pd}\p{Po}\s&&[^\\*]]
  )?
  (?P<markup_start>
    \[
  )
  (?P<content>
    [a-zA-Z0-9]+([-_.]?[a-zA-Z0-9]+)*
  )
  (?P<markup_end>
    \]
  )
  (?P<ref_type>
    __?
  )
  (?P<lookahead>
    \s|[-.,:;!?\\/'")\]}>\p{Pe}\p{Pi}\p{Pf}\p{Pd}\p{Po}&&[^*]]|$
  )
"#;

const SUBSTITUTION_REF_PATTERN: &str = r#"(?x)^
  (?P<lookbehind>
    [-:/'"<(\[{\p{Ps}\p{Pi}\p{Pf}\p{Pd}\p{Po}\s&&[^\\*]]
  )?
  (?P<markup_start>
    \|
  )
  (?P<content>
    [\S&&[^\\]]|\S[\S\s]*?[\S&&[^\\]]
  )
  (?P<markup_end>
    \|
  )
  (?P<ref_type>
    __?
  )?
  (?P<lookahead>
    \s|[-.,:;!?\\/'")\]}>\p{Pe}\p{Pi}\p{Pf}\p{Pd}\p{Po}&&[^*]]|$
  )
"#;

const URI_PATTERN: &str = r#"(?x)^
(?P<lookbehind>
  \s|[-:/'"<(\[{\p{Ps}\p{Pi}\p{Pf}\p{Pd}\p{Po}]
)?
(?P<content>
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
    (?P<authority>
      (//?)?
      (?:
        (?P<userinfo>
          [A-Za-z0-9]+(?:[.][A-Za-z0-9]+)*
        )@
      )?
      (?P<host>
        [a-zA-Z0-9]+(?:[-.+][a-zA-Z0-9]+)* # hostname
        | [0-9]{1,3}(?:\.[0-9]{1,3}){3} # IPv4
        # TODO: add IPv6 pattern here
      )
      (?::
        (?P<port>[0-9]+)
      )?
    )?
    (?P<path>
      /?[a-zA-Z0-9]*(?:/[A-Za-z0-9]+)*/?
    )
    [_~*/=+a-zA-Z0-9] # Allowed URI suffixes
    (?:\?
      (?P<query>
        [=&a-zA-Z0-9]+
        [_~*/=+a-zA-Z0-9] # Allowed URI suffixes
      )
    )?
    (?:\#
      (?P<fragment>
        [a-zA-Z0-9]+
        [_~*/=+a-zA-Z0-9] # Allowed URI suffixes
      )
    )?
  )
  | # if not absolute uri, then email
  (?P<email>
    [-_a-zA-Z0-9]+
    (?:\.[-_!~*'{|}/\#?\^`&=+$%a-zA-Z0-9]+)*
    @
    [-_a-zA-Z0-9]+
    (?:[.-][a-zA-Z0-9]+)*
  )
)
(?P<lookahead>
  \s|[-.,:;!?\\/'")\]}>\p{Pe}\p{Pi}\p{Pf}\p{Pd}\p{Po}]|$
)
"#;

// ======================
//  A+ specific patterns
// ======================

/// A regex pattern relatex to detecting column breaks in multi-column
/// A+ directives, such as points of interest.
const APLUS_COL_BREAK_PATTERN: &str = r#"^(\s+)::newcol"#;

const APLUS_QUESTIONNAIRE_DIRECTIVE_PATTERN: &'static str =
    r"^(\s*)\.\.[ ]+(pick-one|pick-any|freetext)::(?:[ ]+|$)";

const APLUS_PICK_ANY_PATTERN: &'static str = r"^(\s*)\.\.[ ]+pick-any::(?:[ ]+|$)";

/// Correct answers in `pick-one` and `pick-any` directives are marked with `*`.
/// A `pick-any` question may have neutral options, which are marked with `?`.
/// Neutral options are always counted as correct, whether the student selected them or not.
/// Initially selected options may be set with `+`.
/// The initially selected options are pre-selected when the exercise is loaded.
/// The `+` character is written before `*` or `?` if they are combined.
const APLUS_PICK_ONE_CHOICE_PATTERN: &'static str =
    r"^(\s*)(?P<pre_selected>\+)?(?P<correct>\*)?(?P<enumerator>[a-zA-Z0-9])\.(?:[ ]+|$)";

/// Correct answers in `pick-one` and `pick-any` directives are marked with `*`.
/// A `pick-any` question may have neutral options, which are marked with `?`.
/// Neutral options are always counted as correct, whether the student selected them or not.
/// Initially selected options may be set with `+`.
/// The initially selected options are pre-selected when the exercise is loaded.
/// The `+` character is written before `*` or `?` if they are combined.
const APLUS_PICK_ANY_CHOICE_PATTERN: &'static str = r"^(\s*)(?P<pre_selected>\+)?(?:(?P<neutral>\?)|(?P<correct>\*))?(?P<enumerator>[a-zA-Z0-9])\.(?:[ ]+|$)";

const APLUS_FREETEXT_PATTERN: &'static str = r"^(\s*)\.\.[ ]+freetext::(?:[ ]+|$)";
