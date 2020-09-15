/// ## tree_node_types
/// 
/// A submodule that contains the different tree node types.
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;


/// ### TreeNodeType
/// An enumeration of the different possible document node types.
/// 
/// Some of the nodes listed here are redundant. This is because
/// reStructuredText documentation also [lists](https://docutils.sourceforge.io/docs/ref/doctree.html#element-reference)
/// nodes that (multiply) inherit from other nodes in the [implementation](https://sourceforge.net/p/docutils/code/HEAD/tree/trunk/docutils/docutils/nodes.py),
/// but Rust has no concept of inheritance built in.
#[derive(Debug)]
pub enum TreeNodeType {

  /// #### Abbreviation
  /// 
  /// The abbreviation element is an inline element used to represent an abbreviation being used in the document. An example of an abbreviation is 'St' being used instead of 'Street'.
  Abbreviation {
    names: Option<String>,
    classes: Option<String>
  },

  /// ### AbsoluteURI
  /// A reference to a web address.
  AbsoluteURI{
    text: String
  },

  /// #### Acronym
  /// TODO
  Acronym,

  /// #### Address
  /// The address element holds the surface mailing address information for the author (individual or group) of the document, or a third-party contact address. Its structure is identical to that of the literal_block element: whitespace is significant, especially newlines.
  Address,

  /// #### Admonition
  /// Covers all of the standard admonition types of reStructuredText:
  /// 
  /// 1. attention,
  /// 2. caution,
  /// 3. danger,
  /// 4. error,
  /// 5. hint,
  /// 6. important,
  /// 7. note,
  /// 8. tip and
  /// 9. warning,
  /// 10. admonition
  /// 
  /// All of these have the same fields, and vary only in how they will be displayed.
  /// The last variant "admonition" may also contain a title. as one of its fields.
  Admonition {
    content_indent: usize,
    classes: Option<String>,
    name: Option<String>,
    variant: AdmonitionDirective
  },

  /// #### Attribution
  /// An optional attribution of a `BlockQuote`.
  /// An attribution is a text block beginning with "--", "---", or a true em-dash,
  /// flush left within the block quote.
  /// If the attribution consists of multiple lines,
  /// the left edges of the second and subsequent lines must align.
  /// 
  /// If a `BlockQuote` contains an attribution,
  /// the following node may be a `BlockQuote as well,
  /// but not otherwise (as it will be interpreted as a part of the previous quote).
  Attribution {
    raw_text: String,
    // parsed_text: Vec<TreeNodeType>
  },
  
  /// #### Author
  Author,

  /// #### Authors
  Authors,

  /// #### AutomaticSectionNumbering
  /// The "sectnum" (or "section-numbering") directive automatically numbers sections and subsections in a document (if not disabled by the
  /// --no-section-numbering command line option or the sectnum_xform configuration setting).
  ///
  /// Section numbers are of the "multiple enumeration" form, where each level has a number, separated by periods. For example,
  /// the title of section 1, subsection 2, subsubsection 3 would have "1.2.3" prefixed.
  ///
  /// The "sectnum" directive does its work in two passes: the initial parse and a transform. During the initial parse, a "pending" element is
  /// generated which acts as a placeholder, storing any options internally.
  /// At a later stage in the processing, the "pending" element triggers a transform, which adds section numbers to titles. Section numbers are
  /// enclosed in a "generated" element, and titles have their "auto" attribute set to "1".
  ///
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#automatic-section-numbering
  AutomaticSectionNumbering {
    depth:  Option<u32>,
    prefix: Option<String>,
    suffix: Option<String>,
    start:  Option<u32>,
  },

  /// #### BlockQuote
  /// Text indented relative to previous text,
  /// without markup indicating the start of
  /// a list or other container nodes.
  /// A block quote may end with an `Attribution`,
  /// which allows placing multiple block quotes
  /// in a sequence.
  /// 
  /// Also generated by the `epigraph`, `highlights` and `pull-quote` directives.
  BlockQuote {
    body_indent: usize,
  },

  /// #### BulletList
  /// An unnumbered list node. These may only contain `BulletListItem` nodes
  /// or `EmptyLine`s as their direct children.
  BulletList {
    bullet: char,
    bullet_indent: usize,
    text_indent: usize
  },

  /// #### BulletListItem
  /// An unnumbered list item. Cna contain any `Body` level elements
  /// as its direct children.
  BulletListItem{
    bullet: char,
    bullet_indent: usize,
    text_indent: usize
  },

