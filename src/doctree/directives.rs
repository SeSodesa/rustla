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
/// ```rst
/// .. note:: This is a note admonition.
///    This is the second line of the first paragraph.
///
///    - The note contains all indented body elements
///      following.
///    - It includes this bullet list.
/// ```
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
#[derive(Debug)]
pub enum ImageDirective {

  /// #### Image
  /// An "image" is a simple picture:
  /// ```rst
  /// .. image:: picture.png
  /// ```
  /// Inline images can be defined with an "image" directive in a substitution definition
  ///
  /// The URI for the image source file is specified in the directive argument.
  /// As with hyperlink targets, the image URI may begin on the same line as the explicit markup start and target name,
  /// or it may begin in an indented text block immediately following, with no intervening blank lines.
  /// If there are multiple lines in the link block, they are stripped of leading and trailing whitespace and joined together.
  ///
  /// Optionally, the image link block may contain a flat field list, the image options. For example:
  /// ```rst
  /// .. image:: picture.jpeg
  ///    :height: 100px
  ///    :width: 200 px
  ///    :scale: 50 %
  ///    :alt: alternate text
  ///    :align: right
  /// ```
  /// The following options are recognized:
  ///
  /// 1. :alt: text
  ///
  ///     Alternate text: a short description of the image, displayed by applications that cannot display images, or spoken by applications for visually impaired users.
  ///
  /// 2. :height: length
  ///
  ///     The desired height of the image. Used to reserve space or scale the image vertically.
  ///     When the "scale" option is also specified, they are combined. For example, a height of 200px and a scale of 50 is equivalent to a height of 100px with no scale.
  ///
  /// 3. :width: length or percentage of the current line width
  ///
  ///     The width of the image. Used to reserve space or scale the image horizontally. As with "height" above, when the "scale" option is also specified, they are combined.
  ///
  /// 4. :scale: integer percentage (the "%" symbol is optional)
  ///
  ///    The uniform scaling factor of the image. The default is "100 %", i.e. no scaling.
  ///
  /// If no "height" or "width" options are specified, the Python Imaging Library (PIL) may be used to determine them, if it is installed and the image file is available.
  /// align : "top", "middle", "bottom", "left", "center", or "right"
  /// The alignment of the image, equivalent to the HTML <img> tag's deprecated "align" attribute or the corresponding "vertical-align" and "text-align" CSS properties.
  /// The values "top", "middle", and "bottom" control an image's vertical alignment (relative to the text baseline); they are only useful for inline images (substitutions).
  /// The values "left", "center", and "right" control an image's horizontal alignment, allowing the image to float and have the text flow around it.
  /// The specific behavior depends upon the browser or rendering software used.
  /// target : text (URI or reference name)
  /// Makes the image into a hyperlink reference ("clickable"). The option argument may be a URI (relative or absolute), or a reference name with underscore suffix (e.g. `a name`_).
  /// and the common options :class: and :name:.
  Image,

  /// #### Figure
  /// A "figure" consists of image data (including image options), an optional caption (a single paragraph), and an optional legend (arbitrary body elements). For page-based output media,
  /// figures might float to a different position if this helps the page layout.
  /// ```rst
  /// .. figure:: picture.png
  ///    :scale: 50 %
  ///    :alt: map to buried treasure
  ///
  ///    This is the caption of the figure (a simple paragraph).
  ///```
  /// The legend consists of all elements after the caption.  In this
  /// case, the legend consists of this paragraph and the following
  /// table:
  /// ```rst
  /// +-----------------------+-----------------------+
  /// | Symbol                | Meaning               |
  /// +=======================+=======================+
  /// | .. image:: tent.png   | Campground            |
  /// +-----------------------+-----------------------+
  /// | .. image:: waves.png  | Lake                  |
  /// +-----------------------+-----------------------+
  /// | .. image:: peak.png   | Mountain              |
  /// +-----------------------+-----------------------+
  /// ```
  /// There must be blank lines before the caption paragraph and before the legend. To specify a legend without a caption, use an empty comment ("`..`") in place of the caption.
  ///
  /// The "figure" directive supports all of the options of the "image" directive (see image options above). These options (except "align") are passed on to the contained image.
  ///
  /// 1. :align: "left", "center", or "right"
  ///
  ///    The horizontal alignment of the figure, allowing the image to float and have the text flow around it.
  ///    The specific behavior depends upon the browser or rendering software used.
  ///
  /// In addition, the following options are recognized:
  ///
  /// 2. :figwidth: "image", length, or percentage of current line width
  ///
  ///    The width of the figure. Limits the horizontal space used by the figure. A special value of "image" is allowed,
  ///    in which case the included image's actual width is used (requires the Python Imaging Library). If the image file is not found or the required software is unavailable, this option is ignored.
  ///
  /// Sets the "width" attribute of the "figure" doctree element.
  ///
  /// This option does not scale the included image; use the "width" image option for that.
  /// ```rst
  /// +---------------------------+
  /// |        figure             |
  /// |                           |
  /// |<------ figwidth --------->|
  /// |                           |
  /// |  +---------------------+  |
  /// |  |     image           |  |
  /// |  |                     |  |
  /// |  |<--- width --------->|  |
  /// |  +---------------------+  |
  /// |                           |
  /// |The figure's caption should|
  /// |wrap at this width.        |
  /// +---------------------------+
  /// ```
  ///
  /// 3. :figclass: text
  ///
  ///    Set a "classes" attribute value on the figure element. See the class directive below.
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
  /// The directive's sole argument is interpreted as the topic title; the next line must be blank. All subsequent lines make up the topic body, interpreted as body elements. For example:
  /// ```rst
  /// .. topic:: Topic Title
  ///
  ///    Subsequent indented lines comprise
  ///    the body of the topic, and are
  ///    interpreted as body elements.
  /// ```
  Topic,

