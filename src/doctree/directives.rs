/*!
A submodule that contains an enumeration for the different directive types recognized by reStructuredText
and associated functions and metods. The documentation found in the comments in this file was taken from
https://docutils.sourceforge.io/docs/ref/rst/directives.html

This module is currently obsolete, and the only really relevant enumeration still
found in here is the `AdmonitionType` type.

Copyright © 2020 Santtu Söderholm <santtu.soderholm@tuni.fi>
*/
use super::*;

/// An enumeration of the different directive types found in reStructuredText and LarST.
#[derive(Debug)]
pub enum DirectiveNode {
    Admonition {
        content_indent: usize,
        classes: Option<String>,
        name: Option<String>,
        variant: AdmonitionType,
    },
    Image(ImageDirective),
    BodyElement(BodyElementDirective),
    Table(TableDirective),
    DocumentPart(DocumentPartDirective),
    Reference(ReferenceDirective),
    HTMLspecific(HTMLSpecificDirective),
    SubstitutionDef(SubstitutionDefDirective),
    Miscellaneous(MiscellaneousDirective),

    APlusrSTTools(AplusDirective),
}

/// An enumeration of the different admonition types.
/// Admonitions are specially marked "topics" that can
/// appear anywhere an ordinary body element can.
/// They contain arbitrary body elements. Typically,
/// an admonition is rendered as an offset block in a document,
/// sometimes outlined or shaded, with a title matching
/// the admonition type.
///
/// For details, see https://docutils.sourceforge.io/docs/ref/rst/directives.html#admonitions
#[derive(Debug, Clone)]
pub enum AdmonitionType {
    Attention,
    Caution,
    Danger,
    Error,
    Hint,
    Important,
    Note,
    Tip,
    Warning,
    Admonition { title: String },
}

use std::fmt;
impl std::fmt::Display for AdmonitionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let admonition_str = match self {
            Self::Attention => "attention".to_string(),
            Self::Caution => "caution".to_string(),
            Self::Danger => "danger".to_string(),
            Self::Error => "error".to_string(),
            Self::Hint => "hint".to_string(),
            Self::Important => "important".to_string(),
            Self::Note => "note".to_string(),
            Self::Tip => "tip".to_string(),
            Self::Warning => "warning".to_string(),
            Self::Admonition { title } => format!("admonition: {}", title),
        };
        write!(f, "{}", admonition_str)
    }
}

/// An enumeration of different image types.
/// There are two image directives: `image` and `figure`.
///
/// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#images
#[derive(Debug)]
pub enum ImageDirective {
    /// #### Image
    /// An "image" is a simple picture.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#image
    Image {
        /// #### uri
        /// A compulsory image location.
        uri: String,

        // Options
        alt: Option<String>,
        height: Option<String>,
        width: Option<String>,
        scale: Option<String>,
        align: Option<String>,
        target: Option<String>,
        name: Option<String>,
        class: Option<String>,
    },


    /// A "figure" consists of image data (including image options), an optional caption (a single paragraph), and an optional legend (arbitrary body elements). For page-based output media,
    /// figures might float to a different position if this helps the page layout.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#figure
    Figure {
        /// #### uri
        /// A compulsory image location.
        uri: String,

        // Options
        name: Option<String>,
        class: Option<String>,
        alt: Option<String>,
        height: Option<String>,
        width: Option<String>,
        scale: Option<String>,
        align: Option<String>,
        target: Option<String>,
        figwidth: Option<FigWidth>,
        figclass: Option<String>,
    },
}

/// Alternatives to `Figure` width settings.
#[derive(Debug)]
pub enum FigWidth {
    Image,
    Length(u32),
    Percentage(u32),
}

/// An enumeration of different body element directives.
#[derive(Debug)]
pub enum BodyElementDirective {

    /// A topic is like a block quote with a title, or a self-contained section with no subsections.
    /// Use the "topic" directive to indicate a self-contained idea that is separate from the flow of the document.
    /// Topics may occur anywhere a section or transition may occur. Body elements and topics may not contain nested topics.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#topic
    Topic {
        title: String,
        name: Option<String>,
        class: Option<String>,
    },


    /// Sidebars are like miniature, parallel documents that occur inside other documents, providing related or reference material.
    /// A sidebar is typically offset by a border and "floats" to the side of the page; the document's main text may flow around it. Sidebars can also be likened to super-footnotes;
    /// their content is outside of the flow of the document's main text.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#sidebar
    SideBar {
        title: Option<String>,
        name: Option<String>,
        class: Option<String>,
    },


