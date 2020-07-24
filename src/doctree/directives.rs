/// ## directives
/// A submodule that contains an enumeration for the different directive types recognized by reStructuredText
/// and associated functions and metods. The documentation found in the comments in this file was taken from
/// https://docutils.sourceforge.io/docs/ref/rst/directives.html

use super::*;

/// ### DirectiveType
/// An enumeration of the different directive types found in reStructuredText and LarST.
#[derive(Debug)]
pub enum DirectiveType {
  Admonition (AdmonitionDirective),
  Image (ImageDirective),
  BodyElement (BodyElementDirective),
  Table (TableDirective),
  DocumentPart (DocumentPartDirective),
  Reference (ReferenceDirective),
  HTMLspecific (HTMLSpecificDirective),
  SubstitutionDef (SubstitutionDefDirective),
  Miscellaneous (MiscellaneousDirective),
  CommonOptions (CommonOptionDirective),
}


/// ### AdmonitionDirective
/// An enumeration of the different admonition types.
/// Admonitions are specially marked "topics" that can
/// appear anywhere an ordinary body element can.
/// They contain arbitrary body elements. Typically,
/// an admonition is rendered as an offset block in a document,
/// sometimes outlined or shaded, with a title matching
/// the admonition type.
///
/// For details, see https://docutils.sourceforge.io/docs/ref/rst/directives.html#admonitions
#[derive(Debug)]
pub enum AdmonitionDirective {
  Attention,
  Caution,
  Danger,
  Error,
  Hint,
  Important,
  Note,
  Tip,
  Warning,
  Admonition,
}




/// ### ImageDirective
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
  Image,

  /// #### Figure
  /// A "figure" consists of image data (including image options), an optional caption (a single paragraph), and an optional legend (arbitrary body elements). For page-based output media,
  /// figures might float to a different position if this helps the page layout.
  ///
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#figure
  Figure,
}


/// ### BodyElementDirective
/// An enumeration of different body element directives.
#[derive(Debug)]
pub enum BodyElementDirective {

  /// #### Topic
  /// A topic is like a block quote with a title, or a self-contained section with no subsections.
  /// Use the "topic" directive to indicate a self-contained idea that is separate from the flow of the document.
  /// Topics may occur anywhere a section or transition may occur. Body elements and topics may not contain nested topics.
  ///
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#topic
  Topic,

  /// #### SideBar
  /// Sidebars are like miniature, parallel documents that occur inside other documents, providing related or reference material.
  /// A sidebar is typically offset by a border and "floats" to the side of the page; the document's main text may flow around it. Sidebars can also be likened to super-footnotes;
  /// their content is outside of the flow of the document's main text.
  ///
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#sidebar
  SideBar,

  /// #### LineBlock
  ///
  /// Deprecated!
  ///
  /// The "line-block" directive constructs an element where line breaks and
  /// initial indentation is significant and inline markup is supported.
  /// It is equivalent to a parsed literal block with different rendering
  /// typically in an ordinary serif typeface instead of a typewriter/monospaced face,
  /// and not automatically indented. (Have the line-block directive begin a block quote
  /// to get an indented line block.) Line blocks are useful for address blocks
  /// and verse (poetry, song lyrics), where the structure of lines is significant.
  ///
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#line-block
  LineBlock,

  /// #### ParsedLiteralBlock
  /// Unlike an ordinary literal block, the "parsed-literal" directive constructs a literal block
  /// where the text is parsed for inline markup. It is equivalent to a line block with different
  /// rendering: typically in a typewriter/monospaced typeface, like an ordinary literal block.
  /// Parsed literal blocks are useful for adding hyperlinks to code examples.
  ///
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#parsed-literal-block
  ParsedLiteralBlock,

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
  Code,

  /// #### Math
  /// The "math" directive inserts blocks with mathematical content (display formulas, equations)
  /// into the document. The input format is subset of LaTeX math syntax with support for Unicode symbols.
  /// For inline formulas, use the "math" role.
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#math
  Math,


  /// #### Rubric
  /// The "rubric" directive inserts a "rubric" element into the document tree. A rubric is like an informal
  /// heading that doesn't correspond to the document's structure.
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#rubric
  Rubric,

  /// #### Epigraph
  /// An epigraph is an apposite (suitable, apt, or pertinent) short inscription, often a quotation or poem,
  /// at the beginning of a document or section.
  ///
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#epigraph
  Epigraph,

  /// #### Highlights
  /// Highlights summarize the main points of a document or section, often consisting of a list.
  ///
  /// The "highlights" directive produces a "highlights"-class block quote.
  /// See Epigraph above for an analogous example.
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#highlights
  Highlights,

  /// #### PullQuote
  /// A pull-quote is a small selection of text "pulled out and quoted", typically in a larger typeface.
  /// Pull-quotes are used to attract attention, especially in long articles.
  ///
  /// The "pull-quote" directive produces a "pull-quote"-class block quote.
  /// See Epigraph above for an analogous example.
  /// 
  /// Details https://docutils.sourceforge.io/docs/ref/rst/directives.html#pull-quote
  PullQuote,

  /// #### CompundParagraph
  /// The "compound" directive is used to create a compound paragraph,
  /// which is a single logical paragraph containing multiple physical body elements
  /// such as simple paragraphs,literal blocks, tables, lists, etc.,
  /// instead of directly containing text and inline elements. For example:
  ///
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#compound-paragraph
  CompoundParagraph,

