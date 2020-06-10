/// This submodule contains the
/// set of possible states the lexer
/// could be in


#[derive(Hash, Debug, PartialEq, Eq)]
/// ### State
/// This enum lists the possible states the
/// lexer could be in. reStructuredText
/// contains only body and inline elements,
/// so we stick with those.
/// 
/// The input in lexed differently in different
/// states. While in Body, new body elements
/// are scanned. While in inline, inline
/// elements are scanned.
/// 
/// If a body element that might contain
/// inline elements is encountered,
/// a transition to Inline state is made.
/// Once the end of the body element is reached,
/// a switch to Body is made.
/// EOF ends lexing.
pub enum State {
  General,
  Body,
    RFC2822Body,
      RFC2822List,
    SpecializedBody,
      BulletList,
      DefinitionList,
      EnumeratedList,
      FieldList,
        ExtensionOptions,
    OptionList,
    LineBlock,
    Explicit,
    SubstitutionDef,
  Text,
    SpecializedText,
      Definition,
      Line,
  QuotedLiteralBlock,

}

