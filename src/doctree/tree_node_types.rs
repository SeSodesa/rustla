/// ## tree_node_types
/// 
/// A submodule that contains the different tree node types.
/// 
/// Author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi

use super::*;


/// ### TreeNodeType
/// An enumaration of the different possible document node types.
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

  /// #### Attention
  Attention,

  /// #### Attribution
  /// An optional attribution of a `BlockQuote`.
  /// If a `BlockQuote` contains an attribution,
  /// the following node may be a `BlockQuote as well,
  /// but not otherwise.
  Attribution,
  
  /// #### Author
  Author,

  /// #### Authors
  Authors,

  /// #### BlockQuote
  /// Text indented relative to previous text,
  /// without markup indicating the start of
  /// a list or other container nodes.
  /// A block quote may end with an `Attribution`,
  /// which allows placing multiple block quotes
  /// in a sequence.
  BlockQuote,

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
  Caption,

  /// #### Caution
  Caution,

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

  /// #### ColSpec
  ColSpec,

  /// #### Comment
  Comment,

  /// #### Compound
  Compound,

  /// #### Contact
  Contact,

  /// #### Container
  Container,

  /// #### Cooyright
  Copyright,

  /// #### Danger
  Danger,

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

  Directive (DirectiveNode),

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
  /// Contains the (name|absolute path) of the document
  /// as its only field.
  Document{
    doc_name: String
  },

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

  /// #### Error
  Error,

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
  Figure {

    /// ##### body_indent
    /// The indentation of the caption and legend of this `Figure`.
    body_indent: usize,

    /// #### uri
    /// A compulsory image location.
    uri: String,

    // Options
    name:     Option<String>,
    class:    Option<String>,
    alt:      Option<String>,
    height:   Option<String>,
    width:    Option<String>,
    scale:    Option<String>,
    align:    Option<String>,
    target:   Option<String>,
    figwidth: Option<String>,
    figclass: Option<String>
  },

  /// #### Footer
  Footer,

  /// #### Footnote
  /// A foonote citation target. Contains a label and the foornote text itself.
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
  Header,

  /// #### Hint
  Hint,

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

  /// #### Important
  Important,

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
  Legend,

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
  /// An inline math node.
  Math {
    text: String
  },

  /// #### MathBlock
  /// A node for display-style mathematics (LaTeX).
  MathBlock,

  /// #### Note
  Note,

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
  Rubric,

  
  /// #### Section
  /// A section title node, that contains the title text,
  /// in addition to its marker type and (sub)section level.
  Section {
    title_text: String,
    level: usize,
    line_style: SectionLineStyle
  },

  /// #### Sidebar
  Sidebar,

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
  Subscript,

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
    displayed_text: String,
    target_label: String
  },

  /// #### Subtitle
  Subtitle,

  /// #### Superscript
  Superscript,

  /// SystemMessage
  SystemMessage,

  /// #### Table
  Table,

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

  /// #### Tip
  Tip,

  /// #### Title
  Title,

  /// #### TitleReference
  /// A reference to a title.
  TitleReference {
    displayed_text: String,
    target_label: String
  },

  /// #### Topic
  Topic,

  /// #### Transition
  /// A node corresponding to LaTeX's `\hrulefill` command.
  Transition,

  /// #### Version
  Version,


  /// #### Warning
  Warning,


  /// #### Whitespace
  /// Generic whitespace that covers everything from spaces to newlines.
  WhiteSpace{
    text: String
  },
}

use std::collections::HashSet;
use lazy_static::lazy_static;

impl TreeNodeType {

