/// ## node_categories
/// 
/// A submodule that contains the different categories each node type might belong to.
/// 
/// author: Santtu Söderholm
/// email:  santtu.soderholm@tuni.fi


/// ### NodeCategory
/// 
/// An enumeration of the different kinds of categories a node might belong to.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum NodeCategory {

  /// #### Root
  /// This property only belongs to the document itself.
  Root,

  /// #### Titular
  /// A type of title, such as a section or a topic.
  Titular,

  /// #### PreBibliographic
  /// Category of node which may occur before Bibliographic Nodes.
  PreBibliographic,

  /// #### Bibliographic
  /// The docinfo element is an optional child of document.
  /// It groups bibliographic elements together.
  /// All bibliographic elements except authors and field contain text data.
  /// `Authors` contains further bibliographic elements (most notably author). field contains field_name and field_body body subelements.
  Bibliographic,

  /// #### Decorative
  /// The decoration element is also an optional child of document. It groups together elements used to generate page headers and footers.
  Decorative,

  /// #### Structural
  /// Structural elements may only contain child elements;
  /// they do not directly contain text data.
  /// Structural elements may contain body elements or further structural elements.
  /// Structural elements can only be child elements of other structural elements.
  Structural,

  /// #### CompoundStructural
  /// 
  /// Structural nodes that may have children.
  CompoundStructural,

  /// #### SimpleStructural
  /// 
  /// Structural nodes that may not have children.
  SimpleStructural,

  /// #### SubStructural
  /// Structural subelements are child elements of structural elements.
  /// Simple structuctural subelements (title, subtitle) contain text data;
  /// the others are compound and do not directly contain text data.
  SubStructural,

  /// #### Body
  /// Body elements can be located inside structural elements and compund body elements,
  /// an may contain sub-body elements. For example, bullet lists are located inside sections
  /// and may contain bullet list items.
  Body,

  /// #### SubBody
  /// Compound body elements contain specific subelements (e.g. `BulletList` contains `BulletListItem`s).
  /// Subelements may themselves be compound elements (containing further child elements, like field)
  /// or simple data elements (containing text data, like field_name).
  /// These subelements always occur within specific parent elements,
  /// never at the body element level (beside paragraphs, etc.).
  SubBody,

  /// #### SimpleSubBody
  /// Compound body elements contain specific subelements (e.g. `BulletList` contains `BulletListItem`s).
  /// Subelements may themselves be compound elements (containing further child elements, like field)
  /// or simple data elements (containing text data, like field_name).
  /// These subelements always occur within specific parent elements,
  /// never at the body element level (beside paragraphs, etc.).
  SimpleSubBody,

  /// #### CompoundSubBody
  /// Compound body elements contain specific subelements (e.g. `BulletList` contains `BulletListItem`s).
  /// Subelements may themselves be compound elements (containing further child elements, like field)
  /// or simple data elements (containing text data, like field_name).
  /// These subelements always occur within specific parent elements,
  /// never at the body element level (beside paragraphs, etc.).
  CompoundSubBody,

  /// #### SimpleBody
  /// Simple body elements are empty or directly contain text data.
  /// Those that contain text data may also contain inline elements.
  /// Such elements therefore have a "mixed content model".
  SimpleBody,

  /// #### CompoundBody
  /// Compound body elements contain local substructure (body subelements) and further body elements.
  /// They do not directly contain text data.
  CompoundBody,

  /// #### Sequential
  /// Any node that might be placed in a sequence with similar nodes,
  /// such as bullet list items.
  Sequential,

  /// #### Special
  /// Special internal body elements.
  Special,

  /// #### Inline
  /// Inline elements directly contain text data,
  /// and may also contain further inline elements.
  /// Inline elements are contained within simple body elements.
  /// Most inline elements have a "mixed content model".
  Inline,

  /// #### Referential
  /// Any node that can be referenced.
  Referential,


  /// #### Raw
  /// Raw data that is to be passed untouched to the writer.
  Raw,
}


use std::iter::FromIterator;

// Node category constants

