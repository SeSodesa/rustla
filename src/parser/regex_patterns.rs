/*!
This submodule contains different regex patterns that can be compiled into finite automata.
This is done lazily in the module `crate::parser::automata`.

Copyright © 2020 Santtu Söderholm
*/

#[derive(Copy, Clone, Debug)]
/// An enum of transition regex pattern names, both for body and inline level elements.
pub enum Pattern {
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
    Footnote,
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
    CitationRef,
    InlineTarget,    // Reference target in inline text: _`target label`
    SubstitutionRef, // Reference to substitution definition. Is replaced by the definition
    ImplicitURL,
    StandaloneHyperlink,
    WhiteSpace,

    // A+ specific
    AplusColBreak,
    AplusQuestionnaireDirective,
}

/// A pattern for matching attributions inside block quotes.
pub const ATTRIBUTION_PATTERN: &'static str = r"^(\s*)(?:--|---|—) *";

/// A pattern for matching blank lines, as in lines that contain nothing but whitespace.
pub const BLANK_LINE_PATTERN: &'static str = r"^\s*$";

/// A pattern for matching bullet list bullets.
pub const BULLET_PATTERN: &'static str = r"^(\s*)([+\-*\u{2022}\u{2023}\u{2043}])(?: +|$)";

/// A pattern for recognizing enumerators of enumerated lists.
pub const ENUMERATOR_PATTERN: &'static str = r#"^(?x)
    (?P<indent>
      \s*
    )
    (?:
      \(
        (?:
          (?P<arabic_parens>
            [0-9]+
          )
          |
          (?P<lower_roman_parens>
            [ivxlcdm]+
          )
          |
          (?P<upper_roman_parens>
            [IVXLCDM]+
          )
          |
          (?P<lower_alpha_parens>
            [a-z]
          )
          |
          (?P<upper_alpha_parens>
            [A-Z]
          )
          |
          (?P<auto_enumerator_parens>
            \#
          )
        )
      \)
      |
        (?:
          (?P<arabic_rparen>
            [0-9]+
          )
          |
          (?P<lower_roman_rparen>
            [ivxlcdm]+
          )
          |
          (?P<upper_roman_rparen>
            [IVXLCDM]+
          )
          |
          (?P<lower_alpha_rparen>
            [a-z]
          )
          |
          (?P<upper_alpha_rparen>
            [A-Z]
          )
          |
          (?P<auto_enumerator_rparen>
            \#
          )
        )
      \)
      |
        (?:
          (?P<arabic_period>
            [0-9]+
          )
          |
          (?P<lower_roman_period>
            [ivxlcdm]+
          )
          |
          (?P<upper_roman_period>
            [IVXLCDM]+
          )
          |
          (?P<lower_alpha_period>
            [a-z]
          )
          |
          (?P<upper_alpha_period>
            [A-Z]
          )
          |
          (?P<auto_enumerator_period>
            \#
          )
        )
        \.
    )
    (?P<after_marker>
      \ +|$
    )
"#;

/// A pattern that signifies the start of a field list, such as a bibliography.
/// Colons inside field names `:field name:` must be escaped if followed by whitespace,
/// as ": " signifies the end of a list marker.
pub const FIELD_MARKER_PATTERN: &'static str = r"^(\s*):([\S&&[^\\]]|\S.*?[\S&&[^\\]]):(?: +|$)";

/// A pattern for matching against an indented block of text when in `State::LiteralBlock`.
pub const INDENTED_LITERAL_BLOCK_PATTERN: &'static str = r"^(\s+)\S";

/// A pattern for matching against an "quoted" block of text when in `State::LiteralBlock`.
pub const QUOTED_LITERAL_BLOCK_PATTERN: &'static str =
    r#"^(\s*)(!|"|#|\$|%|&|'|\(|\)|\*|\+|,|-|\.|/|:|;|<|=|>|\?|@|\[|\\|\]|\^|_|`|\{|\||\}|~)"#;

/// A pattern that matches the tops of bottoms of grid tables.
pub const GRID_TABLE_TOP_AND_BOT_PATTERN: &'static str = r#"^(\s*)\+-[-+]+-\+ *$"#;

/// A pattern that matches the tops and bottoms of simple tables.
pub const SIMPLE_TABLE_TOP_PATTERN: &'static str = r#"^(\s*)=+( +=+)+ *$"#;