  /// #### Container
  /// The "container" directive surrounds its contents (arbitrary body elements) with a generic block-level "container" element.
  /// Combined with the optional "classes" attribute argument(s), this is an extension mechanism for users & applications.
  /// The "container" directive is the equivalent of HTML's <div> element. It may be used to group a sequence of elements for user-
  /// or application-specific purposes.
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#container
  Container,

}


/// ### TableDirective
/// An enumeration of different table directive types.
///
/// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#tables
#[derive(Debug)]
pub enum TableDirective {

  /// #### Table
  /// The "table" directive is used to associate a title with a table or specify options.
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#table
  Table,

  /// #### CSVTable
  /// The "csv-table" directive is used to create a table from CSV (comma-separated values) data. CSV is
  /// a common data format generated by spreadsheet applications and commercial databases.
  /// The data may be internal (an integral part of the document) or external (a separate file).
  ///
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#id4
  CSVTable,

  /// #### ListTable
  /// The "list-table" directive is used to create a table from data in a uniform two-level bullet list.
  /// "Uniform" means that each sublist (second-level list) must contain the same number of list items.
  ///
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#list-table
  ListTable
}


/// ### DocumentPartDirective
/// An enumeration of different table directive types.
/// 
/// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#document-parts
#[derive(Debug)]
pub enum DocumentPartDirective {

  /// #### TableOfContents
  /// The "contents" directive generates a table of contents (TOC) in a topic.
  /// Topics, and therefore tables of contents, may occur anywhere a section or transition may occur. Body elements and topics may not contain tables of contents.
  ///
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#table-of-contents
  TableOfContents,

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
  AutomaticSectionNumbering,

  /// #### DocumentHeader
  /// The "header" and "footer" directives create document decorations, useful for page navigation, notes, time/datestamp, etc.
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#document-header-footer
  DocumentHeader,

  /// #### DocumentFooter
  /// The "header" and "footer" directives create document decorations, useful for page navigation, notes, time/datestamp, etc.
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#document-header-footer
  DocumentFooter,
}


/// ### ReferenceDirective
/// An enumeration of different reference directive types.
#[derive(Debug)]
pub enum ReferenceDirective {

  /// #### TargetFootnote
  /// The "target-notes" directive creates a footnote for each external target in the text,
  /// and corresponding footnote references after each reference. For every explicit target (of the form, .. _target name: URL) in the text,
  /// a footnote will be generated containing the visible URL as content.
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#target-footnotes
  TargetFootnote,

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


/// ### HTMLSpecificDirective
/// An enumeration of different HTML-specific directive types.
#[derive(Debug)]
pub enum HTMLSpecificDirective {

  /// #### Meta
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

  /// #### ImageMap
  /// Not implemented in docutils.
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#imagemap
  ImageMap,
}


/// ### SubstitutionDefDirective
/// An enumeration of different macro directive types.
/// 
/// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#directives-for-substitution-definitions
#[derive(Debug)]
pub enum SubstitutionDefDirective {

  /// #### ReplacementText
  /// The "replace" directive is used to indicate replacement text for a substitution reference. It may be used within substitution definitions only. For example, this directive can be used to expand abbreviations:
  ///
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#replacement-text
  ReplacementText,

  /// #### UnicodeCharCode
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

  /// #### Date
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


/// ### MiscellaneousDirective
/// An enumeration of different miscellaneous directive types.
/// 
/// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#miscellaneous
#[derive(Debug)]
pub enum MiscellaneousDirective {

  /// #### Include
  ///
  /// ##### !!!WARNING!!!
  /// The "include" directive represents a potential security hole.
  /// It can be disabled with the "file_insertion_enabled" runtime setting.
  ///
  /// The "include" directive reads a text file.
  /// The directive argument is the path to the file to be included,
  /// relative to the document containing the directive.
  /// Unless the options literal or code are given,
  /// the file is parsed in the current document's context at the point of the directive.
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#including-an-external-document-fragment
  Include,

  /// #### RawDataPassthrough
  ///
  /// ##### Warning
  /// The "raw" directive represents a potential security hole. It can be disabled with the "raw_enabled" or "file_insertion_enabled" runtime settings.
  ///
  /// ##### Caution
  /// The "raw" directive is a stop-gap measure allowing the author to bypass reStructuredText's markup. It is a "power-user" feature that should not be overused or abused.
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
  RawDataPassthrough,

  /// #### Class
  ///
  /// The "class" directive sets the "classes" attribute value on its content or on the first immediately following
  /// non-comment element. The directive argument consists of one or more space-separated class names.
  /// The names are transformed to conform to the regular expression [a-z](-?[a-z0-9]+)* (see Identifier Normalization below).
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#class
  Class,

  /// #### CustomInterpretedTextRole
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
  Role,

  /// #### DefaultRole
  /// 
  /// The "default-role" directive sets the default interpreted text role,
  /// the role that is used for interpreted text without an explicit role.
  /// The "default-role" directive sets the default interpreted text role,
  /// the role that is used for interpreted text without an explicit role.
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#setting-the-default-interpreted-text-role
  DefaultRole,

  /// #### MetadataDocTitle
  /// 
  /// The "title" directive specifies the document title as metadata,
  /// which does not become part of the document body.
  /// It overrides a document-supplied title.
  /// For example, in HTML output the metadata document title appears
  /// in the title bar of the browser window.
  /// 
  /// Details: https://docutils.sourceforge.io/docs/ref/rst/directives.html#metadata-document-title
  MetadataDocTitle
}


/// ### CommonOptionDirective
/// An enumeration of different common option directive types.
#[derive(Debug)]
pub enum CommonOptionDirective {

}