    /// Unlike an ordinary literal block, the "parsed-literal" directive constructs a literal block
    /// where the text is parsed for inline markup. It is equivalent to a line block with different
    /// rendering: typically in a typewriter/monospaced typeface, like an ordinary literal block.
    /// Parsed literal blocks are useful for adding hyperlinks to code examples.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#parsed-literal-block
    ParsedLiteralBlock {
        inline_nodes: Vec<TreeNodeType>,
        name: Option<String>,
        class: Option<String>,
    },


    /// The "code" directive constructs a literal block.
    /// If the code language is specified, the content is parsed by the Pygments syntax highlighter
    /// and tokens are stored in nested inline elements with class arguments according to their syntactic category.
    /// The actual highlighting requires a style-sheet (e.g. one generated by Pygments, see the sandbox/stylesheets
    /// for examples).
    ///
    /// The parsing can be turned off with the syntax_highlight configuration setting and command line option or by
    /// specifying the language as :class: option instead of directive argument. This also avoids warnings when Pygments
    /// is not installed or the language is not in the supported languages and markup formats.
    ///
    /// For inline code, use the "code" role.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#code
    Code {
        language: Option<String>,
        name: Option<String>,
        class: Option<String>,
        number_lines: Option<u32>,
    },


    /// The "math" directive inserts blocks with mathematical content (display formulas, equations)
    /// into the document. The input format is subset of LaTeX math syntax with support for Unicode symbols.
    /// For inline formulas, use the "math" role.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#math
    Math {
        name: Option<String>,
        class: Option<String>,
    },


    /// The "rubric" directive inserts a "rubric" element into the document tree. A rubric is like an informal
    /// heading that doesn't correspond to the document's structure.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#rubric
    Rubric {
        name: Option<String>,
        class: Option<String>,
    },


    /// An epigraph is an apposite (suitable, apt, or pertinent) short inscription, often a quotation or poem,
    /// at the beginning of a document or section.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#epigraph
    Epigraph,


    /// Highlights summarize the main points of a document or section, often consisting of a list.
    ///
    /// The "highlights" directive produces a "highlights"-class block quote.
    /// See Epigraph above for an analogous example.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#highlights
    Highlights,


    /// A pull-quote is a small selection of text "pulled out and quoted", typically in a larger typeface.
    /// Pull-quotes are used to attract attention, especially in long articles.
    ///
    /// The "pull-quote" directive produces a "pull-quote"-class block quote.
    /// See Epigraph above for an analogous example.
    ///
    /// Details https://docutils.sourceforge.io/docs/ref/rst/directives.html#pull-quote
    PullQuote,


    /// The "compound" directive is used to create a compound paragraph,
    /// which is a single logical paragraph containing multiple physical body elements
    /// such as simple paragraphs,literal blocks, tables, lists, etc.,
    /// instead of directly containing text and inline elements. For example:
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#compound-paragraph
    CompoundParagraph {
        name: Option<String>,
        class: Option<String>,
    },


    /// The "container" directive surrounds its contents (arbitrary body elements) with a generic block-level "container" element.
    /// Combined with the optional "classes" attribute argument(s), this is an extension mechanism for users & applications.
    /// The "container" directive is the equivalent of HTML's <div> element. It may be used to group a sequence of elements for user-
    /// or application-specific purposes.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#container
    Container {
        class_names: Option<Vec<String>>,
        name: Option<String>,
    },
}

pub enum TableWidth {
    Length(u32),
    Percentage(u32),
}


/// An enumeration of different table directive types.
///
/// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#tables
#[derive(Debug)]
pub enum TableDirective {

    /// The "table" directive is used to associate a title with a table or specify options.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#table
    Table {
        name: Option<String>,
        class: Option<String>,
        align: Option<HorizontalAlignment>,
        widths: Option<Vec<usize>>,
        width: Option<Length>,
    },

    /// The "csv-table" directive is used to create a table from CSV (comma-separated values) data. CSV is
    /// a common data format generated by spreadsheet applications and commercial databases.
    /// The data may be internal (an integral part of the document) or external (a separate file).
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#id4
    CSVTable {
        name: Option<String>,
        class: Option<String>,
        widths: Option<TableColWidths>,
        width: Option<MetricType>,
        header_rows: Option<u32>,
        stub_columns: Option<u32>,
        header: Option<Vec<String>>,
        file: Option<String>,
        url: Option<String>,
        encoding: Option<String>,
        delim: Option<char>,
        quote: Option<char>,
        keepspace: Option<bool>,
        escape: Option<char>,
        align: Option<HorizontalAlignment>,
    },

