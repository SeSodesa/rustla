/// ## directives
/// A submodule that contains an enumeration for the different directive types recognized by reStructuredText
/// and associated functions and metods.

use super::*;

/// ### Directive
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
/// An enumeration of different image types
#[derive(Debug)]
pub enum ImageDirective {
  Image,
  Figure,
}


/// ### BodyElementDirective
/// An enumeration of different body element directive
#[derive(Debug)]
pub enum BodyElementDirective {
  Topic,
  SideBar,
  LineBlock, // deprecated
  ParsedLiteralBlock,
  Code,
  Math,
  Rubric,
  Epigraph,
  Highlights,
  PullQuote,
  CompoundParagraph,
  Container,
}


/// ### TableDirective
/// An enumeration of different table directive types.
#[derive(Debug)]
pub enum TableDirective {
  Table,
  CSVTable,
  ListTable
}


/// ### DocumentPartDirective
/// An enumeration of different table directive types.
#[derive(Debug)]
pub enum DocumentPartDirective {
  TableOfContents,
  AutomaticSectionNumbering,
  DocumentHeader,
  DocumentFooter,
}


/// ### ReferenceDirective
/// An enumeration of different reference directive types.
#[derive(Debug)]
pub enum ReferenceDirective {
  TargetFootnote,
  Footnote, // Not implemented in docutils
  Citation  // Not implemented in docutils
}


/// ### HTMLSpecificDirective
/// An enumeration of different HTML-specific directive types.
#[derive(Debug)]
pub enum HTMLSpecificDirective {
  Meta,
  ImageMap,
}


/// ### SubstitutionDefDirective
/// An enumeration of different macro directive types.
#[derive(Debug)]
pub enum SubstitutionDefDirective {
  ReplacementText,
  UnicodeCharCode,
  Date,

}


/// ### MiscellaneousDirective
/// An enumeration of different miscellaneous directive types.
#[derive(Debug)]
pub enum MiscellaneousDirective {
  Include, // !!!WARNING!!! Security hole!
  RawDataPassthrough,
  Class,
  CustomInterpretedTextRole,
  DefaultRole,
  MetadataDocTitle
}


/// ### CommonOptionDirective
/// An enumeration of different common option directive types.
#[derive(Debug)]
pub enum CommonOptionDirective {

}
