/// This is the token module used by
/// the ruSTLa Lexer and Parser

mod tests;

use std::fmt;

#[derive(PartialEq)]
/// Token is a token of type `TokenType`
pub struct Token {
  pub t_type: TokenType,
  pub lexeme: String,
  pub begin: usize,
  pub end: usize,
}


impl fmt::Debug for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Token")
      .field("t_type", &self.t_type)
      .field("t_type", &self.lexeme)
      .field("t_type", &self.row)
      .finish()
  }
}

/// Methods for the `Token` type
impl Token {
  pub fn new(t_type: TokenType, lexeme: String, row: usize) -> Token {
    Token{
      t_type: t_type,
      lexeme: lexeme,
      row: row,
    }
  }
}

#[derive(Debug, PartialEq)]
/// TokenType lists the possible `Token` types
pub enum TokenType{
  // For testing
  Test,

  // Closing symbols
  RParenth, RSquareBrack, RWavyBrack, RAngleBrack,
  RSingleQuot, RDoubleQuot, RDoubleAngleBrack,
  Exclamation, Question,

  // Unicode delimiters
  Hyphen, NonBreakingHyphen, FigureDash, EnDash,
  EmDash, NonBreakingSpace,

  // Document root elements
  // ----------------------
  
  // Headings
  EqualsOverlinedHeading,
  DashOverlinedHeading,
  BacktickOverlinedHeading,
  ColonOverlinedHeading,
  SquoteOverlinedHeading,
  DquoteOverlinedHeading,
  TildeOverlinedHeading,
  CaretOverlinedHeading,
  UnderscoreOverlinedHeading,
  AsteriskOverlinedHeading,
  PlusOverlinedHeading,
  HashOverlinedHeading,
  LessOverlinedHeading,
  MoreOverlinedHeading,
  EqualsHeading,
  DashHeading,
  BacktickHeading,
  ColonHeading,
  SquoteHeading,
  DquoteHeading,
  TildeHeading,
  CaretHeading,
  UnderscoreHeading,
  AsteriskHeading,
  PlusHeading,
  HashHeading,
  LessHeading,
  MoreHeading,

  // Lists
  // -----
  UnnumberedList,
  NumberedDotList,
  NumberedLRparList,
  NumberedRparList,
  NoBolAlphaDotList, // Watch out the beginning of line
  AlphaDotList,
  AlphaLRparList,
  AlphaRparList,
  DefinitionList,
  FieldList,

  // Blocks
  // ------
  Paragraph,
  LineBlock,
  LiteralBlock,
  PerLineLiteralBlock,
  SourceDirective,
  Directive,
  ReferenceTarget,
  FootnoteOrCitationTarget, // Lexically the same
  SubstitutionDefinition,
  Comment,
  CodeBlock,

  // Directives
  // ----------
  GeneralDirective,
  Admonition,
  GenericAdmonition,
  Image,
  Figure,
  Topic,
  Sidebar,
  LineBlockDirective,
  ParsedLiteralBlock,
  Code,
  Math,
  Rubic,
  Epigraph,
  Highlights,
  PullQuote,
  CompoundParagraph,
  Containers,
  Table,
  CSVTable,
  ListTable,
  Toc,
  SectNum,
  Header,
  Footer,
  TargetFootnote,
  Meta,
  ReplacementText,
  UnicodeCharacterCode,
  Date,
  Class,

  // Inline formatting
  // -----------------
  Escape,
  InlineReference,
  TargetReference,
  SubstitutionReference,
  Role,
  RoleContent, // content second
  ContentRole, // content first
  StrongEmphasis,
  Emphasis,
  FootnoteOrCitation,
  Hyperlink,
  Interpreted,
  Literal,
  Text,

  // End of file
  EOF,
}