  /// #### SideBar
  /// Sidebars are like miniature, parallel documents that occur inside other documents, providing related or reference material.
  /// A sidebar is typically offset by a border and "floats" to the side of the page; the document's main text may flow around it. Sidebars can also be likened to super-footnotes;
  /// their content is outside of the flow of the document's main text.
  ///
  /// Sidebars may occur anywhere a section or transition may occur. Body elements (including sidebars) may not contain nested sidebars.
  ///
  /// The directive's sole argument is interpreted as the sidebar title, which may be followed by a subtitle option (see below); the next line must be blank.
  /// All subsequent lines make up the sidebar body, interpreted as body elements. For example:
  /// ```rst
  /// .. sidebar:: Sidebar Title
  ///    :subtitle: Optional Sidebar Subtitle
  ///
  ///    Subsequent indented lines comprise
  ///    the body of the sidebar, and are
  ///    interpreted as body elements.
  /// ```
  /// The following options are recognized:
  ///
  /// 1. subtitle : text
  ///    The sidebar's subtitle.
  ///
  /// and the common options :class: and :name:.
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
  /// For example, here's a classic:
  /// ```rst
  /// "To Ma Own Beloved Lassie: A Poem on her 17th Birthday", by
  /// Ewan McTeagle (for Lassie O'Shea):
  ///
  ///     .. line-block::
  ///
  ///         Lend us a couple of bob till Thursday.
  ///         I'm absolutely skint.
  ///         But I'm expecting a postal order and I can pay you back
  ///             as soon as it comes.
  ///         Love, Ewan.
  /// ```
  LineBlock,

  /// #### ParsedLiteralBlock
  /// Unlike an ordinary literal block, the "parsed-literal" directive constructs a literal block
  /// where the text is parsed for inline markup. It is equivalent to a line block with different
  /// rendering: typically in a typewriter/monospaced typeface, like an ordinary literal block.
  /// Parsed literal blocks are useful for adding hyperlinks to code examples.
  ///
  /// However, care must be taken with the text, because inline markup is recognized and there
  /// is no protection from parsing. Backslash-escapes may be necessary to prevent unintended parsing.
  /// And because the markup characters are removed by the parser, care must also be taken with
  /// vertical alignment. Parsed "ASCII art" is tricky, and extra whitespace may be necessary.
  ///
  /// For example, all the element names in this content model are links:
  /// ``rst`
  /// .. parsed-literal::
  ///
  ///    ( (title_, subtitle_?)?,
  ///      decoration_?,
  ///      (docinfo_, transition_?)?,
  ///      `%structure.model;`_ )
  /// ```
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
  /// The following options are recognized:
  ///
  /// 1. number-lines : [start line number]
  ///
  ///    Precede every line with a line number. The optional argument is the number of the first line (defaut 1).
  ///
  /// and the common options :class: and :name:.
  /// ```rst
  /// Example::
  ///
  ///     The content of the following directive
  ///
  ///     .. code:: python
  ///
  ///       def my_function():
  ///           "just a test"
  ///           print 8/2
  ///
  ///     is parsed and marked up as Python source code.
  /// ```
  Code,