  /// ### node_categories
  /// According to the [reStructuredText DTD](https://docutils.sourceforge.io/docs/ref/doctree.html)
  /// each node type is associated with a set of categories.
  /// This function returns that set for each defined node type.
  pub fn node_categories (&self) {

    match self {
      Self::Document { .. }   => {

      }

      Self::Section { .. }    => {

      }

      Self::Transition {}     => {

      }

      Self::Paragraph { .. } => {

      }

      Self::BulletList { .. } => {

      }

      Self::EnumeratedList { .. } => {

      }

      Self::DefinitionList { .. } => {

      }

      Self::FieldList { .. }  => {

      }

      Self::OptionList { .. } => {

      }

      Self::LiteralBlock { .. } => {

      }

      Self::LineBlock { .. } => {

      }

      Self::BlockQuote { .. } => {

      }

      Self::DoctestBlock{ .. } => {

      }

      Self::Footnote { .. } => {

      }

      Self::Citation { .. } => {

      }

      Self::ExternalHyperlinkTarget { .. } => {

      }

      Self::IndirectHyperlinkTarget { .. } => {

      }

      Self::Directive (dir_type) => {

        match dir_type {

          DirectiveNode::Admonition { variant, .. } => {

            match variant {
              AdmonitionDirective::Attention          => {},
              AdmonitionDirective::Caution            => {},
              AdmonitionDirective::Danger             => {},
              AdmonitionDirective::Error              => {},
              AdmonitionDirective::Hint               => {},
              AdmonitionDirective::Important          => {},
              AdmonitionDirective::Note               => {},
              AdmonitionDirective::Tip                => {},
              AdmonitionDirective::Warning            => {},
              AdmonitionDirective::Admonition { .. }  => {},
            }

          }

          DirectiveNode::Image (node_type) => {

            match node_type {
              ImageDirective::Image { .. }  => {}
              ImageDirective::Figure { .. } => {}
            }
          }
          DirectiveNode::BodyElement (node_type) => {

            match node_type {
              BodyElementDirective::Topic { .. }              => {}
              BodyElementDirective::SideBar { .. }            => {}
              BodyElementDirective::LineBlock { .. }          => {}
              BodyElementDirective::ParsedLiteralBlock { .. } => {}
              BodyElementDirective::Code { .. }               => {}
              BodyElementDirective::Math { .. }               => {}
              BodyElementDirective::Rubric { .. }             => {}
              BodyElementDirective::Epigraph { .. }           => {}
              BodyElementDirective::Highlights { .. }         => {}
              BodyElementDirective::PullQuote { .. }          => {}
              BodyElementDirective::CompoundParagraph { .. }  => {}
              BodyElementDirective::Container { .. }          => {}
            }
          }
          DirectiveNode::Table (node_type) => {

            match node_type {

              TableDirective::Table { .. }      => {}
              TableDirective::CSVTable { .. }   => {}
              TableDirective::ListTable { .. }  => {}
            }
          }
          DirectiveNode::DocumentPart (node_type) => {

            match node_type {

              DocumentPartDirective::TableOfContents { .. }           => {}
              DocumentPartDirective::AutomaticSectionNumbering { .. } => {}
              DocumentPartDirective::DocumentHeader { .. }            => {}
              DocumentPartDirective::DocumentFooter { .. }            => {}
            }
          }
          DirectiveNode::Reference (node_type) => {

            match node_type {

              ReferenceDirective::TargetFootnote { .. } => {}
              ReferenceDirective::Footnote { .. }       => {}
              ReferenceDirective::Citation { .. }       => {}
            }
          }
          DirectiveNode::HTMLspecific (node_type) => {

            match node_type {

              HTMLSpecificDirective::Meta { .. }      => {}
              HTMLSpecificDirective::ImageMap { .. }  => {}
            }
          }
          DirectiveNode::SubstitutionDef (node_type) => {

            match node_type {

              SubstitutionDefDirective::ReplacementText { .. }  => {}
              SubstitutionDefDirective::UnicodeCharCode { .. }  => {}
              SubstitutionDefDirective::Date { .. }             => {}
            }
          }
          DirectiveNode::Miscellaneous (node_type) => {

            match node_type {

              MiscellaneousDirective::Include { .. }                        => {}
              MiscellaneousDirective::RawDataPassthrough { .. }             => {}
              MiscellaneousDirective::Class { .. }                          => {}
              MiscellaneousDirective::CustomInterpretedTextRole { .. }      => {}
              MiscellaneousDirective::DefaultRole { .. }                    => {}
              MiscellaneousDirective::MetadataDocTitle { .. }               => {}
              MiscellaneousDirective::ReStructuredTextTestDirective { .. }  => {}
            }
          }

          DirectiveNode::APlusrSTTools(node_type) => {

            match node_type {

              AplusDirective::GradedQuestionnaire { .. }        => {}
              AplusDirective::FeedbackQuestionnaire { .. }      => {}
              AplusDirective::SubmittableExercise { .. }        => {}
              AplusDirective::LTIExercise { .. }                => {}
              AplusDirective::RoundSettings { .. }              => {}
              AplusDirective::ActiveElementInput { .. }         => {}
              AplusDirective::ActiveElementOutput { .. }        => {}
              AplusDirective::HiddenBlock { .. }                => {}
              AplusDirective::PointOfInterest { .. }            => {}
              AplusDirective::AnnotatedCodeBlock { .. }         => {}
              AplusDirective::CodeBlockWithLineReference { .. } => {}
              AplusDirective::REPLSession { .. }                => {}
              AplusDirective::SubmittableACOSExercise { .. }    => {}
              AplusDirective::HTMLDiv { .. }                    => {}
              AplusDirective::CSSStyledTopic { .. }             => {}
              AplusDirective::Media { .. }                      => {}
            }
          }
        }
      }

      Self::SubstitutionDefinition { .. } => {

      }

      Self::Comment { .. } => {}

      _ => unreachable!("All nodes have a set of related categories.")
    }

    todo!()
  }
}

/// ### NodeCategory
/// 
/// An anumeration of the different kinds of categories a node might belong to.
#[derive(PartialEq, Eq, Hash)]
pub enum NodeCategory {

  Root,

  Titular,

  PreBibliographic,

  Bibliograhic,

  Decorative,

  Structural,

  SubStructural,

  Body,

  SimpleBody,

  CompoundBody,

  General,

  Sequential,

  Admonition,

  Special,

  Invisible,

  Part,

  Inline,

  Referential,

  Targetable,

  Label,
}