pub const ABBREVIATION_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline
];
pub const ABSOLUTE_URI_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline
];
pub const ACRONYM_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline
];
pub const ADDRESS_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Bibliographic
];
pub const ADMONITION_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::CompoundBody,
];
pub const ATTRIBUTION_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::SimpleSubBody,
];
pub const AUTHOR_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Bibliographic
];
pub const AUTHORS_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Bibliographic
];
pub const AUTO_SECTION_NUMBERING_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline
];
pub const BLOCK_QUOTE_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::CompoundBody,
];
pub const BULLET_LIST_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::CompoundBody,
];
pub const BULLET_LIST_ITEM_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::CompoundSubBody,
];
pub const CAPTION_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::SimpleSubBody,
];
pub const CITATION_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::CompoundBody,
];
pub const CITATION_REFERENCE_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline
];
pub const CLASSIFIER_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::SimpleSubBody,
];
pub const CODE_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::SimpleBody,
];
pub const COLSPEC_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::SimpleSubBody,
];
pub const COMMENT_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::SimpleBody,
];
pub const COMPOUND_PARAGRAPH_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::CompoundBody,
];
pub const CONTACT_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Bibliographic
];
pub const CONTAINER_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::CompoundBody,
];
pub const COPYRIGHT_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Bibliographic
];
pub const CSV_TABLE_CATEGORIES: [NodeCategory; 0] = [

];
pub const DATE_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Bibliographic
];
pub const DECORATION_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::SubStructural,
];
pub const DEFINITION_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::CompoundSubBody,
];
pub const DEFINITION_LIST_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::CompoundBody,
];
pub const DEFINITION_LIST_ITEM_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::CompoundSubBody,
];
pub const DESCRIPTION_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::CompoundSubBody,
];
pub const DOC_INFO_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::SubStructural
];
pub const DOCTEST_BLOCK_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::SimpleBody,
];
pub const DOCUMENT_CATEGORIES: [NodeCategory; 3] = [
  NodeCategory::Root,
  NodeCategory::Structural,
  NodeCategory::CompoundStructural,
];
pub const EMPHASIS_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline
];
pub const EMPTY_LINE_CATEGORIES: [NodeCategory; 6] = [
  NodeCategory::Structural,
  NodeCategory::SimpleStructural,
  NodeCategory::Body,
  NodeCategory::SimpleBody,
  NodeCategory::SubBody,
  NodeCategory::SimpleSubBody,
];
pub const ENTRY_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::CompoundSubBody,
];
pub const ENUMERATED_LIST_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::CompoundBody,
];
pub const ENUMERATED_LIST_ITEM_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::CompoundSubBody,
];
pub const EXTERNAL_HYPERLINK_TARGET_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::SimpleBody,
];
pub const FIELD_CATEGORIES: [NodeCategory; 3] = [
  NodeCategory::Bibliographic,
  NodeCategory::SubBody,
  NodeCategory::CompoundSubBody,
];
pub const FIELD_BODY_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::CompoundSubBody,
];
pub const FIELD_LIST_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::CompoundBody,
];
pub const FIELD_LIST_ITEM_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::CompoundSubBody,
];
pub const FIGURE_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::CompoundBody,
];
pub const FOOTER_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Decorative
];
pub const FOOTNOTE_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::CompoundBody,
];
pub const FOOTNOTE_REFERENCE_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline
];
pub const HEADER_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Decorative
];
pub const GENERATED_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline
];
pub const IMAGE_CATEGORIES: [NodeCategory; 3] = [
  NodeCategory::Body,
  NodeCategory::SimpleBody,
  NodeCategory::Inline,
];
pub const INDIRECT_HYPERLINK_TARGET_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::SimpleBody,
];
pub const INLINE_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline // Isn't this supposed to be one of the categories?
];
pub const INLINE_TARGET_CATEGORIES: [NodeCategory; 0] = [
  // This isn't a node, but turns the following inline node into a ref target.
  // To be removed...
];
pub const INTERPRETED_TEXT_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline,
];
pub const LABEL_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::SimpleSubBody,
];
pub const LEGEND_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::CompoundSubBody,
];
pub const LINE_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::SimpleSubBody,
];
pub const LINE_BLOCK_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::CompoundBody,
];
pub const LIST_TABLE_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::CompoundBody,
];
pub const LITERAL_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline
];
pub const LITERAL_BLOCK_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::SimpleBody,
];
pub const MATH_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline,
];
pub const MATH_BLOCK_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::SimpleBody,
];
pub const OPTION_LIST_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::CompoundBody,
];
pub const OPTION_LIST_ITEM_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::CompoundSubBody,
];
pub const OPTION_STRING_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::SimpleSubBody,
];
pub const ORGANIZATION_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Bibliographic
];
pub const PARAGRAPH_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::SimpleBody,
];
pub const PARSED_LITERAL_BLOCK_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::CompoundBody,
];
pub const PENDING_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::SimpleBody,
];
pub const PROBLEMATIC_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline
];
pub const RAW_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::SimpleBody,
];
pub const REFERENCE_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Inline,
  NodeCategory::Referential,
];
pub const REVISION_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Bibliographic
];
pub const ROW_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::CompoundSubBody,
];
pub const RUBRIC_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::SimpleBody,
];
pub const SECTION_CATEGORIES: [NodeCategory; 3] = [
  NodeCategory::Structural,
  NodeCategory::CompoundStructural,
  NodeCategory::Titular,
];
pub const SIDEBAR_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Structural,
  NodeCategory::CompoundStructural,
];
pub const STATUS_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Bibliographic
];
pub const STANDALONE_EMAIL_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline,
];
pub const STRONG_EMPHASIS_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline,
];
pub const SUBSCRIPT_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline
];
pub const SUBSTITUTION_DEF_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::SimpleBody,
];
pub const SUBSTITUTION_REF_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline
];
pub const SUBTITLE_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::SubStructural,
];
pub const SUPERSCRIPT_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline
];
pub const SYSTEM_MESSAGE_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::CompoundBody,
];
pub const TABLE_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::Body,
  NodeCategory::CompoundBody,
];
pub const TARGET_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline
];
pub const T_BODY_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::CompoundSubBody,
];
pub const TERM_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::SimpleSubBody,
];
pub const TEXT_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline
];
pub const T_GROUP_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::CompoundSubBody,
];
pub const T_HEAD_CATEGORIES: [NodeCategory; 2] = [
  NodeCategory::SubBody,
  NodeCategory::CompoundSubBody,
];
pub const TITLE_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::SubStructural
];
pub const TITLE_REF_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline
];
pub const TOPIC_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Structural
];
pub const TRANSITION_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Structural,
];
pub const VERSION_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Bibliographic
];
pub const WHITESPACE_CATEGORIES: [NodeCategory; 1] = [
  NodeCategory::Inline
];