  /// #### Math
  /// The "math" directive inserts blocks with mathematical content (display formulas, equations)
  /// into the document. The input format is LaTeX math syntax[1] with support for Unicode symbols,
  /// for example:
  /// ```rst
  /// .. math::
  ///
  ///    α_t(i) = P(O_1, O_2, … O_t, q_t = S_i λ)
  /// ```
  /// Support is limited to a subset of LaTeX math by the conversion required for many output formats.
  /// For HTML, the math_output configuration setting (or the corresponding --math-output command line option)
  /// select between alternative output formats with different subsets of supported elements.
  /// If a writer does not support math typesetting at all, the content is inserted verbatim.
  /// The supported LaTeX commands include AMS extensions (see, e.g., the Short Math Guide).
  ///
  /// For inline formulas, use the "math" role.
  Math,


  /// #### Rubric
  /// The "rubric" directive inserts a "rubric" element into the document tree. A rubric is like an informal
  /// heading that doesn't correspond to the document's structure.
  Rubric,

  /// #### Epigraph
  /// An epigraph is an apposite (suitable, apt, or pertinent) short inscription, often a quotation or poem,
  /// at the beginning of a document or section.
  ///
  /// The "epigraph" directive produces an "epigraph"-class block quote. For example, this input:
  /// ```rst
  /// .. epigraph::
  ///
  ///    No matter where you go, there you are.
  ///
  ///    -- Buckaroo Banzai
  /// ```
  /// becomes this document tree fragment:
  /// ```rst
  /// <block_quote classes="epigraph">
  ///     <paragraph>
  ///         No matter where you go, there you are.
  ///     <attribution>
  ///         Buckaroo Banzai
  /// ```
  Epigraph,

  /// #### Highlights
  /// Highlights summarize the main points of a document or section, often consisting of a list.
  ///
  /// The "highlights" directive produces a "highlights"-class block quote. See Epigraph above for an analogous example.
  Highlights,

  /// #### PullQuote
  /// A pull-quote is a small selection of text "pulled out and quoted", typically in a larger typeface.
  /// Pull-quotes are used to attract attention, especially in long articles.
  ///
  /// The "pull-quote" directive produces a "pull-quote"-class block quote.
  /// See Epigraph above for an analogous example.
  PullQuote,

  /// #### CompundParagraph
  /// The "compound" directive is used to create a compound paragraph,
  /// which is a single logical paragraph containing multiple physical body elements
  /// such as simple paragraphs,literal blocks, tables, lists, etc.,
  /// instead of directly containing text and inline elements. For example:
  /// ```rst
  /// .. compound::
  ///
  ///    The 'rm' command is very dangerous.  If you are logged
  ///    in as root and enter ::
  ///
  ///      cd /
  ///      rm -rf *
  ///
  ///    you will erase the entire contents of your file system.
  /// ```
  /// In the example above, a literal block is "embedded" within a sentence that begins in one physical paragraph and ends in another.
  ///
  /// ##### Note
  ///
  /// The "compound" directive is not a generic block-level container like HTML's <div> element.
  /// Do not use it only to group a sequence of elements, or you may get unexpected results.
  ///
  /// If you need a generic block-level container, please use the container directive, described below.
  ///
  /// Compound paragraphs are typically rendered as multiple distinct text blocks, with the possibility of variations to emphasize their logical unity:
  ///
  /// If paragraphs are rendered with a first-line indent, only the first physical paragraph of a compound paragraph should have that indent
  /// -- second and further physical paragraphs should omit the indents;
  /// vertical spacing between physical elements may be reduced; and so on.
  CompoundParagraph,

  /// #### Container
  /// The "container" directive surrounds its contents (arbitrary body elements) with a generic block-level "container" element.
  /// Combined with the optional "classes" attribute argument(s), this is an extension mechanism for users & applications.
  /// For example:
  /// ```rst
  /// .. container:: custom
  ///
  ///    This paragraph might be rendered in a custom way.
  /// ```
  /// Parsing the above results in the following pseudo-XML:
  /// ```rst
  /// <container classes="custom">
  ///  <paragraph>
  ///      This paragraph might be rendered in a custom way.
  /// ```
  /// The "container" directive is the equivalent of HTML's <div> element. It may be used to group a sequence of elements for user- or application-specific purposes.
  Container,

}


/// ### TableDirective
/// An enumeration of different table directive types.
#[derive(Debug)]
pub enum TableDirective {