  /// #### Caption
  /// 
  /// A `Figure` consists of image data (including image options), an optional *caption* (a single paragraph),
  /// and an optional legend (arbitrary body elements).
  /// For page-based output media, figures might float to a different position if this helps the page layout.
  Caption {
    indent: usize,
  },

  /// #### Citation
  /// A generic citation target.
  Citation {
    body_indent: usize,
    label: String,
  },

  /// #### CitationReference
  /// A reference to a bibliographic citation.
  CitationReference {
    displayed_text: String,
    target_label: String
  },

  /// #### Classifier
  /// A classifier for a `DefinitionTerm` in a `DefinitionList`.
  /// Could be the type of a varible in a function decraration, or something similar.
  Classifier,

  /// #### Code
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

    text: String,

    language:     Option<String>,
    name:         Option<String>,
    class:        Option<String>,
    number_lines: Option<String>
  },

  /// #### ColSpec
  ColSpec,

  /// #### Comment
  Comment {
    text: Option<String>
  },

  /// #### CompoundParagraph
  /// The "compound" directive is used to create a compound paragraph,
  /// which is a single logical paragraph containing multiple physical body elements
  /// such as simple paragraphs,literal blocks, tables, lists, etc.,
  /// instead of directly containing text and inline elements. For example:
  ///
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#compound-paragraph
  CompoundParagraph {

    body_indent: usize,

    name:   Option<String>,
    class:  Option<String>,
  },

  /// #### Contact
  Contact,

  /// #### Container
  /// The "container" directive surrounds its contents (arbitrary body elements) with a generic block-level "container" element.
  /// Combined with the optional "classes" attribute argument(s), this is an extension mechanism for users & applications.
  /// The "container" directive is the equivalent of HTML's <div> element. It may be used to group a sequence of elements for user-
  /// or application-specific purposes.
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#container
  Container {

    body_indent: usize,

    class_names: Option<Vec<String>>,
    name:   Option<String>,
  },

  /// #### Copyright
  Copyright,

  /// #### CSVTable
  /// The "csv-table" directive is used to create a table from CSV (comma-separated values) data. CSV is
  /// a common data format generated by spreadsheet applications and commercial databases.
  /// The data may be internal (an integral part of the document) or external (a separate file).
  ///
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#id4
  CSVTable {
    name:         Option<String>,
    class:        Option<String>,
    widths:       Option<TableColWidths>,
    width:        Option<MetricType>,
    header_rows:  Option<u32>,
    stub_columns: Option<u32>,
    header:       Option<Vec<String>>,
    file:         Option<String>,
    url:          Option<String>,
    encoding:     Option<String>,
    delim:        Option<char>,
    quote:        Option<char>,
    keepspace:    Option<bool>,
    escape:       Option<char>,
    align:        Option<HorizontalAlignment>
  },

  /// #### Date
  Date,

  /// #### Decoration
  Decoration,

  /// #### Definition
  Definition,


  /// #### DefinitionList
  /// A list of definitions. Contains `DefinitionListItems` or `EmptyLine` nodes
  /// as its direct children.
  DefinitionList {
    term_indent: usize,
  },

  /// #### DefinitionListItem
  /// A child node type of `DefinitionList`.
  /// Contains a map of `DefinitionTerm`s and the corresponding
  /// `TermDefinitions`, in addition to optional term classifiers.
  DefinitionListItem {
    term: String,
    classifiers: Vec<String>,
    body_indent: usize,
  },

  /// #### Description
  Description,

  /// #### DocInfo
  DocInfo,

  /// #### DoctestBlock
  /// These are interactive Python sessions contained in Python docstrings.
  /// Based on the Python standard library [doctest](http://www.python.org/doc/current/lib/module-doctest.html) module.
  /// 
  /// Doctest blocks begin with ">>>", the python REPL main prompt and end with a blank line.
  /// They are a special case of the literal block and if both are present,
  /// the literal block takes precedence.
  DoctestBlock,

  /// #### Document
  /// The root node of an reStructuredText document tree.
  Document,

  /// #### Emphasis
  /// Emphasised or italicized text.
  Emphasis {
    text: String
  },

  /// #### EmptyLine
  /// A simple empty line, that contains no actual data.
  /// These can be contained in pretty much any container
  /// node, such as lists or list items, in addition to
  /// existing between body level elements.
  EmptyLine,

  /// #### Entry
  Entry,

  /// #### EnumeratedList
  /// An enumerated list node. Can only contain `EnumeratedListItem` and `EmptyLine`
  /// nodes as its direct children.
  EnumeratedList {
    delims: EnumDelims,
    kind: EnumKind,
    start_index: usize,
    n_of_items: usize,
    enumerator_indent: usize,
  },

  /// #### EnumeratedListItem
  /// Child node type of `EnumeratedList`. Can contain any `Body`elements
  /// as its children.
  EnumeratedListItem {
    delims: EnumDelims,
    kind: EnumKind,
    index_in_list: usize,
    enumerator_indent: usize,
    text_indent: usize
  },

  /// #### ExternalHyperlinkTarget
  /// A target for an external hyperlink.
  /// Contains a URI pointing  to an external resource
  ExternalHyperlinkTarget {
    marker_indent: usize,
    target: String,
    uri: String,
  },

  /// #### Field
  Field,

  /// #### FieldBody
  /// The parameter that `FieldName` refers to. May contain arbitrary body elements,
  /// just like bulleted and enumerated list items. The first line after the marker specifies
  /// the indentation used as a reference for parsing the rest of the block.
  FieldBody {
    indentation: usize
  },

  /// #### FieldList
  /// A list of fields, that are used as a part of the
  /// reStructuredText extension syntax, such as directives.
  /// Bibliographies are a special case of these types of lists.
  FieldList {
    marker_indent: usize,
  },

  /// #### FieldListItem
  /// A field item of a `FieldList`. Consists of a marker with a field name and a
  /// field body consisting of arbitrary body elements.
  /// ```text
  /// +--------------------+----------------------+
  /// | ":" field name ":" | field body           |
  /// +-------+------------+                      |
  ///         | (body elements)+                  |
  ///         +-----------------------------------+
  /// ```
  FieldListItem {
    raw_marker_name: String,
    marker_name_as_inline_nodes: Vec<TreeNodeType>,
    marker_indent: usize,
    body_indent: usize,
  },

  /// #### Figure
  /// 
  /// A "figure" consists of image data (including image options),
  /// an optional caption (a single paragraph), and an optional legend (arbitrary body elements).
  /// For page-based output media, figures might float to a different position if this helps the page layout.
  Figure {

    /// ##### body_indent
    /// The indentation of the caption and legend of this `Figure`.
    body_indent: usize,

    // Options
    align:    Option<String>, // not passed to contained image
    figwidth: Option<String>,
    figclass: Option<String>
  },

  /// #### DocumentFooter
  /// The "header" and "footer" directives create document decorations, useful for page navigation, notes, time/datestamp, etc.
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#document-header-footer
  Footer {
    body_indent: usize,
  },

  /// #### Footnote
  /// A foonote citation target. Contains a label and the foornote text itself.
  /// 
  /// The "target-notes" directive creates a footnote for each external target in the text,
  /// and corresponding footnote references after each reference. For every explicit target (of the form, .. _target name: URL) in the text,
  /// a footnote will be generated containing the visible URL as content.
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#target-footnotes
  Footnote {
    body_indent: usize,
    kind: FootnoteKind,
    label: String, // Displayed label
    target: String // Reference target
  },

  /// #### FootnoteReference
  /// A reference to a foot note.
  FootnoteReference {
    displayed_text: String,
    target_label: String
  },

  /// #### Generated
  Generated,

  /// #### Header
  /// The "header" and "footer" directives create document decorations, useful for page navigation, notes, time/datestamp, etc.
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#document-header-footer
  Header {
    body_indent: usize,
  },

  /// #### Image
  Image {

    uri: String,

    // Options

    alt:    Option<String>,
    height: Option<String>,
    width:  Option<String>,
    scale:  Option<String>,
    align:  Option<String>,
    target: Option<String>,
    name:   Option<String>,
    class:  Option<String>,
  },

  /// #### IndirectHyperlinkTarget
  /// An indirect hyperlink target. Contains a hyperlink reference pointing
  /// to an internal or and external hyperlink.
  IndirectHyperlinkTarget {
    marker_indent: usize,
    target: String,
    indirect_target: String,
  },

  /// #### Inline
  Inline,

  /// #### InlineTarget
  /// An inline reference target.
  InlineTarget {
    target_label: String
  },

  /// #### InterpretedText
  /// Text, whose meaning depends entirely on the given `role`:
  /// (:role:`content`|`content`:role:). There are predefined roles
  /// such as `math` or `emphasis`, but others may be defined by applications.
  InterpretedText,

  /// #### Label
  Label,

  /// #### Legend
  /// 
  /// A `Figure` consists of image data (including image options), an optional caption (a single paragraph),
  /// and an optional *legend* (arbitrary body elements).
  /// For page-based output media, figures might float to a different position if this helps the page layout.
  Legend {
    body_indent: usize
  },

  /// #### Line
  /// A general line node. Might signify the start of a transtition or a section title.
  Line,

  /// #### LineBlock
  /// A block of text where each new line begins with an unindented '|',
  /// followed be text with specific left-alignment, used as a reference
  /// for the rest of the block.
  /// Allows writing blocks of text, where the struture of the lines
  /// is meaningful, such as poetry.
  /// 
  /// The symbols '|' may be omitted, as they signify the start of a new
  /// line in the rendered output.
  /// ```txt
  /// +------+-----------------------+
  /// | "| " | line                  |
  /// +------| continuation line     |
  ///        +-----------------------+
  /// ```
  LineBlock,

  /// #### ListTable
  /// The "list-table" directive is used to create a table from data in a uniform two-level bullet list.
  /// "Uniform" means that each sublist (second-level list) must contain the same number of list items.
  ///
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#list-table
  ListTable {
    widths:       Option<TableColWidths>,
    width:        Option<MetricType>,
    header_rows:  Option<u32>,
    stub_columns: Option<u32>,
    align:        Option<HorizontalAlignment>
  },

  /// #### Literal
  /// Literal text, usually reserved for code.
  Literal {
    text: String
  },

  /// #### LiteralBlock
  /// Paragraph (possibly empty) ending in a "::" signifies the start of a literal block of text.
  /// Text contained in a literal block is not interpreted in any way,
  /// but simply stored in this node as is.
  LiteralBlock {
    text: String
  },

  /// #### Math
  /// The "math" directive inserts blocks with mathematical content (display formulas, equations)
  /// into the document. The input format is subset of LaTeX math syntax with support for Unicode symbols.
  /// For inline formulas, use the "math" role.
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#math
  Math {
    text: String,
    name:   Option<String>,
    class:  Option<String>,
  },

  /// #### MathBlock
  /// A node for display-style mathematics (LaTeX).
  MathBlock {
    block_text: String,

    name: Option<String>,
    class: Option<String>,
  },

  /// #### OptionList
  /// A two-column list of command line options, such as the ones typically seen on unix `man` pages.
  /// Four types of options are supported:
  ///
  /// 1. short POSIX options with one '-' and an opion letter,
  /// 2. Long POSIX options with "--", followed by an option word.
  ///    Some systems might use a single dash.
  /// 3. Old GNU-style options starting with a '+', followed by an option letter (!!!deprecated!!!)
  /// 4. DOS/VMS options starting with a '/', followed by an option letter or a word.
  /// 
  /// The recognized syntax is based on Python's `getopt.py` module.
  OptionList {
    option_indent: usize,
  },

  /// #### OptionListItem
  /// A single option in an `OptionList`. Consists of an option,
  /// folllowed by and optional argument and a description.
  /// May contain arbitrary indented body elements after these:
  /// ```text
  /// +----------------------------+-------------+
  /// | option [" " argument] "  " | description |
  /// +-------+--------------------+             |
  ///         | (body elements)+                 |
  ///         +----------------------------------+
  /// ```
  OptionListItem {
    body_indent: usize,
  },

  /// #### OptionString
  OptionString,

  /// #### Organization
  Organization,

  /// #### Paragraph
  /// A node constructed of a left-aligned block of text
  /// with no special starter markers.
  Paragraph {
    indent: usize,
  },

  /// #### ParsedLiteralBlock
  ParsedLiteralBlock,

  /// #### Pending
  Pending,

  /// #### Problematic
  Problematic,

  /// #### Raw
  Raw,

  /// #### Reference
  /// A general reference to a reference target.
  Reference {
    displayed_text: String,
    target_label: String
  },

  /// #### Revision
  Revision,

  /// #### Row
  Row,

  /// #### Rubric
  /// The "rubric" directive inserts a "rubric" element into the document tree. A rubric is like an informal
  /// heading that doesn't correspond to the document's structure.
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#rubric
  Rubric {
    name:   Option<String>,
    class:  Option<String>,
  },

  
  /// #### Section
  /// A section title node, that contains the title text,
  /// in addition to its marker type and (sub)section level.
  Section {
    title_text: String,
    level: usize,
    line_style: SectionLineStyle
  },

  /// #### Sidebar
  /// 
  /// Sidebars are like miniature, parallel documents that occur inside other documents, providing related or reference material.
  /// A sidebar is typically offset by a border and "floats" to the side of the page; the document's main text may flow around it.
  /// Sidebars can also be likened to super-footnotes; their content is outside of the flow of the document's main text.
  ///
  /// The sidebar element is a nonrecursive section-like construct which may occur at the top level of a section wherever a body element
  /// (list, table, etc.) is allowed. In other words, sidebar elements cannot nest inside body elements, so you can't have a sidebar inside
  /// a table or a list, or inside another sidebar (or topic).
  Sidebar {
    body_indent: usize,
  },

  /// #### Status
  Status,

  /// #### StandaloneEmail
  /// A reference to an email address.
  StandaloneEmail{
    text: String
  },

  /// #### StrongEmphasis
  /// Strongly emphasised text, usually rendered in bold.
  StrongEmphasis {
    text:String
  },

  /// #### Subscript
  Subscript {
    text: String
  },

  /// #### SubstitutionDefinition
  /// Explicit markup node, as in begins with ".. " followed by a vertical bar '|',
  /// substitution text and another '|'. The text may not begin or end with whitespace.
  /// Substitution definition blocks may contain a nested, *inline compatible* directive
  /// *without* the leading ".. ", such as `image` or `replace`.
  /// ```text
  /// +-------+-----------------------------------------------------+
  /// | ".. " | "|" substitution text "| " directive type "::" data |
  /// +-------+ directive block                                     |
  ///         |                                                     |
  ///         +-----------------------------------------------------+
  /// ```
  SubstitutionDefinition,

  /// #### SubstitutionReference
  /// A reference that is to be substituted with the reference target directive output.
  SubstitutionReference {
    substitution_label: String,
    // displayed_text: Option<String>,
    target_label: Option<String>
  },

  /// #### Subtitle
  Subtitle,

  /// #### Superscript
  Superscript {
    text: String
  },

  /// SystemMessage
  SystemMessage,

  /// #### Table
  /// The "table" directive is used to associate a title with a table or specify options.
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#table
  Table {
    name:   Option<String>,
    class:  Option<String>,
    align:  Option<HorizontalAlignment>,
    widths: Option<Vec<usize>>,
    width:  Option<LenghtUnit>
  },

  /// #### Target
  Target,

  /// #### TBody
  TBody,

  /// #### Term
  Term,

  /// #### Text
  /// A plain text node, that contains no special markup.
  Text {
    text:String
  },

  /// #### TGroup
  TGroup,

  /// #### THead
  THead,

  /// #### Title
  Title,

  /// #### TitleReference
  /// A reference to a title.
  TitleReference {
    displayed_text: String,
    target_label: String
  },

  /// #### Topic
  /// The topic element is a nonrecursive section-like construct which may occur at the top level of a section wherever a body element (list, table, etc.) is allowed.
  /// In other words, topic elements cannot nest inside body elements, so you can't have a topic inside a table or a list, or inside another topic.
  /// Topics may contain only body elements.
  /// 
  /// The "contents" directive generates a table of contents (TOC) in a topic.
  /// Topics, and therefore tables of contents, may occur anywhere a section or transition may occur.
  /// Body elements and topics may not contain tables of contents.
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#table-of-contents
  Topic {
    title: String,
    depth:      Option<u32>,
    local:      Option<bool>,
    backlinks:  Option<ToCBacklinks>,
    class:      Option<String>
  },

  /// #### Transition
  /// A node corresponding to LaTeX's `\hrulefill` command.
  Transition,

  /// #### Version
  Version,

  /// #### Whitespace
  /// Generic whitespace that covers everything from spaces to newlines.
  WhiteSpace{
    text: String
  },
}

