/// This submodule contains the states and related transitions of
/// the parser state machine.

pub enum State {
  Root,
  Structural,
    SectionLine,
    
  Body,
    Text,
    BulletList,
    EnumeratedList,

    FieldList,
      Field,
    DefinitionList,
      Definition,
    OptionList,
      Option,

  Inline,

}