  /// #### Table
  /// The "table" directive is used to associate a title with a table or specify options, e.g.:
  /// ```rst
  /// .. table:: Truth table for "not"
  ///    :widths: auto
  ///
  ///    =====  =====
  ///      A    not A
  ///    =====  =====
  ///    False  True
  ///    True   False
  ///    =====  =====
  /// ```
  /// The following options are recognized:
  ///
  /// 1. align : "left", "center", or "right"
  ///    The horizontal alignment of the table. (New in Docutils 0.13)
  ///
  /// 2. widths : "auto", "grid" or a list of integers
  ///
  ///    A comma- or space-separated list of column widths. The default is the width of the input columns (in characters).
  ///
  ///    The special values "auto" or "grid" may be used by writers to decide whether to delegate the determination of
  ///    column widths to the backend (LaTeX, the HTML browser, ...). See also the table_style configuration option.
  ///
  /// 3. width : length or percentage of the current line width
  ///
  ///    Forces the width of the table to the specified length or percentage of the line width. If omitted, the renderer determines the width of the table based on its contents.
  ///
  /// and the common options :class: and :name:.
  Table,

  /// #### CSVTable
  /// The "csv-table" directive is used to create a table from CSV (comma-separated values) data. CSV is
  /// a common data format generated by spreadsheet applications and commercial databases.
  /// The data may be internal (an integral part of the document) or external (a separate file).
  ///
  /// Example:
  /// ```rst
  /// .. csv-table:: Frozen Delights!
  ///    :header: "Treat", "Quantity", "Description"
  ///    :widths: 15, 10, 30
  ///
  ///    "Albatross", 2.99, "On a stick!"
  ///    "Crunchy Frog", 1.49, "If we took the bones out, it wouldn't be
  ///    crunchy, now would it?"
  ///    "Gannet Ripple", 1.99, "On a stick!"
  /// ```
  /// Block markup and inline markup within cells is supported. Line ends are recognized within cells.
  ///
  /// Working limitations:
  ///
  /// 1. There is no support for checking that the number of columns in each row is the same. However,
  ///    this directive supports CSV generators that do not insert "empty" entries at the end of short rows, by automatically adding empty entries.
  ///
  /// 2. Whitespace delimiters are supported only for external CSV files.
  ///
  /// 3. (1, 2, 3) With Python 2, the valuess for the delimiter, quote, and escape options must be ASCII characters.
  ///    (The csv module does not support Unicode and all non-ASCII characters are encoded as multi-byte utf-8 string). This limitation does not exist under Python 3.
  ///
  /// The following options are recognized:
  ///
  /// 1. widths : integer [, integer...] or "auto"
  ///
  ///     A comma- or space-separated list of relative column widths. The default is equal-width columns (100%/#columns).
  ///
  ///     The special value "auto" may be used by writers to decide whether to delegate the determination of column widths to the backend (LaTeX, the HTML browser, ...).
  ///
  /// 2. width : length or percentage of the current line width
  ///
  ///    Forces the width of the table to the specified length or percentage of the line width.
  ///    If omitted, the renderer determines the width of the table based on its contents.
  ///
  /// 3. header-rows : integer
  ///
  ///    The number of rows of CSV data to use in the table header. Defaults to 0.
  ///
  /// 4. stub-columns : integer
  ///
  ///    The number of table columns to use as stubs (row titles, on the left). Defaults to 0.
  ///
  /// 5. header : CSV data
  ///
  ///    Supplemental data for the table header, added independently of and before any header-rows from the main CSV data. Must use the same CSV format as the main CSV data.
  ///
  /// 6. file : string (newlines removed)
  ///
  ///    The local filesystem path to a CSV data file.
  ///
  /// 7. url : string (whitespace removed)
  ///
  ///    An Internet URL reference to a CSV data file.
  ///
  /// 8. encoding : name of text encoding
  ///
  ///    The text encoding of the external CSV data (file or URL).
  ///    Defaults to the document's encoding (if specified).
  ///
  /// 9. delim : char | "tab" | "space" [2]
  ///
  ///    A one-character string[3] used to separate fields.
  ///    Defaults to , (comma). May be specified as a Unicode code point; see the unicode directive for syntax details.
  ///
  /// 10. quote : char
  ///
  ///     A one-character string[3] used to quote elements containing the delimiter or which start with the quote character. Defaults to " (quote).
  ///     May be specified as a Unicode code point; see the unicode directive for syntax details.
  ///
  /// 11. keepspace : flag
  ///
  ///     Treat whitespace immediately following the delimiter as significant. The default is to ignore such whitespace.
  ///
  /// 12. escape : char
  ///
  ///     A one-character[3] string used to escape the delimiter or quote characters.
  ///     May be specified as a Unicode code point; see the unicode directive for syntax details.
  ///     Used when the delimiter is used in an unquoted field, or when quote characters are used within a field.
  ///     The default is to double-up the character, e.g. "He said, ""Hi!"""
  ///
  /// 13. align : "left", "center", or "right"
  ///
  ///     The horizontal alignment of the table. (New in Docutils 0.13)
  ///
  /// and the common options :class: and :name:.
  CSVTable,

