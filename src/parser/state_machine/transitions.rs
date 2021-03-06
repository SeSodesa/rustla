/*!
This Module contains lists of transition tuples related to different states and common patterns as &str constants.

Copyright © 2020 Santtu Söderholm
*/

use super::*;
use crate::parser::regex_patterns;

/// =================================
/// StateMachine associated constants
/// =================================
impl State {

    /// An array of transitions related to `State::Body`.
    pub const BODY_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            Pattern::EmptyLine,
            regex_patterns::BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (Pattern::Bullet, regex_patterns::BULLET_PATTERN, body::bullet),
        (
            Pattern::Enumerator,
            regex_patterns::ENUMERATOR_PATTERN,
            body::enumerator,
        ),
        (
            Pattern::FieldMarker,
            regex_patterns::FIELD_MARKER_PATTERN,
            body::field_marker,
        ),
        (
            Pattern::Footnote,
            regex_patterns::FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            Pattern::Citation,
            regex_patterns::CITATION_PATTERN,
            body::citation
        ),
        (
            Pattern::HyperlinkTarget,
            regex_patterns::HYPERLINK_TARGET_PATTERN,
            body::hyperlink_target,
        ),
        (
            Pattern::Directive,
            regex_patterns::DIRECTIVE_PATTERN,
            body::directive
        ),
        (
            Pattern::Comment,
            regex_patterns::COMMENT_PATTERN,
            body::comment
        ),
        (
            Pattern::Line,
            regex_patterns::LINE_PATTERN,
            body::line),
        (
            Pattern::Text,
            regex_patterns::TEXT_PATTERN,
            body::text
        ),
    ];

    /// An array of transitions related to `State::Body`.
    pub const BLOCK_QUOTE_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            Pattern::EmptyLine,
            regex_patterns::BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            Pattern::Attribution,
            regex_patterns::ATTRIBUTION_PATTERN,
            block_quote::attribution,
        ),
        (
            Pattern::Bullet,
            regex_patterns::BULLET_PATTERN,
            body::bullet
        ),
        (
            Pattern::Enumerator,
            regex_patterns::ENUMERATOR_PATTERN,
            body::enumerator,
        ),
        (
            Pattern::FieldMarker,
            regex_patterns::FIELD_MARKER_PATTERN,
            body::field_marker,
        ),
        (
            Pattern::Footnote,
            regex_patterns::FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            Pattern::Citation,
            regex_patterns::CITATION_PATTERN,
            body::citation
        ),
        (
            Pattern::HyperlinkTarget,
            regex_patterns::HYPERLINK_TARGET_PATTERN,
            body::hyperlink_target,
        ),
        (
            Pattern::Directive,
            regex_patterns::DIRECTIVE_PATTERN,
            body::directive
        ),
        (
            Pattern::Comment,
            regex_patterns::COMMENT_PATTERN,
            body::comment
        ),
        (
            Pattern::Line,
            regex_patterns::LINE_PATTERN,
            body::line
        ),
        (
            Pattern::Text,
            regex_patterns::TEXT_PATTERN,
            body::text
        ),
    ];

    /// An array of transitions related to `State::BulletList`.
    pub const BULLET_LIST_TRANSITIONS: [UncompiledTransition; 2] = [
        (
            Pattern::EmptyLine,
            regex_patterns::BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            Pattern::Bullet,
            regex_patterns::BULLET_PATTERN,
            bullet_list::bullet
        ),
    ];

    /// An array of transitions related to `State::DefinitionList`.
    pub const DEFINITION_LIST_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            Pattern::EmptyLine,
            regex_patterns::BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            Pattern::Bullet,
            regex_patterns::BULLET_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            Pattern::Enumerator,
            regex_patterns::ENUMERATOR_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            Pattern::FieldMarker,
            regex_patterns::FIELD_MARKER_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            Pattern::Footnote,
            regex_patterns::FOOTNOTE_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            Pattern::Citation,
            regex_patterns::CITATION_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            Pattern::HyperlinkTarget,
            regex_patterns::HYPERLINK_TARGET_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            Pattern::Directive,
            regex_patterns::DIRECTIVE_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            Pattern::Comment,
            regex_patterns::COMMENT_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            Pattern::Line,
            regex_patterns::LINE_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            Pattern::Text,
            regex_patterns::TEXT_PATTERN,
            definition_list::text
        ),
    ];

    /// An array of transitions related to `State::EnumeratedList`.
    pub const ENUMERATED_LIST_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            Pattern::EmptyLine,
            regex_patterns::BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            Pattern::Enumerator,
            regex_patterns::ENUMERATOR_PATTERN,
            enumerated_list::enumerator,
        ),
    ];

    /// An array of transitions related to `State::FieldList`.
    pub const FIELD_LIST_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            Pattern::EmptyLine,
            regex_patterns::BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            Pattern::FieldMarker,
            regex_patterns::FIELD_MARKER_PATTERN,
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
            Pattern::EmptyLine,
            regex_patterns::BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            Pattern::QuotedLiteralBlock,
            regex_patterns::QUOTED_LITERAL_BLOCK_PATTERN,
            literal_block::literal_block,
        ),
        (
            Pattern::IndentedLiteralBlock,
            regex_patterns::INDENTED_LITERAL_BLOCK_PATTERN,
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
            Pattern::EmptyLine,
            regex_patterns::BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            Pattern::Bullet,
            regex_patterns::BULLET_PATTERN,
            body::bullet
        ),
    ];

    /// An array of transitions allowed in multi-column A+ directives such as points of interest.
    /// These are indentical to those, except the state also recognizes
    pub const APLUS_MULTICOL_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            Pattern::EmptyLine,
            regex_patterns::BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            Pattern::AplusColBreak,
            regex_patterns::APLUS_COL_BREAK_PATTERN,
            aplus::aplus_col_break,
        ),
        (
            Pattern::Bullet,
            regex_patterns::BULLET_PATTERN,
            body::bullet
        ),
        (
            Pattern::Enumerator,
            regex_patterns::ENUMERATOR_PATTERN,
            body::enumerator,
        ),
        (
            Pattern::FieldMarker,
            regex_patterns::FIELD_MARKER_PATTERN,
            body::field_marker,
        ),
        (
            Pattern::Footnote,
            regex_patterns::FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            Pattern::Citation,
            regex_patterns::CITATION_PATTERN,
            body::citation
        ),
        (
            Pattern::HyperlinkTarget,
            regex_patterns::HYPERLINK_TARGET_PATTERN,
            body::hyperlink_target,
        ),
        (
            Pattern::Directive,
            regex_patterns::DIRECTIVE_PATTERN,
            body::directive
        ),
        (
            Pattern::Comment,
            regex_patterns::COMMENT_PATTERN,
            body::comment
        ),
        (
            Pattern::Line,
            regex_patterns::LINE_PATTERN,
            body::line
        ),
        (
            Pattern::Text,
            regex_patterns::TEXT_PATTERN,
            body::text
        ),
    ];

    pub const APLUS_QUESTIONNAIRE_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            Pattern::EmptyLine,
            regex_patterns::BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            Pattern::AplusQuestionnaireDirective,
            regex_patterns::APLUS_QUESTIONNAIRE_DIRECTIVE_PATTERN,
            aplus_questionnaire::parse_aplus_questionnaire_directive,
        ),
        (
            Pattern::Text,
            regex_patterns::TEXT_PATTERN,
            aplus_questionnaire::parse_aplus_questionnaire_text,
        ),
    ];

    /// An array of inline transitions.
    pub const INLINE_TRANSITIONS: [InlineTransition; 13] = [
        (
            Pattern::WhiteSpace,
            regex_patterns::INLINE_WHITESPACE_PATTERN,
            inline::whitespace),
        (
            Pattern::StrongEmphasis,
            regex_patterns::STRONG_EMPH_PATTERN,
            inline::paired_delimiter,
        ),
        (
            Pattern::Emphasis,
            regex_patterns::EMPH_PATTERN,
            inline::paired_delimiter,
        ),
        (
            Pattern::Literal,
            regex_patterns::LITERAL_PATTERN,
            inline::paired_delimiter,
        ),
        (
            Pattern::InlineTarget,
            regex_patterns::INLINE_TARGET_PATTERN,
            inline::inline_target,
        ),
        (
            Pattern::PhraseRef,
            regex_patterns::PHRASE_REF_PATTERN,
            inline::phrase_ref,
        ),
        (
            Pattern::Interpreted,
            regex_patterns::INTERPRETED_TEXT_PATTERN,
            inline::interpreted_text,
        ),
        (
            Pattern::FootNoteRef,
            regex_patterns::FOOTNOTE_REF_PATTERN,
            inline::footnote_ref,
        ),
        (
            Pattern::CitationRef,
            regex_patterns::CITATION_REF_PATTERN,
            inline::citation_ref,
        ),
        (
            Pattern::SimpleRef,
            regex_patterns::SIMPLE_REF_PATTERN,
            inline::simple_ref,
        ),
        (
            Pattern::SubstitutionRef,
            regex_patterns::SUBSTITUTION_REF_PATTERN,
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
        (
            Pattern::StandaloneHyperlink,
            regex_patterns::URI_PATTERN,
            inline::uri
        ),
        (
            Pattern::Text,
            regex_patterns::INLINE_TEXT_PATTERN,
            inline::text
        ),
    ];
}