    /// The "list-table" directive is used to create a table from data in a uniform two-level bullet list.
    /// "Uniform" means that each sublist (second-level list) must contain the same number of list items.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#list-table
    ListTable {
        widths: Option<TableColWidths>,
        width: Option<MetricType>,
        header_rows: Option<u32>,
        stub_columns: Option<u32>,
        align: Option<HorizontalAlignment>,
    },
}

/// An enumeration of different table directive types.
///
/// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#document-parts
#[derive(Debug)]
pub enum DocumentPartDirective {}

/// An enumeration of different reference directive types.
#[derive(Debug)]
pub enum ReferenceDirective {
    /// #### TargetFootnote
    /// The "target-notes" directive creates a footnote for each external target in the text,
    /// and corresponding footnote references after each reference. For every explicit target (of the form, .. _target name: URL) in the text,
    /// a footnote will be generated containing the visible URL as content.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#target-footnotes
    TargetFootnote {
        class: Option<String>,
        name: Option<String>,
    },

    /// #### Footnote
    /// Not implemented in docutils!
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#footnotes
    Footnote,

    /// #### Citation
    /// Not implemented in docutils!
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#citations
    Citation,
}

/// An enumeration of different HTML-specific directive types.
#[derive(Debug)]
pub enum HTMLSpecificDirective {

    /// The "meta" directive is used to specify HTML metadata stored in HTML META tags.
    /// "Metadata" is data about data, in this case data about web pages.
    /// Metadata is used to describe and classify web pages in the World Wide Web, in a form that is easy for search engines to extract and collate.
    ///
    /// Within the directive block, a flat field list provides the syntax for metadata.
    /// The field name becomes the contents of the "name" attribute of the META tag, and the field body
    /// (interpreted as a single string without inline markup) becomes the contents of the "content" attribute.
    ///
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#meta
    Meta,

    /// Not implemented in docutils.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#imagemap
    ImageMap,
}

/// An enumeration of different macro directive types.
///
/// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#directives-for-substitution-definitions
#[derive(Debug)]
pub enum SubstitutionDefDirective {

    /// The "replace" directive is used to indicate replacement text for a substitution reference. It may be used within substitution definitions only. For example, this directive can be used to expand abbreviations:
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#replacement-text
    ReplacementText,

    /// The "unicode" directive converts Unicode character codes (numerical values) to characters, and may be used in substitution definitions only.
    ///
    /// The arguments, separated by spaces, can be:
    ///
    /// * character codes as
    ///   * decimal numbers or
    ///   * hexadecimal numbers, prefixed by 0x, x, \x, U+, u, or \u or as XML-style hexadecimal character entities, e.g. &#x1a2b;
    /// * text, which is used as-is.
    ///
    /// Text following " .. " is a comment and is ignored.
    /// The spaces between the arguments are ignored and thus do not appear in the output.
    /// Hexadecimal codes are case-insensitive.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#unicode-character-codes
    UnicodeCharCode,

    /// The "date" directive generates the current local date and inserts it into the document as text.
    /// This directive may be used in substitution definitions only.
    ///
    /// The optional directive content is interpreted as the desired date format,
    /// using the same codes as Python's time.strftime function.
    /// The default format is "%Y-%m-%d" (ISO 8601 date), but time fields can also be used. Examples:
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#date
    Date,
}

/// An enumeration of different miscellaneous directive types.
///
/// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#miscellaneous
#[derive(Debug)]
pub enum MiscellaneousDirective {

    /// **!!!WARNING!!!** The "include" directive represents a potential security hole.
    /// It can be disabled with the "file_insertion_enabled" runtime setting.
    ///
    /// The "include" directive reads a text file.
    /// The directive argument is the path to the file to be included,
    /// relative to the document containing the directive.
    /// Unless the options literal or code are given,
    /// the file is parsed in the current document's context at the point of the directive.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#including-an-external-document-fragment
    Include {
        start_line: Option<usize>,
        end_line: Option<usize>,
        start_after: Option<String>,
        end_before: Option<String>,
        literal: Option<bool>,
        code: Option<String>,
        number_lines: Option<String>,
        encoding: Option<String>,
        tab_width: Option<usize>,
        class: Option<String>,
        name: Option<String>,
    },

    /// **Warning** The "raw" directive represents a potential security hole.
    /// It can be disabled with the "raw_enabled" or "file_insertion_enabled" runtime settings.
    ///
    /// **Caution** The "raw" directive is a stop-gap measure allowing the author to bypass reStructuredText's markup. It is a "power-user" feature that should not be overused or abused.
    /// The use of "raw" ties documents to specific output formats and makes them less portable.
    ///
    /// If you often need to use the "raw" directive or a "raw"-derived interpreted text role, that is a sign either of overuse/abuse or that functionality may be missing from reStructuredText.
    /// Please describe your situation in a message to the Docutils-users mailing list.
    ///
    /// ##### The Directive
    /// The "raw" directive indicates non-reStructuredText data that is to be passed untouched to the Writer. The names of the output formats are given in the directive arguments.
    /// The interpretation of the raw data is up to the Writer. A Writer may ignore any raw output not matching its format.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#raw-data-pass-through
    RawDataPassthrough {
        file: Option<String>,
        url: Option<String>,
        encoding: Option<String>,
    },