  /// #### ListTable
  /// The "list-table" directive is used to create a table from data in a uniform two-level bullet list.
  /// "Uniform" means that each sublist (second-level list) must contain the same number of list items.
  ///
  /// Example:
  /// ```rst
  /// .. list-table:: Frozen Delights!
  ///    :widths: 15 10 30
  ///    :header-rows: 1
  ///
  ///    * - Treat
  ///      - Quantity
  ///      - Description
  ///    * - Albatross
  ///      - 2.99
  ///      - On a stick!
  ///    * - Crunchy Frog
  ///      - 1.49
  ///      - If we took the bones out, it wouldn't be
  ///        crunchy, now would it?
  ///    * - Gannet Ripple
  ///      - 1.99
  ///      - On a stick!
  /// ```
  /// The following options are recognized:
  ///
  /// 1. widths : integer [integer...] or "auto"
  ///
  ///    A comma- or space-separated list of relative column widths. The default is equal-width columns (100%/#columns).
  ///
  ///    The special value "auto" may be used by writers to decide whether to delegate the determination of column widths to the backend (LaTeX, the HTML browser, ...).
  ///
  /// 2. width : length or percentage of the current line width
  ///    Forces the width of the table to the specified length or percentage of the line width. If omitted, the renderer determines the width of the table based on its contents.
  ///
  /// 3. header-rows : integer
  ///
  ///    The number of rows of list data to use in the table header. Defaults to 0.
  ///
  /// 4. stub-columns : integer
  ///
  ///    The number of table columns to use as stubs (row titles, on the left). Defaults to 0.
  ///
  /// 5. align : "left", "center", or "right"
  ///
  ///    The horizontal alignment of the table. (New in Docutils 0.13)
  ///
  /// and the common options :class: and :name:.
  ListTable
}


/// ### DocumentPartDirective
/// An enumeration of different table directive types.
#[derive(Debug)]
pub enum DocumentPartDirective {

  /// #### TableOfContents
  /// The "contents" directive generates a table of contents (TOC) in a topic.
  /// Topics, and therefore tables of contents, may occur anywhere a section or transition may occur. Body elements and topics may not contain tables of contents.
  ///
  /// Here's the directive in its simplest form:
  /// ```rst
  /// .. contents::
  /// ```
  /// Language-dependent boilerplate text will be used for the title. The English default title text is "Contents".
  ///
  /// An explicit title may be specified:
  /// ```rst
  /// .. contents:: Table of Contents
  /// ```
  /// The title may span lines, although it is not recommended:
  /// ```rst
  /// .. contents:: Here's a very long Table of
  ///    Contents title
  /// ```
  /// Options may be specified for the directive, using a field list:
  /// ```rst
  /// .. contents:: Table of Contents
  ///    :depth: 2
  /// ```
  /// If the default title is to be used, the options field list may begin on the same line as the directive marker:
  /// ```rst
  /// .. contents:: :depth: 2
  /// ```
  /// The following options are recognized:
  ///
  /// 1. depth : integer
  ///
  ///    The number of section levels that are collected in the table of contents. The default is unlimited depth.
  ///
  /// 2. local : flag (empty)
  ///
  ///    Generate a local table of contents. Entries will only include subsections of the section in which the directive is given.
  ///    If no explicit title is given, the table of contents will not be titled.
  ///
  /// 3. backlinks : "entry" or "top" or "none"
  ///
  ///    Generate links from section headers back to the table of contents entries, the table of contents itself, or generate no backlinks.
  ///
  /// 4. class : text
  ///
  ///    Set a "classes" attribute value on the topic element. See the class directive below. 
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
  /// The following options are recognized:
  ///
  /// 1. depth : integer
  ///
  ///    The number of section levels that are numbered by this directive. The default is unlimited depth.
  ///
  /// 2. prefix : string
  ///
  ///    An arbitrary string that is prefixed to the automatically generated section numbers. It may be something like "3.2.",
  ///    which will produce "3.2.1", "3.2.2", "3.2.2.1", and so on. Note that any separating punctuation (in the example, a period, ".") must be explicitly provided. The default is no prefix.
  ///
  /// 3. suffix : string
  ///
  ///    An arbitrary string that is appended to the automatically generated section numbers. The default is no suffix.
  ///
  /// 4. start : integer
  ///
  ///    The value that will be used for the first section number. Combined with prefix, this may be used to force the right numbering for a document split over several source files. The default is 1.
  AutomaticSectionNumbering,