use std::collections::HashSet;
use lazy_static::lazy_static;
use crate::doctree::node_categories::{*};

impl TreeNodeType {

  /// ### body_indent
  /// 
  /// Returns the relevant content indent of each node type.
  /// This is useful, when one is trying to find out whether a node belongs to a parent node.
  /// Node containerzation is determined by indentation in reStructuredText.
  pub fn body_indent (&self) -> Option<usize> {
    match self {
      Self::Abbreviation { .. } => None,
      Self::AbsoluteURI { .. } => None,
      Self::Acronym { .. } => None,
      Self::Address => None,
      Self::Admonition { content_indent, .. } => Some(*content_indent),
      Self::Attribution { .. } => None,
      Self::Author { .. } => None,
      Self::Authors {..} => None,
      Self::AutomaticSectionNumbering {..} => None,
      Self::BlockQuote { body_indent, .. } => Some(*body_indent),
      Self::BulletList { bullet_indent, .. } => Some(*bullet_indent),
      Self::BulletListItem{ text_indent, .. } => Some(*text_indent),
      Self::Caption { .. } => None,
      Self::Citation { body_indent, .. } => Some(*body_indent),
      Self::CitationReference { .. } => None,
      Self::Classifier { .. } => None,
      Self::Code { .. } => None,
      Self::ColSpec { .. } => None,
      Self::Comment { .. } => None,
      Self::CompoundParagraph { body_indent, .. } => Some(*body_indent),
      Self::Contact { .. } => None,
      Self::Container { body_indent, .. } => Some(*body_indent),
      Self::Copyright { .. } => None,
      Self::CSVTable { .. } => None,
      Self::Date => None,
      Self::Decoration => None,
      Self::Definition => None,
      Self::DefinitionList { term_indent, .. } => Some(*term_indent),
      Self::DefinitionListItem { body_indent, .. } => Some(*body_indent),
      Self::Description => None,
      Self::DocInfo => None,
      Self::DoctestBlock{ .. } => None,
      Self::Document { .. }   => Some(0),
      Self::Emphasis { .. } => None,
      Self::EmptyLine => None,
      Self::Entry => None,
      Self::EnumeratedList { enumerator_indent, .. } => Some(*enumerator_indent),
      Self::EnumeratedListItem { text_indent, .. } => Some(*text_indent),
      Self::ExternalHyperlinkTarget { .. } => None,
      Self::Field => None,
      Self::FieldBody { .. } => None,
      Self::FieldList { marker_indent, .. } => Some(*marker_indent),
      Self::FieldListItem { body_indent, .. } => Some(*body_indent),
      Self::Figure { body_indent, .. } => Some(*body_indent),
      Self::Footer { body_indent, .. } => Some(*body_indent),
      Self::Footnote { body_indent, .. } => Some(*body_indent),
      Self::FootnoteReference { .. } => None,
      Self::Header { body_indent, .. } => Some(*body_indent),
      Self::Generated => None,
      Self::Image { .. } => None,
      Self::IndirectHyperlinkTarget { .. } => None,
      Self::Inline { .. } => None,
      Self::InlineTarget { .. } => None,
      Self::InterpretedText { .. } => None,
      Self::Label { .. } => None,
      Self::Legend { body_indent, .. } => Some(*body_indent),
      Self::Line { .. } => None,
      Self::LineBlock { .. } => None,
      Self::ListTable { .. } => None,
      Self::Literal { .. } => None,
      Self::LiteralBlock { .. } => None,
      Self::Math { .. } => None,
      Self::MathBlock { .. } => None,
      Self::OptionList { option_indent, .. } => Some(*option_indent),
      Self::OptionListItem { body_indent, .. } => Some(*body_indent),
      Self::OptionString { .. } => None,
      Self::Organization { .. } => None,
      Self::Paragraph { indent, .. } => Some(*indent),
      Self::ParsedLiteralBlock { .. } => None,
      Self::Pending { .. } => None,
      Self::Problematic { .. } => None,
      Self::Raw { .. } => None,
      Self::Reference { .. } => None,
      Self::Revision { .. } => None,
      Self::Row { .. } => None,
      Self::Rubric { .. } => None,
      Self::Section { .. }    => Some(0),
      Self::Sidebar { body_indent, .. } => Some(*body_indent),
      Self::Status { .. } => None,
      Self::StandaloneEmail { .. } => None,
      Self::StrongEmphasis { .. } => None,
      Self::Subscript { .. } => None,
      Self::SubstitutionDefinition { .. } => None,
      Self::SubstitutionReference { .. } => None,
      Self::Subtitle { .. } => None,
      Self::Superscript { .. } => None,
      Self::SystemMessage { .. } => None,
      Self::Table { .. } => None,
      Self::Target { .. } => None,
      Self::TBody { .. } => None,
      Self::Term { .. } => None,
      Self::Text { .. } => None,
      Self::TGroup { .. } => None,
      Self::THead { .. } => None,
      Self::Title { .. } => None,
      Self::TitleReference { .. } => None,
      Self::Topic { .. } => None,
      Self::Transition   => None,
      Self::Version { .. } => None,
      Self::WhiteSpace { .. } => None,
    }
  }