/// A pattern for matching against the bottoms of entire simple tables and their header rows.
pub const SIMPLE_TABLE_BOTTOM_PATTERN: &'static str = r#"^(\s*)=+( +=+)+ *$"#;

// ========================
// Explicit markup patterns
// ========================

/// A single pattern for recognizing footnotes.
pub const FOOTNOTE_PATTERN: &'static str = r#"^(?x)
    (?P<indent>
        \s*
    )
    \.\.\ +\[
        (?:
            (?P<manual>\d+)
            |(?P<autonumbered>\#)
            |\#(?P<simplename>[a-zA-Z][a-zA-Z0-9]+(?:[-+._:][a-zA-Z0-9]+)*)
            |(?P<autosymbol>\*)
        )
    \](?:\ +|$) # space after marker
"#;

/// A pattern for matching against manually numbered footnotes.
pub const MANUAL_FOOTNOTE_PATTERN: &'static str = r"^(\s*)\.\.[ ]+\[(\d+)\](?:[ ]+|$)";

/// A footnote pattern with the symbol '#' for a label.
/// This triggers automatic numbering for the footnote to be generated.
pub const AUTO_NUM_FOOTNOTE_PATTERN: &'static str = r"^(\s*)\.\.[ ]+\[(\#)\](?:[ ]+|$)";

/// Similar to `AUTO_NUM_FOONOTE_PATTERN`, except allows referencing the same footnote
/// multiple times, as there is a simple reference name pointing to the footnote.
pub const SIMPLE_NAME_FOOTNOTE_PATTERN: &'static str =
    r"^(\s*)\.\.[ ]+\[\#([a-zA-Z][a-zA-Z0-9]+(?:[-+._:][a-zA-Z0-9]+)*)\](?:[ ]+|$)";

/// Prompts the generation of symbolic footnotes, with automatic reference mark generation.
pub const AUTO_SYM_FOOTNOTE_PATTERN: &'static str = r"^(\s*)\.\.[ ]+\[(\*)\](?:[ ]+|$)";

/// A pattern for matching against citations.
/// Similar to `FOOTNOTE_PATTERN`, but only
/// recognizes simple reference names in labels.
pub const CITATION_PATTERN: &'static str =
    r"^(\s*)\.\.[ ]+\[([a-zA-Z][a-zA-Z0-9]*(?:[-+._:][a-zA-Z0-9]+)*)\](?:[ ]+|$)";

/// A pattern for matching hyperlink targets. A hyperlink target may either be labeled with a simple reference name or
/// with and underscore `_`, the latter of which signifies an anonymous link.
pub const HYPERLINK_TARGET_PATTERN: &'static str =
    r"^(\s*)\.\.[ ]+_([a-zA-Z0-9][a-zA-Z0-9 ]*(?:[-+._:][a-zA-Z0-9 ]+)*[a-zA-Z0-9]+|_):(?:[ ]+|$)";

/// A pattern for matching substitution definitions, a.k.a. macros.
pub const SUBSTITUTION_DEF_PATTERN: &'static str = r"^(\s*)\.\.[ ]+\|(\S|\S.*\S)\| ::(?:[ ]+|$)";

/// A pattern for matching directives. The directive label is used to determine the type of directive
/// inside a transition function. The label itself is a simple reference name (an identifier).
pub const DIRECTIVE_PATTERN: &'static str =
    r"^(\s*)\.\.[ ]+([a-zA-Z][a-zA-Z0-9]+(?:[-+._:][a-zA-Z0-9]+)*)[ ]?::(?:[ ]+|$)";

///
/// A pattern for recognizing comments, after no other explicit markup pattern has matched.
pub const COMMENT_PATTERN: &'static str = r"^(\s*)\.\.(?: +|$)";

/// A pattern for recognizing lines related to section titles and transitions.
pub const LINE_PATTERN: &'static str = r#"^(!+|"+|#+|\$+|%+|&+|'+|\(+|\)+|\*+|\++|,+|-+|\.+|/+|:+|;+|<+|=+|>+|\?+|@+|\[+|\\+|\]+|\^+|_+|`+|\{+|\|+|\}+|~+) *$"#;

/// A pattern for detecting any text, possibly beginning with whitespace.
/// This pattern should generally be tested against only after all other
/// possibilities have been eliminated.
pub const TEXT_PATTERN: &'static str = r"^(\s*)\S";

// =================
//  Inline patterns
// =================

pub const INLINE_WHITESPACE_PATTERN: &'static str = r"^\s+";

pub const STRONG_EMPH_PATTERN: &str = r#"(?x)^
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

pub const EMPH_PATTERN: &str = r#"(?x)^
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

pub const LITERAL_PATTERN: &str = r#"(?x)^
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

pub const INLINE_TARGET_PATTERN: &str = r#"(?x)^
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

pub const INTERPRETED_TEXT_PATTERN: &str = r#"(?x)^
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

pub const PHRASE_REF_PATTERN: &str = r#"(?x)^
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

pub const SIMPLE_REF_PATTERN: &str = r#"(?x)^
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

pub const FOOTNOTE_REF_PATTERN: &str = r#"(?x)^
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
        \#
    )
    |\#
    (?P<auto_number_label>
        [a-zA-Z](-?[a-zA-Z0-9]+)*
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

pub const CITATION_REF_PATTERN: &str = r#"(?x)^
  (?P<lookbehind>
    [-:/'"<(\[{\p{Ps}\p{Pi}\p{Pf}\p{Pd}\p{Po}\s&&[^\\*]]
  )?
  (?P<markup_start>
    \[
  )
  (?P<content>
    [a-zA-Z0-9][a-zA-Z0-9]*(?:[-_.]?[a-zA-Z0-9]+)*
  )
  (?P<markup_end>
    \]_
  )
  (?P<lookahead>
    \s|[-.,:;!?\\/'")\]}>\p{Pe}\p{Pi}\p{Pf}\p{Pd}\p{Po}&&[^*]]|$
  )
"#;

pub const SUBSTITUTION_REF_PATTERN: &str = r#"(?x)^
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

pub const URI_PATTERN: &str = r#"(?x)^
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
      /?[a-zA-Z0-9~_.]*(?:/[A-Za-z0-9~_.]+)*/?
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

/// A pattern for inerpreting inline text as is, without any markup.
pub const INLINE_TEXT_PATTERN: &'static str = r"^(\S+)";

// ======================
//  A+ specific patterns
// ======================

/// ### APLUS_COL_BREAK_PATTERN
/// A regex pattern relatex to detecting column breaks in multi-column
/// A+ directives, such as points of interest.
pub const APLUS_COL_BREAK_PATTERN: &str = r#"^(\s+)::newcol"#;

/// ### APLUS_PICK_ONE_PATTERN
pub const APLUS_QUESTIONNAIRE_DIRECTIVE_PATTERN: &'static str =
    r"^(\s*)\.\.[ ]+(pick-one|pick-any|freetext)::(?:[ ]+|$)";

// /// ### APLUS_PICK_ANY_PATTERN
// pub const APLUS_PICK_ANY_PATTERN: &'static str = r"^(\s*)\.\.[ ]+pick-any::(?:[ ]+|$)";

/// ### APLUS_PICK_ONE_CHOICE_PATTERN
/// Correct answers in `pick-one` and `pick-any` directives are marked with `*`.
/// A `pick-any` question may have neutral options, which are marked with `?`.
/// Neutral options are always counted as correct, whether the student selected them or not.
/// Initially selected options may be set with `+`.
/// The initially selected options are pre-selected when the exercise is loaded.
/// The `+` character is written before `*` or `?` if they are combined.
pub const APLUS_PICK_ONE_CHOICE_PATTERN: &'static str =
    r"^(\s*)(?P<pre_selected>\+)?(?P<correct>\*)?(?P<enumerator>[a-zA-Z0-9])\.(?:[ ]+|$)";

/// ### APLUS_PICK_ANY_CHOICE_PATTERN
/// Correct answers in `pick-one` and `pick-any` directives are marked with `*`.
/// A `pick-any` question may have neutral options, which are marked with `?`.
/// Neutral options are always counted as correct, whether the student selected them or not.
/// Initially selected options may be set with `+`.
/// The initially selected options are pre-selected when the exercise is loaded.
/// The `+` character is written before `*` or `?` if they are combined.
pub const APLUS_PICK_ANY_CHOICE_PATTERN: &'static str = r"^(\s*)(?P<pre_selected>\+)?(?:(?P<neutral>\?)|(?P<correct>\*))?(?P<enumerator>[a-zA-Z0-9])\.(?:[ ]+|$)";

// /// ### APLUS_FREETEXT_PATTERN
// pub const APLUS_FREETEXT_PATTERN: &'static str = r"^(\s*)\.\.[ ]+freetext::(?:[ ]+|$)";
