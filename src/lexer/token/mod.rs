/// This is the token module used by
/// the ruSTLa Lexer and Parser

#[cfg(test)]
mod tests;

use std::fmt;

#[derive(PartialEq, Eq)]
/// Token is a token of type `TokenType`
pub struct Token {
  pub t_type: TokenType,
  pub lexeme: String,
  // pub row: usize,
  // pub col: usize,
  pub begin: usize,
  pub  end: usize,
}


impl fmt::Debug for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Token")
      .field("t_type", &self.t_type)
      .field("lexeme", &self.lexeme)
      // .field("row", &self.row)
      // .field("col", &self.col)
      .field("begin", &self.begin)
      .field("end", &self.end)
      .finish()
  }
}

/// Methods for the `Token` type
impl Token {
  pub fn new(t_type: TokenType, lexeme: String, begin: usize, end:usize) -> Token {
    Token{
      t_type: t_type,
      lexeme: lexeme,
      // row: row,
      // col: col,
      begin: begin,
      end: end
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
  Bullet,

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

  // Whitespace
  // ----------
  BlankLine,
  Indentation,


  // Inline formatting
  // -----------------
  Escape,
  Code,
  InlineReference,
  TargetReference,
  LinkAlias,
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
