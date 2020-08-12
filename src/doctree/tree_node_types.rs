/// ## tree_node_types
/// 
/// A submodule that contains the different tree node types.
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;


/// ### TreeNodeType
/// An enumaration of the different possible document
/// node types.
#[derive(Debug)]
pub enum TreeNodeType {

  /// #### Root
  /// The root node of an reStructuredText document tree.
  /// Contains the (name|absolute path) of the document
  /// as its only field.
  Root{
    doc_name: String
  },

  /// #### EmptyLine
  /// A simple empty line, that contains no actual data.
  /// These can be contained in pretty much any container
  /// node, such as lists or list items, in addition to
  /// existing between body level elements.
  EmptyLine,

  /// #### Section
  /// A section title node, that contains the title text,
  /// in addition to its marker type and (sub)section level.
  Section {

  },

  /// #### Transition
  /// A node corresponding to LaTeX's `\hrulefill` command.
  Transition {

  },

  // Body level elements

  /// #### Paragraph
  /// A node constructed of a left-aligned block of text
  /// with no special starter markers.
  Paragraph {
    indent: usize,
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

  /// #### DefinitionList
  /// A list of definitions. Contains `DefinitionListItems` or `EmptyLine` nodes
  /// as its direct children.
  DefinitionList,

  /// #### DefinitionListItem
  /// A child node type of `DefinitionList`.
  /// Contains a map of `DefinitionTerm`s and the corresponding
  /// `TermDefinitions`, in addition to optional term classifiers.
  DefinitionListItem,

  /// #### DefinitionTerm
  /// The term that is to be defined in a `DefinitionList`.
  DefinitionTerm,

  /// #### Classifier
  /// A classifier for a `DefinitionTerm` in a `DefinitionList`.
  /// Could be the type of a varible in a function decraration, or something similar.
  Classifier,

  /// #### TermDefinition
  /// A definition of a term in a `DefinitionList`.
  TermDefinition,

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

  /// #### FieldBody
  /// The parameter that `FieldName` refers to. May contain arbitrary body elements,
  /// just like bulleted and enumerated list items. The first line after the marker specifies
  /// the indentation used as a reference for parsing the rest of the block.
  FieldBody {
    indentation: usize
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
  OptionList,

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
  OptionListItem,

  /// #### LiteralBlock
  /// Paragraph (possibly empty) ending in a "::" signifies the start of a literal block of text.
  /// Text contained in a literal block is not interpreted in any way,
  /// but simply stored in this node as is.
  LiteralBlock {
    text: String
  },

  /// #### DoctestBlock
  /// These are interactive Python sessions contained in Python docstrings.
  /// Based on the Python standard library [doctest](http://www.python.org/doc/current/lib/module-doctest.html) module.
  /// 
  /// Doctest blocks begin with ">>>", the python REPL main prompt and end with a blank line.
  /// They are a special case of the literal block and if both are present,
  /// the literal block takes precedence.
  DoctestBlock,
  
  /// #### MathBlock
  /// A node for display-style mathematics (LaTeX).
  MathBlock,

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

  /// #### Line
  /// A general line node. Might signify the start of a transtition or a section title.
  Line,
  
  /// #### BlockQuote
  /// Text indented relative to previous text,
  /// without markup indicating the start of
  /// a list or other container nodes.
  /// A block quote may end with an `Attribution`,
  /// which allows placing multiple block quotes
  /// in a sequence.
  BlockQuote,

  /// #### Attribution
  /// An optional attribution of a `BlockQuote`.
  /// If a `BlockQuote` contains an attribution,
  /// the following node may be a `BlockQuote as well,
  /// but not otherwise.
  Attribution,

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

  /// #### Footnote
  /// A foonote citation target. Contains a label and the foornote text itself.
  Footnote {
    body_indent: usize,
    kind: FootnoteKind,
    label: String, // Displayed label
    target: String // Reference target
  },

  /// #### Citation
  /// A generic citation target.
  Citation {
    body_indent: usize,
    label: String,
  },

  /// #### ExternalHyperlinkTarget
  /// A target for an external hyperlink.
  /// Contains a URI pointing  to an external resource
  ExternalHyperlinkTarget {
    marker_indent: usize,
    target: String,
    uri: String,
  },

  /// #### IndirectHyperlinkTarget
  /// An indirect hyperlink target. Contains a hyperlink reference pointing
  /// to an internal or and external hyperlink.
  IndirectHyperlinkTarget {
    marker_indent: usize,
    target: String,
    indirect_target: String,
  },

  /// #### Directive
  /// One of many differents kinds of directives.
  Directive {
    dir_type: DirectiveNode,
  },

  /// #### Comment
  /// An rST comment, that might get removed by the writer of the object code.
  Comment,


  // Inline elements
  // ---------------

  /// #### Text
  /// A plain text node, that contains no special markup.
  Text {
    text:String
  },

  /// #### Emphasis
  /// Emphasised or italicized text.
  Emphasis {
    text: String
  },

  /// #### StrongEmphasis
  /// Strongly emphasised text, usually rendered in bold.
  StrongEmphasis {
    text:String
  },

  /// #### InterpretedText
  /// Text, whose meaning depends entirely on the given `role`:
  /// (:role:`content`|`content`:role:). There are predefined roles
  /// such as `math` or `emphasis`, but others may be defined by applications.
  InterpretedText,

  /// #### Literal
  /// Literal text, usually reserved for code.
  Literal {
    text: String
  },

  /// #### InlineTarget
  /// An inline reference target.
  InlineTarget {
    target_label: String
  },

  /// #### Reference
  /// A general reference to a reference target.
  Reference {
    displayed_text: String,
    target_label: String
  },

  /// #### FootnoteReference
  /// A reference to a foot note.
  FootnoteReference {
    displayed_text: String,
    target_label: String
  },

  /// #### CitationReference
  /// A reference to a bibliographic citation.
  CitationReference {
    displayed_text: String,
    target_label: String
  },

  /// #### SubstitutionReference
  /// A reference that is to be substituted with the reference target directive output.
  SubstitutionReference {
    displayed_text: String,
    target_label: String
  },

  /// #### TitleReference
  /// A reference to a title.
  TitleReference {
    displayed_text: String,
    target_label: String
  },

  /// ### AbsoluteURI
  /// A reference to a web address.
  AbsoluteURI{
    text: String
  },

  /// #### StandaloneEmail
  /// A reference to an email address.
  StandaloneEmail{
    text: String
  },

  /// #### Math
  /// An inline math node.
  Math {
    text: String
  },

  /// #### Whitespace
  /// Generic whitespace that covers everything from spaces to newlines.
  WhiteSpace{
    text: String
  },
}