  /// ### node_categories
  /// According to the [reStructuredText DTD](https://docutils.sourceforge.io/docs/ref/doctree.html)
  /// each node type is associated with a set of categories.
  /// This function returns that set (an immutable iterator) for each defined node type.
  pub fn node_categories (&self) -> impl Iterator<Item=&NodeCategory> {

    let categories: &[NodeCategory] = match self {
      Self::Abbreviation { .. } => &ABBREVIATION_CATEGORIES,
      Self::AbsoluteURI { .. } => &ABSOLUTE_URI_CATEGORIES,
      Self::Acronym { .. } => &ACRONYM_CATEGORIES,
      Self::Address => &ADDRESS_CATEGORIES,
      Self::Admonition { .. } => &ADMONITION_CATEGORIES,
      Self::Attribution { .. } => &ATTRIBUTION_CATEGORIES,
      Self::Author { .. } => &AUTHOR_CATEGORIES,
      Self::Authors {..} => &AUTHORS_CATEGORIES,
      Self::AutomaticSectionNumbering {..} => &AUTO_SECTION_NUMBERING_CATEGORIES,
      Self::BlockQuote { .. } => &BLOCK_QUOTE_CATEGORIES,
      Self::BulletList { .. } => &BULLET_LIST_CATEGORIES,
      Self::BulletListItem{ .. } => &BULLET_LIST_ITEM_CATEGORIES,
      Self::Caption { .. } => &CAPTION_CATEGORIES,
      Self::Citation { .. } => &CITATION_CATEGORIES,
      Self::CitationReference { .. } => &CITATION_REFERENCE_CATEGORIES,
      Self::Classifier { .. } => &CLASSIFIER_CATEGORIES,
      Self::Code { .. } => &CODE_CATEGORIES,
      Self::ColSpec { .. } => &COLSPEC_CATEGORIES,
      Self::Comment { .. } => &COMMENT_CATEGORIES,
      Self::CompoundParagraph { .. } => &COMPOUND_PARAGRAPH_CATEGORIES,
      Self::Contact { .. } => &CONTACT_CATEGORIES,
      Self::Container { .. } => &CONTAINER_CATEGORIES,
      Self::Copyright { .. } => &COPYRIGHT_CATEGORIES,
      Self::CSVTable { .. } => &CSV_TABLE_CATEGORIES,
      Self::Date => &DATE_CATEGORIES,
      Self::Decoration => &DECORATION_CATEGORIES,
      Self::Definition => &DEFINITION_CATEGORIES,
      Self::DefinitionList { .. } => &DEFINITION_LIST_CATEGORIES,
      Self::DefinitionListItem { .. } => &DEFINITION_LIST_ITEM_CATEGORIES,
      Self::Description => &DESCRIPTION_CATEGORIES,
      Self::DocInfo => &DOC_INFO_CATEGORIES,
      Self::DoctestBlock{ .. } => &DOCTEST_BLOCK_CATEGORIES,
      Self::Document { .. }   => &DOCUMENT_CATEGORIES,
      Self::Emphasis { .. } => &EMPHASIS_CATEGORIES,
      Self::EmptyLine => &EMPTY_LINE_CATEGORIES,
      Self::Entry => &ENTRY_CATEGORIES,
      Self::EnumeratedList { .. } => &ENUMERATED_LIST_CATEGORIES,
      Self::EnumeratedListItem { .. } => &ENUMERATED_LIST_ITEM_CATEGORIES,
      Self::ExternalHyperlinkTarget { .. } => &EXTERNAL_HYPERLINK_TARGET_CATEGORIES,
      Self::Field => &FIELD_CATEGORIES,
      Self::FieldBody { .. } => &FIELD_BODY_CATEGORIES,
      Self::FieldList { .. } => &FIELD_LIST_CATEGORIES,
      Self::FieldListItem { .. } => &FIELD_LIST_ITEM_CATEGORIES,
      Self::Figure { .. } => &FIGURE_CATEGORIES,
      Self::Footer { .. } => &FOOTER_CATEGORIES,
      Self::Footnote { .. } => &FOOTNOTE_CATEGORIES,
      Self::FootnoteReference { .. } => &FOOTNOTE_REFERENCE_CATEGORIES,
      Self::Header { .. } => &HEADER_CATEGORIES,
      Self::Generated => &GENERATED_CATEGORIES,
      Self::Image { .. } => &IMAGE_CATEGORIES,
      Self::IndirectHyperlinkTarget { .. } => &INDIRECT_HYPERLINK_TARGET_CATEGORIES,
      Self::Inline { .. } => &INLINE_CATEGORIES,
      Self::InlineTarget { .. } => &INLINE_TARGET_CATEGORIES,
      Self::InterpretedText { .. } => &INTERPRETED_TEXT_CATEGORIES,
      Self::Label { .. } => &LABEL_CATEGORIES,
      Self::Legend { .. } => &LEGEND_CATEGORIES,
      Self::Line { .. } => &LINE_CATEGORIES,
      Self::LineBlock { .. } => &LINE_BLOCK_CATEGORIES,
      Self::ListTable { .. } => &LIST_TABLE_CATEGORIES,
      Self::Literal { .. } => &LITERAL_CATEGORIES,
      Self::LiteralBlock { .. } => &LITERAL_BLOCK_CATEGORIES,
      Self::Math { .. } => &MATH_CATEGORIES,
      Self::MathBlock { .. } => &MATH_BLOCK_CATEGORIES,
      Self::OptionList { .. } => &OPTION_LIST_CATEGORIES,
      Self::OptionListItem { .. } => &OPTION_LIST_ITEM_CATEGORIES,
      Self::OptionString { .. } => &OPTION_STRING_CATEGORIES,
      Self::Organization { .. } => &ORGANIZATION_CATEGORIES,
      Self::Paragraph { .. } => &PARAGRAPH_CATEGORIES,
      Self::ParsedLiteralBlock { .. } => &PARSED_LITERAL_BLOCK_CATEGORIES,
      Self::Pending { .. } => &PENDING_CATEGORIES,
      Self::Problematic { .. } => &PROBLEMATIC_CATEGORIES,
      Self::Raw { .. } => &RAW_CATEGORIES,
      Self::Reference { .. } => &REFERENCE_CATEGORIES,
      Self::Revision { .. } => &REVISION_CATEGORIES,
      Self::Row { .. } => &ROW_CATEGORIES,
      Self::Rubric { .. } => &RUBRIC_CATEGORIES,
      Self::Section { .. }    => &SECTION_CATEGORIES,
      Self::Sidebar { .. } => &SIDEBAR_CATEGORIES,
      Self::Status { .. } => &STATUS_CATEGORIES,
      Self::StandaloneEmail { .. } => &STANDALONE_EMAIL_CATEGORIES,
      Self::StrongEmphasis { .. } => &STRONG_EMPHASIS_CATEGORIES,
      Self::Subscript { .. } => &SUBSCRIPT_CATEGORIES,
      Self::SubstitutionDefinition { .. } => &SUBSTITUTION_DEF_CATEGORIES,
      Self::SubstitutionReference { .. } => &SUBSTITUTION_REF_CATEGORIES,
      Self::Subtitle { .. } => &SUBTITLE_CATEGORIES,
      Self::Superscript { .. } => &SUPERSCRIPT_CATEGORIES,
      Self::SystemMessage { .. } => &SYSTEM_MESSAGE_CATEGORIES,
      Self::Table { .. } => &TABLE_CATEGORIES,
      Self::Target { .. } => &TARGET_CATEGORIES,
      Self::TBody { .. } => &T_BODY_CATEGORIES,
      Self::Term { .. } => &TERM_CATEGORIES,
      Self::Text { .. } => &TEXT_CATEGORIES,
      Self::TGroup { .. } => &T_GROUP_CATEGORIES,
      Self::THead { .. } => &T_HEAD_CATEGORIES,
      Self::Title { .. } => &TITLE_CATEGORIES,
      Self::TitleReference { .. } => &TITLE_REF_CATEGORIES,
      Self::Topic { .. } => &TOPIC_CATEGORIES,
      Self::Transition {}     => &TRANSITION_CATEGORIES,
      Self::Version { .. } => &VERSION_CATEGORIES,
      Self::WhiteSpace { .. } => &WHITESPACE_CATEGORIES,
    };

    categories.iter()
  }
}