    /// The "class" directive sets the "classes" attribute value on its content or on the first immediately following
    /// non-comment element. The directive argument consists of one or more space-separated class names.
    /// The names are transformed to conform to the regular expression [a-z](-?[a-z0-9]+)* (see Identifier Normalization below).
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#class
    Class { class_names: Option<Vec<String>> },

    /// The "role" directive dynamically creates a custom interpreted text
    /// role and registers it with the parser.
    /// The role must be declared in a document before it can be used.
    ///
    /// The new role may be based on an existing role,
    /// specified as a second argument in parentheses (whitespace optional).
    ///
    /// A special case is the "raw" role:
    /// derived roles enable inline raw data pass-through.
    /// If no base role is explicitly specified,
    /// a generic custom role is automatically used.
    /// Subsequent interpreted text will produce an "inline" element with
    /// a "classes" attribute, as in the first example above.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#custom-interpreted-text-roles
    CustomInterpretedTextRole,

    /// The "default-role" directive sets the default interpreted text role,
    /// the role that is used for interpreted text without an explicit role.
    /// The "default-role" directive sets the default interpreted text role,
    /// the role that is used for interpreted text without an explicit role.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#setting-the-default-interpreted-text-role
    DefaultRole,

    /// The "title" directive specifies the document title as metadata,
    /// which does not become part of the document body.
    /// It overrides a document-supplied title.
    /// For example, in HTML output the metadata document title appears
    /// in the title bar of the browser window.
    ///
    /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#metadata-document-title
    MetadataDocTitle,

    /// This directive is provided for test purposes only.
    /// (Nobody is expected to type in a name that long!)
    /// It is converted into a level-1 (info) system message showing the directive data,
    /// possibly followed by a literal block containing the rest of the directive block.
    ReStructuredTextTestDirective,
}

/// Directives defined in the A+ rST Tools submodule of the A+ LMS.
///
/// Details: https://github.com/apluslms/a-plus-rst-tools
#[derive(Debug)]
pub enum AplusDirective {

    /// The questionnaire directive arguments define the exercise key and optional max points with the difficulty.
    /// For example, .. questionnaire:: 1 A50 sets key 1, max points 50 and difficulty A.
    /// If not set in the directive arguments, the max points will be set to the sum of the question points.
    /// Setting the difficulty is optional and it can be set even if the max points aren't defined in the argument.
    ///
    /// Details: https://github.com/apluslms/a-plus-rst-tools#1-graded-questionnaire
    GradedQuestionnaire,

    /// A feedback questionnaire is almost like a graded questionnaire. When the feedback option is set,
    /// the questionnaire uses the feedback category and CSS class by default.
    /// Feedback questionnaires always grant full points if all of the required questions are answered.
    ///
    /// The questionnaire options chapter-feedback, weekly-feedback, appendix-feedback,
    /// and course-feedback use a different CSS class (with the same name as the option).
    /// If points are not specified, they are set to zero.
    /// The feedback option can be set only to one questionnaire in an RST file because the exercise key is then hardcoded to feedback.
    ///
    /// Details: https://github.com/apluslms/a-plus-rst-tools#2-feedback-questionnaire
    FeedbackQuestionnaire,

    /// These types of exercises are configured separately for the MOOC grader by linking a YAML configuration file with the config option.
    /// Some settings may also be defined directly with the directive options.
    /// The directive will attach the exercise at this position in the content chapter.
    ///
    /// Its arguments define the exercise key and max points with the optional difficulty.
    /// The instructions can be written in the body of the submit directive.
    /// The body supports RST syntax. If the instructions field is also given in the config.yaml,
    /// the body of the submit directive will be prioritized.
    ///
    /// Details: https://github.com/apluslms/a-plus-rst-tools#3-submittable-exercise
    SubmittableExercise,

    /// This exercise opens an external tool via the LTI launch protocol.
    /// The LTI service must be configured beforehand in A+ by an administrator.
    /// The lti option refers to the label of the LTI service.
    /// The url option may exclude the domain of the service URL since the domain
    /// must be equal to the URL defined in the LTI service anyway.
    ///
    /// Details: https://github.com/apluslms/a-plus-rst-tools#4-external-exercise-lti
    LTIExercise,

