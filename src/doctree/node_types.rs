/// This submodule contains the different
/// document tree node types


/// ### NodeType
/// An enumeration of the different possible document
/// tree node types.
pub enum NodeType {

  // Root element
  Document,

  // Structural elements
  Section,
  Topic,
  Transition,


  // Body elements
  Body,
  
  Paragraph,
  Compound,
  Container,
  BulletList,
  EnumeratedList,
  ListItem,
  DefinitionList,
  DefinitionListItem,
  Term,
  Classifier,
  Definition,
  FieldList,
  Field,
  FieldName,
  FieldBody,

  Option,
  OptionArgument,
  OptionGroup,
  OptionList,
  OptionListItem,
  OptionString,
  Description,
  LiteralBlock,
  DoctestBlock,
  MathBlock,
  LineBlock,
  Line,
  BlockQuote,
  Attribution,
  Attention,
  Caution,
  Danger,
  Error,
  Important,
  Note,
  Tip,
  Hint,
  Warning,
  Admonition,
  Comment,
  SubsstitutionDefinition,
  Target,
  Footnote,
  Citation,
  Label,
  Figure,
  Caption,
  Legend,
  Table,
  TableGroup,
  ColSpec,
  TableHead,
  TableBody,
  TableRow,
  TableEntry,

  // Inline elements
  Emphasis,
  StrongEmphasis,
  Literal,
  Reference,
  FootnoteReference,
  CitationReference,
  SubstitutionReference,
  TitleReference,
  Abbreviation,
  Acronym,
  SuperScript,
  SubScript,
  Math,
  Image,
  Inline,
  Problematic,
  Generated,

}