/*!
This Module contains lists of transition tuples related to different states and common patterns as &str constants.

(c) Santtu Söderholm <santtu.soderholm@tuni.fi>
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
            PatternName::EmptyLine,
            regex_patterns::BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (PatternName::Bullet, regex_patterns::BULLET_PATTERN, body::bullet),
        (
            PatternName::Enumerator,
            regex_patterns::ENUMERATOR_PATTERN,
            body::enumerator,
        ),
        (
            PatternName::FieldMarker,
            regex_patterns::FIELD_MARKER_PATTERN,
            body::field_marker,
        ),
        (
            PatternName::Footnote(FootnoteKind::Manual),
            regex_patterns::MANUAL_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Footnote(FootnoteKind::AutoNumbered),
            regex_patterns::AUTO_NUM_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Footnote(FootnoteKind::SimpleRefName),
            regex_patterns::SIMPLE_NAME_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Footnote(FootnoteKind::AutoSymbol),
            regex_patterns::AUTO_SYM_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Citation,
            regex_patterns::CITATION_PATTERN,
            body::citation
        ),
        (
            PatternName::HyperlinkTarget,
            regex_patterns::HYPERLINK_TARGET_PATTERN,
            body::hyperlink_target,
        ),
        (
            PatternName::Directive,
            regex_patterns::DIRECTIVE_PATTERN,
            body::directive
        ),
        (
            PatternName::Comment,
            regex_patterns::COMMENT_PATTERN,
            body::comment
        ),
        (
            PatternName::Line,
            regex_patterns::LINE_PATTERN,
            body::line),
        (
            PatternName::Text,
            regex_patterns::TEXT_PATTERN,
            body::text
        ),
    ];

    /// An array of transitions related to `State::Body`.
    pub const BLOCK_QUOTE_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            PatternName::EmptyLine,
            regex_patterns::BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            PatternName::Attribution,
            regex_patterns::ATTRIBUTION_PATTERN,
            block_quote::attribution,
        ),
        (
            PatternName::Bullet,
            regex_patterns::BULLET_PATTERN,
            body::bullet
        ),
        (
            PatternName::Enumerator,
            regex_patterns::ENUMERATOR_PATTERN,
            body::enumerator,
        ),
        (
            PatternName::FieldMarker,
            regex_patterns::FIELD_MARKER_PATTERN,
            body::field_marker,
        ),
        (
            PatternName::Footnote(FootnoteKind::Manual),
            regex_patterns::MANUAL_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Footnote(FootnoteKind::AutoNumbered),
            regex_patterns::AUTO_NUM_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Footnote(FootnoteKind::SimpleRefName),
            regex_patterns::SIMPLE_NAME_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Footnote(FootnoteKind::AutoSymbol),
            regex_patterns::AUTO_SYM_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Citation,
            regex_patterns::CITATION_PATTERN,
            body::citation
        ),
        (
            PatternName::HyperlinkTarget,
            regex_patterns::HYPERLINK_TARGET_PATTERN,
            body::hyperlink_target,
        ),
        (
            PatternName::Directive,
            regex_patterns::DIRECTIVE_PATTERN,
            body::directive
        ),
        (
            PatternName::Comment,
            regex_patterns::COMMENT_PATTERN,
            body::comment
        ),
        (
            PatternName::Line,
            regex_patterns::LINE_PATTERN,
            body::line
        ),
        (
            PatternName::Text,
            regex_patterns::TEXT_PATTERN,
            body::text
        ),
    ];

    /// An array of transitions related to `State::BulletList`.
    pub const BULLET_LIST_TRANSITIONS: [UncompiledTransition; 2] = [
        (
            PatternName::EmptyLine,
            regex_patterns::BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            PatternName::Bullet,
            regex_patterns::BULLET_PATTERN,
            bullet_list::bullet
        ),
    ];

    /// An array of transitions related to `State::DefinitionList`.
    pub const DEFINITION_LIST_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            PatternName::EmptyLine,
            regex_patterns::BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            PatternName::Bullet,
            regex_patterns::BULLET_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::Enumerator,
            regex_patterns::ENUMERATOR_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::FieldMarker,
            regex_patterns::FIELD_MARKER_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::Footnote(FootnoteKind::Manual),
            regex_patterns::MANUAL_FOOTNOTE_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::Footnote(FootnoteKind::AutoNumbered),
            regex_patterns::AUTO_NUM_FOOTNOTE_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::Footnote(FootnoteKind::SimpleRefName),
            regex_patterns::SIMPLE_NAME_FOOTNOTE_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::Footnote(FootnoteKind::AutoSymbol),
            regex_patterns::AUTO_SYM_FOOTNOTE_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::Citation,
            regex_patterns::CITATION_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::HyperlinkTarget,
            regex_patterns::HYPERLINK_TARGET_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::Directive,
            regex_patterns::DIRECTIVE_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::Comment,
            regex_patterns::COMMENT_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::Line,
            regex_patterns::LINE_PATTERN,
            unknown_transitions::back_up,
        ),
        (
            PatternName::Text,
            regex_patterns::TEXT_PATTERN,
            definition_list::text
        ),
    ];

    /// An array of transitions related to `State::EnumeratedList`.
    pub const ENUMERATED_LIST_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            PatternName::EmptyLine,
            regex_patterns::BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            PatternName::Enumerator,
            regex_patterns::ENUMERATOR_PATTERN,
            enumerated_list::enumerator,
        ),
    ];

    /// An array of transitions related to `State::FieldList`.
    pub const FIELD_LIST_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            PatternName::EmptyLine,
            regex_patterns::BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            PatternName::FieldMarker,
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
            PatternName::EmptyLine,
            regex_patterns::BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            PatternName::QuotedLiteralBlock,
            regex_patterns::QUOTED_LITERAL_BLOCK_PATTERN,
            literal_block::literal_block,
        ),
        (
            PatternName::IndentedLiteralBlock,
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
            PatternName::EmptyLine,
            regex_patterns::BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            PatternName::Bullet,
            regex_patterns::BULLET_PATTERN,
            body::bullet
        ),
    ];

    /// An array of transitions allowed in multi-column A+ directives such as points of interest.
    /// These are indentical to those, except the state also recognizes
    pub const APLUS_MULTICOL_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            PatternName::EmptyLine,
            regex_patterns::BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            PatternName::AplusColBreak,
            regex_patterns::APLUS_COL_BREAK_PATTERN,
            aplus::aplus_col_break,
        ),
        (
            PatternName::Bullet,
            regex_patterns::BULLET_PATTERN,
            body::bullet
        ),
        (
            PatternName::Enumerator,
            regex_patterns::ENUMERATOR_PATTERN,
            body::enumerator,
        ),
        (
            PatternName::FieldMarker,
            regex_patterns::FIELD_MARKER_PATTERN,
            body::field_marker,
        ),
        (
            PatternName::Footnote(FootnoteKind::Manual),
            regex_patterns::MANUAL_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Footnote(FootnoteKind::AutoNumbered),
            regex_patterns::AUTO_NUM_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Footnote(FootnoteKind::SimpleRefName),
            regex_patterns::SIMPLE_NAME_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Footnote(FootnoteKind::AutoSymbol),
            regex_patterns::AUTO_SYM_FOOTNOTE_PATTERN,
            body::footnote,
        ),
        (
            PatternName::Citation,
            regex_patterns::CITATION_PATTERN,
            body::citation
        ),
        (
            PatternName::HyperlinkTarget,
            regex_patterns::HYPERLINK_TARGET_PATTERN,
            body::hyperlink_target,
        ),
        (
            PatternName::Directive,
            regex_patterns::DIRECTIVE_PATTERN,
            body::directive
        ),
        (
            PatternName::Comment,
            regex_patterns::COMMENT_PATTERN,
            body::comment),
        (
            PatternName::Line,
            regex_patterns::LINE_PATTERN,
            body::line
        ),
        (
            PatternName::Text,
            regex_patterns::TEXT_PATTERN,
            body::text
        ),
    ];

    pub const APLUS_QUESTIONNAIRE_TRANSITIONS: &'static [UncompiledTransition] = &[
        (
            PatternName::EmptyLine,
            regex_patterns::BLANK_LINE_PATTERN,
            common::empty_line,
        ),
        (
            PatternName::AplusQuestionnaireDirective,
            regex_patterns::APLUS_QUESTIONNAIRE_DIRECTIVE_PATTERN,
            aplus_questionnaire::parse_aplus_questionnaire_directive,
        ),
        (
            PatternName::Text,
            regex_patterns::TEXT_PATTERN,
            aplus_questionnaire::parse_aplus_questionnaire_text,
        ),
    ];

    /// An array of inline transitions.
    pub const INLINE_TRANSITIONS: [InlineTransition; 12] = [
        (PatternName::WhiteSpace, r"^\s+", inline::whitespace),
        (
            PatternName::StrongEmphasis,
            regex_patterns::STRONG_EMPH_PATTERN,
            inline::paired_delimiter,
        ),
        (
            PatternName::Emphasis,
            regex_patterns::EMPH_PATTERN,
            inline::paired_delimiter,
        ),
        (
            PatternName::Literal,
            regex_patterns::LITERAL_PATTERN,
            inline::paired_delimiter,
        ),
        (
            PatternName::InlineTarget,
            regex_patterns::INLINE_TARGET_PATTERN,
            inline::inline_target,
        ),
        (
            PatternName::PhraseRef,
            regex_patterns::PHRASE_REF_PATTERN,
            inline::phrase_ref,
        ),
        (
            PatternName::Interpreted,
            regex_patterns::INTERPRETED_TEXT_PATTERN,
            inline::interpreted_text,
        ),
        (
            PatternName::FootNoteRef,
            regex_patterns::FOOTNOTE_REF_PATTERN,
            inline::footnote_ref,
        ),
        (
            PatternName::SimpleRef,
            regex_patterns::SIMPLE_REF_PATTERN,
            inline::simple_ref,
        ),
        (
            PatternName::SubstitutionRef,
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
            PatternName::StandaloneHyperlink,
            regex_patterns::URI_PATTERN,
            inline::uri
        ),
        //(PatternName::Text, r"^([^\\\n\[*`:_]+)(?:[^_][a-zA-Z0-9]+_)?", Inline::text),
        (
            PatternName::Text,
            regex_patterns::INLINE_TEXT_PATTERN,
            inline::text
        ),
    ];
}