  /// #### DocumentHeader
  /// The "header" and "footer" directives create document decorations, useful for page navigation, notes, time/datestamp, etc. For example:
  /// ``` rst
  /// .. header:: This space for rent.
  /// ```
  /// This will add a paragraph to the document header, which will appear at the top of the generated web page or at the top of every printed page.
  ///
  /// These directives may be used multiple times, cumulatively. There is currently support for only one header and footer.
  ///
  /// ##### Note
  ///
  /// While it is possible to use the "header" and "footer" directives to create navigational elements for web pages, you should be aware that Docutils is meant to be used for document processing,
  /// and that a navigation bar is not typically part of a document.
  ///
  /// Thus, you may soon find Docutils' abilities to be insufficient for these purposes. At that time, you should consider using a documentation generator like Sphinx rather than the "header" and "footer" directives.
  ///
  /// In addition to the use of these directives to populate header and footer content, content may also be added automatically by the processing system.
  /// For example, if certain runtime settings are enabled, the document footer is populated with processing information such as a datestamp, a link to the Docutils website, etc.
  DocumentHeader,

  /// #### DocumentFooter
  /// The "header" and "footer" directives create document decorations, useful for page navigation, notes, time/datestamp, etc. For example:
  /// ``` rst
  /// .. header:: This space for rent.
  /// ```
  /// This will add a paragraph to the document header, which will appear at the top of the generated web page or at the top of every printed page.
  ///
  /// These directives may be used multiple times, cumulatively. There is currently support for only one header and footer.
  ///
  /// ##### Note
  ///
  /// While it is possible to use the "header" and "footer" directives to create navigational elements for web pages,
  /// you should be aware that Docutils is meant to be used for document processing, and that a navigation bar is not typically part of a document.
  ///
  /// Thus, you may soon find Docutils' abilities to be insufficient for these purposes. At that time, you should consider using a documentation generator like Sphinx rather than the "header" and "footer" directives.
  ///
  /// In addition to the use of these directives to populate header and footer content, content may also be added automatically by the processing system.
  /// For example, if certain runtime settings are enabled, the document footer is populated with processing information such as a datestamp, a link to the Docutils website, etc.
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
  TargetFootnote,

  /// #### Footnote
  /// Not implemented in docutils!
  Footnote,

  /// #### Citation
  /// Not implemented in docutils!
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
  /// (interpreted as a single string without inline markup) becomes the contents of the "content" attribute. For example:
  /// ```rst
  /// .. meta::
  ///    :description: The reStructuredText plaintext markup language
  ///    :keywords: plaintext, markup language
  /// ```
  /// This would be converted to the following HTML:
  /// ```html
  /// <meta name="description"
  ///     content="The reStructuredText plaintext markup language">
  /// <meta name="keywords" content="plaintext, markup language">
  /// ```
  /// Support for other META attributes ("http-equiv", "scheme", "lang", "dir") are provided through field arguments, which must be of the form "attr=value":
  /// ```rst
  /// .. meta::
  ///    :description lang=en: An amusing story
  ///    :description lang=fr: Une histoire amusante
  /// ```
  /// And their HTML equivalents:
  /// ```html
  /// <meta name="description" lang="en" content="An amusing story">
  /// <meta name="description" lang="fr" content="Une histoire amusante">
  /// ```
  /// Some META tags use an "http-equiv" attribute instead of the "name" attribute. To specify "http-equiv" META tags, simply omit the name:
  /// ```rst
  /// .. meta::
  ///    :http-equiv=Content-Type: text/html; charset=ISO-8859-1
  /// ```
  /// HTML equivalent:
  /// ```html
  /// <meta http-equiv="Content-Type"
  ///      content="text/html; charset=ISO-8859-1">
  /// ```
  Meta,

  /// #### ImageMap
  /// Not implemented in docutils.
  ImageMap,
}


/// ### SubstitutionDefDirective
/// An enumeration of different macro directive types.
#[derive(Debug)]
pub enum SubstitutionDefDirective {