    /// The meta directive is used to define module (exercise round) settings. It should be defined in the RST
    /// file that defines the toctree of the module (module index). Furthermore, it may be used in chapters to hide them (i.e.,
    /// set status hidden) with the hidden option or to set the chapter audience with the audience option.
    ///
    /// Details: Details: https://github.com/apluslms/a-plus-rst-tools#5-meta-exercise-round-settings
    RoundSettings,

    /// This creates an input field for active element.
    ///
    /// Details: https://github.com/apluslms/a-plus-rst-tools#6-active-element-input
    ActiveElementInput,

    /// This creates an output field for active element.
    ///
    /// More active element examples can be found at
    /// https://version.aalto.fi/gitlab/piitulr1/active-element-example
    ///
    /// Details: https://github.com/apluslms/a-plus-rst-tools#7-active-element-output
    ActiveElementOutput,

    /// Directive for creating hidden content blocks. The content can be shown/hidden by clicking the link.
    /// (This uses the Bootstrap collapse component.)
    ///
    /// Details: https://github.com/apluslms/a-plus-rst-tools#8-hidden-block
    HiddenBlock,

    /// Directive for creating a "point of interest" summary block.
    /// This extension must be activated separately in the project conf.py (extensions = ["aplus_setup", "point_of_interest"]).
    /// A point of interest is mostly like a normal admonition ("coloured info box"), but they are also linked to each other with next/previous links.
    /// The links enable the user to quickly navigate between the points of interest.
    ///
    /// Point of interests may also be used to generate separate lecture slides (not directly included in the A+ content chapters).
    /// This requires a separate tool called "presentation maker".
    ///
    /// Details: https://github.com/apluslms/a-plus-rst-tools#9-point-of-interest
    PointOfInterest,

    /// Code blocks may be annotated with comments for specific lines. This extension must be activated separately in the project conf.py
    /// (extensions = ["aplus_setup", "annotated"]).
    /// This extension requires custom JavaScript code and CSS styles in order to highlight the annotations on mouse hover in the web browser.
    /// The frontend code is not distributed in this repository (or anywhere).
    ///
    /// Details: https://github.com/apluslms/a-plus-rst-tools#10-annotated-code-blocks
    AnnotatedCodeBlock,

    /// With the `lineref-code-block`, you may add links from the chapter contents to specific lines of the code block.
    /// You define labels enclosed in :: for lines of the code block. Labels can include alphanumeric characters, underscore (_),
    /// and hyphen (-). The directive is used similarly to the Sphinx directive code-block. This extension must be activated
    /// separately in the project conf.py (extensions = ["aplus_setup", "codeblock_lineref"]).
    ///
    /// Details: https://github.com/apluslms/a-plus-rst-tools#11-code-blocks-with-line-references
    CodeBlockWithLineReference,

    /// The repl directive is used to print a (Scala) REPL session (read-eval-print loop).
    /// This extension must be activated separately in the project conf.py (extensions = ["aplus_setup", "repl"]).
    ///
    /// Details: https://github.com/apluslms/a-plus-rst-tools#12-repl-sessions
    REPLSession,

    /// The custom directive acos-submit behaves almost identically to the normal submit directive.
    /// It is intended for exercises that are hosted outside the MOOC grader, such as the ACOS server.
    /// The directive option url should define the URL path of the exercise in the ACOS server.
    /// The URL domain is added automatically based on the configuration value acos_submit_base_url in conf.py.
    /// The acos-submit directive also automatically uses the ajax flag of the submit directive.
    ///
    /// Details: https://github.com/apluslms/a-plus-rst-tools#13-submittable-acos-exercises
    SubmittableACOSExercise,

    /// The div directive can be used to insert basic <div> html elements into the generated document.
    /// This is useful for styling and other similar reasons.
    ///
    /// Any arguments given to the directive will be added as classes to the resulting element.
    ///
    /// Details: https://github.com/apluslms/a-plus-rst-tools#14-html-div-elements
    HTMLDiv,

    /// Directive that inserts topic elements that are more friendly to css styling using the bootstrap framework.
    ///
    /// Details: https://github.com/apluslms/a-plus-rst-tools#15-css-styled-topics
    CSSStyledTopic,

    /// The media directives were developed basically for a single course and they may
    /// not be quite reusable for other usecases, but they are listed here anyway.
    /// This extension must be activated separately in the project conf.py
    /// (extensions = ["aplus_setup", "media"]).
    ///
    /// Details: https://github.com/apluslms/a-plus-rst-tools#16-media-directives
    Media,
}
