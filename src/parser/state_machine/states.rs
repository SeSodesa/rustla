/// This submodule contains the states and related transitions of
/// the parser state machine.


#[derive(Hash, Debug, PartialEq, Eq)]
/// ### State
/// An enumeration of the possible states of `StateMachine`.
/// The indentations present here are an attempt to reflect
/// the class hierarchy found in the docutils rST parser States module:
///   https://sourceforge.net/p/docutils/code/HEAD/tree/trunk/docutils/docutils/parsers/rst/states.py#l202
pub enum State {
  General,
    Body,
      SpecializedBody,
        BulletList,
        DefinitionList,
        EnumeratedList,
        FieldList,
          ExtensionOptions,
        OptionList,
        RFC2822List,
        LineBlock,
        Explicit,
      SubstitutionDefinition,
    Text,
      SpecializedText,
        Definition,
        Line,
    QuotedLiteralBlock,

}