  /// #### ReplacementText
  /// The "replace" directive is used to indicate replacement text for a substitution reference. It may be used within substitution definitions only. For example, this directive can be used to expand abbreviations:
  /// ```rst
  /// .. |reST| replace:: reStructuredText
  ///
  /// Yes, |reST| is a long word, so I can't blame anyone for wanting to
  /// abbreviate it.
  /// ```
  /// As reStructuredText doesn't support nested inline markup, the only way to create a reference with styled text is to use substitutions with the "replace" directive:
  /// ```rst
  /// I recommend you try |Python|_.
  ///
  /// .. |Python| replace:: Python, *the* best language around
  /// .. _Python: http://www.python.org/
  /// ```
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
  /// Text following " .. " is a comment and is ignored. The spaces between the arguments are ignored and thus do not appear in the output. Hexadecimal codes are case-insensitive.
  ///
  /// For example, the following text:
  /// ```st
  /// Copyright |copy| 2003, |BogusMegaCorp (TM)| |---|
  //// all rights reserved.
  ///
  /// .. |copy| unicode:: 0xA9 .. copyright sign
  /// .. |BogusMegaCorp (TM)| unicode:: BogusMegaCorp U+2122
  /// .. with trademark sign
  /// .. |---| unicode:: U+02014 .. em dash
  /// :trim:
  /// ```
  /// results in:
  /// ```rst
  ///     Copyright © 2003, BogusMegaCorp™—all rights reserved.
  /// ```
  /// The following options are recognized:
  ///
  /// 1. ltrim : flag
  ///
  ///    Whitespace to the left of the substitution reference is removed.
  ///
  /// 2. rtrim : flag
  ///
  ///    Whitespace to the right of the substitution reference is removed.
  ///
  /// 3. trim : flag
  ///    Equivalent to ltrim plus rtrim; whitespace on both sides of the substitution reference is removed.
  UnicodeCharCode,

  /// #### Date
  /// The "date" directive generates the current local date and inserts it into the document as text.
  /// This directive may be used in substitution definitions only.
  ///
  /// The optional directive content is interpreted as the desired date format,
  /// using the same codes as Python's time.strftime function.
  /// The default format is "%Y-%m-%d" (ISO 8601 date), but time fields can also be used. Examples:
  /// ```rst
  /// .. |date| date::
  /// .. |time| date:: %H:%M
  ///
  /// Today's date is |date|.
  ///
  /// This document was generated on |date| at |time|.
  /// ```
  Date,

}


/// ### MiscellaneousDirective
/// An enumeration of different miscellaneous directive types.
#[derive(Debug)]
pub enum MiscellaneousDirective {

  /// #### Include
  ///
  /// ##### !!!WARNING!!!
  /// The "include" directive represents a potential security hole.
  /// It can be disabled with the "file_insertion_enabled" runtime setting.
  ///
  /// The "include" directive reads a text file. The directive argument is the path to the file to be included,
  /// relative to the document containing the directive. Unless the options literal or code are given, the file is
  /// parsed in the current document's context at the point of the directive. For example:
  ///
  /// This first example will be parsed at the document level, and can
  /// thus contain any construct, including section headers.
  /// ```rst
  /// .. include:: inclusion.txt
  ///
  /// Back in the main document:
  ///
  ///     This second example will be parsed in a block quote context.
  ///     Therefore it may only contain body elements.  It may not
  ///     contain section headers.
  ///
  ///     .. include:: inclusion.txt
  /// ```
  /// If an included document fragment contains section structure, the title adornments must match those of the master document.
  ///
  /// Standard data files intended for inclusion in reStructuredText documents are distributed with the Docutils source code,
  /// located in the "docutils" package in the docutils/parsers/rst/include directory. To access these files, use the special
  /// syntax for standard "include" data files, angle brackets around the file name:
  /// ```rst
  /// .. include:: <isonum.txt>
  /// ```
  /// The current set of standard "include" data files consists of sets of substitution definitions. See reStructuredText Standard Definition Files for details.
  ///
  /// The following options are recognized:
  ///
  /// 1. start-line : integer
  ///
  ///    Only the content starting from this line will be included. (As usual in Python, the first line has index 0 and negative values count from the end.)
  ///
  /// 2. end-line : integer
  ///
  ///    Only the content up to (but excluding) this line will be included.
  ///
  /// 3. start-after : text to find in the external data file
  ///
  ///    Only the content after the first occurrence of the specified text will be included.
  ///
  /// 4. end-before : text to find in the external data file
  ///
  ///    Only the content before the first occurrence of the specified text (but after any after text) will be included.
  ///
  /// 5. literal : flag (empty)
  ///
  ///    The entire included text is inserted into the document as a single literal block.
  ///
  /// 6. code : formal language (optional)
  ///
  ///    The argument and the content of the included file are passed to the code directive (useful for program listings). (New in Docutils 0.9)
  ///
  /// 7. number-lines : [start line number]
  ///
  ///    Precede every code line with a line number. The optional argument is the number of the first line (defaut 1). Works only with code or literal. (New in Docutils 0.9)
  ///
  /// 8. encoding : name of text encoding
  ///
  ///    The text encoding of the external data file. Defaults to the document's input_encoding.
  ///
  /// 9. tab-width : integer
  ///
  ///    Number of spaces for hard tab expansion. A negative value prevents expansion of hard tabs. Defaults to the tab_width configuration setting.
  ///
  /// With code or literal the common options :class: and :name: are recognized as well.
  ///
  /// Combining start/end-line and start-after/end-before is possible. The text markers will be searched in the specified lines (further limiting the included content).
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
  /// For example, the following input would be passed untouched by an HTML Writer:
  /// ```rst
  /// .. raw:: html
  ///
  ///    <hr width=50 size=10>
  /// ```
  /// A LaTeX Writer could insert the following raw content into its output stream:
  /// ```rst
  /// .. raw:: latex
  ///
  ///    \setlength{\parindent}{0pt}
  /// ```
  /// Raw data can also be read from an external file, specified in a directive option. In this case, the content block must be empty. For example:
  /// ```rst
  /// .. raw:: html
  ///    :file: inclusion.html
  /// ```
  /// Inline equivalents of the "raw" directive can be defined via custom interpreted text roles derived from the "raw" role.
  ///
  /// The following options are recognized:
  ///
  /// 1. file : string (newlines removed)
  ///
  ///    The local filesystem path of a raw data file to be included.
  ///
  /// 2. url : string (whitespace removed)
  ///
  ///    An Internet URL reference to a raw data file to be included.
  ///
  /// 3. encoding : name of text encoding
  ///
  ///    The text encoding of the external raw data (file or URL). Defaults to the document's encoding (if specified).
  RawDataPassthrough,

  /// #### Class
  ///
  /// The "class" directive sets the "classes" attribute value on its content or on the first immediately following
  /// non-comment element. The directive argument consists of one or more space-separated class names.
  /// The names are transformed to conform to the regular expression [a-z](-?[a-z0-9]+)* (see Identifier Normalization below).
  Class,

  /// #### CustomInterpretedTextRole
  /// The "role" directive dynamically creates a custom interpreted text role and registers it with the parser. This means that after declaring a role like this:
  ///
  /// .. role:: custom
  ///
  /// the document may use the new "custom" role:
  ///
  /// An example of using :custom:`interpreted text`
  ///
  /// This will be parsed into the following document tree fragment:
  /// ```rst
  /// <paragraph>
  ///     An example of using
  ///     <inline classes="custom">
  ///         interpreted text
  /// ```
  /// The role must be declared in a document before it can be used.
  ///
  /// The new role may be based on an existing role, specified as a second argument in parentheses (whitespace optional):
  /// ```rst
  /// .. role:: custom(emphasis)
  ///
  ///    :custom:`text`
  /// ```
  /// The parsed result is as follows:
  /// ```rst
  /// <paragraph>
  ///     <emphasis classes="custom">
  ///         text
  /// ```rst
  /// A special case is the "raw" role: derived roles enable inline raw data pass-through, e.g.:
  /// ```rst
  /// .. role:: raw-role(raw)
  ///    :format: html latex
  ///
  /// :raw-role:`raw text`
  /// ```
  /// If no base role is explicitly specified, a generic custom role is automatically used. Subsequent interpreted text will produce an "inline" element with a "classes" attribute, as in the first example above.
  ///
  /// With most roles, the ":class:" option can be used to set a "classes" attribute that is different from the role name. For example:
  /// ```rst
  /// .. role:: custom
  ///    :class: special
  ///
  /// :custom:`interpreted text`
  /// ```
  /// This is the parsed result:
  /// ```rst
  /// <paragraph>
  ///     <inline classes="special">
  ///         interpreted text
  /// ```
  /// The following option is recognized by the "role" directive for most base roles:
  ///
  /// * class : text
  ///
  ///   Set the "classes" attribute value on the element produced (inline, or element associated with a base class) when the custom interpreted text role is used.
  /// If no directive options are specified, a "class" option with the directive argument (role name) as the value is implied. See the class directive above.
  ///
  /// Specific base roles may support other options and/or directive content. See the reStructuredText Interpreted Text Roles document for details.
  Role,

  /// #### DefaultRole
  DefaultRole,

  /// #### MetadataDocTitle
  MetadataDocTitle
}


/// ### CommonOptionDirective
/// An enumeration of different common option directive types.
#[derive(Debug)]
pub enum CommonOptionDirective {

}
